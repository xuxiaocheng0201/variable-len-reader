macro_rules! write_float_varint_long {
    ($primitive: ty, $func: ident, $write_internal: ident) => {
        write_float_varint_long!(f cfg(feature = "sync_float_varint_long"), $primitive, $func, $write_internal);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $write_internal: ident) => {
        write_float_varint!(f $feature, $primitive, $func, $write_internal);
    };
}
macro_rules! define_write_float_varint_long {
    () => {
        write_float_varint_long!(f32, write_f32_varint_2_le, write_u32_varint_2_le);
        write_float_varint_long!(f32, write_f32_varint_2_be, write_u32_varint_2_be);
        write_float_varint_long!(f32, write_f32_varint_4_le, write_u32_varint_4_le);
        write_float_varint_long!(f32, write_f32_varint_4_be, write_u32_varint_4_be);

        write_float_varint_long!(f64, write_f64_varint_2_le, write_u64_varint_2_le);
        write_float_varint_long!(f64, write_f64_varint_2_be, write_u64_varint_2_be);
        write_float_varint_long!(f64, write_f64_varint_4_le, write_u64_varint_4_le);
        write_float_varint_long!(f64, write_f64_varint_4_be, write_u64_varint_4_be);
        write_float_varint_long!(f64, write_f64_varint_8_le, write_u64_varint_8_le);
        write_float_varint_long!(f64, write_f64_varint_8_be, write_u64_varint_8_be);
    };
}

#[cfg(all(feature = "sync_float_varint_long", not(feature = "sync_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
