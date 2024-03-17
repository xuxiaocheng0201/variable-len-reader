macro_rules! write_signed_varint_long_size_future {
    (cp, $future: ident, $inner_future: ident) => {
        write_signed_varint_long_size_future!(f cfg(feature = "async_signed_varint_long_size"), cp, isize, $future, usize, $inner_future);
    };
    (ap, $future: ident, $inner_future: ident) => {
        write_signed_varint_long_size_future!(f cfg(feature = "async_signed_varint_long_size"), ap, isize, $future, i128, $inner_future);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_signed_varint_long_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_size_ap_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
}
macro_rules! write_signed_varint_long_size_func {
    (cp, $future: ident, $func: ident) => {
        write_signed_varint_long_size_func!(f cfg(feature = "async_signed_varint_long_size"), cp, isize, $future, $func);
    };
    (ap, $future: ident, $func: ident) => {
        write_signed_varint_long_size_func!(f cfg(feature = "async_signed_varint_long_size"), ap, isize, $future, $func);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $func: ident) => {
        write_signed_varint_long_func!(f $feature, $primitive, $future, $func);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $func: ident) => {
        write_wrap_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_signed_varint_long_size_future {
    () => {
        write_signed_varint_long_size_future!(cp, WriteIsizeVarint2Le, WriteUsizeVarint2Le);
        write_signed_varint_long_size_future!(cp, WriteIsizeVarint2Be, WriteUsizeVarint2Be);
        write_signed_varint_long_size_future!(cp, WriteIsizeVarint4Le, WriteUsizeVarint4Le);
        write_signed_varint_long_size_future!(cp, WriteIsizeVarint4Be, WriteUsizeVarint4Be);
        write_signed_varint_long_size_future!(cp, WriteIsizeVarint8Le, WriteUsizeVarint8Le);
        write_signed_varint_long_size_future!(cp, WriteIsizeVarint8Be, WriteUsizeVarint8Be);
        write_signed_varint_long_size_future!(cp, WriteIsizeVarint16Le, WriteUsizeVarint16Le);
        write_signed_varint_long_size_future!(cp, WriteIsizeVarint16Be, WriteUsizeVarint16Be);

        write_signed_varint_long_size_future!(ap, WriteIsizeVarint2LeAp, WriteI128Varint2Le);
        write_signed_varint_long_size_future!(ap, WriteIsizeVarint2BeAp, WriteI128Varint2Be);
        write_signed_varint_long_size_future!(ap, WriteIsizeVarint4LeAp, WriteI128Varint4Le);
        write_signed_varint_long_size_future!(ap, WriteIsizeVarint4BeAp, WriteI128Varint4Be);
        write_signed_varint_long_size_future!(ap, WriteIsizeVarint8LeAp, WriteI128Varint8Le);
        write_signed_varint_long_size_future!(ap, WriteIsizeVarint8BeAp, WriteI128Varint8Be);
        write_signed_varint_long_size_future!(ap, WriteIsizeVarint16LeAp, WriteI128Varint16Le);
        write_signed_varint_long_size_future!(ap, WriteIsizeVarint16BeAp, WriteI128Varint16Be);
    };
}
macro_rules! define_write_signed_varint_long_size_func {
    () => {
        write_signed_varint_long_size_func!(cp, WriteIsizeVarint2Le, write_isize_varint_2_le);
        write_signed_varint_long_size_func!(cp, WriteIsizeVarint2Be, write_isize_varint_2_be);
        write_signed_varint_long_size_func!(cp, WriteIsizeVarint4Le, write_isize_varint_4_le);
        write_signed_varint_long_size_func!(cp, WriteIsizeVarint4Be, write_isize_varint_4_be);
        write_signed_varint_long_size_func!(cp, WriteIsizeVarint8Le, write_isize_varint_8_le);
        write_signed_varint_long_size_func!(cp, WriteIsizeVarint8Be, write_isize_varint_8_be);
        write_signed_varint_long_size_func!(cp, WriteIsizeVarint16Le, write_isize_varint_16_le);
        write_signed_varint_long_size_func!(cp, WriteIsizeVarint16Be, write_isize_varint_16_be);

        write_signed_varint_long_size_func!(ap, WriteIsizeVarint2LeAp, write_isize_varint_2_le_ap);
        write_signed_varint_long_size_func!(ap, WriteIsizeVarint2BeAp, write_isize_varint_2_be_ap);
        write_signed_varint_long_size_func!(ap, WriteIsizeVarint4LeAp, write_isize_varint_4_le_ap);
        write_signed_varint_long_size_func!(ap, WriteIsizeVarint4BeAp, write_isize_varint_4_be_ap);
        write_signed_varint_long_size_func!(ap, WriteIsizeVarint8LeAp, write_isize_varint_8_le_ap);
        write_signed_varint_long_size_func!(ap, WriteIsizeVarint8BeAp, write_isize_varint_8_be_ap);
        write_signed_varint_long_size_func!(ap, WriteIsizeVarint16LeAp, write_isize_varint_16_le_ap);
        write_signed_varint_long_size_func!(ap, WriteIsizeVarint16BeAp, write_isize_varint_16_be_ap);
    };
}

define_write_signed_varint_long_size_future!();

#[cfg(all(feature = "async_signed_varint_long_size", not(feature = "async_varint_long_size"), not(feature = "async_signed_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
