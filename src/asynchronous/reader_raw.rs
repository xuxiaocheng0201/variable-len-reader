#[cfg(feature = "async_raw")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_raw")))]
macro_rules! read_raw_future {
    ($primitive: ty, $future: ident, $poll_func: ident, $buf: ident, $struct_buf: ident) => {
        #[derive(Debug)]
        struct $struct_buf {
            buf: $buf,
        }
        impl $struct_buf {
            fn new() -> Self {
                Self { buf: $buf::new() }
            }
            fn reset(&mut self) {
                self.buf.clear();
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
        impl<'a, R: $crate::AsyncVariableReadable + Unpin+ ?Sized> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$primitive>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                R::$poll_func(Pin::new(&mut *me.reader), cx, me.inner)
            }
        }
    };
}
#[cfg(feature = "async_raw")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_raw")))]
macro_rules! read_raw_poll {
    ($primitive: ty, $poll_func: ident, $from: ident, $struct_buf: ident) => {
        read_raw_poll!($primitive, $primitive, $poll_func, $from, $struct_buf);
    };
    ($primitive: ty, $target: ty, $poll_func: ident, $from: ident, $struct_buf: ident) => {
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
#[cfg(feature = "async_raw")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_raw")))]
macro_rules! read_raw_func {
    ($func: ident, $future: ident, $struct_buf: ident) => {
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, inner: $struct_buf::new() }
        }
    };
}
#[cfg(feature = "async_raw_size")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
macro_rules! read_raw_size_future {
    ($primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident) => {
        read_raw_future!($primitive, $future, $poll_func, OwnedReadBuf128, $struct_buf);
    };
}
#[cfg(feature = "async_raw_size")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
macro_rules! read_raw_size_poll {
    ($primitive: ty, $poll_func: ident, $internal: ident, $from: ident, $struct_buf: ident) => {
        read_raw_poll!($internal, $primitive, $poll_func, $from, $struct_buf);
    };
}
#[cfg(feature = "async_raw_size")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
macro_rules! read_raw_size_func {
    ($func: ident, $future: ident, $struct_buf: ident) => {
        read_raw_func!($func, $future, $struct_buf);
    };
}
#[cfg(feature = "async_raw")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_raw")))]
macro_rules! define_read_raw_futures {
    () => {
        read_raw_future!(u8, ReadU8Raw, poll_read_u8_raw, OwnedReadBuf8, InternalReadU8Raw);
        read_raw_future!(i8, ReadI8Raw, poll_read_i8_raw, OwnedReadBuf8, InternalReadI8Raw);

        read_raw_future!(u16, ReadU16RawLe, poll_read_u16_raw_le, OwnedReadBuf16, InternalReadU16RawLe);
        read_raw_future!(u16, ReadU16RawBe, poll_read_u16_raw_be, OwnedReadBuf16, InternalReadU16RawBe);
        read_raw_future!(i16, ReadI16RawLe, poll_read_i16_raw_le, OwnedReadBuf16, InternalReadI16RawLe);
        read_raw_future!(i16, ReadI16RawBe, poll_read_i16_raw_be, OwnedReadBuf16, InternalReadI16RawBe);

        read_raw_future!(u32, ReadU32RawLe, poll_read_u32_raw_le, OwnedReadBuf32, InternalReadU32RawLe);
        read_raw_future!(u32, ReadU32RawBe, poll_read_u32_raw_be, OwnedReadBuf32, InternalReadU32RawBe);
        read_raw_future!(i32, ReadI32RawLe, poll_read_i32_raw_le, OwnedReadBuf32, InternalReadI32RawLe);
        read_raw_future!(i32, ReadI32RawBe, poll_read_i32_raw_be, OwnedReadBuf32, InternalReadI32RawBe);

        read_raw_future!(u64, ReadU64RawLe, poll_read_u64_raw_le, OwnedReadBuf64, InternalReadU64RawLe);
        read_raw_future!(u64, ReadU64RawBe, poll_read_u64_raw_be, OwnedReadBuf64, InternalReadU64RawBe);
        read_raw_future!(i64, ReadI64RawLe, poll_read_i64_raw_le, OwnedReadBuf64, InternalReadI64RawLe);
        read_raw_future!(i64, ReadI64RawBe, poll_read_i64_raw_be, OwnedReadBuf64, InternalReadI64RawBe);

        read_raw_future!(u128, ReadU128RawLe, poll_read_u128_raw_le, OwnedReadBuf128, InternalReadU128RawLe);
        read_raw_future!(u128, ReadU128RawBe, poll_read_u128_raw_be, OwnedReadBuf128, InternalReadU128RawBe);
        read_raw_future!(i128, ReadI128RawLe, poll_read_i128_raw_le, OwnedReadBuf128, InternalReadI128RawLe);
        read_raw_future!(i128, ReadI128RawBe, poll_read_i128_raw_be, OwnedReadBuf128, InternalReadI128RawBe);

        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_future!(usize, ReadUsizeRawLe, poll_read_usize_raw_le, InternalReadUsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_future!(usize, ReadUsizeRawBe, poll_read_usize_raw_be, InternalReadUsizeRawBe);
        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_future!(isize, ReadIsizeRawLe, poll_read_isize_raw_le, InternalReadIsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_future!(isize, ReadIsizeRawBe, poll_read_isize_raw_be, InternalReadIsizeRawBe);
    };
}
#[cfg(feature = "async_raw")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_raw")))]
macro_rules! define_read_raw_poll {
    () => {
        read_raw_poll!(u8, poll_read_u8_raw, from_ne_bytes, InternalReadU8Raw);
        read_raw_poll!(i8, poll_read_i8_raw, from_ne_bytes, InternalReadI8Raw);

        read_raw_poll!(u16, poll_read_u16_raw_le, from_le_bytes, InternalReadU16RawLe);
        read_raw_poll!(u16, poll_read_u16_raw_be, from_be_bytes, InternalReadU16RawBe);
        read_raw_poll!(i16, poll_read_i16_raw_le, from_le_bytes, InternalReadI16RawLe);
        read_raw_poll!(i16, poll_read_i16_raw_be, from_be_bytes, InternalReadI16RawBe);

        read_raw_poll!(u32, poll_read_u32_raw_le, from_le_bytes, InternalReadU32RawLe);
        read_raw_poll!(u32, poll_read_u32_raw_be, from_be_bytes, InternalReadU32RawBe);
        read_raw_poll!(i32, poll_read_i32_raw_le, from_le_bytes, InternalReadI32RawLe);
        read_raw_poll!(i32, poll_read_i32_raw_be, from_be_bytes, InternalReadI32RawBe);

        read_raw_poll!(u64, poll_read_u64_raw_le, from_le_bytes, InternalReadU64RawLe);
        read_raw_poll!(u64, poll_read_u64_raw_be, from_be_bytes, InternalReadU64RawBe);
        read_raw_poll!(i64, poll_read_i64_raw_le, from_le_bytes, InternalReadI64RawLe);
        read_raw_poll!(i64, poll_read_i64_raw_be, from_be_bytes, InternalReadI64RawBe);

        read_raw_poll!(u128, poll_read_u128_raw_le, from_le_bytes, InternalReadU128RawLe);
        read_raw_poll!(u128, poll_read_u128_raw_be, from_be_bytes, InternalReadU128RawBe);
        read_raw_poll!(i128, poll_read_i128_raw_le, from_le_bytes, InternalReadI128RawLe);
        read_raw_poll!(i128, poll_read_i128_raw_be, from_be_bytes, InternalReadI128RawBe);

        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_poll!(usize, poll_read_usize_raw_le, u128, from_le_bytes, InternalReadUsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_poll!(usize, poll_read_usize_raw_be, u128, from_be_bytes, InternalReadUsizeRawBe);
        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_poll!(isize, poll_read_isize_raw_le, i128, from_le_bytes, InternalReadIsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_poll!(isize, poll_read_isize_raw_be, i128, from_be_bytes, InternalReadIsizeRawBe);
    };
}
#[cfg(feature = "async_raw")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_raw")))]
macro_rules! define_read_raw_func {
    () => {
        read_raw_func!(read_u8_raw, ReadU8Raw, InternalReadU8Raw);
        read_raw_func!(read_i8_raw, ReadI8Raw, InternalReadI8Raw);

        read_raw_func!(read_u16_raw_le, ReadU16RawLe, InternalReadU16RawLe);
        read_raw_func!(read_u16_raw_be, ReadU16RawBe, InternalReadU16RawBe);
        read_raw_func!(read_i16_raw_le, ReadI16RawLe, InternalReadI16RawLe);
        read_raw_func!(read_i16_raw_be, ReadI16RawBe, InternalReadI16RawBe);

        read_raw_func!(read_u32_raw_le, ReadU32RawLe, InternalReadU32RawLe);
        read_raw_func!(read_u32_raw_be, ReadU32RawBe, InternalReadU32RawBe);
        read_raw_func!(read_i32_raw_le, ReadI32RawLe, InternalReadI32RawLe);
        read_raw_func!(read_i32_raw_be, ReadI32RawBe, InternalReadI32RawBe);

        read_raw_func!(read_u64_raw_le, ReadU64RawLe, InternalReadU64RawLe);
        read_raw_func!(read_u64_raw_be, ReadU64RawBe, InternalReadU64RawBe);
        read_raw_func!(read_i64_raw_le, ReadI64RawLe, InternalReadI64RawLe);
        read_raw_func!(read_i64_raw_be, ReadI64RawBe, InternalReadI64RawBe);

        read_raw_func!(read_u128_raw_le, ReadU128RawLe, InternalReadU128RawLe);
        read_raw_func!(read_u128_raw_be, ReadU128RawBe, InternalReadU128RawBe);
        read_raw_func!(read_i128_raw_le, ReadI128RawLe, InternalReadI128RawLe);
        read_raw_func!(read_i128_raw_be, ReadI128RawBe, InternalReadI128RawBe);

        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_func!(read_usize_raw_le, ReadUsizeRawLe, InternalReadUsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_func!(read_usize_raw_be, ReadUsizeRawBe, InternalReadUsizeRawBe);
        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_func!(read_isize_raw_le, ReadIsizeRawLe, InternalReadIsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_raw_size")))]
        read_raw_size_func!(read_isize_raw_be, ReadIsizeRawBe, InternalReadIsizeRawBe);
    };
}
#[cfg(feature = "async_raw")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_raw")))]
define_read_raw_futures!();
