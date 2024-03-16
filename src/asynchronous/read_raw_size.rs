macro_rules! read_raw_size_future {
    (cp, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident) => {
        read_raw_size_future!(f cfg(feature = "async_raw_size"), cp, $primitive, $future, $poll_func, $struct_buf);
    };
    (ap, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_raw_size_future!(f cfg(feature = "async_raw_size"), ap, $primitive, $future, $inner_future);
    };
    (f $feature: meta, cp, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident) => {
        read_raw_future!(f $feature, $primitive, $future, $poll_func, OwnedReadBufSize, $struct_buf);
    };
    (f $feature: meta, ap, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_size_ap_feature!(f $future, $primitive, $future, $inner_future);
    };
}
macro_rules! read_raw_size_func {
    ($primitive: ty, $func: ident, $future: ident, $poll_func: ident, $internal: ident, $from: ident, $struct_buf: ident) => {
        read_raw_poll!(cfg(feature = "async_raw_size"), $internal, $primitive, $poll_func, $from, $struct_buf);
    };
}

macro_rules! define_read_raw_size_future {
    () => {
        read_raw_size_future!(cp, usize, ReadUsizeRawLe, poll_read_usize_raw_le, InternalReadUsizeRawLe);
        read_raw_size_future!(cp, usize, ReadUsizeRawBe, poll_read_usize_raw_be, InternalReadUsizeRawBe);
        read_raw_size_future!(cp, isize, ReadIsizeRawLe, poll_read_isize_raw_le, InternalReadIsizeRawLe);
        read_raw_size_future!(cp, isize, ReadIsizeRawBe, poll_read_isize_raw_be, InternalReadIsizeRawBe);

        read_raw_size_future!(ap, usize, ReadUsizeRawLeAp, poll_read_usize_raw_le_ap, InternalReadUsizeRawLeAp);
        read_raw_size_future!(ap, usize, ReadUsizeRawBeAp, poll_read_usize_raw_be_ap, InternalReadUsizeRawBeAp);
        read_raw_size_future!(ap, isize, ReadIsizeRawLeAp, poll_read_isize_raw_le_ap, InternalReadIsizeRawLeAp);
        read_raw_size_future!(ap, isize, ReadIsizeRawBeAp, poll_read_isize_raw_be_ap, InternalReadIsizeRawBeAp);
    };
}
macro_rules! define_read_raw_size_func {
    () => {
        read_raw_func!(usize, read_usize_raw_le, ReadUsizeRawLe, poll_read_usize_raw_le, u128, from_le_bytes, InternalReadUsizeRawLe);
        read_raw_func!(usize, read_usize_raw_be, ReadUsizeRawBe, poll_read_usize_raw_be, u128, from_be_bytes, InternalReadUsizeRawBe);
        read_raw_func!(isize, read_isize_raw_le, ReadIsizeRawLe, poll_read_isize_raw_le, i128, from_le_bytes, InternalReadIsizeRawLe);
        read_raw_func!(isize, read_isize_raw_be, ReadIsizeRawBe, poll_read_isize_raw_be, i128, from_be_bytes, InternalReadIsizeRawBe);
    };
}

define_read_raw_size_future!();
