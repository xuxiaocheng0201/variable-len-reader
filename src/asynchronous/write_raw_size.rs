macro_rules! write_raw_size_future {
    (cp, $primitive: ty, $future: ident, $to: ident) => {
        write_raw_size_future!(f cfg(feature = "async_raw_size"), cp, $primitive, $future, $to);
    };
    (ap, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_raw_size_future!(f cfg(feature = "async_raw_size"), ap, $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $to: ident) => {
        write_raw_future!(f $feature, $primitive, $future, $to);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_size_ap_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
}
macro_rules! write_raw_size_func {
    (cp, $primitive: ty, $future: ident, $func: ident) => {
        write_raw_size_func!(f cfg(feature = "async_raw_size"), cp, $primitive, $future, $func);
    };
    (ap, $primitive: ty, $future: ident, $func: ident) => {
        write_raw_size_func!(f cfg(feature = "async_raw_size"), ap, $primitive, $future, $func);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $func: ident) => {
        write_raw_func!(f $feature, $primitive, $future, $func);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $func: ident) => {
        write_wrap_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_raw_size_future {
    () => {
        write_raw_size_future!(cp, usize, WriteUsizeRawLe, to_le_bytes);
        write_raw_size_future!(cp, usize, WriteUsizeRawBe, to_be_bytes);
        write_raw_size_future!(cp, isize, WriteIsizeRawLe, to_le_bytes);
        write_raw_size_future!(cp, isize, WriteIsizeRawBe, to_be_bytes);

        write_raw_size_future!(ap, usize, WriteUsizeRawLeAp, u128, WriteU128RawLe);
        write_raw_size_future!(ap, usize, WriteUsizeRawBeAp, u128, WriteU128RawBe);
        write_raw_size_future!(ap, isize, WriteIsizeRawLeAp, i128, WriteI128RawLe);
        write_raw_size_future!(ap, isize, WriteIsizeRawBeAp, i128, WriteI128RawBe);
    };
}
macro_rules! define_write_raw_size_func {
    () => {
        write_raw_size_func!(cp, usize, WriteUsizeRawLe, write_usize_raw_le);
        write_raw_size_func!(cp, usize, WriteUsizeRawBe, write_usize_raw_be);
        write_raw_size_func!(cp, isize, WriteIsizeRawLe, write_isize_raw_le);
        write_raw_size_func!(cp, isize, WriteIsizeRawBe, write_isize_raw_be);

        write_raw_size_func!(ap, usize, WriteUsizeRawLeAp, write_usize_raw_le_ap);
        write_raw_size_func!(ap, usize, WriteUsizeRawBeAp, write_usize_raw_be_ap);
        write_raw_size_func!(ap, isize, WriteIsizeRawLeAp, write_isize_raw_le_ap);
        write_raw_size_func!(ap, isize, WriteIsizeRawBeAp, write_isize_raw_be_ap);
    };
}

define_write_raw_size_future!();

#[cfg(all(feature = "async_raw_size", not(feature = "async_raw")))]
compile_error!("developer error: please check Cargo.toml");
