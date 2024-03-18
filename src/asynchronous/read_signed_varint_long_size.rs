macro_rules! read_signed_varint_long_size_future {
    (cp, $future: ident, $inner_future: ident) => {
        read_signed_varint_long_size_future!(f cfg(feature = "async_signed_varint_long_size"), cp, isize, $future, $inner_future);
    };
    (ap, $future: ident, $inner_future: ident) => {
        read_signed_varint_long_size_future!(f cfg(feature = "async_signed_varint_long_size"), ap, isize, $future, $inner_future);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_signed_varint_long_future!(f $feature, $primitive, $future, $inner_future);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_size_ap_future!(f $feature, $primitive, $future, $inner_future);
    };
}
macro_rules! read_signed_varint_long_size_func {
    (cp, $future: ident, $func: ident, $inner_func: ident) => {
        read_signed_varint_long_size_func!(f cfg(feature = "async_signed_varint_long_size"), cp, $future, $func, $inner_func);
    };
    (ap, $future: ident, $func: ident, $inner_func: ident) => {
        read_signed_varint_long_size_func!(f cfg(feature = "async_signed_varint_long_size"), ap, $future, $func, $inner_func);
    };
    (f $feature: meta, cp, $future: ident, $func: ident, $inner_func: ident) => {
        read_signed_varint_long_func!(f $feature, $future, $func, $inner_func);
    };
    (f $feature: meta, ap, $future: ident, $func: ident, $inner_func: ident) => {
        read_wrap_func!(f $feature, $future, $func, $inner_func);
    };
}

macro_rules! define_read_signed_varint_long_size_future {
    () => {
        read_signed_varint_long_size_future!(cp, ReadIsizeVarint2Le, ReadUsizeVarint2Le);
        read_signed_varint_long_size_future!(cp, ReadIsizeVarint2Be, ReadUsizeVarint2Be);
        read_signed_varint_long_size_future!(cp, ReadIsizeVarint4Le, ReadUsizeVarint4Le);
        read_signed_varint_long_size_future!(cp, ReadIsizeVarint4Be, ReadUsizeVarint4Be);
        read_signed_varint_long_size_future!(cp, ReadIsizeVarint8Le, ReadUsizeVarint8Le);
        read_signed_varint_long_size_future!(cp, ReadIsizeVarint8Be, ReadUsizeVarint8Be);
        read_signed_varint_long_size_future!(cp, ReadIsizeVarint16Le, ReadUsizeVarint16Le);
        read_signed_varint_long_size_future!(cp, ReadIsizeVarint16Be, ReadUsizeVarint16Be);

        read_signed_varint_long_size_future!(ap, ReadIsizeVarint2LeAp, ReadI128Varint2Le);
        read_signed_varint_long_size_future!(ap, ReadIsizeVarint2BeAp, ReadI128Varint2Be);
        read_signed_varint_long_size_future!(ap, ReadIsizeVarint4LeAp, ReadI128Varint4Le);
        read_signed_varint_long_size_future!(ap, ReadIsizeVarint4BeAp, ReadI128Varint4Be);
        read_signed_varint_long_size_future!(ap, ReadIsizeVarint8LeAp, ReadI128Varint8Le);
        read_signed_varint_long_size_future!(ap, ReadIsizeVarint8BeAp, ReadI128Varint8Be);
        read_signed_varint_long_size_future!(ap, ReadIsizeVarint16LeAp, ReadI128Varint16Le);
        read_signed_varint_long_size_future!(ap, ReadIsizeVarint16BeAp, ReadI128Varint16Be);
    };
}
macro_rules! define_read_signed_varint_long_size_func {
    () => {
        read_signed_varint_long_size_func!(cp, ReadIsizeVarint2Le, read_isize_varint_2_le, read_usize_varint_2_le);
        read_signed_varint_long_size_func!(cp, ReadIsizeVarint2Be, read_isize_varint_2_be, read_usize_varint_2_be);
        read_signed_varint_long_size_func!(cp, ReadIsizeVarint4Le, read_isize_varint_4_le, read_usize_varint_4_le);
        read_signed_varint_long_size_func!(cp, ReadIsizeVarint4Be, read_isize_varint_4_be, read_usize_varint_4_be);
        read_signed_varint_long_size_func!(cp, ReadIsizeVarint8Le, read_isize_varint_8_le, read_usize_varint_8_le);
        read_signed_varint_long_size_func!(cp, ReadIsizeVarint8Be, read_isize_varint_8_be, read_usize_varint_8_be);
        read_signed_varint_long_size_func!(cp, ReadIsizeVarint16Le, read_isize_varint_16_le, read_usize_varint_16_le);
        read_signed_varint_long_size_func!(cp, ReadIsizeVarint16Be, read_isize_varint_16_be, read_usize_varint_16_be);

        read_signed_varint_long_size_func!(ap, ReadIsizeVarint2LeAp, read_isize_varint_2_le_ap, read_i128_varint_2_le);
        read_signed_varint_long_size_func!(ap, ReadIsizeVarint2BeAp, read_isize_varint_2_be_ap, read_i128_varint_2_be);
        read_signed_varint_long_size_func!(ap, ReadIsizeVarint4LeAp, read_isize_varint_4_le_ap, read_i128_varint_4_le);
        read_signed_varint_long_size_func!(ap, ReadIsizeVarint4BeAp, read_isize_varint_4_be_ap, read_i128_varint_4_be);
        read_signed_varint_long_size_func!(ap, ReadIsizeVarint8LeAp, read_isize_varint_8_le_ap, read_i128_varint_8_le);
        read_signed_varint_long_size_func!(ap, ReadIsizeVarint8BeAp, read_isize_varint_8_be_ap, read_i128_varint_8_be);
        read_signed_varint_long_size_func!(ap, ReadIsizeVarint16LeAp, read_isize_varint_16_le_ap, read_i128_varint_16_le);
        read_signed_varint_long_size_func!(ap, ReadIsizeVarint16BeAp, read_isize_varint_16_be_ap, read_i128_varint_16_be);
    };
}

define_read_signed_varint_long_size_future!();

#[cfg(all(feature = "async_signed_varint_long_size", not(feature = "async_varint_long_size"), not(feature = "async_signed_varint_long"), not(feature = "async_varint")))]
compile_error!("developer error: please check Cargo.toml");
