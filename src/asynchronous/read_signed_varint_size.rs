macro_rules! read_signed_varint_size_future {
    (cp, $future: ident, $inner_future: ident) => {
        read_signed_varint_size_future!(f cfg(feature = "async_signed_varint_size"), cp, isize, $future, $inner_future);
    };
    (ap, $future: ident, $inner_future: ident) => {
        read_signed_varint_size_future!(f cfg(feature = "async_signed_varint_size"), ap, isize, $future, $inner_future);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_signed_varint_future!(f $feature, $primitive, $future, $inner_future);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_size_ap_future!(f $feature, $primitive, $future, $inner_future);
    };
}
macro_rules! read_signed_varint_size_func {
    (cp, $future: ident, $func: ident, $inner_func: ident) => {
        read_signed_varint_size_func!(f cfg(feature = "async_signed_varint_size"), cp, $future, $func, $inner_func);
    };
    (ap, $future: ident, $func: ident, $inner_func: ident) => {
        read_signed_varint_size_func!(f cfg(feature = "async_signed_varint_size"), ap, $future, $func, $inner_func);
    };
    (f $feature: meta, cp, $future: ident, $func: ident, $inner_func: ident) => {
        read_signed_varint_func!(f $feature, $future, $func, $inner_func);
    };
    (f $feature: meta, ap, $future: ident, $func: ident, $inner_func: ident) => {
        read_wrap_func!(f $feature, $future, $func, $inner_func);
    };
}

macro_rules! define_read_signed_varint_size_future {
    () => {
        read_signed_varint_size_future!(cp, ReadIsizeVarint, ReadUsizeVarint);

        read_signed_varint_size_future!(ap, ReadIsizeVarintAp, ReadI128Varint);
    };
}
macro_rules! define_read_signed_varint_size_func {
    () => {
        read_signed_varint_size_func!(cp, ReadIsizeVarint, read_isize_varint, read_usize_varint);

        read_signed_varint_size_func!(ap, ReadIsizeVarintAp, read_isize_varint_ap, read_i128_varint);
    };
}

define_read_signed_varint_size_future!();

#[cfg(all(feature = "async_signed_varint_size", not(feature = "async_varint_size"), not(feature = "async_signed_varint"), not(feature = "async_varint")))]
compile_error!("developer error: please check Cargo.toml");
