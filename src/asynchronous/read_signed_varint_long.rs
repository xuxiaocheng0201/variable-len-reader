macro_rules! read_signed_varint_long_future {
    ($primitive: ty, $future: ident, $inner_future: ident) => {
        read_signed_varint_long_future!(f cfg(feature = "async_signed_varint_long"), $primitive, $future, $inner_future);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_signed_varint_future!(f $feature, $primitive, $future, $inner_future);
    };
}
macro_rules! read_signed_varint_long_func {
    ($future: ident, $func: ident, $inner_func: ident) => {
        read_signed_varint_long_func!(f cfg(feature = "async_signed_varint_long"), $future, $func, $inner_func);
    };
    (f $feature: meta, $future: ident, $func: ident, $inner_func: ident) => {
        read_signed_varint_func!(f $feature, $future, $func, $inner_func);
    };
}

macro_rules! define_read_signed_varint_long_future {
    () => {
        read_signed_varint_long_future!(i8, ReadI8Varint, ReadU8Varint);

        read_signed_varint_long_future!(i16, ReadI16Varint2Le, ReadU16Varint2Le);
        read_signed_varint_long_future!(i16, ReadI16Varint2Be, ReadU16Varint2Be);

        read_signed_varint_long_future!(i32, ReadI32Varint2Le, ReadU32Varint2Le);
        read_signed_varint_long_future!(i32, ReadI32Varint2Be, ReadU32Varint2Be);
        read_signed_varint_long_future!(i32, ReadI32Varint4Le, ReadU32Varint4Le);
        read_signed_varint_long_future!(i32, ReadI32Varint4Be, ReadU32Varint4Be);

        read_signed_varint_long_future!(i64, ReadI64Varint2Le, ReadU64Varint2Le);
        read_signed_varint_long_future!(i64, ReadI64Varint2Be, ReadU64Varint2Be);
        read_signed_varint_long_future!(i64, ReadI64Varint4Le, ReadU64Varint4Le);
        read_signed_varint_long_future!(i64, ReadI64Varint4Be, ReadU64Varint4Be);
        read_signed_varint_long_future!(i64, ReadI64Varint8Le, ReadU64Varint8Le);
        read_signed_varint_long_future!(i64, ReadI64Varint8Be, ReadU64Varint8Be);

        read_signed_varint_long_future!(i128, ReadI128Varint2Le, ReadU128Varint2Le);
        read_signed_varint_long_future!(i128, ReadI128Varint2Be, ReadU128Varint2Be);
        read_signed_varint_long_future!(i128, ReadI128Varint4Le, ReadU128Varint4Le);
        read_signed_varint_long_future!(i128, ReadI128Varint4Be, ReadU128Varint4Be);
        read_signed_varint_long_future!(i128, ReadI128Varint8Le, ReadU128Varint8Le);
        read_signed_varint_long_future!(i128, ReadI128Varint8Be, ReadU128Varint8Be);
        read_signed_varint_long_future!(i128, ReadI128Varint16Le, ReadU128Varint16Le);
        read_signed_varint_long_future!(i128, ReadI128Varint16Be, ReadU128Varint16Be);
    };
}
macro_rules! define_read_signed_varint_long_func {
    () => {
        read_signed_varint_long_func!(ReadI8Varint, read_i8_varint, read_u8_varint);

        read_signed_varint_long_func!(ReadI16Varint2Le, read_i16_varint_2_le, read_u16_varint_2_le);
        read_signed_varint_long_func!(ReadI16Varint2Be, read_i16_varint_2_be, read_u16_varint_2_be);

        read_signed_varint_long_func!(ReadI32Varint2Le, read_i32_varint_2_le, read_u32_varint_2_le);
        read_signed_varint_long_func!(ReadI32Varint2Be, read_i32_varint_2_be, read_u32_varint_2_be);
        read_signed_varint_long_func!(ReadI32Varint4Le, read_i32_varint_4_le, read_u32_varint_4_le);
        read_signed_varint_long_func!(ReadI32Varint4Be, read_i32_varint_4_be, read_u32_varint_4_be);

        read_signed_varint_long_func!(ReadI64Varint2Le, read_i64_varint_2_le, read_u64_varint_2_le);
        read_signed_varint_long_func!(ReadI64Varint2Be, read_i64_varint_2_be, read_u64_varint_2_be);
        read_signed_varint_long_func!(ReadI64Varint4Le, read_i64_varint_4_le, read_u64_varint_4_le);
        read_signed_varint_long_func!(ReadI64Varint4Be, read_i64_varint_4_be, read_u64_varint_4_be);
        read_signed_varint_long_func!(ReadI64Varint8Le, read_i64_varint_8_le, read_u64_varint_8_le);
        read_signed_varint_long_func!(ReadI64Varint8Be, read_i64_varint_8_be, read_u64_varint_8_be);

        read_signed_varint_long_func!(ReadI128Varint2Le, read_i128_varint_2_le, read_u128_varint_2_le);
        read_signed_varint_long_func!(ReadI128Varint2Be, read_i128_varint_2_be, read_u128_varint_2_be);
        read_signed_varint_long_func!(ReadI128Varint4Le, read_i128_varint_4_le, read_u128_varint_4_le);
        read_signed_varint_long_func!(ReadI128Varint4Be, read_i128_varint_4_be, read_u128_varint_4_be);
        read_signed_varint_long_func!(ReadI128Varint8Le, read_i128_varint_8_le, read_u128_varint_8_le);
        read_signed_varint_long_func!(ReadI128Varint8Be, read_i128_varint_8_be, read_u128_varint_8_be);
        read_signed_varint_long_func!(ReadI128Varint16Le, read_i128_varint_16_le, read_u128_varint_16_le);
        read_signed_varint_long_func!(ReadI128Varint16Be, read_i128_varint_16_be, read_u128_varint_16_be);
    };
}

define_read_signed_varint_long_future!();

#[cfg(all(feature = "async_signed_varint_long", not(feature = "async_varint"), not(feature = "async_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
