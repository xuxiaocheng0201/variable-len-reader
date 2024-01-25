#[cfg(feature = "signed")]
#[cfg_attr(docsrs, doc(cfg(feature = "signed")))]
macro_rules! read_signed {
    ($primitive: ty, $func: ident, $read_internal: ident) => {
        fn $func(&mut self) -> Result<$primitive> {
            use $crate::util::zigzag::Zigzag;
            self.$read_internal().map(|v| v.zigzag())
        }
    };
}
#[cfg(all(feature = "signed", feature = "varint_size"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "signed", feature = "varint_size"))))]
macro_rules! read_signed_size {
    ($func: ident, $read_internal: ident) => {
        #[inline]
        fn $func(&mut self) -> std::io::Result<isize> {
            self.$read_internal().map(|v| v as isize)
        }
    };
}
#[cfg(feature = "signed")]
#[cfg_attr(docsrs, doc(cfg(feature = "signed")))]
macro_rules! define_read_signed {
    () => {
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i8, read_i8_varint, read_u8_varint);

        read_signed!(i16, read_i16_varint, read_u16_varint);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i16, read_i16_varint_2_le, read_u16_varint_2_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i16, read_i16_varint_2_be, read_u16_varint_2_be);

        read_signed!(i32, read_i32_varint, read_u32_varint);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i32, read_i32_varint_2_le, read_u32_varint_2_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i32, read_i32_varint_2_be, read_u32_varint_2_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i32, read_i32_varint_4_le, read_u32_varint_4_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i32, read_i32_varint_4_be, read_u32_varint_4_be);

        read_signed!(i64, read_i64_varint, read_u64_varint);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i64, read_i64_varint_2_le, read_u64_varint_2_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i64, read_i64_varint_2_be, read_u64_varint_2_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i64, read_i64_varint_4_le, read_u64_varint_4_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i64, read_i64_varint_4_be, read_u64_varint_4_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i64, read_i64_varint_8_le, read_u64_varint_8_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i64, read_i64_varint_8_be, read_u64_varint_8_be);

        read_signed!(i128, read_i128_varint, read_u128_varint);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i128, read_i128_varint_2_le, read_u128_varint_2_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i128, read_i128_varint_2_be, read_u128_varint_2_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i128, read_i128_varint_4_le, read_u128_varint_4_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i128, read_i128_varint_4_be, read_u128_varint_4_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i128, read_i128_varint_8_le, read_u128_varint_8_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i128, read_i128_varint_8_be, read_u128_varint_8_be);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i128, read_i128_varint_16_le, read_u128_varint_16_le);
        #[cfg(feature = "long_signed")]
        #[cfg_attr(docsrs, doc(cfg(feature = "long_signed")))]
        read_signed!(i128, read_i128_varint_16_be, read_u128_varint_16_be);

        #[cfg(feature = "varint_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "varint_size")))]
        read_signed_size!(read_isize_varint, read_i128_varint);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        read_signed_size!(read_isize_varint_2_le, read_i128_varint_2_le);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        read_signed_size!(read_isize_varint_2_be, read_i128_varint_2_be);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        read_signed_size!(read_isize_varint_4_le, read_i128_varint_4_le);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        read_signed_size!(read_isize_varint_4_be, read_i128_varint_4_be);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        read_signed_size!(read_isize_varint_8_le, read_i128_varint_8_le);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        read_signed_size!(read_isize_varint_8_be, read_i128_varint_8_be);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        read_signed_size!(read_isize_varint_16_le, read_i128_varint_16_le);
        #[cfg(all(feature = "varint_size", feature = "long_signed"))]
        #[cfg_attr(docsrs, doc(cfg(all(feature = "varint_size", feature = "long_signed"))))]
        read_signed_size!(read_isize_varint_16_be, read_i128_varint_16_be);
    };
}
