macro_rules! write_varint_size {
    (cp, $func: ident) => {
        write_varint_size!(f cfg(feature = "sync_varint_size"), cp, usize, $func, u8, write_u8_raw);
    };
    (ap, $func: ident) => {
        write_varint_size!(f cfg(feature = "sync_varint_size"), ap, usize, $func, u128, write_u128_varint);
    };
    (f $feature: meta, cp, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        write_varint!(f $feature, $primitive, $func, $internal, $write_internal);
    };
    (f $feature: meta, ap, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        write_size_ap!(f $feature, $primitive, $func, $internal, $write_internal);
    };
}

macro_rules! define_write_varint_size {
    () => {
        write_varint_size!(cp, write_usize_varint);

        write_varint_size!(ap, write_usize_varint_ap);
    };
}

#[cfg(all(feature = "sync_varint_size", not(feature = "sync_raw"), not(feature = "sync_varint")))]
compile_error!("developer error: please check Cargo.toml");
