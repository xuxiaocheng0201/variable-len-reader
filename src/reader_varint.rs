macro_rules! read_varint {
    (varint, $primitive: ty, $func: ident, $internal: ty, $read_internal: ident) => {
        read_varint!(cfg(feature = "varint"), $primitive, $func, $internal, $read_internal);
    };
    (long_varint, $primitive: ty, $func: ident, $internal: ty, $read_internal: ident) => {
        read_varint!(cfg(feature = "long_varint"), $primitive, $func, $internal, $read_internal);
    };
    ($feature: meta, $primitive: ty, $func: ident, $internal: ty, $read_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        fn $func(&mut self) -> std::io::Result<$primitive> {
            const SIZE: usize = std::mem::size_of::<$primitive>() << 3; // * 8
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
                    return Err(Error::new(ErrorKind::InvalidData, format!("Varint {} in stream is too long.", stringify!($func))));
                }
            }
            Ok(value)
        }
    };
}
macro_rules! read_varint_size {
    (varint, $func: ident, $read_internal: ident) => {
        read_varint_size!(cfg(feature = "varint_size"), $func, $read_internal);
    };
    (long_varint, $func: ident, $read_internal: ident) => {
        read_varint_size!(cfg(all(feature = "varint_size", feature = "long_varint")), $func, $read_internal);
    };
    ($feature: meta, $func: ident, $read_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> std::io::Result<usize> {
            self.$read_internal().map(|v| v as usize)
        }
    };
}
macro_rules! define_read_varint {
    () => {
        read_varint!(long_varint, u8, read_u8_varint, u8, read_u8_raw);

        read_varint!(varint, u16, read_u16_varint, u8, read_u8_raw);
        read_varint!(long_varint, u16, read_u16_varint_2_le, u16, read_u16_raw_le);
        read_varint!(long_varint, u16, read_u16_varint_2_be, u16, read_u16_raw_be);

        read_varint!(varint, u32, read_u32_varint, u8, read_u8_raw);
        read_varint!(long_varint, u32, read_u32_varint_2_le, u16, read_u16_raw_le);
        read_varint!(long_varint, u32, read_u32_varint_2_be, u16, read_u16_raw_be);
        read_varint!(long_varint, u32, read_u32_varint_4_le, u32, read_u32_raw_le);
        read_varint!(long_varint, u32, read_u32_varint_4_be, u32, read_u32_raw_be);

        read_varint!(varint, u64, read_u64_varint, u8, read_u8_raw);
        read_varint!(long_varint, u64, read_u64_varint_2_le, u16, read_u16_raw_le);
        read_varint!(long_varint, u64, read_u64_varint_2_be, u16, read_u16_raw_be);
        read_varint!(long_varint, u64, read_u64_varint_4_le, u32, read_u32_raw_le);
        read_varint!(long_varint, u64, read_u64_varint_4_be, u32, read_u32_raw_be);
        read_varint!(long_varint, u64, read_u64_varint_8_le, u64, read_u64_raw_le);
        read_varint!(long_varint, u64, read_u64_varint_8_be, u64, read_u64_raw_be);

        read_varint!(varint, u128, read_u128_varint, u8, read_u8_raw);
        read_varint!(long_varint, u128, read_u128_varint_2_le, u16, read_u16_raw_le);
        read_varint!(long_varint, u128, read_u128_varint_2_be, u16, read_u16_raw_be);
        read_varint!(long_varint, u128, read_u128_varint_4_le, u32, read_u32_raw_le);
        read_varint!(long_varint, u128, read_u128_varint_4_be, u32, read_u32_raw_be);
        read_varint!(long_varint, u128, read_u128_varint_8_le, u64, read_u64_raw_le);
        read_varint!(long_varint, u128, read_u128_varint_8_be, u64, read_u64_raw_be);
        read_varint!(long_varint, u128, read_u128_varint_16_le, u128, read_u128_raw_le);
        read_varint!(long_varint, u128, read_u128_varint_16_be, u128, read_u128_raw_be);

        read_varint_size!(varint, read_usize_varint, read_u128_varint);
        read_varint_size!(long_varint, read_usize_varint_2_le, read_u128_varint_2_le);
        read_varint_size!(long_varint, read_usize_varint_2_be, read_u128_varint_2_be);
        read_varint_size!(long_varint, read_usize_varint_4_le, read_u128_varint_4_le);
        read_varint_size!(long_varint, read_usize_varint_4_be, read_u128_varint_4_be);
        read_varint_size!(long_varint, read_usize_varint_8_le, read_u128_varint_8_le);
        read_varint_size!(long_varint, read_usize_varint_8_be, read_u128_varint_8_be);
        read_varint_size!(long_varint, read_usize_varint_16_le, read_u128_varint_16_le);
        read_varint_size!(long_varint, read_usize_varint_16_be, read_u128_varint_16_be);
    };
}
