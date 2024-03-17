macro_rules! read_raw_size_future {
    (cp, $primitive: ty, $future: ident, $from: ident) => {
        read_raw_size_future!(f cfg(feature = "async_raw_size"), cp, $primitive, $future, $from);
    };
    (ap, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_raw_size_future!(f cfg(feature = "async_raw_size"), ap, $primitive, $future, $inner_future);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $from: ident) => {
        read_raw_future!(f $feature, $primitive, $future, $from);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_size_ap_future!(f $feature, $primitive, $future, $inner_future);
    };
}
macro_rules! read_raw_size_func {
    (cp, $primitive: ty, $func: ident, $future: ident) => {
        read_raw_size_func!(f cfg(feature = "async_raw_size"), cp, $primitive, $func, $future);
    };
    (ap, $future: ident, $func: ident, $inner_func: ident) => {
        read_raw_size_func!(f cfg(feature = "async_raw_size"), ap, $future, $func, $inner_func);
    };
    (f $feature: meta, cp, $primitive: ty, $func: ident, $future: ident) => {
        read_raw_func!(f $feature, $primitive, $func, $future);
    };
    (f $feature: meta, ap, $future: ident, $func: ident, $inner_func: ident) => {
        read_wrap_func!(f $feature, $future, $func, $inner_func);
    };
}

macro_rules! define_read_raw_size_future {
    () => {
        read_raw_size_future!(cp, usize, ReadUsizeRawLe, from_le_bytes);
        read_raw_size_future!(cp, usize, ReadUsizeRawBe, from_be_bytes);
        read_raw_size_future!(cp, isize, ReadIsizeRawLe, from_le_bytes);
        read_raw_size_future!(cp, isize, ReadIsizeRawBe, from_be_bytes);

        read_raw_size_future!(ap, usize, ReadUsizeRawLeAp, ReadU128RawLe);
        read_raw_size_future!(ap, usize, ReadUsizeRawBeAp, ReadU128RawBe);
        read_raw_size_future!(ap, isize, ReadIsizeRawLeAp, ReadI128RawLe);
        read_raw_size_future!(ap, isize, ReadIsizeRawBeAp, ReadI128RawBe);
    };
}
macro_rules! define_read_raw_size_func {
    () => {
        read_raw_size_func!(cp, usize, read_usize_raw_le, ReadUsizeRawLe);
        read_raw_size_func!(cp, usize, read_usize_raw_be, ReadUsizeRawBe);
        read_raw_size_func!(cp, isize, read_isize_raw_le, ReadIsizeRawLe);
        read_raw_size_func!(cp, isize, read_isize_raw_be, ReadIsizeRawBe);

        read_raw_size_func!(ap, ReadUsizeRawLeAp, read_usize_raw_le_ap, read_u128_raw_le);
        read_raw_size_func!(ap, ReadUsizeRawBeAp, read_usize_raw_be_ap, read_u128_raw_be);
        read_raw_size_func!(ap, ReadIsizeRawLeAp, read_isize_raw_le_ap, read_i128_raw_le);
        read_raw_size_func!(ap, ReadIsizeRawBeAp, read_isize_raw_be_ap, read_i128_raw_be);
    };
}

define_read_raw_size_future!();

#[cfg(all(feature = "async_raw_size", not(feature = "async_raw")))]
compile_error!("developer error: please check Cargo.toml");
