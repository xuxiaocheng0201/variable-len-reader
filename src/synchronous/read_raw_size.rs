macro_rules! read_raw_size {
    (cp, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_raw_size!(f cfg(feature = "sync_raw_size"), cp, $primitive, $func, $read_internal);
    };
    (ap, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_raw_size!(f cfg(feature = "sync_raw_size"), ap, $primitive, $func, $read_internal);
    };
    (f $feature: meta, cp, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_raw!(f $feature, $primitive, $func, $read_internal);
    };
    (f $feature: meta, ap, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_size_ap!(f $feature, $primitive, $func, $read_internal);
    };
}

macro_rules! define_read_raw_size {
    () => {
        read_raw_size!(cp, usize, read_usize_raw_le, from_le_bytes);
        read_raw_size!(cp, usize, read_usize_raw_be, from_be_bytes);
        read_raw_size!(cp, isize, read_isize_raw_le, from_le_bytes);
        read_raw_size!(cp, isize, read_isize_raw_be, from_be_bytes);

        read_raw_size!(ap, usize, read_usize_raw_le_ap, read_u128_raw_le);
        read_raw_size!(ap, usize, read_usize_raw_be_ap, read_u128_raw_be);
        read_raw_size!(ap, isize, read_isize_raw_le_ap, read_i128_raw_le);
        read_raw_size!(ap, isize, read_isize_raw_be_ap, read_i128_raw_be);
    };
}

#[cfg(all(feature = "sync_raw_size", not(feature = "sync_raw")))]
compile_error!("developer error: please check Cargo.toml");
