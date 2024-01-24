#[cfg(feature = "async_signed")]
macro_rules! read_signed_future {
    ($primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        #[derive(Debug)]
        struct $struct_buf {
            internal: $internal_struct,
        }
        impl $struct_buf {
            fn new() -> Self {
                Self { internal: $internal_struct::new() }
            }
        }
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
                inner: $struct_buf,
            }
        }
        impl<'a, R: $crate::asynchronous::AsyncVariableReadable + Unpin> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$primitive>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                R::$poll_func(Pin::new(&mut *me.reader), cx, me.inner)
            }
        }
    };
}
#[cfg(feature = "async_signed")]
macro_rules! read_signed_poll {
    ($primitive: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        #[inline]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, inner: &mut $struct_buf) -> std::task::Poll<std::io::Result<$primitive>> {
            use $crate::util::zigzag::Zigzag;
            let varint = ready!(self.$poll_internal(cx, &mut inner.internal))?;
            Poll::Ready(Ok(varint.zigzag()))
        }
    };
}
#[cfg(feature = "async_signed")]
macro_rules! read_signed_func {
    ($func: ident, $future: ident, $struct_buf: ident) => {
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, inner: $struct_buf::new() }
        }
    };
}
#[cfg(feature = "async_signed")]
macro_rules! define_read_signed_futures {
    () => {
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i8, ReadI8Varint, poll_read_i8_varint, InternalReadI8Varint, InternalReadU8Varint);

        read_signed_future!(i16, ReadI16Varint, poll_read_i16_varint, InternalReadI16Varint, InternalReadU16Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i16, ReadI16Varint2Le, poll_read_i16_varint_2_le, InternalReadI16Varint2Le, InternalReadU16Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i16, ReadI16Varint2Be, poll_read_i16_varint_2_be, InternalReadI16Varint2Be, InternalReadU16Varint2Be);

        read_signed_future!(i32, ReadI32Varint, poll_read_i32_varint, InternalReadI32Varint, InternalReadU32Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i32, ReadI32Varint2Le, poll_read_i32_varint_2_le, InternalReadI32Varint2Le, InternalReadU32Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i32, ReadI32Varint2Be, poll_read_i32_varint_2_be, InternalReadI32Varint2Be, InternalReadU32Varint2Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i32, ReadI32Varint4Le, poll_read_i32_varint_4_le, InternalReadI32Varint4Le, InternalReadU32Varint4Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i32, ReadI32Varint4Be, poll_read_i32_varint_4_be, InternalReadI32Varint4Be, InternalReadU32Varint4Be);

        read_signed_future!(i64, ReadI64Varint, poll_read_i64_varint, InternalReadI64Varint, InternalReadU64Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i64, ReadI64Varint2Le, poll_read_i64_varint_2_le, InternalReadI64Varint2Le, InternalReadU64Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i64, ReadI64Varint2Be, poll_read_i64_varint_2_be, InternalReadI64Varint2Be, InternalReadU64Varint2Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i64, ReadI64Varint4Le, poll_read_i64_varint_4_le, InternalReadI64Varint4Le, InternalReadU64Varint4Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i64, ReadI64Varint4Be, poll_read_i64_varint_4_be, InternalReadI64Varint4Be, InternalReadU64Varint4Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i64, ReadI64Varint8Le, poll_read_i64_varint_8_le, InternalReadI64Varint8Le, InternalReadU64Varint8Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i64, ReadI64Varint8Be, poll_read_i64_varint_8_be, InternalReadI64Varint8Be, InternalReadU64Varint8Be);

        read_signed_future!(i128, ReadI128Varint, poll_read_i128_varint, InternalReadI128Varint, InternalReadU128Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i128, ReadI128Varint2Le, poll_read_i128_varint_2_le, InternalReadI128Varint2Le, InternalReadU128Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i128, ReadI128Varint2Be, poll_read_i128_varint_2_be, InternalReadI128Varint2Be, InternalReadU128Varint2Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i128, ReadI128Varint4Le, poll_read_i128_varint_4_le, InternalReadI128Varint4Le, InternalReadU128Varint4Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i128, ReadI128Varint4Be, poll_read_i128_varint_4_be, InternalReadI128Varint4Be, InternalReadU128Varint4Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i128, ReadI128Varint8Le, poll_read_i128_varint_8_le, InternalReadI128Varint8Le, InternalReadU128Varint8Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i128, ReadI128Varint8Be, poll_read_i128_varint_8_be, InternalReadI128Varint8Be, InternalReadU128Varint8Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i128, ReadI128Varint16Le, poll_read_i128_varint_16_le, InternalReadI128Varint16Le, InternalReadU128Varint16Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_future!(i128, ReadI128Varint16Be, poll_read_i128_varint_16_be, InternalReadI128Varint16Be, InternalReadU128Varint16Be);

        // #[cfg(feature = "async_varint_size")]
        // read_varint_size_future!(ReadUsizeVarint, poll_read_usize_varint, OwnedReadBuf8, InternalReadIsizeVarint, InternalReadUsizeVarint);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_future!(ReadUsizeVarint2Le, poll_read_usize_varint_2_le, OwnedReadBuf16, InternalReadIsizeVarint2Le, InternalReadUsizeVarint2Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_future!(ReadUsizeVarint2Be, poll_read_usize_varint_2_be, OwnedReadBuf16, InternalReadIsizeVarint2Be, InternalReadUsizeVarint2Be);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_future!(ReadUsizeVarint4Le, poll_read_usize_varint_4_le, OwnedReadBuf32, InternalReadIsizeVarint4Le, InternalReadUsizeVarint4Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_future!(ReadUsizeVarint4Be, poll_read_usize_varint_4_be, OwnedReadBuf32, InternalReadIsizeVarint4Be, InternalReadUsizeVarint4Be);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_future!(ReadUsizeVarint8Le, poll_read_usize_varint_8_le, OwnedReadBuf64, InternalReadIsizeVarint8Le, InternalReadUsizeVarint8Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_future!(ReadUsizeVarint8Be, poll_read_usize_varint_8_be, OwnedReadBuf64, InternalReadIsizeVarint8Be, InternalReadUsizeVarint8Be);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_future!(ReadUsizeVarint16Le, poll_read_usize_varint_16_le, OwnedReadBuf128, InternalReadIsizeVarint16Le, InternalReadUsizeVarint16Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_future!(ReadUsizeVarint16Be, poll_read_usize_varint_16_be, OwnedReadBuf128, InternalReadIsizeVarint16Be, InternalReadUsizeVarint16Be);
    };
}
#[cfg(feature = "async_signed")]
macro_rules! define_read_signed_poll {
    () => {
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i8, poll_read_i8_varint, poll_read_u8_varint, InternalReadI8Varint);

        read_signed_poll!(i16, poll_read_i16_varint, poll_read_u16_varint, InternalReadI16Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i16, poll_read_i16_varint_2_le, poll_read_u16_varint_2_le, InternalReadI16Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i16, poll_read_i16_varint_2_be, poll_read_u16_varint_2_be, InternalReadI16Varint2Be);

        read_signed_poll!(i32, poll_read_i32_varint, poll_read_u32_varint, InternalReadI32Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i32, poll_read_i32_varint_2_le, poll_read_u32_varint_2_le, InternalReadI32Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i32, poll_read_i32_varint_2_be, poll_read_u32_varint_2_be, InternalReadI32Varint2Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i32, poll_read_i32_varint_4_le, poll_read_u32_varint_4_le, InternalReadI32Varint4Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i32, poll_read_i32_varint_4_be, poll_read_u32_varint_4_be, InternalReadI32Varint4Be);

        read_signed_poll!(i64, poll_read_i64_varint, poll_read_u64_varint, InternalReadI64Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i64, poll_read_i64_varint_2_le, poll_read_u64_varint_2_le, InternalReadI64Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i64, poll_read_i64_varint_2_be, poll_read_u64_varint_2_be, InternalReadI64Varint2Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i64, poll_read_i64_varint_4_le, poll_read_u64_varint_4_le, InternalReadI64Varint4Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i64, poll_read_i64_varint_4_be, poll_read_u64_varint_4_be, InternalReadI64Varint4Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i64, poll_read_i64_varint_8_le, poll_read_u64_varint_8_le, InternalReadI64Varint8Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i64, poll_read_i64_varint_8_be, poll_read_u64_varint_8_be, InternalReadI64Varint8Be);

        read_signed_poll!(i128, poll_read_i128_varint, poll_read_u128_varint, InternalReadI128Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i128, poll_read_i128_varint_2_le, poll_read_u128_varint_2_le, InternalReadI128Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i128, poll_read_i128_varint_2_be, poll_read_u128_varint_2_be, InternalReadI128Varint2Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i128, poll_read_i128_varint_4_le, poll_read_u128_varint_4_le, InternalReadI128Varint4Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i128, poll_read_i128_varint_4_be, poll_read_u128_varint_4_be, InternalReadI128Varint4Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i128, poll_read_i128_varint_8_le, poll_read_u128_varint_8_le, InternalReadI128Varint8Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i128, poll_read_i128_varint_8_be, poll_read_u128_varint_8_be, InternalReadI128Varint8Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i128, poll_read_i128_varint_16_le, poll_read_u128_varint_16_le, InternalReadI128Varint16Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_poll!(i128, poll_read_i128_varint_16_be, poll_read_u128_varint_16_be, InternalReadI128Varint16Be);

        // #[cfg(feature = "async_varint_size")]
        // read_varint_size_poll!(read_usize_varint, u8, poll_read_usize_varint, poll_read_u8_raw, InternalReadIsizeVarint);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_poll!(read_usize_varint_2_le, u16, poll_read_usize_varint_2_le, poll_read_u16_raw_le, InternalReadIsizeVarint2Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_poll!(read_usize_varint_2_be, u16, poll_read_usize_varint_2_be, poll_read_u16_raw_be, InternalReadIsizeVarint2Be);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_poll!(read_usize_varint_4_le, u32, poll_read_usize_varint_4_le, poll_read_u32_raw_le, InternalReadIsizeVarint4Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_poll!(read_usize_varint_4_be, u32, poll_read_usize_varint_4_be, poll_read_u32_raw_be, InternalReadIsizeVarint4Be);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_poll!(read_usize_varint_8_le, u64, poll_read_usize_varint_8_le, poll_read_u64_raw_le, InternalReadIsizeVarint8Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_poll!(read_usize_varint_8_be, u64, poll_read_usize_varint_8_be, poll_read_u64_raw_be, InternalReadIsizeVarint8Be);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_poll!(read_usize_varint_16_le, u128, poll_read_usize_varint_16_le, poll_read_u128_raw_le, InternalReadIsizeVarint16Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_poll!(read_usize_varint_16_be, u128, poll_read_usize_varint_16_be, poll_read_u128_raw_be, InternalReadIsizeVarint16Be);
    };
}
#[cfg(feature = "async_signed")]
macro_rules! define_read_signed_func {
    () => {
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i8_varint, ReadI8Varint, InternalReadI8Varint);

        read_signed_func!(read_i16_varint, ReadI16Varint, InternalReadI16Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i16_varint_2_le, ReadI16Varint2Le, InternalReadI16Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i16_varint_2_be, ReadI16Varint2Be, InternalReadI16Varint2Be);

        read_signed_func!(read_i32_varint, ReadI32Varint, InternalReadI32Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i32_varint_2_le, ReadI32Varint2Le, InternalReadI32Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i32_varint_2_be, ReadI32Varint2Be, InternalReadI32Varint2Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i32_varint_4_le, ReadI32Varint4Le, InternalReadI32Varint4Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i32_varint_4_be, ReadI32Varint4Be, InternalReadI32Varint4Be);

        read_signed_func!(read_i64_varint, ReadI64Varint, InternalReadI64Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i64_varint_2_le, ReadI64Varint2Le, InternalReadI64Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i64_varint_2_be, ReadI64Varint2Be, InternalReadI64Varint2Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i64_varint_4_le, ReadI64Varint4Le, InternalReadI64Varint4Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i64_varint_4_be, ReadI64Varint4Be, InternalReadI64Varint4Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i64_varint_8_le, ReadI64Varint8Le, InternalReadI64Varint8Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i64_varint_8_be, ReadI64Varint8Be, InternalReadI64Varint8Be);

        read_signed_func!(read_i128_varint, ReadI128Varint, InternalReadI128Varint);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i128_varint_2_le, ReadI128Varint2Le, InternalReadI128Varint2Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i128_varint_2_be, ReadI128Varint2Be, InternalReadI128Varint2Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i128_varint_4_le, ReadI128Varint4Le, InternalReadI128Varint4Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i128_varint_4_be, ReadI128Varint4Be, InternalReadI128Varint4Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i128_varint_8_le, ReadI128Varint8Le, InternalReadI128Varint8Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i128_varint_8_be, ReadI128Varint8Be, InternalReadI128Varint8Be);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i128_varint_16_le, ReadI128Varint16Le, InternalReadI128Varint16Le);
        #[cfg(feature = "async_long_signed")]
        read_signed_func!(read_i128_varint_16_be, ReadI128Varint16Be, InternalReadI128Varint16Be);

        // #[cfg(feature = "async_varint_size")]
        // read_varint_size_func!(read_usize_varint, ReadUsizeVarint, InternalReadIsizeVarint);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_func!(read_usize_varint_2_le, ReadUsizeVarint2Le, InternalReadIsizeVarint2Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_func!(read_usize_varint_2_be, ReadUsizeVarint2Be, InternalReadIsizeVarint2Be);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_func!(read_usize_varint_4_le, ReadUsizeVarint4Le, InternalReadIsizeVarint4Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_func!(read_usize_varint_4_be, ReadUsizeVarint4Be, InternalReadIsizeVarint4Be);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_func!(read_usize_varint_8_le, ReadUsizeVarint8Le, InternalReadIsizeVarint8Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_func!(read_usize_varint_8_be, ReadUsizeVarint8Be, InternalReadIsizeVarint8Be);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_func!(read_usize_varint_16_le, ReadUsizeVarint16Le, InternalReadUsizeVarint16Le);
        // #[cfg(all(feature = "async_varint_size", feature = "async_long_signed"))]
        // read_varint_size_func!(read_usize_varint_16_be, ReadUsizeVarint16Be, InternalReadUsizeVarint16Be);
    };
}
#[cfg(feature = "async_signed")]
define_read_signed_futures!();