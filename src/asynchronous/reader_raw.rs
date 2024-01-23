#[cfg(feature = "async_raw")]
macro_rules! read_raw_future {
    ($primitive: ty, $future: ident, $poll_func: ident, $buf: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
                buf: $buf,
            }
        }
        impl<'a, R: $crate::asynchronous::AsyncVariableReadable + Unpin> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$primitive>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                R::$poll_func(Pin::new(&mut *me.reader), cx, me.buf)
            }
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! read_raw_poll {
    ($primitive: ty, $poll_func: ident, $from: ident, $buf: ident) => {
        #[inline]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &mut $buf) -> std::task::Poll<std::io::Result<$primitive>> {
            let mut ref_buf = buf.into();
            let res = self.poll_read_more(cx, &mut ref_buf);
            let filled = ref_buf.filled();
            buf.set_filled(filled);
            ready!(res)?;
            Poll::Ready(Ok(<$primitive>::$from(buf.into_inner())))
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! read_raw_func {
    ($func: ident, $future: ident, $buf: ident) => {
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, buf: $buf::new() }
        }
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! read_raw_size_future {
    ($primitive: ty, $future: ident, $poll_func: ident) => {
        read_raw_future!($primitive, $future, $poll_func, OwnedReadBuf128);
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! read_raw_size_poll {
    ($primitive: ty, $poll_func: ident, $poll_internal: ident) => {
        #[inline]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &mut OwnedReadBuf128) -> std::task::Poll<std::io::Result<$primitive>> {
            Poll::Ready(Ok(ready!(self.$poll_internal(cx, buf))? as $primitive))
        }
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! read_raw_size_func {
    ($func: ident, $future: ident) => {
        read_raw_func!($func, $future, OwnedReadBuf128);
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_read_raw_futures {
    () => {
        read_raw_future!(u8, ReadU8Raw, poll_read_u8_raw, OwnedReadBuf8);
        read_raw_future!(i8, ReadI8Raw, poll_read_i8_raw, OwnedReadBuf8);

        read_raw_future!(u16, ReadU16RawLe, poll_read_u16_raw_le, OwnedReadBuf16);
        read_raw_future!(u16, ReadU16RawBe, poll_read_u16_raw_be, OwnedReadBuf16);
        read_raw_future!(i16, ReadI16RawLe, poll_read_i16_raw_le, OwnedReadBuf16);
        read_raw_future!(i16, ReadI16RawBe, poll_read_i16_raw_be, OwnedReadBuf16);

        read_raw_future!(u32, ReadU32RawLe, poll_read_u32_raw_le, OwnedReadBuf32);
        read_raw_future!(u32, ReadU32RawBe, poll_read_u32_raw_be, OwnedReadBuf32);
        read_raw_future!(i32, ReadI32RawLe, poll_read_i32_raw_le, OwnedReadBuf32);
        read_raw_future!(i32, ReadI32RawBe, poll_read_i32_raw_be, OwnedReadBuf32);

        read_raw_future!(u64, ReadU64RawLe, poll_read_u64_raw_le, OwnedReadBuf64);
        read_raw_future!(u64, ReadU64RawBe, poll_read_u64_raw_be, OwnedReadBuf64);
        read_raw_future!(i64, ReadI64RawLe, poll_read_i64_raw_le, OwnedReadBuf64);
        read_raw_future!(i64, ReadI64RawBe, poll_read_i64_raw_be, OwnedReadBuf64);

        read_raw_future!(u128, ReadU128RawLe, poll_read_u128_raw_le, OwnedReadBuf128);
        read_raw_future!(u128, ReadU128RawBe, poll_read_u128_raw_be, OwnedReadBuf128);
        read_raw_future!(i128, ReadI128RawLe, poll_read_i128_raw_le, OwnedReadBuf128);
        read_raw_future!(i128, ReadI128RawBe, poll_read_i128_raw_be, OwnedReadBuf128);

        #[cfg(feature = "async_raw_size")]
        read_raw_size_future!(usize, ReadUsizeRawLe, poll_read_usize_raw_le);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_future!(usize, ReadUsizeRawBe, poll_read_usize_raw_be);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_future!(isize, ReadIsizeRawLe, poll_read_isize_raw_le);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_future!(isize, ReadIsizeRawBe, poll_read_isize_raw_be);
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_read_raw_poll {
    () => {
        read_raw_poll!(u8, poll_read_u8_raw, from_ne_bytes, OwnedReadBuf8);
        read_raw_poll!(i8, poll_read_i8_raw, from_ne_bytes, OwnedReadBuf8);

        read_raw_poll!(u16, poll_read_u16_raw_le, from_le_bytes, OwnedReadBuf16);
        read_raw_poll!(u16, poll_read_u16_raw_be, from_be_bytes, OwnedReadBuf16);
        read_raw_poll!(i16, poll_read_i16_raw_le, from_le_bytes, OwnedReadBuf16);
        read_raw_poll!(i16, poll_read_i16_raw_be, from_be_bytes, OwnedReadBuf16);

        read_raw_poll!(u32, poll_read_u32_raw_le, from_le_bytes, OwnedReadBuf32);
        read_raw_poll!(u32, poll_read_u32_raw_be, from_be_bytes, OwnedReadBuf32);
        read_raw_poll!(i32, poll_read_i32_raw_le, from_le_bytes, OwnedReadBuf32);
        read_raw_poll!(i32, poll_read_i32_raw_be, from_be_bytes, OwnedReadBuf32);

        read_raw_poll!(u64, poll_read_u64_raw_le, from_le_bytes, OwnedReadBuf64);
        read_raw_poll!(u64, poll_read_u64_raw_be, from_be_bytes, OwnedReadBuf64);
        read_raw_poll!(i64, poll_read_i64_raw_le, from_le_bytes, OwnedReadBuf64);
        read_raw_poll!(i64, poll_read_i64_raw_be, from_be_bytes, OwnedReadBuf64);

        read_raw_poll!(u128, poll_read_u128_raw_le, from_le_bytes, OwnedReadBuf128);
        read_raw_poll!(u128, poll_read_u128_raw_be, from_be_bytes, OwnedReadBuf128);
        read_raw_poll!(i128, poll_read_i128_raw_le, from_le_bytes, OwnedReadBuf128);
        read_raw_poll!(i128, poll_read_i128_raw_be, from_be_bytes, OwnedReadBuf128);

        #[cfg(feature = "async_raw_size")]
        read_raw_size_poll!(usize, poll_read_usize_raw_le, poll_read_u128_raw_le);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_poll!(usize, poll_read_usize_raw_be, poll_read_u128_raw_be);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_poll!(isize, poll_read_isize_raw_le, poll_read_i128_raw_le);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_poll!(isize, poll_read_isize_raw_be, poll_read_i128_raw_be);
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_read_raw_func {
    () => {
        read_raw_func!(read_u8_raw, ReadU8Raw, OwnedReadBuf8);
        read_raw_func!(read_i8_raw, ReadI8Raw, OwnedReadBuf8);

        read_raw_func!(read_u16_raw_le, ReadU16RawLe, OwnedReadBuf16);
        read_raw_func!(read_u16_raw_be, ReadU16RawBe, OwnedReadBuf16);
        read_raw_func!(read_i16_raw_le, ReadI16RawLe, OwnedReadBuf16);
        read_raw_func!(read_i16_raw_be, ReadI16RawBe, OwnedReadBuf16);

        read_raw_func!(read_u32_raw_le, ReadU32RawLe, OwnedReadBuf32);
        read_raw_func!(read_u32_raw_be, ReadU32RawBe, OwnedReadBuf32);
        read_raw_func!(read_i32_raw_le, ReadI32RawLe, OwnedReadBuf32);
        read_raw_func!(read_i32_raw_be, ReadI32RawBe, OwnedReadBuf32);

        read_raw_func!(read_u64_raw_le, ReadU64RawLe, OwnedReadBuf64);
        read_raw_func!(read_u64_raw_be, ReadU64RawBe, OwnedReadBuf64);
        read_raw_func!(read_i64_raw_le, ReadI64RawLe, OwnedReadBuf64);
        read_raw_func!(read_i64_raw_be, ReadI64RawBe, OwnedReadBuf64);

        read_raw_func!(read_u128_raw_le, ReadU128RawLe, OwnedReadBuf128);
        read_raw_func!(read_u128_raw_be, ReadU128RawBe, OwnedReadBuf128);
        read_raw_func!(read_i128_raw_le, ReadI128RawLe, OwnedReadBuf128);
        read_raw_func!(read_i128_raw_be, ReadI128RawBe, OwnedReadBuf128);

        #[cfg(feature = "async_raw_size")]
        read_raw_size_func!(read_usize_raw_le, ReadUsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_func!(read_usize_raw_be, ReadUsizeRawBe);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_func!(read_isize_raw_le, ReadIsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_func!(read_isize_raw_be, ReadIsizeRawBe);
    };
}
#[cfg(feature = "async_raw")]
define_read_raw_futures!();
