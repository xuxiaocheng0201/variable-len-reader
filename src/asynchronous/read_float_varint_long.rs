macro_rules! read_float_varint_long_future {
    ($primitive: ty, $future: ident, $inner_future: ident) => {
        read_float_varint_long_future!(f cfg(feature = "async_float_varint_long"), $primitive, $future, $inner_future);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_float_varint_future!(f $feature, $primitive, $future, $inner_future);
    };
}
macro_rules! read_float_varint_long_func {
    ($future: ident, $func: ident, $inner_func: ident) => {
        read_float_varint_long_func!(f cfg(feature = "async_float_varint_long"), $future, $func, $inner_func);
    };
    (f $feature: meta, $future: ident, $func: ident, $inner_func: ident) => {
        read_float_varint_func!(f $feature, $future, $func, $inner_func);
    };
}

macro_rules! define_read_float_varint_long_future {
    () => {
        read_float_varint_long_future!(f32, ReadF32Varint2Le, ReadU32Varint2Le);
        read_float_varint_long_future!(f32, ReadF32Varint2Be, ReadU32Varint2Be);
        read_float_varint_long_future!(f32, ReadF32Varint4Le, ReadU32Varint4Le);
        read_float_varint_long_future!(f32, ReadF32Varint4Be, ReadU32Varint4Be);

        read_float_varint_long_future!(f64, ReadF64Varint2Le, ReadU64Varint2Le);
        read_float_varint_long_future!(f64, ReadF64Varint2Be, ReadU64Varint2Be);
        read_float_varint_long_future!(f64, ReadF64Varint4Le, ReadU64Varint4Le);
        read_float_varint_long_future!(f64, ReadF64Varint4Be, ReadU64Varint4Be);
        read_float_varint_long_future!(f64, ReadF64Varint8Le, ReadU64Varint8Le);
        read_float_varint_long_future!(f64, ReadF64Varint8Be, ReadU64Varint8Be);
    };
}
macro_rules! define_read_float_varint_long_func {
    () => {
        read_float_varint_long_func!(ReadF32Varint2Le, read_f32_varint_2_le, read_u32_varint_2_le);
        read_float_varint_long_func!(ReadF32Varint2Be, read_f32_varint_2_be, read_u32_varint_2_be);
        read_float_varint_long_func!(ReadF32Varint4Le, read_f32_varint_4_le, read_u32_varint_4_le);
        read_float_varint_long_func!(ReadF32Varint4Be, read_f32_varint_4_be, read_u32_varint_4_be);

        read_float_varint_long_func!(ReadF64Varint2Le, read_f64_varint_2_le, read_u64_varint_2_le);
        read_float_varint_long_func!(ReadF64Varint2Be, read_f64_varint_2_be, read_u64_varint_2_be);
        read_float_varint_long_func!(ReadF64Varint4Le, read_f64_varint_4_le, read_u64_varint_4_le);
        read_float_varint_long_func!(ReadF64Varint4Be, read_f64_varint_4_be, read_u64_varint_4_be);
        read_float_varint_long_func!(ReadF64Varint8Le, read_f64_varint_8_le, read_u64_varint_8_le);
        read_float_varint_long_func!(ReadF64Varint8Be, read_f64_varint_8_be, read_u64_varint_8_be);
    };
}

define_read_float_varint_long_future!();

#[cfg(all(feature = "async_float_varint_long", not(feature = "async_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
