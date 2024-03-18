macro_rules! write_varint_long_size {
    (cp, $func: ident, $internal: ty, $write_internal: ident) => {
        write_varint_long_size!(f cfg(feature = "sync_varint_long_size"), cp, usize, $func, $internal, $write_internal);
    };
    (ap, $func: ident, $write_internal: ident) => {
        write_varint_long_size!(f cfg(feature = "sync_varint_long_size"), ap, usize, $func, u128, $write_internal);
    };
    (f $feature: meta, cp, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        write_varint_long!(f $feature, $primitive, $func, $internal, $write_internal);
    };
    (f $feature: meta, ap, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        write_size_ap!(f $feature, $primitive, $func, $internal, $write_internal);
    };
}

macro_rules! define_write_varint_long_size {
    () => {
        write_varint_long_size!(cp, write_usize_varint_2_le, u16, write_u16_raw_le);
        write_varint_long_size!(cp, write_usize_varint_2_be, u16, write_u16_raw_be);
        write_varint_long_size!(cp, write_usize_varint_4_le, u32, write_u32_raw_le);
        write_varint_long_size!(cp, write_usize_varint_4_be, u32, write_u32_raw_be);
        write_varint_long_size!(cp, write_usize_varint_8_le, u64, write_u64_raw_le);
        write_varint_long_size!(cp, write_usize_varint_8_be, u64, write_u64_raw_be);
        write_varint_long_size!(cp, write_usize_varint_16_le, u128, write_u128_raw_le);
        write_varint_long_size!(cp, write_usize_varint_16_be, u128, write_u128_raw_be);

        write_varint_long_size!(ap, write_usize_varint_2_le_ap, write_u128_varint_2_le);
        write_varint_long_size!(ap, write_usize_varint_2_be_ap, write_u128_varint_2_be);
        write_varint_long_size!(ap, write_usize_varint_4_le_ap, write_u128_varint_4_le);
        write_varint_long_size!(ap, write_usize_varint_4_be_ap, write_u128_varint_4_be);
        write_varint_long_size!(ap, write_usize_varint_8_le_ap, write_u128_varint_8_le);
        write_varint_long_size!(ap, write_usize_varint_8_be_ap, write_u128_varint_8_be);
        write_varint_long_size!(ap, write_usize_varint_16_le_ap, write_u128_varint_16_le);
        write_varint_long_size!(ap, write_usize_varint_16_be_ap, write_u128_varint_16_be);
    };
}

#[cfg(all(feature = "sync_varint_long_size", not(feature = "sync_raw"), not(feature = "sync_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
