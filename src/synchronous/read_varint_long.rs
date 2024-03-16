macro_rules! read_varint_long {
    ($primitive: ty, $func: ident, $internal: ty, $read_internal: ident) => {
        read_varint_long!(f cfg(feature = "sync_varint_long"), $primitive, $func, $internal, $read_internal);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $internal: ty, $read_internal: ident) => {
        read_varint!(f $feature, $primitive, $func, $internal, $read_internal);
    };
}
macro_rules! define_read_varint_long {
    () => {
        read_varint_long!(u8, read_u8_varint, u8, read_u8_raw);

        read_varint_long!(u16, read_u16_varint_2_le, u16, read_u16_raw_le);
        read_varint_long!(u16, read_u16_varint_2_be, u16, read_u16_raw_be);

        read_varint_long!(u32, read_u32_varint_2_le, u16, read_u16_raw_le);
        read_varint_long!(u32, read_u32_varint_2_be, u16, read_u16_raw_be);
        read_varint_long!(u32, read_u32_varint_4_le, u32, read_u32_raw_le);
        read_varint_long!(u32, read_u32_varint_4_be, u32, read_u32_raw_be);

        read_varint_long!(u64, read_u64_varint_2_le, u16, read_u16_raw_le);
        read_varint_long!(u64, read_u64_varint_2_be, u16, read_u16_raw_be);
        read_varint_long!(u64, read_u64_varint_4_le, u32, read_u32_raw_le);
        read_varint_long!(u64, read_u64_varint_4_be, u32, read_u32_raw_be);
        read_varint_long!(u64, read_u64_varint_8_le, u64, read_u64_raw_le);
        read_varint_long!(u64, read_u64_varint_8_be, u64, read_u64_raw_be);

        read_varint_long!(u128, read_u128_varint_2_le, u16, read_u16_raw_le);
        read_varint_long!(u128, read_u128_varint_2_be, u16, read_u16_raw_be);
        read_varint_long!(u128, read_u128_varint_4_le, u32, read_u32_raw_le);
        read_varint_long!(u128, read_u128_varint_4_be, u32, read_u32_raw_be);
        read_varint_long!(u128, read_u128_varint_8_le, u64, read_u64_raw_le);
        read_varint_long!(u128, read_u128_varint_8_be, u64, read_u64_raw_be);
        read_varint_long!(u128, read_u128_varint_16_le, u128, read_u128_raw_le);
        read_varint_long!(u128, read_u128_varint_16_be, u128, read_u128_raw_be);
    };
}

#[cfg(all(feature = "sync_varint_long", not(feature = "sync_varint")))]
compile_error!("developer error: please check Cargo.toml");
