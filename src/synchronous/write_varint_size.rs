macro_rules! write_varint_size {
    (cp, $func: ident, $write_internal: ident) => {
        write_varint_size!(f cfg(feature = "sync_varint_size"), cp, $func, $write_internal);
    };
    (ap, $func: ident, $write_internal: ident) => {
        write_varint_size!(f cfg(feature = "sync_varint_size"), ap, $func, $write_internal);
    };
    (f $feature: meta, cp, $func: ident, $write_internal: ident) => {
        write_varint!(f $feature, usize, $func, u8, $write_internal);
    };
    (f $feature: meta, ap, $func: ident, $write_internal: ident) => {
        write_size_ap!(f $feature, usize, $func, u128, $write_internal);
    };
}
macro_rules! define_write_varint_size {
    () => {
        write_varint_size!(cp, write_usize_varint, write_u8_raw);

        write_varint_size!(ap, write_usize_varint_ap, write_u128_varint);
    };
}

#[cfg(all(feature = "sync_varint_size", not(feature = "sync_raw"), not(feature = "sync_varint")))]
compile_error!("developer error: please check Cargo.toml");
