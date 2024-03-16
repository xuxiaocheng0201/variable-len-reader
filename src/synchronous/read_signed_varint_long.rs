macro_rules! read_signed_varint_long {
    ($primitive: ty, $func: ident, $read_internal: ident) => {
        read_signed_varint_long!(f cfg(feature = "sync_signed_varint_long"), $primitive, $func, $read_internal);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_signed_varint!(f $feature, $primitive, $func, $read_internal);
    };
}
macro_rules! define_read_signed_varint_long {
    () => {
        read_signed_varint_long!(i8, read_i8_varint, read_u8_varint);

        read_signed_varint_long!(i16, read_i16_varint_2_le, read_u16_varint_2_le);
        read_signed_varint_long!(i16, read_i16_varint_2_be, read_u16_varint_2_be);

        read_signed_varint_long!(i32, read_i32_varint_2_le, read_u32_varint_2_le);
        read_signed_varint_long!(i32, read_i32_varint_2_be, read_u32_varint_2_be);
        read_signed_varint_long!(i32, read_i32_varint_4_le, read_u32_varint_4_le);
        read_signed_varint_long!(i32, read_i32_varint_4_be, read_u32_varint_4_be);

        read_signed_varint_long!(i64, read_i64_varint_2_le, read_u64_varint_2_le);
        read_signed_varint_long!(i64, read_i64_varint_2_be, read_u64_varint_2_be);
        read_signed_varint_long!(i64, read_i64_varint_4_le, read_u64_varint_4_le);
        read_signed_varint_long!(i64, read_i64_varint_4_be, read_u64_varint_4_be);
        read_signed_varint_long!(i64, read_i64_varint_8_le, read_u64_varint_8_le);
        read_signed_varint_long!(i64, read_i64_varint_8_be, read_u64_varint_8_be);

        read_signed_varint_long!(i128, read_i128_varint_2_le, read_u128_varint_2_le);
        read_signed_varint_long!(i128, read_i128_varint_2_be, read_u128_varint_2_be);
        read_signed_varint_long!(i128, read_i128_varint_4_le, read_u128_varint_4_le);
        read_signed_varint_long!(i128, read_i128_varint_4_be, read_u128_varint_4_be);
        read_signed_varint_long!(i128, read_i128_varint_8_le, read_u128_varint_8_le);
        read_signed_varint_long!(i128, read_i128_varint_8_be, read_u128_varint_8_be);
        read_signed_varint_long!(i128, read_i128_varint_16_le, read_u128_varint_16_le);
        read_signed_varint_long!(i128, read_i128_varint_16_be, read_u128_varint_16_be);
    };
}

#[cfg(all(feature = "sync_signed_varint_long", not(feature = "sync_varint"), not(feature = "sync_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
