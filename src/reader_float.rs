macro_rules! read_float {
    (varint, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_float!(cfg(feature = "float"), $primitive, $func, $read_internal);
    };
    (long_varint, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_float!(cfg(feature = "long_float"), $primitive, $func, $read_internal);
    };
    ($feature: meta, $primitive: ty, $func: ident, $read_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> Result<$primitive> {
            self.$read_internal().map(|v| <$primitive>::from_bits(v))
        }
    };
}
macro_rules! define_read_float {
    () => {
        read_float!(varint, f32, read_f32_varint, read_u32_varint);
        read_float!(long_varint, f32, read_f32_varint_2_le, read_u32_varint_2_le);
        read_float!(long_varint, f32, read_f32_varint_2_be, read_u32_varint_2_be);
        read_float!(long_varint, f32, read_f32_varint_4_le, read_u32_varint_4_le);
        read_float!(long_varint, f32, read_f32_varint_4_be, read_u32_varint_4_be);

        read_float!(varint, f64, read_f64_varint, read_u64_varint);
        read_float!(long_varint, f64, read_f64_varint_2_le, read_u64_varint_2_le);
        read_float!(long_varint, f64, read_f64_varint_2_be, read_u64_varint_2_be);
        read_float!(long_varint, f64, read_f64_varint_4_le, read_u64_varint_4_le);
        read_float!(long_varint, f64, read_f64_varint_4_be, read_u64_varint_4_be);
        read_float!(long_varint, f64, read_f64_varint_8_le, read_u64_varint_8_le);
        read_float!(long_varint, f64, read_f64_varint_8_be, read_u64_varint_8_be);
    };
}
