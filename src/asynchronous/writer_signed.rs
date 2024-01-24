#[cfg(feature = "async_signed")]
macro_rules! write_signed_future {
    ($primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        #[derive(Debug)]
        struct $struct_buf {
            internal: $internal_struct,
        }
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
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                inner: $struct_buf,
            }
        }
        impl<'a, W: $crate::AsyncVariableWritable + Unpin+ ?Sized> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                W::$poll_func(Pin::new(&mut *me.writer), cx, me.inner)
            }
        }
    };
}
#[cfg(feature = "async_signed")]
macro_rules! write_signed_poll {
    ($poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        #[inline]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, inner: &mut $struct_buf) -> std::task::Poll<std::io::Result<usize>> {
            self.$poll_internal(cx, &mut inner.internal)
        }
    };
}
#[cfg(feature = "async_signed")]
macro_rules! write_signed_func {
    ($primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            $future { writer: self, inner: $struct_buf::new(num) }
        }
    };
}
#[cfg(all(feature = "async_varint_size", feature = "async_signed"))]
macro_rules! write_signed_size_future {
    ($future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_signed_future!(isize, $future, $poll_func, $struct_buf, $internal_struct);
    };
}
#[cfg(all(feature = "async_varint_size", feature = "async_signed"))]
macro_rules! write_signed_size_poll {
    ($poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        write_signed_poll!($poll_func, $poll_internal, $struct_buf);
    };
}
#[cfg(all(feature = "async_varint_size", feature = "async_signed"))]
macro_rules! write_signed_size_func {
    ($func: ident, $future: ident, $struct_buf: ident) => {
        write_signed_func!(isize, $func, $future, $struct_buf);
    };
}
#[cfg(feature = "async_signed")]
macro_rules! define_write_signed_futures {
    () => {
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i8, WriteI8Varint, poll_write_i8_varint, InternalWriteI8Varint, InternalWriteU8Varint);

        write_signed_future!(i16, WriteI16Varint, poll_write_i16_varint, InternalWriteI16Varint, InternalWriteU16Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i16, WriteI16Varint2Le, poll_write_i16_varint_2_le, InternalWriteI16Varint2Le, InternalWriteU16Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i16, WriteI16Varint2Be, poll_write_i16_varint_2_be, InternalWriteI16Varint2Be, InternalWriteU16Varint2Be);

        write_signed_future!(i32, WriteI32Varint, poll_write_i32_varint, InternalWriteI32Varint, InternalWriteU32Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i32, WriteI32Varint2Le, poll_write_i32_varint_2_le, InternalWriteI32Varint2Le, InternalWriteU32Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i32, WriteI32Varint2Be, poll_write_i32_varint_2_be, InternalWriteI32Varint2Be, InternalWriteU32Varint2Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i32, WriteI32Varint4Le, poll_write_i32_varint_4_le, InternalWriteI32Varint4Le, InternalWriteU32Varint4Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i32, WriteI32Varint4Be, poll_write_i32_varint_4_be, InternalWriteI32Varint4Be, InternalWriteU32Varint4Be);

        write_signed_future!(i64, WriteI64Varint, poll_write_i64_varint, InternalWriteI64Varint, InternalWriteU64Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i64, WriteI64Varint2Le, poll_write_i64_varint_2_le, InternalWriteI64Varint2Le, InternalWriteU64Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i64, WriteI64Varint2Be, poll_write_i64_varint_2_be, InternalWriteI64Varint2Be, InternalWriteU64Varint2Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i64, WriteI64Varint4Le, poll_write_i64_varint_4_le, InternalWriteI64Varint4Le, InternalWriteU64Varint4Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i64, WriteI64Varint4Be, poll_write_i64_varint_4_be, InternalWriteI64Varint4Be, InternalWriteU64Varint4Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i64, WriteI64Varint8Le, poll_write_i64_varint_8_le, InternalWriteI64Varint8Le, InternalWriteU64Varint8Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i64, WriteI64Varint8Be, poll_write_i64_varint_8_be, InternalWriteI64Varint8Be, InternalWriteU64Varint8Be);

        write_signed_future!(i128, WriteI128Varint, poll_write_i128_varint, InternalWriteI128Varint, InternalWriteU128Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i128, WriteI128Varint2Le, poll_write_i128_varint_2_le, InternalWriteI128Varint2Le, InternalWriteU128Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i128, WriteI128Varint2Be, poll_write_i128_varint_2_be, InternalWriteI128Varint2Be, InternalWriteU128Varint2Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i128, WriteI128Varint4Le, poll_write_i128_varint_4_le, InternalWriteI128Varint4Le, InternalWriteU128Varint4Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i128, WriteI128Varint4Be, poll_write_i128_varint_4_be, InternalWriteI128Varint4Be, InternalWriteU128Varint4Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i128, WriteI128Varint8Le, poll_write_i128_varint_8_le, InternalWriteI128Varint8Le, InternalWriteU128Varint8Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i128, WriteI128Varint8Be, poll_write_i128_varint_8_be, InternalWriteI128Varint8Be, InternalWriteU128Varint8Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i128, WriteI128Varint16Le, poll_write_i128_varint_16_le, InternalWriteI128Varint16Le, InternalWriteU128Varint16Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(i128, WriteI128Varint16Be, poll_write_i128_varint_16_be, InternalWriteI128Varint16Be, InternalWriteU128Varint16Be);

        #[cfg(feature = "async_varint_size")]
        write_signed_size_future!(WriteIsizeVarint, poll_write_isize_varint, InternalWriteIsizeVarint, InternalWriteUsizeVarint);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_future!(WriteIsizeVarint2Le, poll_write_isize_varint_2_le, InternalWriteIsizeVarint2Le, InternalWriteUsizeVarint2Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_future!(WriteIsizeVarint2Be, poll_write_isize_varint_2_be, InternalWriteIsizeVarint2Be, InternalWriteUsizeVarint2Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_future!(WriteIsizeVarint4Le, poll_write_isize_varint_4_le, InternalWriteIsizeVarint4Le, InternalWriteUsizeVarint4Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_future!(WriteIsizeVarint4Be, poll_write_isize_varint_4_be, InternalWriteIsizeVarint4Be, InternalWriteUsizeVarint4Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_future!(WriteIsizeVarint8Le, poll_write_isize_varint_8_le, InternalWriteIsizeVarint8Le, InternalWriteUsizeVarint8Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_future!(WriteIsizeVarint8Be, poll_write_isize_varint_8_be, InternalWriteIsizeVarint8Be, InternalWriteUsizeVarint8Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_future!(WriteIsizeVarint16Le, poll_write_isize_varint_16_le, InternalWriteIsizeVarint16Le, InternalWriteUsizeVarint16Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_future!(WriteIsizeVarint16Be, poll_write_isize_varint_16_be, InternalWriteIsizeVarint16Be, InternalWriteUsizeVarint16Be);
    };
}
#[cfg(feature = "async_signed")]
macro_rules! define_write_signed_poll {
    () => {
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i8_varint, poll_write_u8_varint, InternalWriteI8Varint);

        write_signed_poll!(poll_write_i16_varint, poll_write_u16_varint, InternalWriteI16Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i16_varint_2_le, poll_write_u16_varint_2_le, InternalWriteI16Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i16_varint_2_be, poll_write_u16_varint_2_be, InternalWriteI16Varint2Be);

        write_signed_poll!(poll_write_i32_varint, poll_write_u32_varint, InternalWriteI32Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i32_varint_2_le, poll_write_u32_varint_2_le, InternalWriteI32Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i32_varint_2_be, poll_write_u32_varint_2_be, InternalWriteI32Varint2Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i32_varint_4_le, poll_write_u32_varint_4_le, InternalWriteI32Varint4Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i32_varint_4_be, poll_write_u32_varint_4_be, InternalWriteI32Varint4Be);

        write_signed_poll!(poll_write_i64_varint, poll_write_u64_varint, InternalWriteI64Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i64_varint_2_le, poll_write_u64_varint_2_le, InternalWriteI64Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i64_varint_2_be, poll_write_u64_varint_2_be, InternalWriteI64Varint2Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i64_varint_4_le, poll_write_u64_varint_4_le, InternalWriteI64Varint4Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i64_varint_4_be, poll_write_u64_varint_4_be, InternalWriteI64Varint4Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i64_varint_8_le, poll_write_u64_varint_8_le, InternalWriteI64Varint8Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i64_varint_8_be, poll_write_u64_varint_8_be, InternalWriteI64Varint8Be);

        write_signed_poll!(poll_write_i128_varint, poll_write_u128_varint, InternalWriteI128Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i128_varint_2_le, poll_write_u128_varint_2_le, InternalWriteI128Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i128_varint_2_be, poll_write_u128_varint_2_be, InternalWriteI128Varint2Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i128_varint_4_le, poll_write_u128_varint_4_le, InternalWriteI128Varint4Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i128_varint_4_be, poll_write_u128_varint_4_be, InternalWriteI128Varint4Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i128_varint_8_le, poll_write_u128_varint_8_le, InternalWriteI128Varint8Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i128_varint_8_be, poll_write_u128_varint_8_be, InternalWriteI128Varint8Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i128_varint_16_le, poll_write_u128_varint_16_le, InternalWriteI128Varint16Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_poll!(poll_write_i128_varint_16_be, poll_write_u128_varint_16_be, InternalWriteI128Varint16Be);

        #[cfg(feature = "async_varint_size")]
        write_signed_size_poll!(poll_write_isize_varint, poll_write_usize_varint, InternalWriteIsizeVarint);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_poll!(poll_write_isize_varint_2_le, poll_write_usize_varint_2_le, InternalWriteIsizeVarint2Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_poll!(poll_write_isize_varint_2_be, poll_write_usize_varint_2_be, InternalWriteIsizeVarint2Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_poll!(poll_write_isize_varint_4_le, poll_write_usize_varint_4_le, InternalWriteIsizeVarint4Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_poll!(poll_write_isize_varint_4_be, poll_write_usize_varint_4_be, InternalWriteIsizeVarint4Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_poll!(poll_write_isize_varint_8_le, poll_write_usize_varint_8_le, InternalWriteIsizeVarint8Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_poll!(poll_write_isize_varint_8_be, poll_write_usize_varint_8_be, InternalWriteIsizeVarint8Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_poll!(poll_write_isize_varint_16_le, poll_write_usize_varint_16_le, InternalWriteIsizeVarint16Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_poll!(poll_write_isize_varint_16_be, poll_write_usize_varint_16_be, InternalWriteIsizeVarint16Be);
    };
}
#[cfg(feature = "async_signed")]
macro_rules! define_write_signed_func {
    () => {
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i8, write_i8_varint, WriteI8Varint, InternalWriteI8Varint);

        write_signed_func!(i16, write_i16_varint, WriteI16Varint, InternalWriteI16Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i16, write_i16_varint_2_le, WriteI16Varint2Le, InternalWriteI16Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i16, write_i16_varint_2_be, WriteI16Varint2Be, InternalWriteI16Varint2Be);

        write_signed_func!(i32, write_i32_varint, WriteI32Varint, InternalWriteI32Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i32, write_i32_varint_2_le, WriteI32Varint2Le, InternalWriteI32Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i32, write_i32_varint_2_be, WriteI32Varint2Be, InternalWriteI32Varint2Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i32, write_i32_varint_4_le, WriteI32Varint4Le, InternalWriteI32Varint4Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i32, write_i32_varint_4_be, WriteI32Varint4Be, InternalWriteI32Varint4Be);

        write_signed_func!(i64, write_i64_varint, WriteI64Varint, InternalWriteI64Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, write_i64_varint_2_le, WriteI64Varint2Le, InternalWriteI64Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, write_i64_varint_2_be, WriteI64Varint2Be, InternalWriteI64Varint2Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, write_i64_varint_4_le, WriteI64Varint4Le, InternalWriteI64Varint4Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, write_i64_varint_4_be, WriteI64Varint4Be, InternalWriteI64Varint4Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, write_i64_varint_8_le, WriteI64Varint8Le, InternalWriteI64Varint8Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, write_i64_varint_8_be, WriteI64Varint8Be, InternalWriteI64Varint8Be);

        write_signed_func!(i128, write_i128_varint, WriteI128Varint, InternalWriteI128Varint);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, write_i128_varint_2_le, WriteI128Varint2Le, InternalWriteI128Varint2Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, write_i128_varint_2_be, WriteI128Varint2Be, InternalWriteI128Varint2Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, write_i128_varint_4_le, WriteI128Varint4Le, InternalWriteI128Varint4Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, write_i128_varint_4_be, WriteI128Varint4Be, InternalWriteI128Varint4Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, write_i128_varint_8_le, WriteI128Varint8Le, InternalWriteI128Varint8Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, write_i128_varint_8_be, WriteI128Varint8Be, InternalWriteI128Varint8Be);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, write_i128_varint_16_le, WriteI128Varint16Le, InternalWriteI128Varint16Le);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, write_i128_varint_16_be, WriteI128Varint16Be, InternalWriteI128Varint16Be);

        write_signed_size_func!(write_isize_varint, WriteIsizeVarint, InternalWriteIsizeVarint);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_func!(write_isize_varint_2_le, WriteIsizeVarint2Le, InternalWriteIsizeVarint2Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_func!(write_isize_varint_2_be, WriteIsizeVarint2Be, InternalWriteIsizeVarint2Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_func!(write_isize_varint_4_le, WriteIsizeVarint4Le, InternalWriteIsizeVarint4Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_func!(write_isize_varint_4_be, WriteIsizeVarint4Be, InternalWriteIsizeVarint4Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_func!(write_isize_varint_8_le, WriteIsizeVarint8Le, InternalWriteIsizeVarint8Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_func!(write_isize_varint_8_be, WriteIsizeVarint8Be, InternalWriteIsizeVarint8Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_func!(write_isize_varint_16_le, WriteIsizeVarint16Le, InternalWriteIsizeVarint16Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        write_signed_size_func!(write_isize_varint_16_be, WriteIsizeVarint16Be, InternalWriteIsizeVarint16Be);
    };
}
#[cfg(feature = "async_signed")]
define_write_signed_futures!();
