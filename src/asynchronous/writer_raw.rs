#[cfg(feature = "async_raw")]
macro_rules! write_raw_future {
    ($future: ident, $poll_func: ident, $buf: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                buf: $buf,
            }
        }
        impl<'a, W: $crate::asynchronous::AsyncVariableWritable + Unpin> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                W::$poll_func(Pin::new(&mut *me.writer), cx, me.buf)
            }
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! write_raw_poll {
    ($poll_func: ident, $buf: ident) => {
        #[inline]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &mut $buf) -> std::task::Poll<std::io::Result<usize>> {
            let mut ref_buf = buf.into();
            let res = self.poll_write_more(cx, &mut ref_buf);
            let read = ref_buf.read();
            buf.set_read(read);
            res
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! write_raw_func {
    ($primitive: ty, $func: ident, $future: ident, $to: ident, $buf: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            $future { writer: self, buf: $buf::new(<$primitive>::$to(num)) }
        }
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! write_raw_size_future {
    ($future: ident, $poll_func: ident) => {
        write_raw_future!($future, $poll_func, OwnedWriteBuf128);
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! write_raw_size_poll {
    ($poll_func: ident) => {
        write_raw_poll!($poll_func, OwnedWriteBuf128);
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! write_raw_size_func {
    ($primitive: ty, $func: ident, $future: ident, $internal: ident, $to: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            $future { writer: self, buf: OwnedWriteBuf128::new(<$internal>::$to(num as $internal)) }
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_write_raw_futures {
    () => {
        write_raw_future!(WriteU8Raw, poll_write_u8_raw, OwnedWriteBuf8);
        write_raw_future!(WriteI8Raw, poll_write_i8_raw, OwnedWriteBuf8);

        write_raw_future!(WriteU16RawLe, poll_write_u16_raw_le, OwnedWriteBuf16);
        write_raw_future!(WriteU16RawBe, poll_write_u16_raw_be, OwnedWriteBuf16);
        write_raw_future!(WriteI16RawLe, poll_write_i16_raw_le, OwnedWriteBuf16);
        write_raw_future!(WriteI16RawBe, poll_write_i16_raw_be, OwnedWriteBuf16);

        write_raw_future!(WriteU32RawLe, poll_write_u32_raw_le, OwnedWriteBuf32);
        write_raw_future!(WriteU32RawBe, poll_write_u32_raw_be, OwnedWriteBuf32);
        write_raw_future!(WriteI32RawLe, poll_write_i32_raw_le, OwnedWriteBuf32);
        write_raw_future!(WriteI32RawBe, poll_write_i32_raw_be, OwnedWriteBuf32);

        write_raw_future!(WriteU64RawLe, poll_write_u64_raw_le, OwnedWriteBuf64);
        write_raw_future!(WriteU64RawBe, poll_write_u64_raw_be, OwnedWriteBuf64);
        write_raw_future!(WriteI64RawLe, poll_write_i64_raw_le, OwnedWriteBuf64);
        write_raw_future!(WriteI64RawBe, poll_write_i64_raw_be, OwnedWriteBuf64);

        write_raw_future!(WriteU128RawLe, poll_write_u128_raw_le, OwnedWriteBuf128);
        write_raw_future!(WriteU128RawBe, poll_write_u128_raw_be, OwnedWriteBuf128);
        write_raw_future!(WriteI128RawLe, poll_write_i128_raw_le, OwnedWriteBuf128);
        write_raw_future!(WriteI128RawBe, poll_write_i128_raw_be, OwnedWriteBuf128);

        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteUsizeRawLe, poll_write_usize_raw_le);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteUsizeRawBe, poll_write_usize_raw_be);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteIsizeRawLe, poll_write_isize_raw_le);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteIsizeRawBe, poll_write_isize_raw_be);
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_write_raw_poll {
    () => {
        write_raw_poll!(poll_write_u8_raw, OwnedWriteBuf8);
        write_raw_poll!(poll_write_i8_raw, OwnedWriteBuf8);

        write_raw_poll!(poll_write_u16_raw_le, OwnedWriteBuf16);
        write_raw_poll!(poll_write_u16_raw_be, OwnedWriteBuf16);
        write_raw_poll!(poll_write_i16_raw_le, OwnedWriteBuf16);
        write_raw_poll!(poll_write_i16_raw_be, OwnedWriteBuf16);

        write_raw_poll!(poll_write_u32_raw_le, OwnedWriteBuf32);
        write_raw_poll!(poll_write_u32_raw_be, OwnedWriteBuf32);
        write_raw_poll!(poll_write_i32_raw_le, OwnedWriteBuf32);
        write_raw_poll!(poll_write_i32_raw_be, OwnedWriteBuf32);

        write_raw_poll!(poll_write_u64_raw_le, OwnedWriteBuf64);
        write_raw_poll!(poll_write_u64_raw_be, OwnedWriteBuf64);
        write_raw_poll!(poll_write_i64_raw_le, OwnedWriteBuf64);
        write_raw_poll!(poll_write_i64_raw_be, OwnedWriteBuf64);

        write_raw_poll!(poll_write_u128_raw_le, OwnedWriteBuf128);
        write_raw_poll!(poll_write_u128_raw_be, OwnedWriteBuf128);
        write_raw_poll!(poll_write_i128_raw_le, OwnedWriteBuf128);
        write_raw_poll!(poll_write_i128_raw_be, OwnedWriteBuf128);

        #[cfg(feature = "async_raw_size")]
        write_raw_size_poll!(poll_write_usize_raw_le);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_poll!(poll_write_usize_raw_be);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_poll!(poll_write_isize_raw_le);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_poll!(poll_write_isize_raw_be);
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_write_raw_func {
    () => {
        write_raw_func!(u8, write_u8_raw, WriteU8Raw, to_ne_bytes, OwnedWriteBuf8);
        write_raw_func!(i8, write_i8_raw, WriteI8Raw, to_ne_bytes, OwnedWriteBuf8);

        write_raw_func!(u16, write_u16_raw_le, WriteU16RawLe, to_le_bytes, OwnedWriteBuf16);
        write_raw_func!(u16, write_u16_raw_be, WriteU16RawBe, to_be_bytes, OwnedWriteBuf16);
        write_raw_func!(i16, write_i16_raw_le, WriteI16RawLe, to_le_bytes, OwnedWriteBuf16);
        write_raw_func!(i16, write_i16_raw_be, WriteI16RawBe, to_be_bytes, OwnedWriteBuf16);

        write_raw_func!(u32, write_u32_raw_le, WriteU32RawLe, to_le_bytes, OwnedWriteBuf32);
        write_raw_func!(u32, write_u32_raw_be, WriteU32RawBe, to_be_bytes, OwnedWriteBuf32);
        write_raw_func!(i32, write_i32_raw_le, WriteI32RawLe, to_le_bytes, OwnedWriteBuf32);
        write_raw_func!(i32, write_i32_raw_be, WriteI32RawBe, to_be_bytes, OwnedWriteBuf32);

        write_raw_func!(u64, write_u64_raw_le, WriteU64RawLe, to_le_bytes, OwnedWriteBuf64);
        write_raw_func!(u64, write_u64_raw_be, WriteU64RawBe, to_be_bytes, OwnedWriteBuf64);
        write_raw_func!(i64, write_i64_raw_le, WriteI64RawLe, to_le_bytes, OwnedWriteBuf64);
        write_raw_func!(i64, write_i64_raw_be, WriteI64RawBe, to_be_bytes, OwnedWriteBuf64);

        write_raw_func!(u128, write_u128_raw_le, WriteU128RawLe, to_le_bytes, OwnedWriteBuf128);
        write_raw_func!(u128, write_u128_raw_be, WriteU128RawBe, to_be_bytes, OwnedWriteBuf128);
        write_raw_func!(i128, write_i128_raw_le, WriteI128RawLe, to_le_bytes, OwnedWriteBuf128);
        write_raw_func!(i128, write_i128_raw_be, WriteI128RawBe, to_be_bytes, OwnedWriteBuf128);

        #[cfg(feature = "async_raw_size")]
        write_raw_size_func!(usize, write_usize_raw_le, WriteUsizeRawLe, u128, to_le_bytes);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_func!(usize, write_usize_raw_be, WriteUsizeRawBe, u128, to_be_bytes);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_func!(isize, write_isize_raw_le, WriteIsizeRawLe, i128, to_le_bytes);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_func!(isize, write_isize_raw_be, WriteIsizeRawBe, i128, to_be_bytes);
    };
}
#[cfg(feature = "async_raw")]
define_write_raw_futures!();
