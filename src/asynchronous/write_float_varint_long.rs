macro_rules! write_float_varint_long_future {
    ($primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_float_varint_long_future!(f cfg(feature = "async_float_varint_long"), $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_float_varint_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
}
macro_rules! write_float_varint_long_func {
    ($primitive: ty, $future: ident, $func: ident) => {
        write_float_varint_long_func!(f cfg(feature = "async_float_varint_long"), $primitive, $future, $func);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $func: ident) => {
        write_float_varint_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_float_long_future {
    () => {
        write_float_varint_long_future!(f32, WriteF32Varint2Le, u32, WriteU32Varint2Le);
        write_float_varint_long_future!(f32, WriteF32Varint2Be, u32, WriteU32Varint2Be);
        write_float_varint_long_future!(f32, WriteF32Varint4Le, u32, WriteU32Varint4Le);
        write_float_varint_long_future!(f32, WriteF32Varint4Be, u32, WriteU32Varint4Be);

        write_float_varint_long_future!(f64, WriteF64Varint2Le, u64, WriteU64Varint2Le);
        write_float_varint_long_future!(f64, WriteF64Varint2Be, u64, WriteU64Varint2Be);
        write_float_varint_long_future!(f64, WriteF64Varint4Le, u64, WriteU64Varint4Le);
        write_float_varint_long_future!(f64, WriteF64Varint4Be, u64, WriteU64Varint4Be);
        write_float_varint_long_future!(f64, WriteF64Varint8Le, u64, WriteU64Varint8Le);
        write_float_varint_long_future!(f64, WriteF64Varint8Be, u64, WriteU64Varint8Be);
    };
}
macro_rules! define_write_float_long_func {
    () => {
        write_float_varint_long_func!(f32, WriteF32Varint2Le, write_f32_varint_2_le);
        write_float_varint_long_func!(f32, WriteF32Varint2Be, write_f32_varint_2_be);
        write_float_varint_long_func!(f32, WriteF32Varint4Le, write_f32_varint_4_le);
        write_float_varint_long_func!(f32, WriteF32Varint4Be, write_f32_varint_4_be);

        write_float_varint_long_func!(f64, WriteF64Varint2Le, write_f64_varint_2_le);
        write_float_varint_long_func!(f64, WriteF64Varint2Be, write_f64_varint_2_be);
        write_float_varint_long_func!(f64, WriteF64Varint4Le, write_f64_varint_4_le);
        write_float_varint_long_func!(f64, WriteF64Varint4Be, write_f64_varint_4_be);
        write_float_varint_long_func!(f64, WriteF64Varint8Le, write_f64_varint_8_le);
        write_float_varint_long_func!(f64, WriteF64Varint8Be, write_f64_varint_8_be);
    };
}

define_write_float_long_future!();
