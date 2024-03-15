macro_rules! read_signed {
    (varint, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_signed!(cfg(feature = "sync_signed"), $primitive, $func, $read_internal);
    };
    (long_varint, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_signed!(cfg(feature = "sync_long_signed"), $primitive, $func, $read_internal);
    };
    ($feature: meta, $primitive: ty, $func: ident, $read_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> Result<$primitive> {
            use $crate::util::zigzag::Zigzag;
            self.$read_internal().map(|v| v.zigzag())
        }
    };
}
macro_rules! read_signed_size {
    (varint, $func: ident, $read_internal: ident) => {
        read_signed_size!(cfg(all(feature = "sync_varint_size", feature = "sync_signed")), $func, $read_internal);
    };
    (long_varint, $func: ident, $read_internal: ident) => {
        read_signed_size!(cfg(all(feature = "sync_varint_size", feature = "sync_long_signed")), $func, $read_internal);
    };
    ($feature: meta, $func: ident, $read_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> std::io::Result<isize> {
            self.$read_internal().map(|v| v as isize)
        }
    };
}
macro_rules! define_read_signed {
    () => {
        read_signed!(long_varint, i8, read_i8_varint, read_u8_varint);

        read_signed!(varint, i16, read_i16_varint, read_u16_varint);
        read_signed!(long_varint, i16, read_i16_varint_2_le, read_u16_varint_2_le);
        read_signed!(long_varint, i16, read_i16_varint_2_be, read_u16_varint_2_be);

        read_signed!(varint, i32, read_i32_varint, read_u32_varint);
        read_signed!(long_varint, i32, read_i32_varint_2_le, read_u32_varint_2_le);
        read_signed!(long_varint, i32, read_i32_varint_2_be, read_u32_varint_2_be);
        read_signed!(long_varint, i32, read_i32_varint_4_le, read_u32_varint_4_le);
        read_signed!(long_varint, i32, read_i32_varint_4_be, read_u32_varint_4_be);

        read_signed!(varint, i64, read_i64_varint, read_u64_varint);
        read_signed!(long_varint, i64, read_i64_varint_2_le, read_u64_varint_2_le);
        read_signed!(long_varint, i64, read_i64_varint_2_be, read_u64_varint_2_be);
        read_signed!(long_varint, i64, read_i64_varint_4_le, read_u64_varint_4_le);
        read_signed!(long_varint, i64, read_i64_varint_4_be, read_u64_varint_4_be);
        read_signed!(long_varint, i64, read_i64_varint_8_le, read_u64_varint_8_le);
        read_signed!(long_varint, i64, read_i64_varint_8_be, read_u64_varint_8_be);

        read_signed!(varint, i128, read_i128_varint, read_u128_varint);
        read_signed!(long_varint, i128, read_i128_varint_2_le, read_u128_varint_2_le);
        read_signed!(long_varint, i128, read_i128_varint_2_be, read_u128_varint_2_be);
        read_signed!(long_varint, i128, read_i128_varint_4_le, read_u128_varint_4_le);
        read_signed!(long_varint, i128, read_i128_varint_4_be, read_u128_varint_4_be);
        read_signed!(long_varint, i128, read_i128_varint_8_le, read_u128_varint_8_le);
        read_signed!(long_varint, i128, read_i128_varint_8_be, read_u128_varint_8_be);
        read_signed!(long_varint, i128, read_i128_varint_16_le, read_u128_varint_16_le);
        read_signed!(long_varint, i128, read_i128_varint_16_be, read_u128_varint_16_be);

        read_signed_size!(varint, read_isize_varint, read_i128_varint);
        read_signed_size!(long_varint, read_isize_varint_2_le, read_i128_varint_2_le);
        read_signed_size!(long_varint, read_isize_varint_2_be, read_i128_varint_2_be);
        read_signed_size!(long_varint, read_isize_varint_4_le, read_i128_varint_4_le);
        read_signed_size!(long_varint, read_isize_varint_4_be, read_i128_varint_4_be);
        read_signed_size!(long_varint, read_isize_varint_8_le, read_i128_varint_8_le);
        read_signed_size!(long_varint, read_isize_varint_8_be, read_i128_varint_8_be);
        read_signed_size!(long_varint, read_isize_varint_16_le, read_i128_varint_16_le);
        read_signed_size!(long_varint, read_isize_varint_16_be, read_i128_varint_16_be);
    };
}
