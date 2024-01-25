#[cfg(feature = "signed")]
#[cfg_attr(docsrs, doc(cfg(feature = "signed")))]
macro_rules! write_signed {
    ($primitive: ty, $func: ident, $write_internal: ident) => {
        fn $func(&mut self, num: $primitive) -> std::io::Result<usize> {
            use $crate::util::zigzag::Zigzag;
            self.$write_internal(num.zigzag())
        }
    };
}
#[cfg(all(feature = "signed", feature = "varint_size"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "signed", feature = "varint_size"))))]
macro_rules! write_signed_size {
    ($func: ident, $write_internal: ident) => {
        #[inline]
        fn $func(&mut self, num: isize) -> std::io::Result<usize> {
            self.$write_internal(num as i128)
        }
    };
}
#[cfg(feature = "signed")]
#[cfg_attr(docsrs, doc(cfg(feature = "signed")))]
macro_rules! define_write_signed {
    () => {
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i8, write_i8_varint, write_u8_varint);

        write_signed!(i16, write_i16_varint, write_u16_varint);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i16, write_i16_varint_2_le, write_u16_varint_2_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i16, write_i16_varint_2_be, write_u16_varint_2_be);

        write_signed!(i32, write_i32_varint, write_u32_varint);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i32, write_i32_varint_2_le, write_u32_varint_2_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i32, write_i32_varint_2_be, write_u32_varint_2_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
                write_signed!(i32, write_i32_varint_4_le, write_u32_varint_4_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i32, write_i32_varint_4_be, write_u32_varint_4_be);

        write_signed!(i64, write_i64_varint, write_u64_varint);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i64, write_i64_varint_2_le, write_u64_varint_2_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i64, write_i64_varint_2_be, write_u64_varint_2_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i64, write_i64_varint_4_le, write_u64_varint_4_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i64, write_i64_varint_4_be, write_u64_varint_4_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i64, write_i64_varint_8_le, write_u64_varint_8_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i64, write_i64_varint_8_be, write_u64_varint_8_be);

        write_signed!(i128, write_i128_varint, write_u128_varint);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i128, write_i128_varint_2_le, write_u128_varint_2_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i128, write_i128_varint_2_be, write_u128_varint_2_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i128, write_i128_varint_4_le, write_u128_varint_4_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i128, write_i128_varint_4_be, write_u128_varint_4_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i128, write_i128_varint_8_le, write_u128_varint_8_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i128, write_i128_varint_8_be, write_u128_varint_8_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i128, write_i128_varint_16_le, write_u128_varint_16_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        write_signed!(i128, write_i128_varint_16_be, write_u128_varint_16_be);

        #[cfg(feature = "varint_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "varint_size")))]
        write_signed_size!(write_isize_varint, write_i128_varint);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        write_signed_size!(write_isize_varint_2_le, write_i128_varint_2_le);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        write_signed_size!(write_isize_varint_2_be, write_i128_varint_2_be);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        write_signed_size!(write_isize_varint_4_le, write_i128_varint_4_le);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        write_signed_size!(write_isize_varint_4_be, write_i128_varint_4_be);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        write_signed_size!(write_isize_varint_8_le, write_i128_varint_8_le);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        write_signed_size!(write_isize_varint_8_be, write_i128_varint_8_be);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        write_signed_size!(write_isize_varint_16_le, write_i128_varint_16_le);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        write_signed_size!(write_isize_varint_16_be, write_i128_varint_16_be);
    };
}
