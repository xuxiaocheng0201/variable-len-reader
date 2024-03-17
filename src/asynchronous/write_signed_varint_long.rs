macro_rules! write_signed_varint_long_future {
    ($primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_signed_varint_long_future!(f cfg(feature = "async_signed_varint_long"), $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_signed_varint_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
}
macro_rules! write_signed_varint_long_func {
    ($primitive: ty, $future: ident, $func: ident) => {
        write_signed_varint_long_func!(f cfg(feature = "async_signed_varint_long"), $primitive, $future, $func);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $func: ident) => {
        write_signed_varint_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_signed_varint_long_future {
    () => {
        write_signed_varint_long_future!(i8, WriteI8Varint, u8, WriteU8Varint);

        write_signed_varint_long_future!(i16, WriteI16Varint2Le, u16, WriteU16Varint2Le);
        write_signed_varint_long_future!(i16, WriteI16Varint2Be, u16, WriteU16Varint2Be);

        write_signed_varint_long_future!(i32, WriteI32Varint2Le, u32, WriteU32Varint2Le);
        write_signed_varint_long_future!(i32, WriteI32Varint2Be, u32, WriteU32Varint2Be);
        write_signed_varint_long_future!(i32, WriteI32Varint4Le, u32, WriteU32Varint4Le);
        write_signed_varint_long_future!(i32, WriteI32Varint4Be, u32, WriteU32Varint4Be);

        write_signed_varint_long_future!(i64, WriteI64Varint2Le, u64, WriteU64Varint2Le);
        write_signed_varint_long_future!(i64, WriteI64Varint2Be, u64, WriteU64Varint2Be);
        write_signed_varint_long_future!(i64, WriteI64Varint4Le, u64, WriteU64Varint4Le);
        write_signed_varint_long_future!(i64, WriteI64Varint4Be, u64, WriteU64Varint4Be);
        write_signed_varint_long_future!(i64, WriteI64Varint8Le, u64, WriteU64Varint8Le);
        write_signed_varint_long_future!(i64, WriteI64Varint8Be, u64, WriteU64Varint8Be);

        write_signed_varint_long_future!(i128, WriteI128Varint2Le, u128, WriteU128Varint2Le);
        write_signed_varint_long_future!(i128, WriteI128Varint2Be, u128, WriteU128Varint2Be);
        write_signed_varint_long_future!(i128, WriteI128Varint4Le, u128, WriteU128Varint4Le);
        write_signed_varint_long_future!(i128, WriteI128Varint4Be, u128, WriteU128Varint4Be);
        write_signed_varint_long_future!(i128, WriteI128Varint8Le, u128, WriteU128Varint8Le);
        write_signed_varint_long_future!(i128, WriteI128Varint8Be, u128, WriteU128Varint8Be);
        write_signed_varint_long_future!(i128, WriteI128Varint16Le, u128, WriteU128Varint16Le);
        write_signed_varint_long_future!(i128, WriteI128Varint16Be, u128, WriteU128Varint16Be);
    };
}
macro_rules! define_write_signed_varint_long_func {
    () => {
        write_signed_varint_long_func!(i8, WriteI8Varint, write_i8_varint);

        write_signed_varint_long_func!(i16, WriteI16Varint2Le, write_i16_varint_2_le);
        write_signed_varint_long_func!(i16, WriteI16Varint2Be, write_i16_varint_2_be);

        write_signed_varint_long_func!(i32, WriteI32Varint2Le, write_i32_varint_2_le);
        write_signed_varint_long_func!(i32, WriteI32Varint2Be, write_i32_varint_2_be);
        write_signed_varint_long_func!(i32, WriteI32Varint4Le, write_i32_varint_4_le);
        write_signed_varint_long_func!(i32, WriteI32Varint4Be, write_i32_varint_4_be);

        write_signed_varint_long_func!(i64, WriteI64Varint2Le, write_i64_varint_2_le);
        write_signed_varint_long_func!(i64, WriteI64Varint2Be, write_i64_varint_2_be);
        write_signed_varint_long_func!(i64, WriteI64Varint4Le, write_i64_varint_4_le);
        write_signed_varint_long_func!(i64, WriteI64Varint4Be, write_i64_varint_4_be);
        write_signed_varint_long_func!(i64, WriteI64Varint8Le, write_i64_varint_8_le);
        write_signed_varint_long_func!(i64, WriteI64Varint8Be, write_i64_varint_8_be);

        write_signed_varint_long_func!(i128, WriteI128Varint2Le, write_i128_varint_2_le);
        write_signed_varint_long_func!(i128, WriteI128Varint2Be, write_i128_varint_2_be);
        write_signed_varint_long_func!(i128, WriteI128Varint4Le, write_i128_varint_4_le);
        write_signed_varint_long_func!(i128, WriteI128Varint4Be, write_i128_varint_4_be);
        write_signed_varint_long_func!(i128, WriteI128Varint8Le, write_i128_varint_8_le);
        write_signed_varint_long_func!(i128, WriteI128Varint8Be, write_i128_varint_8_be);
        write_signed_varint_long_func!(i128, WriteI128Varint16Le, write_i128_varint_16_le);
        write_signed_varint_long_func!(i128, WriteI128Varint16Be, write_i128_varint_16_be);
    };
}

define_write_signed_varint_long_future!();

#[cfg(all(feature = "async_signed_varint_long", not(feature = "async_varint"), not(feature = "async_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
