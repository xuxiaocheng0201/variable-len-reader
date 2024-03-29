macro_rules! read_signed_varint_size {
    (cp, $func: ident, $read_internal: ident) => {
        read_signed_varint_size!(f cfg(feature = "sync_signed_varint_size"), cp, isize, $func, $read_internal);
    };
    (ap, $func: ident, $read_internal: ident) => {
        read_signed_varint_size!(f cfg(feature = "sync_signed_varint_size"), ap, isize, $func, $read_internal);
    };
    (f $feature: meta, cp, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_signed_varint!(f $feature, $primitive, $func, $read_internal);
    };
    (f $feature: meta, ap, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_size_ap!(f $feature, $primitive, $func, $read_internal);
    };
}

macro_rules! define_read_signed_varint_size {
    () => {
        read_signed_varint_size!(cp, read_isize_varint, read_usize_varint);

        read_signed_varint_size!(ap, read_isize_varint_ap, read_i128_varint);
    };
}

#[cfg(all(feature = "sync_signed_varint_size", not(feature = "sync_varint_size"), not(feature = "sync_signed_varint"), not(feature = "sync_varint")))]
compile_error!("developer error: please check Cargo.toml");
