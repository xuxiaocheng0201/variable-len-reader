macro_rules! write_varint {
    ($primitive: ty, $func: ident) => {
        write_varint!(f cfg(feature = "sync_varint"), $primitive, $func, u8, write_u8_raw);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[allow(arithmetic_overflow)] // Safety: only used internally.
        fn $func(&mut self, value: $primitive) -> ::core::result::Result<(), Self::Error> {
            const NUM_BITS: $internal = <$internal>::MAX >> 1;
            const SIGN_BIT: $internal = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
            let mut value = value;
            while (value & NUM_BITS as $primitive) != value {
                self.$write_internal(((value & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT)?;
                value >>= POS_OFFSET;
            }
            self.$write_internal(value as $internal)?;
            Ok(())
        }
    };
}

macro_rules! define_write_varint {
    () => {
        write_varint!(u16, write_u16_varint);
        write_varint!(u32, write_u32_varint);
        write_varint!(u64, write_u64_varint);
        write_varint!(u128, write_u128_varint);
    };
}

#[cfg(all(feature = "sync_varint", not(feature = "sync_raw")))]
compile_error!("developer error: please check Cargo.toml");
