macro_rules! read_float_varint_long {
    ($primitive: ty, $func: ident, $read_internal: ident) => {
        read_float_varint_long!(f cfg(feature = "sync_float_varint_long"), $primitive, $func, $read_internal);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_float_varint!(f $feature, $primitive, $func, $read_internal);
    };
}
macro_rules! define_read_float_varint_long {
    () => {
        read_float_varint_long!(f32, read_f32_varint_2_le, read_u32_varint_2_le);
        read_float_varint_long!(f32, read_f32_varint_2_be, read_u32_varint_2_be);
        read_float_varint_long!(f32, read_f32_varint_4_le, read_u32_varint_4_le);
        read_float_varint_long!(f32, read_f32_varint_4_be, read_u32_varint_4_be);

        read_float_varint_long!(f64, read_f64_varint_2_le, read_u64_varint_2_le);
        read_float_varint_long!(f64, read_f64_varint_2_be, read_u64_varint_2_be);
        read_float_varint_long!(f64, read_f64_varint_4_le, read_u64_varint_4_le);
        read_float_varint_long!(f64, read_f64_varint_4_be, read_u64_varint_4_be);
        read_float_varint_long!(f64, read_f64_varint_8_le, read_u64_varint_8_le);
        read_float_varint_long!(f64, read_f64_varint_8_be, read_u64_varint_8_be);
    };
}

#[cfg(all(feature = "sync_float_varint_long", not(feature = "sync_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
