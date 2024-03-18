macro_rules! write_raw_size {
    (cp, $primitive: ty, $func: ident, $to: ident) => {
        write_raw_size!(f cfg(feature = "sync_raw_size"), cp, $primitive, $func, $to);
    };
    (ap, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        write_raw_size!(f cfg(feature = "sync_raw_size"), ap, $primitive, $func, $internal, $write_internal);
    };
    (f $feature: meta, cp, $primitive: ty, $func: ident, $to: ident) => {
        write_raw!(f $feature, $primitive, $func, $to);
    };
    (f $feature: meta, ap, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        write_size_ap!(f $feature, $primitive, $func, $internal, $write_internal);
    };
}

macro_rules! define_write_raw_size {
    () => {
        write_raw_size!(cp, usize, write_usize_raw_le, to_le_bytes);
        write_raw_size!(cp, usize, write_usize_raw_be, to_be_bytes);
        write_raw_size!(cp, isize, write_isize_raw_le, to_le_bytes);
        write_raw_size!(cp, isize, write_isize_raw_be, to_be_bytes);

        write_raw_size!(ap, usize, write_usize_raw_le_ap, u128, write_u128_raw_le);
        write_raw_size!(ap, usize, write_usize_raw_be_ap, u128, write_u128_raw_be);
        write_raw_size!(ap, isize, write_isize_raw_le_ap, i128, write_i128_raw_le);
        write_raw_size!(ap, isize, write_isize_raw_be_ap, i128, write_i128_raw_be);
    };
}

#[cfg(all(feature = "sync_raw_size", not(feature = "sync_raw")))]
compile_error!("developer error: please check Cargo.toml");
