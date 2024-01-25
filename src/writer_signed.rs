macro_rules! write_signed {
    (varint, $primitive: ty, $func: ident, $write_internal: ident) => {
        #[cfg(feature = "signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "signed")))]
        write_signed!($primitive, $func, $write_internal);
    };
    (long_varint, $primitive: ty, $func: ident, $write_internal: ident) => {
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!($primitive, $func, $write_internal);
    };
    ($primitive: ty, $func: ident, $write_internal: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> std::io::Result<usize> {
            use $crate::util::zigzag::Zigzag;
            self.$write_internal(num.zigzag())
        }
    };
}
macro_rules! write_signed_size {
    (varint, $func: ident, $write_internal: ident) => {
        #[cfg(all(feature = "signed", feature = "varint_size"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "signed", feature = "varint_size"))))]
        write_signed_size!($func, $write_internal);
    };
    (long_varint, $func: ident, $write_internal: ident) => {
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        write_signed_size!($func, $write_internal);
    };
    ($func: ident, $write_internal: ident) => {
        #[inline]
        fn $func(&mut self, num: isize) -> std::io::Result<usize> {
            self.$write_internal(num as i128)
        }
    };
}
macro_rules! define_write_signed {
    () => {
        write_signed!(long_varint, i8, write_i8_varint, write_u8_varint);

        write_signed!(varint, i16, write_i16_varint, write_u16_varint);
        write_signed!(long_varint, i16, write_i16_varint_2_le, write_u16_varint_2_le);
        write_signed!(long_varint, i16, write_i16_varint_2_be, write_u16_varint_2_be);

        write_signed!(varint, i32, write_i32_varint, write_u32_varint);
        write_signed!(long_varint, i32, write_i32_varint_2_le, write_u32_varint_2_le);
        write_signed!(long_varint, i32, write_i32_varint_2_be, write_u32_varint_2_be);
        write_signed!(long_varint, i32, write_i32_varint_4_le, write_u32_varint_4_le);
        write_signed!(long_varint, i32, write_i32_varint_4_be, write_u32_varint_4_be);

        write_signed!(varint, i64, write_i64_varint, write_u64_varint);
        write_signed!(long_varint, i64, write_i64_varint_2_le, write_u64_varint_2_le);
        write_signed!(long_varint, i64, write_i64_varint_2_be, write_u64_varint_2_be);
        write_signed!(long_varint, i64, write_i64_varint_4_le, write_u64_varint_4_le);
        write_signed!(long_varint, i64, write_i64_varint_4_be, write_u64_varint_4_be);
        write_signed!(long_varint, i64, write_i64_varint_8_le, write_u64_varint_8_le);
        write_signed!(long_varint, i64, write_i64_varint_8_be, write_u64_varint_8_be);

        write_signed!(varint, i128, write_i128_varint, write_u128_varint);
        write_signed!(long_varint, i128, write_i128_varint_2_le, write_u128_varint_2_le);
        write_signed!(long_varint, i128, write_i128_varint_2_be, write_u128_varint_2_be);
        write_signed!(long_varint, i128, write_i128_varint_4_le, write_u128_varint_4_le);
        write_signed!(long_varint, i128, write_i128_varint_4_be, write_u128_varint_4_be);
        write_signed!(long_varint, i128, write_i128_varint_8_le, write_u128_varint_8_le);
        write_signed!(long_varint, i128, write_i128_varint_8_be, write_u128_varint_8_be);
        write_signed!(long_varint, i128, write_i128_varint_16_le, write_u128_varint_16_le);
        write_signed!(long_varint, i128, write_i128_varint_16_be, write_u128_varint_16_be);

        write_signed_size!(varint, write_isize_varint, write_i128_varint);
        write_signed_size!(long_varint, write_isize_varint_2_le, write_i128_varint_2_le);
        write_signed_size!(long_varint, write_isize_varint_2_be, write_i128_varint_2_be);
        write_signed_size!(long_varint, write_isize_varint_4_le, write_i128_varint_4_le);
        write_signed_size!(long_varint, write_isize_varint_4_be, write_i128_varint_4_be);
        write_signed_size!(long_varint, write_isize_varint_8_le, write_i128_varint_8_le);
        write_signed_size!(long_varint, write_isize_varint_8_be, write_i128_varint_8_be);
        write_signed_size!(long_varint, write_isize_varint_16_le, write_i128_varint_16_le);
        write_signed_size!(long_varint, write_isize_varint_16_be, write_i128_varint_16_be);
    };
}
