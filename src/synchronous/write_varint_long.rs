macro_rules! write_varint_long {
    ($primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        write_varint_long!(f cfg(feature = "sync_varint_long"), $primitive, $func, $internal, $write_internal);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        write_varint!(f $feature, $primitive, $func, $internal, $write_internal);
    };
}
macro_rules! define_write_varint_long {
    () => {
        write_varint_long!(u8, write_u8_varint, u8, write_u8_raw);

        write_varint_long!(u16, write_u16_varint_2_le, u16, write_u16_raw_le);
        write_varint_long!(u16, write_u16_varint_2_be, u16, write_u16_raw_be);

        write_varint_long!(u32, write_u32_varint_2_le, u16, write_u16_raw_le);
        write_varint_long!(u32, write_u32_varint_2_be, u16, write_u16_raw_be);
        write_varint_long!(u32, write_u32_varint_4_le, u32, write_u32_raw_le);
        write_varint_long!(u32, write_u32_varint_4_be, u32, write_u32_raw_be);

        write_varint_long!(u64, write_u64_varint_2_le, u16, write_u16_raw_le);
        write_varint_long!(u64, write_u64_varint_2_be, u16, write_u16_raw_be);
        write_varint_long!(u64, write_u64_varint_4_le, u32, write_u32_raw_le);
        write_varint_long!(u64, write_u64_varint_4_be, u32, write_u32_raw_be);
        write_varint_long!(u64, write_u64_varint_8_le, u64, write_u64_raw_le);
        write_varint_long!(u64, write_u64_varint_8_be, u64, write_u64_raw_be);

        write_varint_long!(u128, write_u128_varint_2_le, u16, write_u16_raw_le);
        write_varint_long!(u128, write_u128_varint_2_be, u16, write_u16_raw_be);
        write_varint_long!(u128, write_u128_varint_4_le, u32, write_u32_raw_le);
        write_varint_long!(u128, write_u128_varint_4_be, u32, write_u32_raw_be);
        write_varint_long!(u128, write_u128_varint_8_le, u64, write_u64_raw_le);
        write_varint_long!(u128, write_u128_varint_8_be, u64, write_u64_raw_be);
        write_varint_long!(u128, write_u128_varint_16_le, u128, write_u128_raw_le);
        write_varint_long!(u128, write_u128_varint_16_be, u128, write_u128_raw_be);
    };
}

#[cfg(all(feature = "sync_varint_long", not(feature = "sync_raw")))]
compile_error!("developer error: please check Cargo.toml");
