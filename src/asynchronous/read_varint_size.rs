macro_rules! read_varint_size_future {
    (cp, $future: ident) => {
        read_varint_size_future!(f cfg(feature = "async_varint_size"), cp, usize, $future, u8, ReadU8Raw);
    };
    (ap, $future: ident, $inner_future: ident) => {
        read_varint_size_future!(f cfg(feature = "async_varint_size"), ap, usize, $future, $inner_future);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        read_varint_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_size_ap_future!(f $feature, $primitive, $future, $inner_future);
    };
}
macro_rules! read_varint_size_func {
    (cp, $future: ident, $func: ident) => {
        read_varint_size_func!(f cfg(feature = "async_varint_size"), cp, $future, $func, read_u8_raw);
    };
    (ap, $future: ident, $func: ident, $inner_func: ident) => {
        read_varint_size_func!(f cfg(feature = "async_varint_size"), ap, $future, $func, $inner_func);
    };
    (f $feature: meta, cp, $future: ident, $func: ident, $inner_func: ident) => {
        read_varint_func!(f $feature, $future, $func, $inner_func);
    };
    (f $feature: meta, ap, $future: ident, $func: ident, $inner_func: ident) => {
        read_wrap_func!(f $feature, $future, $func, $inner_func);
    };
}

macro_rules! define_read_varint_size_future {
    () => {
        read_varint_size_future!(cp, ReadUsizeVarint);

        read_varint_size_future!(ap, ReadUsizeVarintAp, ReadU128Varint);
    };
}
macro_rules! define_read_varint_size_func {
    () => {
        read_varint_size_func!(cp, ReadUsizeVarint, read_usize_varint);

        read_varint_size_func!(ap, ReadUsizeVarintAp, read_usize_varint_ap, read_u128_varint);
    };
}

define_read_varint_size_future!();

#[cfg(all(feature = "async_varint_size", not(feature = "async_raw"), not(feature = "async_varint")))]
compile_error!("developer error: please check Cargo.toml");
