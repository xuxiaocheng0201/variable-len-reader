macro_rules! read_varint_size {
    (cp, $func: ident, $internal: ty, $read_internal: ident) => {
        read_varint_size!(f cfg(feature = "sync_varint_size"), cp, usize, $func, $internal, $read_internal);
    };
    (ap, $func: ident, $read_internal: ident) => {
        read_varint_size!(f cfg(feature = "sync_varint_size"), ap, usize, $func, $read_internal);
    };
    (f $feature: meta, cp, $primitive: ty, $func: ident, $internal: ty, $read_internal: ident) => {
        read_varint!(f $feature, $primitive, $func, $internal, $read_internal);
    };
    (f $feature: meta, ap, $primitive: ty, $func: ident, $read_internal: ident) => {
        read_size_ap!(f $feature, $primitive, $func, $read_internal);
    };
}
macro_rules! define_read_varint_size {
    () => {
        read_varint_size!(cp, read_usize_varint, u8, read_u8_raw);

        read_varint_size!(ap, read_usize_varint_ap, read_u128_varint);
    };
}

#[cfg(all(feature = "sync_varint_size", not(feature = "sync_raw"), not(feature = "sync_varint")))]
compile_error!("developer error: please check Cargo.toml");
