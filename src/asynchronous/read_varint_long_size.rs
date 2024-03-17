macro_rules! read_varint_long_size_future {
    (cp, $future: ident, $internal: ty, $inner_future: ident) => {
        read_varint_long_size_future!(f cfg(feature = "async_varint_long_size"), cp, usize, $future, $internal, $inner_future);
    };
    (ap, $future: ident, $inner_future: ident) => {
        read_varint_long_size_future!(f cfg(feature = "async_varint_long_size"), ap, usize, $future, $inner_future);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        read_varint_long_future!(f $feature, $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_size_ap_future!(f $feature, $primitive, $future, $inner_future);
    };
}
macro_rules! read_varint_long_size_func {
    (cp, $future: ident, $func: ident, $inner_func: ident) => {
        read_varint_long_size_func!(f cfg(feature = "async_varint_long_size"), cp, $future, $func, $inner_func);
    };
    (ap, $future: ident, $func: ident, $inner_func: ident) => {
        read_varint_long_size_func!(f cfg(feature = "async_varint_long_size"), ap, $future, $func, $inner_func);
    };
    (f $feature: meta, cp, $future: ident, $func: ident, $inner_func: ident) => {
        read_varint_long_func!(f $feature, $future, $func, $inner_func);
    };
    (f $feature: meta, ap, $future: ident, $func: ident, $inner_func: ident) => {
        read_wrap_func!(f $feature, $future, $func, $inner_func);
    };
}

macro_rules! define_read_varint_long_size_future {
    () => {
        read_varint_long_size_future!(cp, ReadUsizeVarint2Le, u16, ReadU16RawLe);
        read_varint_long_size_future!(cp, ReadUsizeVarint2Be, u16, ReadU16RawBe);
        read_varint_long_size_future!(cp, ReadUsizeVarint4Le, u32, ReadU32RawLe);
        read_varint_long_size_future!(cp, ReadUsizeVarint4Be, u32, ReadU32RawBe);
        read_varint_long_size_future!(cp, ReadUsizeVarint8Le, u64, ReadU64RawLe);
        read_varint_long_size_future!(cp, ReadUsizeVarint8Be, u64, ReadU64RawBe);
        read_varint_long_size_future!(cp, ReadUsizeVarint16Le, u128, ReadU128RawLe);
        read_varint_long_size_future!(cp, ReadUsizeVarint16Be, u128, ReadU128RawBe);

        read_varint_long_size_future!(ap, ReadUsizeVarint2LeAp, ReadU128Varint2Le);
        read_varint_long_size_future!(ap, ReadUsizeVarint2BeAp, ReadU128Varint2Be);
        read_varint_long_size_future!(ap, ReadUsizeVarint4LeAp, ReadU128Varint4Le);
        read_varint_long_size_future!(ap, ReadUsizeVarint4BeAp, ReadU128Varint4Be);
        read_varint_long_size_future!(ap, ReadUsizeVarint8LeAp, ReadU128Varint8Le);
        read_varint_long_size_future!(ap, ReadUsizeVarint8BeAp, ReadU128Varint8Be);
        read_varint_long_size_future!(ap, ReadUsizeVarint16LeAp, ReadU128Varint16Le);
        read_varint_long_size_future!(ap, ReadUsizeVarint16BeAp, ReadU128Varint16Be);
    };
}
macro_rules! define_read_varint_long_size_func {
    () => {
        read_varint_long_size_func!(cp, ReadUsizeVarint2Le, read_usize_varint_2_le, read_u16_raw_le);
        read_varint_long_size_func!(cp, ReadUsizeVarint2Be, read_usize_varint_2_be, read_u16_raw_be);
        read_varint_long_size_func!(cp, ReadUsizeVarint4Le, read_usize_varint_4_le, read_u32_raw_le);
        read_varint_long_size_func!(cp, ReadUsizeVarint4Be, read_usize_varint_4_be, read_u32_raw_be);
        read_varint_long_size_func!(cp, ReadUsizeVarint8Le, read_usize_varint_8_le, read_u64_raw_le);
        read_varint_long_size_func!(cp, ReadUsizeVarint8Be, read_usize_varint_8_be, read_u64_raw_be);
        read_varint_long_size_func!(cp, ReadUsizeVarint16Le, read_usize_varint_16_le, read_u128_raw_le);
        read_varint_long_size_func!(cp, ReadUsizeVarint16Be, read_usize_varint_16_be, read_u128_raw_be);

        read_varint_long_size_func!(ap, ReadUsizeVarint2LeAp, read_usize_varint_2_le_ap, read_u128_varint_2_le);
        read_varint_long_size_func!(ap, ReadUsizeVarint2BeAp, read_usize_varint_2_be_ap, read_u128_varint_2_be);
        read_varint_long_size_func!(ap, ReadUsizeVarint4LeAp, read_usize_varint_4_le_ap, read_u128_varint_4_le);
        read_varint_long_size_func!(ap, ReadUsizeVarint4BeAp, read_usize_varint_4_be_ap, read_u128_varint_4_be);
        read_varint_long_size_func!(ap, ReadUsizeVarint8LeAp, read_usize_varint_8_le_ap, read_u128_varint_8_le);
        read_varint_long_size_func!(ap, ReadUsizeVarint8BeAp, read_usize_varint_8_be_ap, read_u128_varint_8_be);
        read_varint_long_size_func!(ap, ReadUsizeVarint16LeAp, read_usize_varint_16_le_ap, read_u128_varint_16_le);
        read_varint_long_size_func!(ap, ReadUsizeVarint16BeAp, read_usize_varint_16_be_ap, read_u128_varint_16_be);
    };
}

define_read_varint_long_size_future!();

#[cfg(all(feature = "async_varint_long_size", not(feature = "async_raw"), not(feature = "async_varint_long"), not(feature = "async_varint")))]
compile_error!("developer error: please check Cargo.toml");
