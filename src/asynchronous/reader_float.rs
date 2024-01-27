macro_rules! read_float_future {
    (varint, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_float_future!(cfg(feature = "async_float"), $primitive, $future, $poll_func, $struct_buf, $internal_struct);
    };
    (long_varint, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_float_future!(cfg(feature = "async_long_float"), $primitive, $future, $poll_func, $struct_buf, $internal_struct);
    };
    ($feature: meta, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_signed_future!($feature, $primitive, $future, $poll_func, $struct_buf, $internal_struct); // The same code.
    };
}
macro_rules! read_float_poll {
    (varint, $primitive: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_float_poll!(cfg(feature = "async_float"), $primitive, $poll_func, $poll_internal, $struct_buf);
    };
    (long_varint, $primitive: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_float_poll!(cfg(feature = "async_long_float"), $primitive, $poll_func, $poll_internal, $struct_buf);
    };
    ($feature: meta, $primitive: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, inner: &mut $struct_buf) -> std::task::Poll<std::io::Result<$primitive>> {
            let varint = ready!(self.$poll_internal(cx, &mut inner.internal))?;
            std::task::Poll::Ready(Ok(<$primitive>::from_bits(varint)))
        }
    };
}
macro_rules! read_float_func {
    (varint, $func: ident, $future: ident, $struct_buf: ident) => {
        read_float_func!(cfg(feature = "async_float"), $func, $future, $struct_buf);
    };
    (long_varint, $func: ident, $future: ident, $struct_buf: ident) => {
        read_float_func!(cfg(feature = "async_long_float"), $func, $future, $struct_buf);
    };
    ($feature: meta, $func: ident, $future: ident, $struct_buf: ident) => {
        read_signed_func!($feature, $func, $future, $struct_buf); // The same code.
    };
}
macro_rules! define_read_float_futures {
    () => {
        read_float_future!(varint, f32, ReadF32Varint, poll_read_f32_varint, InternalReadF32Varint, InternalReadU32Varint);
        read_float_future!(long_varint, f32, ReadF32Varint2Le, poll_read_f32_varint_2_le, InternalReadF32Varint2Le, InternalReadU32Varint2Le);
        read_float_future!(long_varint, f32, ReadF32Varint2Be, poll_read_f32_varint_2_be, InternalReadF32Varint2Be, InternalReadU32Varint2Be);
        read_float_future!(long_varint, f32, ReadF32Varint4Le, poll_read_f32_varint_4_le, InternalReadF32Varint4Le, InternalReadU32Varint4Le);
        read_float_future!(long_varint, f32, ReadF32Varint4Be, poll_read_f32_varint_4_be, InternalReadF32Varint4Be, InternalReadU32Varint4Be);

        read_float_future!(varint, f64, ReadF64Varint, poll_read_f64_varint, InternalReadF64Varint, InternalReadU64Varint);
        read_float_future!(long_varint, f64, ReadF64Varint2Le, poll_read_f64_varint_2_le, InternalReadF64Varint2Le, InternalReadU64Varint2Le);
        read_float_future!(long_varint, f64, ReadF64Varint2Be, poll_read_f64_varint_2_be, InternalReadF64Varint2Be, InternalReadU64Varint2Be);
        read_float_future!(long_varint, f64, ReadF64Varint4Le, poll_read_f64_varint_4_le, InternalReadF64Varint4Le, InternalReadU64Varint4Le);
        read_float_future!(long_varint, f64, ReadF64Varint4Be, poll_read_f64_varint_4_be, InternalReadF64Varint4Be, InternalReadU64Varint4Be);
        read_float_future!(long_varint, f64, ReadF64Varint8Le, poll_read_f64_varint_8_le, InternalReadF64Varint8Le, InternalReadU64Varint8Le);
        read_float_future!(long_varint, f64, ReadF64Varint8Be, poll_read_f64_varint_8_be, InternalReadF64Varint8Be, InternalReadU64Varint8Be);
    };
}
macro_rules! define_read_float_poll {
    () => {
        read_float_poll!(varint, f32, poll_read_f32_varint, poll_read_u32_varint, InternalReadF32Varint);
        read_float_poll!(long_varint, f32, poll_read_f32_varint_2_le, poll_read_u32_varint_2_le, InternalReadF32Varint2Le);
        read_float_poll!(long_varint, f32, poll_read_f32_varint_2_be, poll_read_u32_varint_2_be, InternalReadF32Varint2Be);
        read_float_poll!(long_varint, f32, poll_read_f32_varint_4_le, poll_read_u32_varint_4_le, InternalReadF32Varint4Le);
        read_float_poll!(long_varint, f32, poll_read_f32_varint_4_be, poll_read_u32_varint_4_be, InternalReadF32Varint4Be);

        read_float_poll!(varint, f64, poll_read_f64_varint, poll_read_u64_varint, InternalReadF64Varint);
        read_float_poll!(long_varint, f64, poll_read_f64_varint_2_le, poll_read_u64_varint_2_le, InternalReadF64Varint2Le);
        read_float_poll!(long_varint, f64, poll_read_f64_varint_2_be, poll_read_u64_varint_2_be, InternalReadF64Varint2Be);
        read_float_poll!(long_varint, f64, poll_read_f64_varint_4_le, poll_read_u64_varint_4_le, InternalReadF64Varint4Le);
        read_float_poll!(long_varint, f64, poll_read_f64_varint_4_be, poll_read_u64_varint_4_be, InternalReadF64Varint4Be);
        read_float_poll!(long_varint, f64, poll_read_f64_varint_8_le, poll_read_u64_varint_8_le, InternalReadF64Varint8Le);
        read_float_poll!(long_varint, f64, poll_read_f64_varint_8_be, poll_read_u64_varint_8_be, InternalReadF64Varint8Be);
    };
}
macro_rules! define_read_float_func {
    () => {
        read_float_func!(varint, read_f32_varint, ReadF32Varint, InternalReadF32Varint);
        read_float_func!(long_varint, read_f32_varint_2_le, ReadF32Varint2Le, InternalReadF32Varint2Le);
        read_float_func!(long_varint, read_f32_varint_2_be, ReadF32Varint2Be, InternalReadF32Varint2Be);
        read_float_func!(long_varint, read_f32_varint_4_le, ReadF32Varint4Le, InternalReadF32Varint4Le);
        read_float_func!(long_varint, read_f32_varint_4_be, ReadF32Varint4Be, InternalReadF32Varint4Be);

        read_float_func!(varint, read_f64_varint, ReadF64Varint, InternalReadF64Varint);
        read_float_func!(long_varint, read_f64_varint_2_le, ReadF64Varint2Le, InternalReadF64Varint2Le);
        read_float_func!(long_varint, read_f64_varint_2_be, ReadF64Varint2Be, InternalReadF64Varint2Be);
        read_float_func!(long_varint, read_f64_varint_4_le, ReadF64Varint4Le, InternalReadF64Varint4Le);
        read_float_func!(long_varint, read_f64_varint_4_be, ReadF64Varint4Be, InternalReadF64Varint4Be);
        read_float_func!(long_varint, read_f64_varint_8_le, ReadF64Varint8Le, InternalReadF64Varint8Le);
        read_float_func!(long_varint, read_f64_varint_8_be, ReadF64Varint8Be, InternalReadF64Varint8Be);
    };
}
define_read_float_futures!();
