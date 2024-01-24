#[cfg(feature = "async_raw")]
macro_rules! write_raw_future {
    ($primitive: ty, $future: ident, $poll_func: ident, $buf: ident, $to: ident, $struct_buf: ident) => {
        #[derive(Debug)]
        struct $struct_buf {
            buf: $buf,
        }
        impl $struct_buf {
            fn new(num: $primitive) -> Self {
                Self { buf: $buf::new(<$primitive>::$to(num)) }
            }
            fn reset(&mut self, num: $primitive) {
                self.buf.reset();
                self.buf.set_buf(<$primitive>::$to(num));
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
        impl<'a, W: $crate::asynchronous::AsyncVariableWritable + Unpin> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                W::$poll_func(Pin::new(&mut *me.writer), cx, me.inner)
            }
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! write_raw_poll {
    ($poll_func: ident, $struct_buf: ident) => {
        #[inline]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, inner: &mut $struct_buf) -> std::task::Poll<std::io::Result<usize>> {
            let mut ref_buf = (&mut inner.buf).into();
            let res = self.poll_write_more(cx, &mut ref_buf);
            let read = ref_buf.read();
            inner.buf.set_read(read);
            res
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! write_raw_func {
    ($primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        write_raw_func!($primitive, $primitive, $func, $future, $struct_buf);
    };
    ($primitive: ty, $target: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            $future { writer: self, inner: $struct_buf::new(num as $target) }
        }
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! write_raw_size_future {
    ($future: ident, $internal: ident, $poll_func: ident, $to: ident, $struct_buf: ident) => {
        write_raw_future!($internal, $future, $poll_func, OwnedWriteBuf128, $to, $struct_buf);
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! write_raw_size_poll {
    ($poll_func: ident, $struct_buf: ident) => {
        write_raw_poll!($poll_func, $struct_buf);
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! write_raw_size_func {
    ($primitive: ty, $func: ident, $future: ident, $internal: ident, $struct_buf: ident) => {
        write_raw_func!($primitive, $internal, $func, $future, $struct_buf);
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_write_raw_futures {
    () => {
        write_raw_future!(u8, WriteU8Raw, poll_write_u8_raw, OwnedWriteBuf8, to_ne_bytes, InternalWriteU8Raw);
        write_raw_future!(i8, WriteI8Raw, poll_write_i8_raw, OwnedWriteBuf8, to_ne_bytes, InternalWriteI8Raw);

        write_raw_future!(u16, WriteU16RawLe, poll_write_u16_raw_le, OwnedWriteBuf16, to_le_bytes, InternalWriteU16RawLe);
        write_raw_future!(u16, WriteU16RawBe, poll_write_u16_raw_be, OwnedWriteBuf16, to_be_bytes, InternalWriteU16RawBe);
        write_raw_future!(i16, WriteI16RawLe, poll_write_i16_raw_le, OwnedWriteBuf16, to_le_bytes, InternalWriteI16RawLe);
        write_raw_future!(i16, WriteI16RawBe, poll_write_i16_raw_be, OwnedWriteBuf16, to_be_bytes, InternalWriteI16RawBe);

        write_raw_future!(u32, WriteU32RawLe, poll_write_u32_raw_le, OwnedWriteBuf32, to_le_bytes, InternalWriteU32RawLe);
        write_raw_future!(u32, WriteU32RawBe, poll_write_u32_raw_be, OwnedWriteBuf32, to_be_bytes, InternalWriteU32RawBe);
        write_raw_future!(i32, WriteI32RawLe, poll_write_i32_raw_le, OwnedWriteBuf32, to_le_bytes, InternalWriteI32RawLe);
        write_raw_future!(i32, WriteI32RawBe, poll_write_i32_raw_be, OwnedWriteBuf32, to_be_bytes, InternalWriteI32RawBe);

        write_raw_future!(u64, WriteU64RawLe, poll_write_u64_raw_le, OwnedWriteBuf64, to_le_bytes, InternalWriteU64RawLe);
        write_raw_future!(u64, WriteU64RawBe, poll_write_u64_raw_be, OwnedWriteBuf64, to_be_bytes, InternalWriteU64RawBe);
        write_raw_future!(i64, WriteI64RawLe, poll_write_i64_raw_le, OwnedWriteBuf64, to_le_bytes, InternalWriteI64RawLe);
        write_raw_future!(i64, WriteI64RawBe, poll_write_i64_raw_be, OwnedWriteBuf64, to_be_bytes, InternalWriteI64RawBe);

        write_raw_future!(u128, WriteU128RawLe, poll_write_u128_raw_le, OwnedWriteBuf128, to_le_bytes, InternalWriteU128RawLe);
        write_raw_future!(u128, WriteU128RawBe, poll_write_u128_raw_be, OwnedWriteBuf128, to_be_bytes, InternalWriteU128RawBe);
        write_raw_future!(i128, WriteI128RawLe, poll_write_i128_raw_le, OwnedWriteBuf128, to_le_bytes, InternalWriteI128RawLe);
        write_raw_future!(i128, WriteI128RawBe, poll_write_i128_raw_be, OwnedWriteBuf128, to_be_bytes, InternalWriteI128RawBe);

        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteUsizeRawLe, u128, poll_write_usize_raw_le, to_le_bytes, InternalWriteUsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteUsizeRawBe, u128, poll_write_usize_raw_be, to_be_bytes, InternalWriteUsizeRawBe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteIsizeRawLe, i128, poll_write_isize_raw_le, to_le_bytes, InternalWriteIsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteIsizeRawBe, i128, poll_write_isize_raw_be, to_be_bytes, InternalWriteIsizeRawBe);
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_write_raw_poll {
    () => {
        write_raw_poll!(poll_write_u8_raw, InternalWriteU8Raw);
        write_raw_poll!(poll_write_i8_raw, InternalWriteI8Raw);

        write_raw_poll!(poll_write_u16_raw_le, InternalWriteU16RawLe);
        write_raw_poll!(poll_write_u16_raw_be, InternalWriteU16RawBe);
        write_raw_poll!(poll_write_i16_raw_le, InternalWriteI16RawLe);
        write_raw_poll!(poll_write_i16_raw_be, InternalWriteI16RawBe);

        write_raw_poll!(poll_write_u32_raw_le, InternalWriteU32RawLe);
        write_raw_poll!(poll_write_u32_raw_be, InternalWriteU32RawBe);
        write_raw_poll!(poll_write_i32_raw_le, InternalWriteI32RawLe);
        write_raw_poll!(poll_write_i32_raw_be, InternalWriteI32RawBe);

        write_raw_poll!(poll_write_u64_raw_le, InternalWriteU64RawLe);
        write_raw_poll!(poll_write_u64_raw_be, InternalWriteU64RawBe);
        write_raw_poll!(poll_write_i64_raw_le, InternalWriteI64RawLe);
        write_raw_poll!(poll_write_i64_raw_be, InternalWriteI64RawBe);

        write_raw_poll!(poll_write_u128_raw_le, InternalWriteU128RawLe);
        write_raw_poll!(poll_write_u128_raw_be, InternalWriteU128RawBe);
        write_raw_poll!(poll_write_i128_raw_le, InternalWriteI128RawLe);
        write_raw_poll!(poll_write_i128_raw_be, InternalWriteI128RawBe);

        #[cfg(feature = "async_raw_size")]
        write_raw_size_poll!(poll_write_usize_raw_le, InternalWriteUsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_poll!(poll_write_usize_raw_be, InternalWriteUsizeRawBe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_poll!(poll_write_isize_raw_le, InternalWriteIsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_poll!(poll_write_isize_raw_be, InternalWriteIsizeRawBe);
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_write_raw_func {
    () => {
        write_raw_func!(u8, write_u8_raw, WriteU8Raw, InternalWriteU8Raw);
        write_raw_func!(i8, write_i8_raw, WriteI8Raw, InternalWriteI8Raw);

        write_raw_func!(u16, write_u16_raw_le, WriteU16RawLe, InternalWriteU16RawLe);
        write_raw_func!(u16, write_u16_raw_be, WriteU16RawBe, InternalWriteU16RawBe);
        write_raw_func!(i16, write_i16_raw_le, WriteI16RawLe, InternalWriteI16RawLe);
        write_raw_func!(i16, write_i16_raw_be, WriteI16RawBe, InternalWriteI16RawBe);

        write_raw_func!(u32, write_u32_raw_le, WriteU32RawLe, InternalWriteU32RawLe);
        write_raw_func!(u32, write_u32_raw_be, WriteU32RawBe, InternalWriteU32RawBe);
        write_raw_func!(i32, write_i32_raw_le, WriteI32RawLe, InternalWriteI32RawLe);
        write_raw_func!(i32, write_i32_raw_be, WriteI32RawBe, InternalWriteI32RawBe);

        write_raw_func!(u64, write_u64_raw_le, WriteU64RawLe, InternalWriteU64RawLe);
        write_raw_func!(u64, write_u64_raw_be, WriteU64RawBe, InternalWriteU64RawBe);
        write_raw_func!(i64, write_i64_raw_le, WriteI64RawLe, InternalWriteI64RawLe);
        write_raw_func!(i64, write_i64_raw_be, WriteI64RawBe, InternalWriteI64RawBe);

        write_raw_func!(u128, write_u128_raw_le, WriteU128RawLe, InternalWriteU128RawLe);
        write_raw_func!(u128, write_u128_raw_be, WriteU128RawBe, InternalWriteU128RawBe);
        write_raw_func!(i128, write_i128_raw_le, WriteI128RawLe, InternalWriteI128RawLe);
        write_raw_func!(i128, write_i128_raw_be, WriteI128RawBe, InternalWriteI128RawBe);

        #[cfg(feature = "async_raw_size")]
        write_raw_size_func!(usize, write_usize_raw_le, WriteUsizeRawLe, u128, InternalWriteUsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_func!(usize, write_usize_raw_be, WriteUsizeRawBe, u128, InternalWriteUsizeRawBe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_func!(isize, write_isize_raw_le, WriteIsizeRawLe, i128, InternalWriteIsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_func!(isize, write_isize_raw_be, WriteIsizeRawBe, i128, InternalWriteIsizeRawBe);
    };
}
#[cfg(feature = "async_raw")]
define_write_raw_futures!();
