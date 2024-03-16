macro_rules! read_varint_long_size {
    (cp, $func: ident, $internal: ty, $read_internal: ident) => {
        read_varint_long_size!(f cfg(feature = "sync_varint_long_size"), cp, $func, $internal, $read_internal);
    };
    (ap, $func: ident, $read_internal: ident) => {
        read_varint_long_size!(f cfg(feature = "sync_varint_long_size"), ap, $func, $read_internal);
    };
    (f $feature: meta, cp, $func: ident, $internal: ty, $read_internal: ident) => {
        read_varint_long!(f $feature, usize, $func, $internal, $read_internal);
    };
    (f $feature: meta, ap, $func: ident, $read_internal: ident) => {
        read_size_ap!(f $feature, usize, $func, $read_internal);
    };
}
macro_rules! define_read_varint_long_size {
    () => {
        read_varint_long_size!(cp, read_usize_varint_2_le, u16, read_u16_raw_le);
        read_varint_long_size!(cp, read_usize_varint_2_be, u16, read_u16_raw_be);
        read_varint_long_size!(cp, read_usize_varint_4_le, u32, read_u32_raw_le);
        read_varint_long_size!(cp, read_usize_varint_4_be, u32, read_u32_raw_be);
        read_varint_long_size!(cp, read_usize_varint_8_le, u64, read_u64_raw_le);
        read_varint_long_size!(cp, read_usize_varint_8_be, u64, read_u64_raw_be);
        read_varint_long_size!(cp, read_usize_varint_16_le, u128, read_u128_raw_le);
        read_varint_long_size!(cp, read_usize_varint_16_be, u128, read_u128_raw_be);

        read_varint_long_size!(ap, read_usize_varint_2_le_ap, read_u128_varint_2_le);
        read_varint_long_size!(ap, read_usize_varint_2_be_ap, read_u128_varint_2_be);
        read_varint_long_size!(ap, read_usize_varint_4_le_ap, read_u128_varint_4_le);
        read_varint_long_size!(ap, read_usize_varint_4_be_ap, read_u128_varint_4_be);
        read_varint_long_size!(ap, read_usize_varint_8_le_ap, read_u128_varint_8_le);
        read_varint_long_size!(ap, read_usize_varint_8_be_ap, read_u128_varint_8_be);
        read_varint_long_size!(ap, read_usize_varint_16_le_ap, read_u128_varint_16_le);
        read_varint_long_size!(ap, read_usize_varint_16_be_ap, read_u128_varint_16_be);
    };
}

#[cfg(all(feature = "sync_varint_long_size", not(feature = "sync_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
