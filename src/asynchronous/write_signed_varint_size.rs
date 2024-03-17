macro_rules! write_signed_varint_size_future {
    (cp, $future: ident) => {
        write_signed_varint_size_future!(f cfg(feature = "async_signed_varint_size"), cp, isize, $future, usize, WriteUsizeVarint);
    };
    (ap, $future: ident) => {
        write_signed_varint_size_future!(f cfg(feature = "async_signed_varint_size"), ap, isize, $future, u8, WriteU8Raw);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_signed_varint_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_size_ap_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
}
macro_rules! write_signed_varint_size_func {
    (cp, $future: ident, $func: ident) => {
        write_signed_varint_size_func!(f cfg(feature = "async_signed_varint_size"), cp, isize, $future, $func);
    };
    (ap, $future: ident, $func: ident) => {
        write_signed_varint_size_func!(f cfg(feature = "async_signed_varint_size"), ap, isize, $future, $func);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $func: ident) => {
        write_signed_varint_func!(f $feature, $primitive, $future, $func);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $func: ident) => {
        write_wrap_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_signed_varint_size_future {
    () => {
        write_signed_varint_size_future!(cp, WriteIsizeVarint);

        write_signed_varint_size_future!(ap, WriteIsizeVarintAp);
    };
}
macro_rules! define_write_signed_varint_size_func {
    () => {
        write_signed_varint_size_func!(cp, WriteIsizeVarint, write_isize_varint);

        write_signed_varint_size_func!(ap, WriteIsizeVarintAp, write_isize_varint_ap);
    };
}

define_write_signed_varint_size_future!();

#[cfg(all(feature = "async_signed_varint_size", not(feature = "async_varint_size"), not(feature = "async_signed_varint")))]
compile_error!("developer error: please check Cargo.toml");
