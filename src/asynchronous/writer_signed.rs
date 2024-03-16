macro_rules! write_signed_future {
    (varint, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_signed_future!(cfg(feature = "async_signed"), $primitive, $future, $poll_func, $struct_buf, $internal_struct);
    };
    (long_varint, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_signed_future!(cfg(feature = "async_long_signed"), $primitive, $future, $poll_func, $struct_buf, $internal_struct);
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
            fn new(num: $primitive) -> Self {
                use $crate::util::zigzag::Zigzag;
                Self { internal: $internal_struct::new(num.zigzag()) }
            }
            fn reset(&mut self, num: $primitive) {
                use $crate::util::zigzag::Zigzag;
                self.internal.reset(num.zigzag());
            }
        }
        #[$feature]
        $crate::pin_project_lite::pin_project! {
            #[cfg_attr(docsrs, doc($feature))]
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                inner: $struct_buf,
            }
        }
        #[$feature]
        impl<'a, W: $crate::AsyncVariableWritable + Unpin + ?Sized> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                W::$poll_func(Pin::new(&mut *me.writer), cx, me.inner)
            }
        }
    };
}
macro_rules! write_signed_poll {
    (varint, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        write_signed_poll!(cfg(feature = "async_signed"), $poll_func, $poll_internal, $struct_buf);
    };
    (long_varint, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        write_signed_poll!(cfg(feature = "async_long_signed"), $poll_func, $poll_internal, $struct_buf);
    };
    ($feature: meta, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, inner: &mut $struct_buf) -> std::task::Poll<std::io::Result<usize>> {
            self.$poll_internal(cx, &mut inner.internal)
        }
    };
}
macro_rules! write_signed_func {
    (varint, $primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        write_signed_func!(cfg(feature = "async_signed"), $primitive, $func, $future, $struct_buf);
    };
    (long_varint, $primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        write_signed_func!(cfg(feature = "async_long_signed"), $primitive, $func, $future, $struct_buf);
    };
    ($feature: meta, $primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            $future { writer: self, inner: $struct_buf::new(num) }
        }
    };
}
macro_rules! write_signed_size_future {
    (varint, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_signed_size_future!(cfg(all(feature = "async_varint_size", feature = "async_signed")), $future, $poll_func, $struct_buf, $internal_struct);
    };
    (long_varint, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_signed_size_future!(cfg(all(feature = "async_varint_size", feature = "async_long_signed")), $future, $poll_func, $struct_buf, $internal_struct);
    };
    ($feature: meta, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_signed_future!($feature, isize, $future, $poll_func, $struct_buf, $internal_struct);
    };
}
macro_rules! write_signed_size_poll {
    (varint, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        write_signed_size_poll!(cfg(all(feature = "async_varint_size", feature = "async_signed")), $poll_func, $poll_internal, $struct_buf);
    };
    (long_varint, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        write_signed_size_poll!(cfg(all(feature = "async_varint_size", feature = "async_long_signed")), $poll_func, $poll_internal, $struct_buf);
    };
    ($feature: meta, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        write_signed_poll!($feature, $poll_func, $poll_internal, $struct_buf);
    };
}
macro_rules! write_signed_size_func {
    (varint, $func: ident, $future: ident, $struct_buf: ident) => {
        write_signed_size_func!(cfg(all(feature = "async_varint_size", feature = "async_signed")), $func, $future, $struct_buf);
    };
    (long_varint, $func: ident, $future: ident, $struct_buf: ident) => {
        write_signed_size_func!(cfg(all(feature = "async_varint_size", feature = "async_long_signed")), $func, $future, $struct_buf);
    };
    ($feature: meta, $func: ident, $future: ident, $struct_buf: ident) => {
        write_signed_func!($feature, isize, $func, $future, $struct_buf);
    };
}
macro_rules! define_write_signed_futures {
    () => {
        write_signed_future!(long_varint, i8, WriteI8Varint, poll_write_i8_varint, InternalWriteI8Varint, InternalWriteU8Varint);

        write_signed_future!(varint, i16, WriteI16Varint, poll_write_i16_varint, InternalWriteI16Varint, InternalWriteU16Varint);
        write_signed_future!(long_varint, i16, WriteI16Varint2Le, poll_write_i16_varint_2_le, InternalWriteI16Varint2Le, InternalWriteU16Varint2Le);
        write_signed_future!(long_varint, i16, WriteI16Varint2Be, poll_write_i16_varint_2_be, InternalWriteI16Varint2Be, InternalWriteU16Varint2Be);

        write_signed_future!(varint, i32, WriteI32Varint, poll_write_i32_varint, InternalWriteI32Varint, InternalWriteU32Varint);
        write_signed_future!(long_varint, i32, WriteI32Varint2Le, poll_write_i32_varint_2_le, InternalWriteI32Varint2Le, InternalWriteU32Varint2Le);
        write_signed_future!(long_varint, i32, WriteI32Varint2Be, poll_write_i32_varint_2_be, InternalWriteI32Varint2Be, InternalWriteU32Varint2Be);
        write_signed_future!(long_varint, i32, WriteI32Varint4Le, poll_write_i32_varint_4_le, InternalWriteI32Varint4Le, InternalWriteU32Varint4Le);
        write_signed_future!(long_varint, i32, WriteI32Varint4Be, poll_write_i32_varint_4_be, InternalWriteI32Varint4Be, InternalWriteU32Varint4Be);

        write_signed_future!(varint, i64, WriteI64Varint, poll_write_i64_varint, InternalWriteI64Varint, InternalWriteU64Varint);
        write_signed_future!(long_varint, i64, WriteI64Varint2Le, poll_write_i64_varint_2_le, InternalWriteI64Varint2Le, InternalWriteU64Varint2Le);
        write_signed_future!(long_varint, i64, WriteI64Varint2Be, poll_write_i64_varint_2_be, InternalWriteI64Varint2Be, InternalWriteU64Varint2Be);
        write_signed_future!(long_varint, i64, WriteI64Varint4Le, poll_write_i64_varint_4_le, InternalWriteI64Varint4Le, InternalWriteU64Varint4Le);
        write_signed_future!(long_varint, i64, WriteI64Varint4Be, poll_write_i64_varint_4_be, InternalWriteI64Varint4Be, InternalWriteU64Varint4Be);
        write_signed_future!(long_varint, i64, WriteI64Varint8Le, poll_write_i64_varint_8_le, InternalWriteI64Varint8Le, InternalWriteU64Varint8Le);
        write_signed_future!(long_varint, i64, WriteI64Varint8Be, poll_write_i64_varint_8_be, InternalWriteI64Varint8Be, InternalWriteU64Varint8Be);

        write_signed_future!(varint, i128, WriteI128Varint, poll_write_i128_varint, InternalWriteI128Varint, InternalWriteU128Varint);
        write_signed_future!(long_varint, i128, WriteI128Varint2Le, poll_write_i128_varint_2_le, InternalWriteI128Varint2Le, InternalWriteU128Varint2Le);
        write_signed_future!(long_varint, i128, WriteI128Varint2Be, poll_write_i128_varint_2_be, InternalWriteI128Varint2Be, InternalWriteU128Varint2Be);
        write_signed_future!(long_varint, i128, WriteI128Varint4Le, poll_write_i128_varint_4_le, InternalWriteI128Varint4Le, InternalWriteU128Varint4Le);
        write_signed_future!(long_varint, i128, WriteI128Varint4Be, poll_write_i128_varint_4_be, InternalWriteI128Varint4Be, InternalWriteU128Varint4Be);
        write_signed_future!(long_varint, i128, WriteI128Varint8Le, poll_write_i128_varint_8_le, InternalWriteI128Varint8Le, InternalWriteU128Varint8Le);
        write_signed_future!(long_varint, i128, WriteI128Varint8Be, poll_write_i128_varint_8_be, InternalWriteI128Varint8Be, InternalWriteU128Varint8Be);
        write_signed_future!(long_varint, i128, WriteI128Varint16Le, poll_write_i128_varint_16_le, InternalWriteI128Varint16Le, InternalWriteU128Varint16Le);
        write_signed_future!(long_varint, i128, WriteI128Varint16Be, poll_write_i128_varint_16_be, InternalWriteI128Varint16Be, InternalWriteU128Varint16Be);

        write_signed_size_future!(varint, WriteIsizeVarint, poll_write_isize_varint, InternalWriteIsizeVarint, InternalWriteUsizeVarint);
        write_signed_size_future!(long_varint, WriteIsizeVarint2Le, poll_write_isize_varint_2_le, InternalWriteIsizeVarint2Le, InternalWriteUsizeVarint2Le);
        write_signed_size_future!(long_varint, WriteIsizeVarint2Be, poll_write_isize_varint_2_be, InternalWriteIsizeVarint2Be, InternalWriteUsizeVarint2Be);
        write_signed_size_future!(long_varint, WriteIsizeVarint4Le, poll_write_isize_varint_4_le, InternalWriteIsizeVarint4Le, InternalWriteUsizeVarint4Le);
        write_signed_size_future!(long_varint, WriteIsizeVarint4Be, poll_write_isize_varint_4_be, InternalWriteIsizeVarint4Be, InternalWriteUsizeVarint4Be);
        write_signed_size_future!(long_varint, WriteIsizeVarint8Le, poll_write_isize_varint_8_le, InternalWriteIsizeVarint8Le, InternalWriteUsizeVarint8Le);
        write_signed_size_future!(long_varint, WriteIsizeVarint8Be, poll_write_isize_varint_8_be, InternalWriteIsizeVarint8Be, InternalWriteUsizeVarint8Be);
        write_signed_size_future!(long_varint, WriteIsizeVarint16Le, poll_write_isize_varint_16_le, InternalWriteIsizeVarint16Le, InternalWriteUsizeVarint16Le);
        write_signed_size_future!(long_varint, WriteIsizeVarint16Be, poll_write_isize_varint_16_be, InternalWriteIsizeVarint16Be, InternalWriteUsizeVarint16Be);
    };
}
macro_rules! define_write_signed_poll {
    () => {
        write_signed_poll!(long_varint, poll_write_i8_varint, poll_write_u8_varint, InternalWriteI8Varint);

        write_signed_poll!(varint, poll_write_i16_varint, poll_write_u16_varint, InternalWriteI16Varint);
        write_signed_poll!(long_varint, poll_write_i16_varint_2_le, poll_write_u16_varint_2_le, InternalWriteI16Varint2Le);
        write_signed_poll!(long_varint, poll_write_i16_varint_2_be, poll_write_u16_varint_2_be, InternalWriteI16Varint2Be);

        write_signed_poll!(varint, poll_write_i32_varint, poll_write_u32_varint, InternalWriteI32Varint);
        write_signed_poll!(long_varint, poll_write_i32_varint_2_le, poll_write_u32_varint_2_le, InternalWriteI32Varint2Le);
        write_signed_poll!(long_varint, poll_write_i32_varint_2_be, poll_write_u32_varint_2_be, InternalWriteI32Varint2Be);
        write_signed_poll!(long_varint, poll_write_i32_varint_4_le, poll_write_u32_varint_4_le, InternalWriteI32Varint4Le);
        write_signed_poll!(long_varint, poll_write_i32_varint_4_be, poll_write_u32_varint_4_be, InternalWriteI32Varint4Be);

        write_signed_poll!(varint, poll_write_i64_varint, poll_write_u64_varint, InternalWriteI64Varint);
        write_signed_poll!(long_varint, poll_write_i64_varint_2_le, poll_write_u64_varint_2_le, InternalWriteI64Varint2Le);
        write_signed_poll!(long_varint, poll_write_i64_varint_2_be, poll_write_u64_varint_2_be, InternalWriteI64Varint2Be);
        write_signed_poll!(long_varint, poll_write_i64_varint_4_le, poll_write_u64_varint_4_le, InternalWriteI64Varint4Le);
        write_signed_poll!(long_varint, poll_write_i64_varint_4_be, poll_write_u64_varint_4_be, InternalWriteI64Varint4Be);
        write_signed_poll!(long_varint, poll_write_i64_varint_8_le, poll_write_u64_varint_8_le, InternalWriteI64Varint8Le);
        write_signed_poll!(long_varint, poll_write_i64_varint_8_be, poll_write_u64_varint_8_be, InternalWriteI64Varint8Be);

        write_signed_poll!(varint, poll_write_i128_varint, poll_write_u128_varint, InternalWriteI128Varint);
        write_signed_poll!(long_varint, poll_write_i128_varint_2_le, poll_write_u128_varint_2_le, InternalWriteI128Varint2Le);
        write_signed_poll!(long_varint, poll_write_i128_varint_2_be, poll_write_u128_varint_2_be, InternalWriteI128Varint2Be);
        write_signed_poll!(long_varint, poll_write_i128_varint_4_le, poll_write_u128_varint_4_le, InternalWriteI128Varint4Le);
        write_signed_poll!(long_varint, poll_write_i128_varint_4_be, poll_write_u128_varint_4_be, InternalWriteI128Varint4Be);
        write_signed_poll!(long_varint, poll_write_i128_varint_8_le, poll_write_u128_varint_8_le, InternalWriteI128Varint8Le);
        write_signed_poll!(long_varint, poll_write_i128_varint_8_be, poll_write_u128_varint_8_be, InternalWriteI128Varint8Be);
        write_signed_poll!(long_varint, poll_write_i128_varint_16_le, poll_write_u128_varint_16_le, InternalWriteI128Varint16Le);
        write_signed_poll!(long_varint, poll_write_i128_varint_16_be, poll_write_u128_varint_16_be, InternalWriteI128Varint16Be);

        write_signed_size_poll!(varint, poll_write_isize_varint, poll_write_usize_varint, InternalWriteIsizeVarint);
        write_signed_size_poll!(long_varint, poll_write_isize_varint_2_le, poll_write_usize_varint_2_le, InternalWriteIsizeVarint2Le);
        write_signed_size_poll!(long_varint, poll_write_isize_varint_2_be, poll_write_usize_varint_2_be, InternalWriteIsizeVarint2Be);
        write_signed_size_poll!(long_varint, poll_write_isize_varint_4_le, poll_write_usize_varint_4_le, InternalWriteIsizeVarint4Le);
        write_signed_size_poll!(long_varint, poll_write_isize_varint_4_be, poll_write_usize_varint_4_be, InternalWriteIsizeVarint4Be);
        write_signed_size_poll!(long_varint, poll_write_isize_varint_8_le, poll_write_usize_varint_8_le, InternalWriteIsizeVarint8Le);
        write_signed_size_poll!(long_varint, poll_write_isize_varint_8_be, poll_write_usize_varint_8_be, InternalWriteIsizeVarint8Be);
        write_signed_size_poll!(long_varint, poll_write_isize_varint_16_le, poll_write_usize_varint_16_le, InternalWriteIsizeVarint16Le);
        write_signed_size_poll!(long_varint, poll_write_isize_varint_16_be, poll_write_usize_varint_16_be, InternalWriteIsizeVarint16Be);
    };
}
macro_rules! define_write_signed_func {
    () => {
        write_signed_func!(long_varint, i8, write_i8_varint, WriteI8Varint, InternalWriteI8Varint);

        write_signed_func!(varint, i16, write_i16_varint, WriteI16Varint, InternalWriteI16Varint);
        write_signed_func!(long_varint, i16, write_i16_varint_2_le, WriteI16Varint2Le, InternalWriteI16Varint2Le);
        write_signed_func!(long_varint, i16, write_i16_varint_2_be, WriteI16Varint2Be, InternalWriteI16Varint2Be);

        write_signed_func!(varint, i32, write_i32_varint, WriteI32Varint, InternalWriteI32Varint);
        write_signed_func!(long_varint, i32, write_i32_varint_2_le, WriteI32Varint2Le, InternalWriteI32Varint2Le);
        write_signed_func!(long_varint, i32, write_i32_varint_2_be, WriteI32Varint2Be, InternalWriteI32Varint2Be);
        write_signed_func!(long_varint, i32, write_i32_varint_4_le, WriteI32Varint4Le, InternalWriteI32Varint4Le);
        write_signed_func!(long_varint, i32, write_i32_varint_4_be, WriteI32Varint4Be, InternalWriteI32Varint4Be);

        write_signed_func!(varint, i64, write_i64_varint, WriteI64Varint, InternalWriteI64Varint);
        write_signed_func!(long_varint, i64, write_i64_varint_2_le, WriteI64Varint2Le, InternalWriteI64Varint2Le);
        write_signed_func!(long_varint, i64, write_i64_varint_2_be, WriteI64Varint2Be, InternalWriteI64Varint2Be);
        write_signed_func!(long_varint, i64, write_i64_varint_4_le, WriteI64Varint4Le, InternalWriteI64Varint4Le);
        write_signed_func!(long_varint, i64, write_i64_varint_4_be, WriteI64Varint4Be, InternalWriteI64Varint4Be);
        write_signed_func!(long_varint, i64, write_i64_varint_8_le, WriteI64Varint8Le, InternalWriteI64Varint8Le);
        write_signed_func!(long_varint, i64, write_i64_varint_8_be, WriteI64Varint8Be, InternalWriteI64Varint8Be);

        write_signed_func!(varint, i128, write_i128_varint, WriteI128Varint, InternalWriteI128Varint);
        write_signed_func!(long_varint, i128, write_i128_varint_2_le, WriteI128Varint2Le, InternalWriteI128Varint2Le);
        write_signed_func!(long_varint, i128, write_i128_varint_2_be, WriteI128Varint2Be, InternalWriteI128Varint2Be);
        write_signed_func!(long_varint, i128, write_i128_varint_4_le, WriteI128Varint4Le, InternalWriteI128Varint4Le);
        write_signed_func!(long_varint, i128, write_i128_varint_4_be, WriteI128Varint4Be, InternalWriteI128Varint4Be);
        write_signed_func!(long_varint, i128, write_i128_varint_8_le, WriteI128Varint8Le, InternalWriteI128Varint8Le);
        write_signed_func!(long_varint, i128, write_i128_varint_8_be, WriteI128Varint8Be, InternalWriteI128Varint8Be);
        write_signed_func!(long_varint, i128, write_i128_varint_16_le, WriteI128Varint16Le, InternalWriteI128Varint16Le);
        write_signed_func!(long_varint, i128, write_i128_varint_16_be, WriteI128Varint16Be, InternalWriteI128Varint16Be);

        write_signed_size_func!(varint, write_isize_varint, WriteIsizeVarint, InternalWriteIsizeVarint);
        write_signed_size_func!(long_varint, write_isize_varint_2_le, WriteIsizeVarint2Le, InternalWriteIsizeVarint2Le);
        write_signed_size_func!(long_varint, write_isize_varint_2_be, WriteIsizeVarint2Be, InternalWriteIsizeVarint2Be);
        write_signed_size_func!(long_varint, write_isize_varint_4_le, WriteIsizeVarint4Le, InternalWriteIsizeVarint4Le);
        write_signed_size_func!(long_varint, write_isize_varint_4_be, WriteIsizeVarint4Be, InternalWriteIsizeVarint4Be);
        write_signed_size_func!(long_varint, write_isize_varint_8_le, WriteIsizeVarint8Le, InternalWriteIsizeVarint8Le);
        write_signed_size_func!(long_varint, write_isize_varint_8_be, WriteIsizeVarint8Be, InternalWriteIsizeVarint8Be);
        write_signed_size_func!(long_varint, write_isize_varint_16_le, WriteIsizeVarint16Le, InternalWriteIsizeVarint16Le);
        write_signed_size_func!(long_varint, write_isize_varint_16_be, WriteIsizeVarint16Be, InternalWriteIsizeVarint16Be);
    };
}
define_write_signed_futures!();
