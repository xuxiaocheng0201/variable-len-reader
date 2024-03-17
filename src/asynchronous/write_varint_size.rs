macro_rules! write_varint_size_future {
    (cp, $future: ident) => {
        write_varint_size_future!(f cfg(feature = "async_varint_size"), cp, usize, $future, u8, WriteU8Raw);
    };
    (ap, $future: ident) => {
        write_varint_size_future!(f cfg(feature = "async_varint_size"), ap, usize, $future, u8, WriteU8Raw);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_varint_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_size_ap_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
}
macro_rules! write_varint_size_func {
    (cp, $future: ident, $func: ident) => {
        write_varint_size_func!(f cfg(feature = "async_varint_size"), cp, usize, $future, $func);
    };
    (ap, $future: ident, $func: ident) => {
        write_varint_size_func!(f cfg(feature = "async_varint_size"), ap, usize, $future, $func);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $func: ident) => {
        write_varint_func!(f $feature, $primitive, $future, $func);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $func: ident) => {
        write_wrap_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_varint_size_future {
    () => {
        write_varint_size_future!(cp, WriteUsizeVarint);

        write_varint_size_future!(ap, WriteUsizeVarintAp);
     };
}
macro_rules! define_write_varint_size_func {
    () => {
        write_varint_size_func!(cp, WriteUsizeVarint, write_usize_varint);

        write_varint_size_func!(ap, WriteUsizeVarintAp, write_usize_varint_ap);
    };
}

define_write_varint_size_future!();

#[cfg(all(feature = "async_varint_size", not(feature = "async_raw"), not(feature = "async_varint")))]
compile_error!("developer error: please check Cargo.toml");
