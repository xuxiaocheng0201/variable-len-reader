macro_rules! write_signed_varint_long_size {
    (cp, $func: ident, $write_internal: ident) => {
        write_signed_varint_long_size!(f cfg(feature = "sync_signed_varint_long_size"), cp, $func, $write_internal);
    };
    (ap, $func: ident, $write_internal: ident) => {
        write_signed_varint_long_size!(f cfg(feature = "sync_signed_varint_long_size"), ap, $func, $write_internal);
    };
    (f $feature: meta, cp, $func: ident, $write_internal: ident) => {
        write_signed_varint_long!(f $feature, isize, $func, $write_internal);
    };
    (f $feature: meta, ap, $func: ident, $write_internal: ident) => {
        write_size_ap!(f $feature, isize, $func, i128, $write_internal);
    };
}
macro_rules! define_write_signed_varint_long_size {
    () => {
        write_signed_varint_long_size!(cp, write_isize_varint_2_le, write_usize_varint_2_le);
        write_signed_varint_long_size!(cp, write_isize_varint_2_be, write_usize_varint_2_be);
        write_signed_varint_long_size!(cp, write_isize_varint_4_le, write_usize_varint_4_le);
        write_signed_varint_long_size!(cp, write_isize_varint_4_be, write_usize_varint_4_be);
        write_signed_varint_long_size!(cp, write_isize_varint_8_le, write_usize_varint_8_le);
        write_signed_varint_long_size!(cp, write_isize_varint_8_be, write_usize_varint_8_be);
        write_signed_varint_long_size!(cp, write_isize_varint_16_le, write_usize_varint_16_le);
        write_signed_varint_long_size!(cp, write_isize_varint_16_be, write_usize_varint_16_be);

        write_signed_varint_long_size!(ap, write_isize_varint_2_le_ap, write_i128_varint_2_le);
        write_signed_varint_long_size!(ap, write_isize_varint_2_be_ap, write_i128_varint_2_be);
        write_signed_varint_long_size!(ap, write_isize_varint_4_le_ap, write_i128_varint_4_le);
        write_signed_varint_long_size!(ap, write_isize_varint_4_be_ap, write_i128_varint_4_be);
        write_signed_varint_long_size!(ap, write_isize_varint_8_le_ap, write_i128_varint_8_le);
        write_signed_varint_long_size!(ap, write_isize_varint_8_be_ap, write_i128_varint_8_be);
        write_signed_varint_long_size!(ap, write_isize_varint_16_le_ap, write_i128_varint_16_le);
        write_signed_varint_long_size!(ap, write_isize_varint_16_be_ap, write_i128_varint_16_be);
    };
}

#[cfg(all(feature = "sync_signed_varint_long_size", not(feature = "sync_varint_long_size"), not(feature = "sync_signed_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
