macro_rules! read_raw_future {
    (raw, $primitive: ty, $future: ident, $poll_func: ident, $buf: ident, $struct_buf: ident) => {
        read_raw_future!(cfg(feature = "async_raw"), $primitive, $future, $poll_func, $buf, $struct_buf);
    };
    (raw_size, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident) => {
        read_raw_future!(cfg(feature = "async_raw_size"), $primitive, $future, $poll_func, OwnedReadBuf128, $struct_buf);
    };
    ($feature: meta, $primitive: ty, $future: ident, $poll_func: ident, $buf: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[derive(Debug)]
        struct $struct_buf {
            buf: $buf,
        }
        #[$feature]
        impl $struct_buf {
            fn new() -> Self {
                Self { buf: $buf::new() }
            }
            fn reset(&mut self) {
                self.buf.clear();
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
        impl<'a, R: $crate::AsyncVariableReadable + Unpin+ ?Sized> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$primitive>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                R::$poll_func(Pin::new(&mut *me.reader), cx, me.inner)
            }
        }
    };
}
macro_rules! read_raw_poll {
    (raw, $primitive: ty, $poll_func: ident, $from: ident, $struct_buf: ident) => {
        read_raw_poll!(cfg(feature = "async_raw"), $primitive, $primitive, $poll_func, $from, $struct_buf);
    };
    (raw_size, $primitive: ty, $poll_func: ident, $internal: ident, $from: ident, $struct_buf: ident) => {
        read_raw_poll!(cfg(feature = "async_raw_size"), $internal, $primitive, $poll_func, $from, $struct_buf);
    };
    ($feature: meta, $primitive: ty, $target: ty, $poll_func: ident, $from: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, inner: &mut $struct_buf) -> std::task::Poll<std::io::Result<$target>> {
            let mut ref_buf = (&mut inner.buf).into();
            let res = self.poll_read_more(cx, &mut ref_buf);
            let filled = ref_buf.filled();
            inner.buf.set_filled(filled);
            ready!(res)?;
            Poll::Ready(Ok(<$primitive>::$from(inner.buf.into_inner()) as $target))
        }
    };
}
macro_rules! read_raw_func {
    (raw, $func: ident, $future: ident, $struct_buf: ident) => {
        read_raw_func!(cfg(feature = "async_raw"), $func, $future, $struct_buf);
    };
    (raw_size, $func: ident, $future: ident, $struct_buf: ident) => {
        read_raw_func!(cfg(feature = "async_raw_size"), $func, $future, $struct_buf);
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
macro_rules! define_read_raw_futures {
    () => {
        read_raw_future!(raw, u8, ReadU8Raw, poll_read_u8_raw, OwnedReadBuf8, InternalReadU8Raw);
        read_raw_future!(raw, i8, ReadI8Raw, poll_read_i8_raw, OwnedReadBuf8, InternalReadI8Raw);

        read_raw_future!(raw, u16, ReadU16RawLe, poll_read_u16_raw_le, OwnedReadBuf16, InternalReadU16RawLe);
        read_raw_future!(raw, u16, ReadU16RawBe, poll_read_u16_raw_be, OwnedReadBuf16, InternalReadU16RawBe);
        read_raw_future!(raw, i16, ReadI16RawLe, poll_read_i16_raw_le, OwnedReadBuf16, InternalReadI16RawLe);
        read_raw_future!(raw, i16, ReadI16RawBe, poll_read_i16_raw_be, OwnedReadBuf16, InternalReadI16RawBe);

        read_raw_future!(raw, u32, ReadU32RawLe, poll_read_u32_raw_le, OwnedReadBuf32, InternalReadU32RawLe);
        read_raw_future!(raw, u32, ReadU32RawBe, poll_read_u32_raw_be, OwnedReadBuf32, InternalReadU32RawBe);
        read_raw_future!(raw, i32, ReadI32RawLe, poll_read_i32_raw_le, OwnedReadBuf32, InternalReadI32RawLe);
        read_raw_future!(raw, i32, ReadI32RawBe, poll_read_i32_raw_be, OwnedReadBuf32, InternalReadI32RawBe);

        read_raw_future!(raw, u64, ReadU64RawLe, poll_read_u64_raw_le, OwnedReadBuf64, InternalReadU64RawLe);
        read_raw_future!(raw, u64, ReadU64RawBe, poll_read_u64_raw_be, OwnedReadBuf64, InternalReadU64RawBe);
        read_raw_future!(raw, i64, ReadI64RawLe, poll_read_i64_raw_le, OwnedReadBuf64, InternalReadI64RawLe);
        read_raw_future!(raw, i64, ReadI64RawBe, poll_read_i64_raw_be, OwnedReadBuf64, InternalReadI64RawBe);

        read_raw_future!(raw, u128, ReadU128RawLe, poll_read_u128_raw_le, OwnedReadBuf128, InternalReadU128RawLe);
        read_raw_future!(raw, u128, ReadU128RawBe, poll_read_u128_raw_be, OwnedReadBuf128, InternalReadU128RawBe);
        read_raw_future!(raw, i128, ReadI128RawLe, poll_read_i128_raw_le, OwnedReadBuf128, InternalReadI128RawLe);
        read_raw_future!(raw, i128, ReadI128RawBe, poll_read_i128_raw_be, OwnedReadBuf128, InternalReadI128RawBe);

        read_raw_future!(raw_size, usize, ReadUsizeRawLe, poll_read_usize_raw_le, InternalReadUsizeRawLe);
        read_raw_future!(raw_size, usize, ReadUsizeRawBe, poll_read_usize_raw_be, InternalReadUsizeRawBe);
        read_raw_future!(raw_size, isize, ReadIsizeRawLe, poll_read_isize_raw_le, InternalReadIsizeRawLe);
        read_raw_future!(raw_size, isize, ReadIsizeRawBe, poll_read_isize_raw_be, InternalReadIsizeRawBe);
    };
}
macro_rules! define_read_raw_poll {
    () => {
        read_raw_poll!(raw, u8, poll_read_u8_raw, from_ne_bytes, InternalReadU8Raw);
        read_raw_poll!(raw, i8, poll_read_i8_raw, from_ne_bytes, InternalReadI8Raw);

        read_raw_poll!(raw, u16, poll_read_u16_raw_le, from_le_bytes, InternalReadU16RawLe);
        read_raw_poll!(raw, u16, poll_read_u16_raw_be, from_be_bytes, InternalReadU16RawBe);
        read_raw_poll!(raw, i16, poll_read_i16_raw_le, from_le_bytes, InternalReadI16RawLe);
        read_raw_poll!(raw, i16, poll_read_i16_raw_be, from_be_bytes, InternalReadI16RawBe);

        read_raw_poll!(raw, u32, poll_read_u32_raw_le, from_le_bytes, InternalReadU32RawLe);
        read_raw_poll!(raw, u32, poll_read_u32_raw_be, from_be_bytes, InternalReadU32RawBe);
        read_raw_poll!(raw, i32, poll_read_i32_raw_le, from_le_bytes, InternalReadI32RawLe);
        read_raw_poll!(raw, i32, poll_read_i32_raw_be, from_be_bytes, InternalReadI32RawBe);

        read_raw_poll!(raw, u64, poll_read_u64_raw_le, from_le_bytes, InternalReadU64RawLe);
        read_raw_poll!(raw, u64, poll_read_u64_raw_be, from_be_bytes, InternalReadU64RawBe);
        read_raw_poll!(raw, i64, poll_read_i64_raw_le, from_le_bytes, InternalReadI64RawLe);
        read_raw_poll!(raw, i64, poll_read_i64_raw_be, from_be_bytes, InternalReadI64RawBe);

        read_raw_poll!(raw, u128, poll_read_u128_raw_le, from_le_bytes, InternalReadU128RawLe);
        read_raw_poll!(raw, u128, poll_read_u128_raw_be, from_be_bytes, InternalReadU128RawBe);
        read_raw_poll!(raw, i128, poll_read_i128_raw_le, from_le_bytes, InternalReadI128RawLe);
        read_raw_poll!(raw, i128, poll_read_i128_raw_be, from_be_bytes, InternalReadI128RawBe);

        read_raw_poll!(raw_size, usize, poll_read_usize_raw_le, u128, from_le_bytes, InternalReadUsizeRawLe);
        read_raw_poll!(raw_size, usize, poll_read_usize_raw_be, u128, from_be_bytes, InternalReadUsizeRawBe);
        read_raw_poll!(raw_size, isize, poll_read_isize_raw_le, i128, from_le_bytes, InternalReadIsizeRawLe);
        read_raw_poll!(raw_size, isize, poll_read_isize_raw_be, i128, from_be_bytes, InternalReadIsizeRawBe);
    };
}
macro_rules! define_read_raw_func {
    () => {
        read_raw_func!(raw, read_u8_raw, ReadU8Raw, InternalReadU8Raw);
        read_raw_func!(raw, read_i8_raw, ReadI8Raw, InternalReadI8Raw);

        read_raw_func!(raw, read_u16_raw_le, ReadU16RawLe, InternalReadU16RawLe);
        read_raw_func!(raw, read_u16_raw_be, ReadU16RawBe, InternalReadU16RawBe);
        read_raw_func!(raw, read_i16_raw_le, ReadI16RawLe, InternalReadI16RawLe);
        read_raw_func!(raw, read_i16_raw_be, ReadI16RawBe, InternalReadI16RawBe);

        read_raw_func!(raw, read_u32_raw_le, ReadU32RawLe, InternalReadU32RawLe);
        read_raw_func!(raw, read_u32_raw_be, ReadU32RawBe, InternalReadU32RawBe);
        read_raw_func!(raw, read_i32_raw_le, ReadI32RawLe, InternalReadI32RawLe);
        read_raw_func!(raw, read_i32_raw_be, ReadI32RawBe, InternalReadI32RawBe);

        read_raw_func!(raw, read_u64_raw_le, ReadU64RawLe, InternalReadU64RawLe);
        read_raw_func!(raw, read_u64_raw_be, ReadU64RawBe, InternalReadU64RawBe);
        read_raw_func!(raw, read_i64_raw_le, ReadI64RawLe, InternalReadI64RawLe);
        read_raw_func!(raw, read_i64_raw_be, ReadI64RawBe, InternalReadI64RawBe);

        read_raw_func!(raw, read_u128_raw_le, ReadU128RawLe, InternalReadU128RawLe);
        read_raw_func!(raw, read_u128_raw_be, ReadU128RawBe, InternalReadU128RawBe);
        read_raw_func!(raw, read_i128_raw_le, ReadI128RawLe, InternalReadI128RawLe);
        read_raw_func!(raw, read_i128_raw_be, ReadI128RawBe, InternalReadI128RawBe);

        read_raw_func!(raw_size, read_usize_raw_le, ReadUsizeRawLe, InternalReadUsizeRawLe);
        read_raw_func!(raw_size, read_usize_raw_be, ReadUsizeRawBe, InternalReadUsizeRawBe);
        read_raw_func!(raw_size, read_isize_raw_le, ReadIsizeRawLe, InternalReadIsizeRawLe);
        read_raw_func!(raw_size, read_isize_raw_be, ReadIsizeRawBe, InternalReadIsizeRawBe);
    };
}
define_read_raw_futures!();
