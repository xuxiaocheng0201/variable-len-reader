#[cfg(feature = "raw")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw")))]
macro_rules! write_raw {
    ($primitive: ty, $func: ident, $to: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> std::io::Result<usize> {
            self.write_more(&<$primitive>::$to(num))
        }
    };
}
#[cfg(feature = "raw_size")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw_size")))]
macro_rules! write_raw_size {
    ($primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> std::io::Result<usize> {
            self.$write_internal(num as $internal)
        }
    };
}
#[cfg(feature = "raw")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw")))]
macro_rules! define_write_raw {
    () => {
        write_raw!(u8, write_u8_raw, to_ne_bytes);
        write_raw!(i8, write_i8_raw, to_ne_bytes);

        write_raw!(u16, write_u16_raw_le, to_le_bytes);
        write_raw!(u16, write_u16_raw_be, to_be_bytes);
        write_raw!(i16, write_i16_raw_le, to_le_bytes);
        write_raw!(i16, write_i16_raw_be, to_be_bytes);

        write_raw!(u32, write_u32_raw_le, to_le_bytes);
        write_raw!(u32, write_u32_raw_be, to_be_bytes);
        write_raw!(i32, write_i32_raw_le, to_le_bytes);
        write_raw!(i32, write_i32_raw_be, to_be_bytes);

        write_raw!(u64, write_u64_raw_le, to_le_bytes);
        write_raw!(u64, write_u64_raw_be, to_be_bytes);
        write_raw!(i64, write_i64_raw_le, to_le_bytes);
        write_raw!(i64, write_i64_raw_be, to_be_bytes);

        write_raw!(u128, write_u128_raw_le, to_le_bytes);
        write_raw!(u128, write_u128_raw_be, to_be_bytes);
        write_raw!(i128, write_i128_raw_le, to_le_bytes);
        write_raw!(i128, write_i128_raw_be, to_be_bytes);

        #[cfg(feature = "raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "raw_size")))]
        write_raw_size!(usize, write_usize_raw_le, u128, write_u128_raw_le);
        #[cfg(feature = "raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "raw_size")))]
        write_raw_size!(usize, write_usize_raw_be, u128, write_u128_raw_be);
        #[cfg(feature = "raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "raw_size")))]
        write_raw_size!(isize, write_isize_raw_le, i128, write_i128_raw_le);
        #[cfg(feature = "raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "raw_size")))]
        write_raw_size!(isize, write_isize_raw_be, i128, write_i128_raw_be);
    };
}
