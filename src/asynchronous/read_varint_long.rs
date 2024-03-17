macro_rules! read_varint_long_future {
    ($primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        read_varint_long_future!(f cfg(feature = "async_varint_long"), $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        read_varint_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
}
macro_rules! read_varint_long_func {
    ($future: ident, $func: ident, $inner_func: ident) => {
        read_varint_long_func!(f cfg(feature = "async_varint_long"), $future, $func, $inner_func);
    };
    (f $feature: meta, $future: ident, $func: ident, $inner_func: ident) => {
        read_varint_func!(f $feature, $future, $func, $inner_func);
    };
}

macro_rules! define_read_varint_long_future {
    () => {
        read_varint_long_future!(u8, ReadU8Varint, u8, ReadU8Raw);

        read_varint_long_future!(u16, ReadU16Varint2Le, u16, ReadU16RawLe);
        read_varint_long_future!(u16, ReadU16Varint2Be, u16, ReadU16RawBe);

        read_varint_long_future!(u32, ReadU32Varint2Le, u16, ReadU16RawLe);
        read_varint_long_future!(u32, ReadU32Varint2Be, u16, ReadU16RawBe);
        read_varint_long_future!(u32, ReadU32Varint4Le, u32, ReadU32RawLe);
        read_varint_long_future!(u32, ReadU32Varint4Be, u32, ReadU32RawBe);

        read_varint_long_future!(u64, ReadU64Varint2Le, u16, ReadU16RawLe);
        read_varint_long_future!(u64, ReadU64Varint2Be, u16, ReadU16RawBe);
        read_varint_long_future!(u64, ReadU64Varint4Le, u32, ReadU32RawLe);
        read_varint_long_future!(u64, ReadU64Varint4Be, u32, ReadU32RawBe);
        read_varint_long_future!(u64, ReadU64Varint8Le, u64, ReadU64RawLe);
        read_varint_long_future!(u64, ReadU64Varint8Be, u64, ReadU64RawBe);

        read_varint_long_future!(u128, ReadU128Varint2Le, u16, ReadU16RawLe);
        read_varint_long_future!(u128, ReadU128Varint2Be, u16, ReadU16RawBe);
        read_varint_long_future!(u128, ReadU128Varint4Le, u32, ReadU32RawLe);
        read_varint_long_future!(u128, ReadU128Varint4Be, u32, ReadU32RawBe);
        read_varint_long_future!(u128, ReadU128Varint8Le, u64, ReadU64RawLe);
        read_varint_long_future!(u128, ReadU128Varint8Be, u64, ReadU64RawBe);
        read_varint_long_future!(u128, ReadU128Varint16Le, u128, ReadU128RawLe);
        read_varint_long_future!(u128, ReadU128Varint16Be, u128, ReadU128RawBe);
    };
}
macro_rules! define_read_varint_long_func {
    () => {
        read_varint_long_func!(ReadU8Varint, read_u8_varint, read_u8_raw);

        read_varint_long_func!(ReadU16Varint2Le, read_u16_varint_2_le, read_u16_raw_le);
        read_varint_long_func!(ReadU16Varint2Be, read_u16_varint_2_be, read_u16_raw_be);

        read_varint_long_func!(ReadU32Varint2Le, read_u32_varint_2_le, read_u16_raw_le);
        read_varint_long_func!(ReadU32Varint2Be, read_u32_varint_2_be, read_u16_raw_be);
        read_varint_long_func!(ReadU32Varint4Le, read_u32_varint_4_le, read_u32_raw_le);
        read_varint_long_func!(ReadU32Varint4Be, read_u32_varint_4_be, read_u32_raw_be);

        read_varint_long_func!(ReadU64Varint2Le, read_u64_varint_2_le, read_u16_raw_le);
        read_varint_long_func!(ReadU64Varint2Be, read_u64_varint_2_be, read_u16_raw_be);
        read_varint_long_func!(ReadU64Varint4Le, read_u64_varint_4_le, read_u32_raw_le);
        read_varint_long_func!(ReadU64Varint4Be, read_u64_varint_4_be, read_u32_raw_be);
        read_varint_long_func!(ReadU64Varint8Le, read_u64_varint_8_le, read_u64_raw_le);
        read_varint_long_func!(ReadU64Varint8Be, read_u64_varint_8_be, read_u64_raw_be);

        read_varint_long_func!(ReadU128Varint2Le, read_u128_varint_2_le, read_u16_raw_le);
        read_varint_long_func!(ReadU128Varint2Be, read_u128_varint_2_be, read_u16_raw_be);
        read_varint_long_func!(ReadU128Varint4Le, read_u128_varint_4_le, read_u32_raw_le);
        read_varint_long_func!(ReadU128Varint4Be, read_u128_varint_4_be, read_u32_raw_be);
        read_varint_long_func!(ReadU128Varint8Le, read_u128_varint_8_le, read_u64_raw_le);
        read_varint_long_func!(ReadU128Varint8Be, read_u128_varint_8_be, read_u64_raw_be);
        read_varint_long_func!(ReadU128Varint16Le, read_u128_varint_16_le, read_u128_raw_le);
        read_varint_long_func!(ReadU128Varint16Be, read_u128_varint_16_be, read_u128_raw_be);
    };
}

define_read_varint_long_future!();

#[cfg(all(feature = "async_varint_long", not(feature = "async_raw"), not(feature = "async_varint")))]
compile_error!("developer error: please check Cargo.toml");
