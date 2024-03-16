macro_rules! read_signed_future {
    (varint, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_signed_future!(cfg(feature = "async_signed"), $primitive, $future, $poll_func, $struct_buf, $internal_struct);
    };
    (long_varint, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_signed_future!(cfg(feature = "async_long_signed"), $primitive, $future, $poll_func, $struct_buf, $internal_struct);
    };
    ($feature: meta, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[derive(Debug)]
        struct $struct_buf {
            internal: $internal_struct,
        }
        #[$feature]
        impl $struct_buf {
            fn new() -> Self {
                Self { internal: $internal_struct::new() }
            }
            fn reset(&mut self) {
                self.internal.reset();
            }
        }
        #[$feature]
        $crate::pin_project_lite::pin_project! {
            #[cfg_attr(docsrs, doc($feature))]
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
                inner: $struct_buf,
            }
        }
        #[$feature]
        impl<'a, R: $crate::AsyncVariableReadable + Unpin + ?Sized> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$primitive>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                R::$poll_func(Pin::new(&mut *me.reader), cx, me.inner)
            }
        }
    };
}
macro_rules! read_signed_poll {
    (varint, $primitive: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_signed_poll!(cfg(feature = "async_signed"), $primitive, $poll_func, $poll_internal, $struct_buf);
    };
    (long_varint, $primitive: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_signed_poll!(cfg(feature = "async_long_signed"), $primitive, $poll_func, $poll_internal, $struct_buf);
    };
    ($feature: meta, $primitive: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, inner: &mut $struct_buf) -> std::task::Poll<std::io::Result<$primitive>> {
            use $crate::util::zigzag::Zigzag;
            let varint = ready!(self.$poll_internal(cx, &mut inner.internal))?;
            std::task::Poll::Ready(Ok(varint.zigzag() as $primitive))
        }
    };
}
macro_rules! read_signed_func {
    (varint, $func: ident, $future: ident, $struct_buf: ident) => {
        read_signed_func!(cfg(feature = "async_signed"), $func, $future, $struct_buf);
    };
    (long_varint, $func: ident, $future: ident, $struct_buf: ident) => {
        read_signed_func!(cfg(feature = "async_long_signed"), $func, $future, $struct_buf);
    };
    ($feature: meta, $func: ident, $future: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, inner: $struct_buf::new() }
        }
    };
}
macro_rules! read_signed_size_future {
    (varint, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_signed_size_future!(cfg(all(feature = "async_varint_size", feature = "async_signed")), $future, $poll_func, $struct_buf, $internal_struct);
    };
    (long_varint, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_signed_size_future!(cfg(all(feature = "async_varint_size", feature = "async_long_signed")), $future, $poll_func, $struct_buf, $internal_struct);
    };
    ($feature: meta, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_signed_future!($feature, isize, $future, $poll_func, $struct_buf, $internal_struct);
    };
}
macro_rules! read_signed_size_poll {
    (varint, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_signed_size_poll!(cfg(all(feature = "async_varint_size", feature = "async_signed")), $poll_func, $poll_internal, $struct_buf);
    };
    (long_varint, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_signed_size_poll!(cfg(all(feature = "async_varint_size", feature = "async_long_signed")), $poll_func, $poll_internal, $struct_buf);
    };
    ($feature: meta, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_signed_poll!($feature, isize, $poll_func, $poll_internal, $struct_buf);
    };
}
macro_rules! read_signed_size_func {
    (varint, $func: ident, $future: ident, $struct_buf: ident) => {
        read_signed_size_func!(cfg(all(feature = "async_varint_size", feature = "async_signed")), $func, $future, $struct_buf);
    };
    (long_varint, $func: ident, $future: ident, $struct_buf: ident) => {
        read_signed_size_func!(cfg(all(feature = "async_varint_size", feature = "async_long_signed")), $func, $future, $struct_buf);
    };
    ($feature: meta, $func: ident, $future: ident, $struct_buf: ident) => {
        read_signed_func!($feature, $func, $future, $struct_buf);
    };
}
macro_rules! define_read_signed_futures {
    () => {
        read_signed_future!(long_varint, i8, ReadI8Varint, poll_read_i8_varint, InternalReadI8Varint, InternalReadU8Varint);

        read_signed_future!(varint, i16, ReadI16Varint, poll_read_i16_varint, InternalReadI16Varint, InternalReadU16Varint);
        read_signed_future!(long_varint, i16, ReadI16Varint2Le, poll_read_i16_varint_2_le, InternalReadI16Varint2Le, InternalReadU16Varint2Le);
        read_signed_future!(long_varint, i16, ReadI16Varint2Be, poll_read_i16_varint_2_be, InternalReadI16Varint2Be, InternalReadU16Varint2Be);

        read_signed_future!(varint, i32, ReadI32Varint, poll_read_i32_varint, InternalReadI32Varint, InternalReadU32Varint);
        read_signed_future!(long_varint, i32, ReadI32Varint2Le, poll_read_i32_varint_2_le, InternalReadI32Varint2Le, InternalReadU32Varint2Le);
        read_signed_future!(long_varint, i32, ReadI32Varint2Be, poll_read_i32_varint_2_be, InternalReadI32Varint2Be, InternalReadU32Varint2Be);
        read_signed_future!(long_varint, i32, ReadI32Varint4Le, poll_read_i32_varint_4_le, InternalReadI32Varint4Le, InternalReadU32Varint4Le);
        read_signed_future!(long_varint, i32, ReadI32Varint4Be, poll_read_i32_varint_4_be, InternalReadI32Varint4Be, InternalReadU32Varint4Be);

        read_signed_future!(varint, i64, ReadI64Varint, poll_read_i64_varint, InternalReadI64Varint, InternalReadU64Varint);
        read_signed_future!(long_varint, i64, ReadI64Varint2Le, poll_read_i64_varint_2_le, InternalReadI64Varint2Le, InternalReadU64Varint2Le);
        read_signed_future!(long_varint, i64, ReadI64Varint2Be, poll_read_i64_varint_2_be, InternalReadI64Varint2Be, InternalReadU64Varint2Be);
        read_signed_future!(long_varint, i64, ReadI64Varint4Le, poll_read_i64_varint_4_le, InternalReadI64Varint4Le, InternalReadU64Varint4Le);
        read_signed_future!(long_varint, i64, ReadI64Varint4Be, poll_read_i64_varint_4_be, InternalReadI64Varint4Be, InternalReadU64Varint4Be);
        read_signed_future!(long_varint, i64, ReadI64Varint8Le, poll_read_i64_varint_8_le, InternalReadI64Varint8Le, InternalReadU64Varint8Le);
        read_signed_future!(long_varint, i64, ReadI64Varint8Be, poll_read_i64_varint_8_be, InternalReadI64Varint8Be, InternalReadU64Varint8Be);

        read_signed_future!(varint, i128, ReadI128Varint, poll_read_i128_varint, InternalReadI128Varint, InternalReadU128Varint);
        read_signed_future!(long_varint, i128, ReadI128Varint2Le, poll_read_i128_varint_2_le, InternalReadI128Varint2Le, InternalReadU128Varint2Le);
        read_signed_future!(long_varint, i128, ReadI128Varint2Be, poll_read_i128_varint_2_be, InternalReadI128Varint2Be, InternalReadU128Varint2Be);
        read_signed_future!(long_varint, i128, ReadI128Varint4Le, poll_read_i128_varint_4_le, InternalReadI128Varint4Le, InternalReadU128Varint4Le);
        read_signed_future!(long_varint, i128, ReadI128Varint4Be, poll_read_i128_varint_4_be, InternalReadI128Varint4Be, InternalReadU128Varint4Be);
        read_signed_future!(long_varint, i128, ReadI128Varint8Le, poll_read_i128_varint_8_le, InternalReadI128Varint8Le, InternalReadU128Varint8Le);
        read_signed_future!(long_varint, i128, ReadI128Varint8Be, poll_read_i128_varint_8_be, InternalReadI128Varint8Be, InternalReadU128Varint8Be);
        read_signed_future!(long_varint, i128, ReadI128Varint16Le, poll_read_i128_varint_16_le, InternalReadI128Varint16Le, InternalReadU128Varint16Le);
        read_signed_future!(long_varint, i128, ReadI128Varint16Be, poll_read_i128_varint_16_be, InternalReadI128Varint16Be, InternalReadU128Varint16Be);

        read_signed_size_future!(varint, ReadIsizeVarint, poll_read_isize_varint, InternalReadIsizeVarint, InternalReadUsizeVarint);
        read_signed_size_future!(long_varint, ReadIsizeVarint2Le, poll_read_isize_varint_2_le, InternalReadIsizeVarint2Le, InternalReadUsizeVarint2Le);
        read_signed_size_future!(long_varint, ReadIsizeVarint2Be, poll_read_isize_varint_2_be, InternalReadIsizeVarint2Be, InternalReadUsizeVarint2Be);
        read_signed_size_future!(long_varint, ReadIsizeVarint4Le, poll_read_isize_varint_4_le, InternalReadIsizeVarint4Le, InternalReadUsizeVarint4Le);
        read_signed_size_future!(long_varint, ReadIsizeVarint4Be, poll_read_isize_varint_4_be, InternalReadIsizeVarint4Be, InternalReadUsizeVarint4Be);
        read_signed_size_future!(long_varint, ReadIsizeVarint8Le, poll_read_isize_varint_8_le, InternalReadIsizeVarint8Le, InternalReadUsizeVarint8Le);
        read_signed_size_future!(long_varint, ReadIsizeVarint8Be, poll_read_isize_varint_8_be, InternalReadIsizeVarint8Be, InternalReadUsizeVarint8Be);
        read_signed_size_future!(long_varint, ReadIsizeVarint16Le, poll_read_isize_varint_16_le, InternalReadIsizeVarint16Le, InternalReadUsizeVarint16Le);
        read_signed_size_future!(long_varint, ReadIsizeVarint16Be, poll_read_isize_varint_16_be, InternalReadIsizeVarint16Be, InternalReadUsizeVarint16Be);
    };
}
macro_rules! define_read_signed_poll {
    () => {
        read_signed_poll!(long_varint, i8, poll_read_i8_varint, poll_read_u8_varint, InternalReadI8Varint);

        read_signed_poll!(varint, i16, poll_read_i16_varint, poll_read_u16_varint, InternalReadI16Varint);
        read_signed_poll!(long_varint, i16, poll_read_i16_varint_2_le, poll_read_u16_varint_2_le, InternalReadI16Varint2Le);
        read_signed_poll!(long_varint, i16, poll_read_i16_varint_2_be, poll_read_u16_varint_2_be, InternalReadI16Varint2Be);

        read_signed_poll!(varint, i32, poll_read_i32_varint, poll_read_u32_varint, InternalReadI32Varint);
        read_signed_poll!(long_varint, i32, poll_read_i32_varint_2_le, poll_read_u32_varint_2_le, InternalReadI32Varint2Le);
        read_signed_poll!(long_varint, i32, poll_read_i32_varint_2_be, poll_read_u32_varint_2_be, InternalReadI32Varint2Be);
        read_signed_poll!(long_varint, i32, poll_read_i32_varint_4_le, poll_read_u32_varint_4_le, InternalReadI32Varint4Le);
        read_signed_poll!(long_varint, i32, poll_read_i32_varint_4_be, poll_read_u32_varint_4_be, InternalReadI32Varint4Be);

        read_signed_poll!(varint, i64, poll_read_i64_varint, poll_read_u64_varint, InternalReadI64Varint);
        read_signed_poll!(long_varint, i64, poll_read_i64_varint_2_le, poll_read_u64_varint_2_le, InternalReadI64Varint2Le);
        read_signed_poll!(long_varint, i64, poll_read_i64_varint_2_be, poll_read_u64_varint_2_be, InternalReadI64Varint2Be);
        read_signed_poll!(long_varint, i64, poll_read_i64_varint_4_le, poll_read_u64_varint_4_le, InternalReadI64Varint4Le);
        read_signed_poll!(long_varint, i64, poll_read_i64_varint_4_be, poll_read_u64_varint_4_be, InternalReadI64Varint4Be);
        read_signed_poll!(long_varint, i64, poll_read_i64_varint_8_le, poll_read_u64_varint_8_le, InternalReadI64Varint8Le);
        read_signed_poll!(long_varint, i64, poll_read_i64_varint_8_be, poll_read_u64_varint_8_be, InternalReadI64Varint8Be);

        read_signed_poll!(varint, i128, poll_read_i128_varint, poll_read_u128_varint, InternalReadI128Varint);
        read_signed_poll!(long_varint, i128, poll_read_i128_varint_2_le, poll_read_u128_varint_2_le, InternalReadI128Varint2Le);
        read_signed_poll!(long_varint, i128, poll_read_i128_varint_2_be, poll_read_u128_varint_2_be, InternalReadI128Varint2Be);
        read_signed_poll!(long_varint, i128, poll_read_i128_varint_4_le, poll_read_u128_varint_4_le, InternalReadI128Varint4Le);
        read_signed_poll!(long_varint, i128, poll_read_i128_varint_4_be, poll_read_u128_varint_4_be, InternalReadI128Varint4Be);
        read_signed_poll!(long_varint, i128, poll_read_i128_varint_8_le, poll_read_u128_varint_8_le, InternalReadI128Varint8Le);
        read_signed_poll!(long_varint, i128, poll_read_i128_varint_8_be, poll_read_u128_varint_8_be, InternalReadI128Varint8Be);
        read_signed_poll!(long_varint, i128, poll_read_i128_varint_16_le, poll_read_u128_varint_16_le, InternalReadI128Varint16Le);
        read_signed_poll!(long_varint, i128, poll_read_i128_varint_16_be, poll_read_u128_varint_16_be, InternalReadI128Varint16Be);

        read_signed_size_poll!(varint, poll_read_isize_varint, poll_read_usize_varint, InternalReadIsizeVarint);
        read_signed_size_poll!(long_varint, poll_read_isize_varint_2_le, poll_read_usize_varint_2_le, InternalReadIsizeVarint2Le);
        read_signed_size_poll!(long_varint, poll_read_isize_varint_2_be, poll_read_usize_varint_2_be, InternalReadIsizeVarint2Be);
        read_signed_size_poll!(long_varint, poll_read_isize_varint_4_le, poll_read_usize_varint_4_le, InternalReadIsizeVarint4Le);
        read_signed_size_poll!(long_varint, poll_read_isize_varint_4_be, poll_read_usize_varint_4_be, InternalReadIsizeVarint4Be);
        read_signed_size_poll!(long_varint, poll_read_isize_varint_8_le, poll_read_usize_varint_8_le, InternalReadIsizeVarint8Le);
        read_signed_size_poll!(long_varint, poll_read_isize_varint_8_be, poll_read_usize_varint_8_be, InternalReadIsizeVarint8Be);
        read_signed_size_poll!(long_varint, poll_read_isize_varint_16_le, poll_read_usize_varint_16_le, InternalReadIsizeVarint16Le);
        read_signed_size_poll!(long_varint, poll_read_isize_varint_16_be, poll_read_usize_varint_16_be, InternalReadIsizeVarint16Be);
    };
}
macro_rules! define_read_signed_func {
    () => {
        read_signed_func!(long_varint, read_i8_varint, ReadI8Varint, InternalReadI8Varint);

        read_signed_func!(varint, read_i16_varint, ReadI16Varint, InternalReadI16Varint);
        read_signed_func!(long_varint, read_i16_varint_2_le, ReadI16Varint2Le, InternalReadI16Varint2Le);
        read_signed_func!(long_varint, read_i16_varint_2_be, ReadI16Varint2Be, InternalReadI16Varint2Be);

        read_signed_func!(varint, read_i32_varint, ReadI32Varint, InternalReadI32Varint);
        read_signed_func!(long_varint, read_i32_varint_2_le, ReadI32Varint2Le, InternalReadI32Varint2Le);
        read_signed_func!(long_varint, read_i32_varint_2_be, ReadI32Varint2Be, InternalReadI32Varint2Be);
        read_signed_func!(long_varint, read_i32_varint_4_le, ReadI32Varint4Le, InternalReadI32Varint4Le);
        read_signed_func!(long_varint, read_i32_varint_4_be, ReadI32Varint4Be, InternalReadI32Varint4Be);

        read_signed_func!(varint, read_i64_varint, ReadI64Varint, InternalReadI64Varint);
        read_signed_func!(long_varint, read_i64_varint_2_le, ReadI64Varint2Le, InternalReadI64Varint2Le);
        read_signed_func!(long_varint, read_i64_varint_2_be, ReadI64Varint2Be, InternalReadI64Varint2Be);
        read_signed_func!(long_varint, read_i64_varint_4_le, ReadI64Varint4Le, InternalReadI64Varint4Le);
        read_signed_func!(long_varint, read_i64_varint_4_be, ReadI64Varint4Be, InternalReadI64Varint4Be);
        read_signed_func!(long_varint, read_i64_varint_8_le, ReadI64Varint8Le, InternalReadI64Varint8Le);
        read_signed_func!(long_varint, read_i64_varint_8_be, ReadI64Varint8Be, InternalReadI64Varint8Be);

        read_signed_func!(varint, read_i128_varint, ReadI128Varint, InternalReadI128Varint);
        read_signed_func!(long_varint, read_i128_varint_2_le, ReadI128Varint2Le, InternalReadI128Varint2Le);
        read_signed_func!(long_varint, read_i128_varint_2_be, ReadI128Varint2Be, InternalReadI128Varint2Be);
        read_signed_func!(long_varint, read_i128_varint_4_le, ReadI128Varint4Le, InternalReadI128Varint4Le);
        read_signed_func!(long_varint, read_i128_varint_4_be, ReadI128Varint4Be, InternalReadI128Varint4Be);
        read_signed_func!(long_varint, read_i128_varint_8_le, ReadI128Varint8Le, InternalReadI128Varint8Le);
        read_signed_func!(long_varint, read_i128_varint_8_be, ReadI128Varint8Be, InternalReadI128Varint8Be);
        read_signed_func!(long_varint, read_i128_varint_16_le, ReadI128Varint16Le, InternalReadI128Varint16Le);
        read_signed_func!(long_varint, read_i128_varint_16_be, ReadI128Varint16Be, InternalReadI128Varint16Be);

        read_signed_size_func!(varint, read_isize_varint, ReadIsizeVarint, InternalReadIsizeVarint);
        read_signed_size_func!(long_varint, read_isize_varint_2_le, ReadIsizeVarint2Le, InternalReadIsizeVarint2Le);
        read_signed_size_func!(long_varint, read_isize_varint_2_be, ReadIsizeVarint2Be, InternalReadIsizeVarint2Be);
        read_signed_size_func!(long_varint, read_isize_varint_4_le, ReadIsizeVarint4Le, InternalReadIsizeVarint4Le);
        read_signed_size_func!(long_varint, read_isize_varint_4_be, ReadIsizeVarint4Be, InternalReadIsizeVarint4Be);
        read_signed_size_func!(long_varint, read_isize_varint_8_le, ReadIsizeVarint8Le, InternalReadIsizeVarint8Le);
        read_signed_size_func!(long_varint, read_isize_varint_8_be, ReadIsizeVarint8Be, InternalReadIsizeVarint8Be);
        read_signed_size_func!(long_varint, read_isize_varint_16_le, ReadIsizeVarint16Le, InternalReadIsizeVarint16Le);
        read_signed_size_func!(long_varint, read_isize_varint_16_be, ReadIsizeVarint16Be, InternalReadIsizeVarint16Be);
    };
}
define_read_signed_futures!();
