macro_rules! write_varint {
    (varint, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        #[cfg(feature = "varint")]
        #[cfg_attr(docsrs, doc(cfg(feature = "varint")))]
        write_varint!($primitive, $func, $internal, $write_internal);
    };
    (long_varint, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        #[cfg(feature = "long_varint")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_varint")))]
        write_varint!($primitive, $func, $internal, $write_internal);
    };
    ($primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        fn $func(&mut self, num: $primitive) -> std::io::Result<usize> {
            const NUM_BITS: $internal = <$internal>::MAX >> 1;
            const SIGN_BIT: $internal = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
            let mut size = 0;
            let mut value = num;
            while value >= SIGN_BIT as $primitive {
                size += self.$write_internal(((value & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT)?;
                value >>= POS_OFFSET;
            }
            size += self.$write_internal(value as $internal)?;
            Ok(size)
        }
    };
}
macro_rules! write_varint_size {
    (varint, $func: ident, $write_internal: ident) => {
        #[cfg(feature = "varint_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "varint_size")))]
        write_varint_size!($func, $write_internal);
    };
    (long_varint, $func: ident, $write_internal: ident) => {
        #[cfg(all(feature = "varint_size", feature = "long_varint"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_varint"))))]
        write_varint_size!($func, $write_internal);
    };
    ($func: ident, $write_internal: ident) => {
        #[inline]
        fn $func(&mut self, num: usize) -> std::io::Result<usize> {
            self.$write_internal(num as u128)
        }
    };
}
macro_rules! define_write_varint {
    () => {
        write_varint!(long_varint, u8, write_u8_varint, u8, write_u8_raw);

        write_varint!(varint, u16, write_u16_varint, u8, write_u8_raw);
        write_varint!(long_varint, u16, write_u16_varint_2_le, u16, write_u16_raw_le);
        write_varint!(long_varint, u16, write_u16_varint_2_be, u16, write_u16_raw_be);

        write_varint!(varint, u32, write_u32_varint, u8, write_u8_raw);
        write_varint!(long_varint, u32, write_u32_varint_2_le, u16, write_u16_raw_le);
        write_varint!(long_varint, u32, write_u32_varint_2_be, u16, write_u16_raw_be);
        write_varint!(long_varint, u32, write_u32_varint_4_le, u32, write_u32_raw_le);
        write_varint!(long_varint, u32, write_u32_varint_4_be, u32, write_u32_raw_be);

        write_varint!(varint, u64, write_u64_varint, u8, write_u8_raw);
        write_varint!(long_varint, u64, write_u64_varint_2_le, u16, write_u16_raw_le);
        write_varint!(long_varint, u64, write_u64_varint_2_be, u16, write_u16_raw_be);
        write_varint!(long_varint, u64, write_u64_varint_4_le, u32, write_u32_raw_le);
        write_varint!(long_varint, u64, write_u64_varint_4_be, u32, write_u32_raw_be);
        write_varint!(long_varint, u64, write_u64_varint_8_le, u64, write_u64_raw_le);
        write_varint!(long_varint, u64, write_u64_varint_8_be, u64, write_u64_raw_be);

        write_varint!(varint, u128, write_u128_varint, u8, write_u8_raw);
        write_varint!(long_varint, u128, write_u128_varint_2_le, u16, write_u16_raw_le);
        write_varint!(long_varint, u128, write_u128_varint_2_be, u16, write_u16_raw_be);
        write_varint!(long_varint, u128, write_u128_varint_4_le, u32, write_u32_raw_le);
        write_varint!(long_varint, u128, write_u128_varint_4_be, u32, write_u32_raw_be);
        write_varint!(long_varint, u128, write_u128_varint_8_le, u64, write_u64_raw_le);
        write_varint!(long_varint, u128, write_u128_varint_8_be, u64, write_u64_raw_be);
        write_varint!(long_varint, u128, write_u128_varint_16_le, u128, write_u128_raw_le);
        write_varint!(long_varint, u128, write_u128_varint_16_be, u128, write_u128_raw_be);

        write_varint_size!(varint, write_usize_varint, write_u128_varint);
        write_varint_size!(long_varint, write_usize_varint_2_le, write_u128_varint_2_le);
        write_varint_size!(long_varint, write_usize_varint_2_be, write_u128_varint_2_be);
        write_varint_size!(long_varint, write_usize_varint_4_le, write_u128_varint_4_le);
        write_varint_size!(long_varint, write_usize_varint_4_be, write_u128_varint_4_be);
        write_varint_size!(long_varint, write_usize_varint_8_le, write_u128_varint_8_le);
        write_varint_size!(long_varint, write_usize_varint_8_be, write_u128_varint_8_be);
        write_varint_size!(long_varint, write_usize_varint_16_le, write_u128_varint_16_le);
        write_varint_size!(long_varint, write_usize_varint_16_be, write_u128_varint_16_be);
    };
}
