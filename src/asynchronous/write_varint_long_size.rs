macro_rules! write_varint_long_size_future {
    (cp, $future: ident, $internal: ty, $inner_future: ident) => {
        write_varint_long_size_future!(f cfg(feature = "async_varint_long_size"), cp, usize, $future, $internal, $inner_future);
    };
    (ap, $future: ident, $inner_future: ident) => {
        write_varint_long_size_future!(f cfg(feature = "async_varint_long_size"), ap, usize, $future, u128, $inner_future);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_varint_long_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_size_ap_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
}
macro_rules! write_varint_long_size_func {
    (cp, $future: ident, $func: ident) => {
        write_varint_long_size_func!(f cfg(feature = "async_varint_long_size"), cp, usize, $future, $func);
    };
    (ap, $future: ident, $func: ident) => {
        write_varint_long_size_func!(f cfg(feature = "async_varint_long_size"), ap, usize, $future, $func);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $func: ident) => {
        write_varint_long_func!(f $feature, $primitive, $future, $func);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $func: ident) => {
        write_wrap_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_varint_long_size_future {
    () => {
        write_varint_long_size_future!(cp, WriteUsizeVarint2Le, u16, WriteU16RawLe);
        write_varint_long_size_future!(cp, WriteUsizeVarint2Be, u16, WriteU16RawBe);
        write_varint_long_size_future!(cp, WriteUsizeVarint4Le, u32, WriteU32RawLe);
        write_varint_long_size_future!(cp, WriteUsizeVarint4Be, u32, WriteU32RawBe);
        write_varint_long_size_future!(cp, WriteUsizeVarint8Le, u64, WriteU64RawLe);
        write_varint_long_size_future!(cp, WriteUsizeVarint8Be, u64, WriteU64RawBe);
        write_varint_long_size_future!(cp, WriteUsizeVarint16Le, u128, WriteU128RawLe);
        write_varint_long_size_future!(cp, WriteUsizeVarint16Be, u128, WriteU128RawBe);

        write_varint_long_size_future!(ap, WriteUsizeVarint2LeAp, WriteU128Varint2Le);
        write_varint_long_size_future!(ap, WriteUsizeVarint2BeAp, WriteU128Varint2Be);
        write_varint_long_size_future!(ap, WriteUsizeVarint4LeAp, WriteU128Varint4Le);
        write_varint_long_size_future!(ap, WriteUsizeVarint4BeAp, WriteU128Varint4Be);
        write_varint_long_size_future!(ap, WriteUsizeVarint8LeAp, WriteU128Varint8Le);
        write_varint_long_size_future!(ap, WriteUsizeVarint8BeAp, WriteU128Varint8Be);
        write_varint_long_size_future!(ap, WriteUsizeVarint16LeAp, WriteU128Varint16Le);
        write_varint_long_size_future!(ap, WriteUsizeVarint16BeAp, WriteU128Varint16Be);
    };
}
macro_rules! define_write_varint_long_size_func {
    () => {
        write_varint_long_size_func!(cp, WriteUsizeVarint2Le, write_usize_varint_2_le);
        write_varint_long_size_func!(cp, WriteUsizeVarint2Be, write_usize_varint_2_be);
        write_varint_long_size_func!(cp, WriteUsizeVarint4Le, write_usize_varint_4_le);
        write_varint_long_size_func!(cp, WriteUsizeVarint4Be, write_usize_varint_4_be);
        write_varint_long_size_func!(cp, WriteUsizeVarint8Le, write_usize_varint_8_le);
        write_varint_long_size_func!(cp, WriteUsizeVarint8Be, write_usize_varint_8_be);
        write_varint_long_size_func!(cp, WriteUsizeVarint16Le, write_usize_varint_16_le);
        write_varint_long_size_func!(cp, WriteUsizeVarint16Be, write_usize_varint_16_be);

        write_varint_long_size_func!(ap, WriteUsizeVarint2LeAp, write_usize_varint_2_le_ap);
        write_varint_long_size_func!(ap, WriteUsizeVarint2BeAp, write_usize_varint_2_be_ap);
        write_varint_long_size_func!(ap, WriteUsizeVarint4LeAp, write_usize_varint_4_le_ap);
        write_varint_long_size_func!(ap, WriteUsizeVarint4BeAp, write_usize_varint_4_be_ap);
        write_varint_long_size_func!(ap, WriteUsizeVarint8LeAp, write_usize_varint_8_le_ap);
        write_varint_long_size_func!(ap, WriteUsizeVarint8BeAp, write_usize_varint_8_be_ap);
        write_varint_long_size_func!(ap, WriteUsizeVarint16LeAp, write_usize_varint_16_le_ap);
        write_varint_long_size_func!(ap, WriteUsizeVarint16BeAp, write_usize_varint_16_be_ap);
    };
}

define_write_varint_long_size_future!();

#[cfg(all(feature = "async_varint_long_size", not(feature = "async_raw"), not(feature = "async_varint_long")))]
compile_error!("developer error: please check Cargo.toml");
