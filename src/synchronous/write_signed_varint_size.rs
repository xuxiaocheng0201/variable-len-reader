macro_rules! write_signed_varint_size {
    (cp, $func: ident, $write_internal: ident) => {
        write_signed_varint_size!(f cfg(feature = "sync_signed_varint_size"), cp, isize, $func, $write_internal);
    };
    (ap, $func: ident, $write_internal: ident) => {
        write_signed_varint_size!(f cfg(feature = "sync_signed_varint_size"), ap, isize, $func, i128, $write_internal);
    };
    (f $feature: meta, cp, $primitive: ty, $func: ident, $write_internal: ident) => {
        write_signed_varint!(f $feature, $primitive, $func, $write_internal);
    };
    (f $feature: meta, ap, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        write_size_ap!(f $feature, $primitive, $func, $internal, $write_internal);
    };
}

macro_rules! define_write_signed_varint_size {
    () => {
        write_signed_varint_size!(cp, write_isize_varint, write_usize_varint);

        write_signed_varint_size!(ap, write_isize_varint_ap, write_i128_varint);
    };
}

#[cfg(all(feature = "sync_signed_varint_size", not(feature = "sync_varint_size"), not(feature = "sync_signed_varint")))]
compile_error!("developer error: please check Cargo.toml");
