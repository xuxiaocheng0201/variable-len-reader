macro_rules! write_varint_long_future {
    ($primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_varint_long_future!(f cfg(feature = "async_varint_long"), $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_varint_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
}
macro_rules! write_varint_long_func {
    ($primitive: ty, $future: ident, $func: ident) => {
        write_varint_long_func!(f cfg(feature = "async_varint_long"), $primitive, $future, $func);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $func: ident) => {
        write_varint_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_varint_long_future {
    () => {
        write_varint_long_future!(u8, WriteU8Varint, u8, WriteU8Raw);

        write_varint_long_future!(u16, WriteU16Varint2Le, u16, WriteU16RawLe);
        write_varint_long_future!(u16, WriteU16Varint2Be, u16, WriteU16RawBe);

        write_varint_long_future!(u32, WriteU32Varint2Le, u16, WriteU16RawLe);
        write_varint_long_future!(u32, WriteU32Varint2Be, u16, WriteU16RawBe);
        write_varint_long_future!(u32, WriteU32Varint4Le, u32, WriteU32RawLe);
        write_varint_long_future!(u32, WriteU32Varint4Be, u32, WriteU32RawBe);

        write_varint_long_future!(u64, WriteU64Varint2Le, u16, WriteU16RawLe);
        write_varint_long_future!(u64, WriteU64Varint2Be, u16, WriteU16RawBe);
        write_varint_long_future!(u64, WriteU64Varint4Le, u32, WriteU32RawLe);
        write_varint_long_future!(u64, WriteU64Varint4Be, u32, WriteU32RawBe);
        write_varint_long_future!(u64, WriteU64Varint8Le, u64, WriteU64RawLe);
        write_varint_long_future!(u64, WriteU64Varint8Be, u64, WriteU64RawBe);

        write_varint_long_future!(u128, WriteU128Varint2Le, u16, WriteU16RawLe);
        write_varint_long_future!(u128, WriteU128Varint2Be, u16, WriteU16RawBe);
        write_varint_long_future!(u128, WriteU128Varint4Le, u32, WriteU32RawLe);
        write_varint_long_future!(u128, WriteU128Varint4Be, u32, WriteU32RawBe);
        write_varint_long_future!(u128, WriteU128Varint8Le, u64, WriteU64RawLe);
        write_varint_long_future!(u128, WriteU128Varint8Be, u64, WriteU64RawBe);
        write_varint_long_future!(u128, WriteU128Varint16Le, u128, WriteU128RawLe);
        write_varint_long_future!(u128, WriteU128Varint16Be, u128, WriteU128RawBe);
    };
}
macro_rules! define_write_varint_long_func {
    () => {
        write_varint_long_func!(u8, WriteU8Varint, write_u8_varint);

        write_varint_long_func!(u16, WriteU16Varint2Le, write_u16_varint_2_le);
        write_varint_long_func!(u16, WriteU16Varint2Be, write_u16_varint_2_be);

        write_varint_long_func!(u32, WriteU32Varint2Le, write_u32_varint_2_le);
        write_varint_long_func!(u32, WriteU32Varint2Be, write_u32_varint_2_be);
        write_varint_long_func!(u32, WriteU32Varint4Le, write_u32_varint_4_le);
        write_varint_long_func!(u32, WriteU32Varint4Be, write_u32_varint_4_be);

        write_varint_long_func!(u64, WriteU64Varint2Le, write_u64_varint_2_le);
        write_varint_long_func!(u64, WriteU64Varint2Be, write_u64_varint_2_be);
        write_varint_long_func!(u64, WriteU64Varint4Le, write_u64_varint_4_le);
        write_varint_long_func!(u64, WriteU64Varint4Be, write_u64_varint_4_be);
        write_varint_long_func!(u64, WriteU64Varint8Le, write_u64_varint_8_le);
        write_varint_long_func!(u64, WriteU64Varint8Be, write_u64_varint_8_be);

        write_varint_long_func!(u128, WriteU128Varint2Le, write_u128_varint_2_le);
        write_varint_long_func!(u128, WriteU128Varint2Be, write_u128_varint_2_be);
        write_varint_long_func!(u128, WriteU128Varint4Le, write_u128_varint_4_le);
        write_varint_long_func!(u128, WriteU128Varint4Be, write_u128_varint_4_be);
        write_varint_long_func!(u128, WriteU128Varint8Le, write_u128_varint_8_le);
        write_varint_long_func!(u128, WriteU128Varint8Be, write_u128_varint_8_be);
        write_varint_long_func!(u128, WriteU128Varint16Le, write_u128_varint_16_le);
        write_varint_long_func!(u128, WriteU128Varint16Be, write_u128_varint_16_be);
    };
}

define_write_varint_long_future!();

#[cfg(all(feature = "async_varint_long", not(feature = "async_raw")))]
compile_error!("developer error: please check Cargo.toml");
