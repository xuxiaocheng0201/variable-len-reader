macro_rules! read_signed_varint_long_size {
    (cp, $func: ident, $read_internal: ident) => {
        read_signed_varint_long_size!(f cfg(feature = "sync_signed_varint_long_size"), cp, $func, $read_internal);
    };
    (ap, $func: ident, $read_internal: ident) => {
        read_signed_varint_long_size!(f cfg(feature = "sync_signed_varint_long_size"), ap, $func, $read_internal);
    };
    (f $feature: meta, cp, $func: ident, $read_internal: ident) => {
        read_signed_varint_long!(f $feature, isize, $func, $read_internal);
    };
    (f $feature: meta, ap, $func: ident, $read_internal: ident) => {
        read_size_ap!(f $feature, isize, $func, $read_internal);
    };
}
macro_rules! define_read_signed_varint_long_size {
    () => {
        read_signed_varint_long_size!(cp, read_isize_varint_2_le, read_usize_varint_2_le);
        read_signed_varint_long_size!(cp, read_isize_varint_2_be, read_usize_varint_2_be);
        read_signed_varint_long_size!(cp, read_isize_varint_4_le, read_usize_varint_4_le);
        read_signed_varint_long_size!(cp, read_isize_varint_4_be, read_usize_varint_4_be);
        read_signed_varint_long_size!(cp, read_isize_varint_8_le, read_usize_varint_8_le);
        read_signed_varint_long_size!(cp, read_isize_varint_8_be, read_usize_varint_8_be);
        read_signed_varint_long_size!(cp, read_isize_varint_16_le, read_usize_varint_16_le);
        read_signed_varint_long_size!(cp, read_isize_varint_16_be, read_usize_varint_16_be);

        read_signed_varint_long_size!(ap, read_isize_varint_2_le_ap, read_i128_varint_2_le);
        read_signed_varint_long_size!(ap, read_isize_varint_2_be_ap, read_i128_varint_2_be);
        read_signed_varint_long_size!(ap, read_isize_varint_4_le_ap, read_i128_varint_4_le);
        read_signed_varint_long_size!(ap, read_isize_varint_4_be_ap, read_i128_varint_4_be);
        read_signed_varint_long_size!(ap, read_isize_varint_8_le_ap, read_i128_varint_8_le);
        read_signed_varint_long_size!(ap, read_isize_varint_8_be_ap, read_i128_varint_8_be);
        read_signed_varint_long_size!(ap, read_isize_varint_16_le_ap, read_i128_varint_16_le);
        read_signed_varint_long_size!(ap, read_isize_varint_16_be_ap, read_i128_varint_16_be);
    };
}

#[cfg(all(feature = "sync_signed_varint_long_size", not(feature = "sync_varint_long_size"), not(feature = "sync_signed_varint_long"), not(feature = "sync_varint")))]
compile_error!("developer error: please check Cargo.toml");
