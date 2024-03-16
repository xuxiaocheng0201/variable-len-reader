macro_rules! read_varint {
    ($primitive: ty, $func: ident, $internal: ty, $read_internal: ident) => {
        read_varint!(f cfg(feature = "sync_varint"), $primitive, $func, $internal, $read_internal);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $internal: ty, $read_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        fn $func(&mut self) -> ::core::result::Result<$primitive, Self::Error> {
            const SIZE: usize = ::core::mem::size_of::<$primitive>() << 3; // * 8
            const NUM_BITS: $internal = <$internal>::MAX >> 1;
            const SIGN_BIT: $internal = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
            let mut value = 0;
            let mut position = 0;
            loop {
                let current = self.$read_internal()?;
                value |= ((current & NUM_BITS) as $primitive) << position;
                if current & SIGN_BIT == 0 {
                    break;
                }
                position += POS_OFFSET;
                if position >= SIZE {
                    return Err(Self::read_varint_error(stringify!($func), current as u128));
                }
            }
            Ok(value)
        }
    };
}
macro_rules! define_read_varint {
    () => {
        #[cfg(feature = "sync_varint")]
        #[cfg_attr(docsrs, doc(cfg(feature = "sync_varint")))]
        fn read_varint_error(func_name: &'static str, current: u128) -> Self::Error;

        read_varint!(u16, read_u16_varint, u8, read_u8_raw);
        read_varint!(u32, read_u32_varint, u8, read_u8_raw);
        read_varint!(u64, read_u64_varint, u8, read_u8_raw);
        read_varint!(u128, read_u128_varint, u8, read_u8_raw);
    };
}

#[cfg(all(feature = "sync_varint", not(feature = "sync_raw")))]
compile_error!("developer error: please check Cargo.toml");
