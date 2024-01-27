macro_rules! write_float {
    (varint, $primitive: ty, $func: ident, $write_internal: ident) => {
        write_float!(cfg(feature = "float"), $primitive, $func, $write_internal);
    };
    (long_varint, $primitive: ty, $func: ident, $write_internal: ident) => {
        write_float!(cfg(feature = "long_float"), $primitive, $func, $write_internal);
    };
    ($feature: meta, $primitive: ty, $func: ident, $write_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self, num: $primitive) -> std::io::Result<usize> {
            self.$write_internal(num.to_bits())
        }
    };
}
macro_rules! define_write_float {
    () => {
        write_float!(varint, f32, write_f32_varint, write_u32_varint);
        write_float!(long_varint, f32, write_f32_varint_2_le, write_u32_varint_2_le);
        write_float!(long_varint, f32, write_f32_varint_2_be, write_u32_varint_2_be);
        write_float!(long_varint, f32, write_f32_varint_4_le, write_u32_varint_4_le);
        write_float!(long_varint, f32, write_f32_varint_4_be, write_u32_varint_4_be);

        write_float!(varint, f64, write_f64_varint, write_u64_varint);
        write_float!(long_varint, f64, write_f64_varint_2_le, write_u64_varint_2_le);
        write_float!(long_varint, f64, write_f64_varint_2_be, write_u64_varint_2_be);
        write_float!(long_varint, f64, write_f64_varint_4_le, write_u64_varint_4_le);
        write_float!(long_varint, f64, write_f64_varint_4_be, write_u64_varint_4_be);
        write_float!(long_varint, f64, write_f64_varint_8_le, write_u64_varint_8_le);
        write_float!(long_varint, f64, write_f64_varint_8_be, write_u64_varint_8_be);
    };
}
