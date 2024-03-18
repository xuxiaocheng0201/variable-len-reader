macro_rules! write_signed_varint_long {
    ($primitive: ty, $func: ident, $write_internal: ident) => {
        write_signed_varint_long!(f cfg(feature = "sync_signed_varint_long"), $primitive, $func, $write_internal);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $write_internal: ident) => {
        write_signed_varint!(f $feature, $primitive, $func, $write_internal);
    };
}

macro_rules! define_write_signed_varint_long {
    () => {
        write_signed_varint_long!(i8, write_i8_varint, write_u8_varint);

        write_signed_varint_long!(i16, write_i16_varint_2_le, write_u16_varint_2_le);
        write_signed_varint_long!(i16, write_i16_varint_2_be, write_u16_varint_2_be);

        write_signed_varint_long!(i32, write_i32_varint_2_le, write_u32_varint_2_le);
        write_signed_varint_long!(i32, write_i32_varint_2_be, write_u32_varint_2_be);
        write_signed_varint_long!(i32, write_i32_varint_4_le, write_u32_varint_4_le);
        write_signed_varint_long!(i32, write_i32_varint_4_be, write_u32_varint_4_be);

        write_signed_varint_long!(i64, write_i64_varint_2_le, write_u64_varint_2_le);
        write_signed_varint_long!(i64, write_i64_varint_2_be, write_u64_varint_2_be);
        write_signed_varint_long!(i64, write_i64_varint_4_le, write_u64_varint_4_le);
        write_signed_varint_long!(i64, write_i64_varint_4_be, write_u64_varint_4_be);
        write_signed_varint_long!(i64, write_i64_varint_8_le, write_u64_varint_8_le);
        write_signed_varint_long!(i64, write_i64_varint_8_be, write_u64_varint_8_be);

        write_signed_varint_long!(i128, write_i128_varint_2_le, write_u128_varint_2_le);
        write_signed_varint_long!(i128, write_i128_varint_2_be, write_u128_varint_2_be);
        write_signed_varint_long!(i128, write_i128_varint_4_le, write_u128_varint_4_le);
        write_signed_varint_long!(i128, write_i128_varint_4_be, write_u128_varint_4_be);
        write_signed_varint_long!(i128, write_i128_varint_8_le, write_u128_varint_8_le);
        write_signed_varint_long!(i128, write_i128_varint_8_be, write_u128_varint_8_be);
        write_signed_varint_long!(i128, write_i128_varint_16_le, write_u128_varint_16_le);
        write_signed_varint_long!(i128, write_i128_varint_16_be, write_u128_varint_16_be);
    };
}

#[cfg(all(feature = "sync_signed_varint_long", not(feature = "sync_varint"), not(feature = "sync_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
