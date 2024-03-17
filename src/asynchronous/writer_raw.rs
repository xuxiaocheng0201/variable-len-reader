macro_rules! write_raw_future {
    (raw, $primitive: ty, $future: ident, $poll_func: ident, $buf: ident, $to: ident, $struct_buf: ident) => {
        write_raw_future!(cfg(feature = "async_raw"), $primitive, $future, $poll_func, $buf, $to, $struct_buf);
    };
    (raw_size, $future: ident, $internal: ident, $poll_func: ident, $to: ident, $struct_buf: ident) => {
        write_raw_future!(cfg(feature = "async_raw_size"), $internal, $future, $poll_func, OwnedWriteBuf128, $to, $struct_buf);
    };
    ($feature: meta, $primitive: ty, $future: ident, $poll_func: ident, $buf: ident, $to: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[derive(Debug)]
        struct $struct_buf {
            buf: $buf,
        }
        #[$feature]
        impl $struct_buf {
            fn new(num: $primitive) -> Self {
                Self { buf: $buf::new(<$primitive>::$to(num)) }
            }
            fn reset(&mut self, num: $primitive) {
                self.buf.reset();
                self.buf.set_buf(<$primitive>::$to(num));
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
        impl<'a, W: $crate::AsyncVariableWritable + Unpin + ?Sized> Future for $future<'a, W> {
            type Output = ::core::result::Result<usize>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut me = self.project();
                W::$poll_func(Pin::new(&mut *me.writer), cx, me.inner)
            }
        }
    };
}
macro_rules! write_raw_poll {
    (raw, $poll_func: ident, $struct_buf: ident) => {
        write_raw_poll!(cfg(feature = "async_raw"), $poll_func, $struct_buf);
    };
    (raw_size, $poll_func: ident, $struct_buf: ident) => {
        write_raw_poll!(cfg(feature = "async_raw_size"), $poll_func, $struct_buf);
    };
    ($feature: meta, $poll_func: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $poll_func(self: Pin<&mut Self>, cx: &mut Context<'_>, inner: &mut $struct_buf) -> Poll<::core::result::Result<usize>> {
            let mut ref_buf = (&inner.buf).into();
            let res = self.poll_write_more(cx, &mut ref_buf);
            let read = ref_buf.read();
            inner.buf.set_read(read);
            res
        }
    };
}
macro_rules! write_raw_func {
    (raw, $primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        write_raw_func!(cfg(feature = "async_raw"), $primitive, $primitive, $func, $future, $struct_buf);
    };
    (raw_size, $primitive: ty, $func: ident, $future: ident, $internal: ident, $struct_buf: ident) => {
        write_raw_func!(cfg(feature = "async_raw_size"), $primitive, $internal, $func, $future, $struct_buf);
    };
    ($feature: meta, $primitive: ty, $target: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            $future { writer: self, inner: $struct_buf::new(num as $target) }
        }
    };
}
macro_rules! define_write_raw_futures {
    () => {
        write_raw_future!(raw, u8, WriteU8Raw, poll_write_u8_raw, OwnedWriteBuf8, to_ne_bytes, InternalWriteU8Raw);
        write_raw_future!(raw, i8, WriteI8Raw, poll_write_i8_raw, OwnedWriteBuf8, to_ne_bytes, InternalWriteI8Raw);

        write_raw_future!(raw, u16, WriteU16RawLe, poll_write_u16_raw_le, OwnedWriteBuf16, to_le_bytes, InternalWriteU16RawLe);
        write_raw_future!(raw, u16, WriteU16RawBe, poll_write_u16_raw_be, OwnedWriteBuf16, to_be_bytes, InternalWriteU16RawBe);
        write_raw_future!(raw, i16, WriteI16RawLe, poll_write_i16_raw_le, OwnedWriteBuf16, to_le_bytes, InternalWriteI16RawLe);
        write_raw_future!(raw, i16, WriteI16RawBe, poll_write_i16_raw_be, OwnedWriteBuf16, to_be_bytes, InternalWriteI16RawBe);

        write_raw_future!(raw, u32, WriteU32RawLe, poll_write_u32_raw_le, OwnedWriteBuf32, to_le_bytes, InternalWriteU32RawLe);
        write_raw_future!(raw, u32, WriteU32RawBe, poll_write_u32_raw_be, OwnedWriteBuf32, to_be_bytes, InternalWriteU32RawBe);
        write_raw_future!(raw, i32, WriteI32RawLe, poll_write_i32_raw_le, OwnedWriteBuf32, to_le_bytes, InternalWriteI32RawLe);
        write_raw_future!(raw, i32, WriteI32RawBe, poll_write_i32_raw_be, OwnedWriteBuf32, to_be_bytes, InternalWriteI32RawBe);

        write_raw_future!(raw, u64, WriteU64RawLe, poll_write_u64_raw_le, OwnedWriteBuf64, to_le_bytes, InternalWriteU64RawLe);
        write_raw_future!(raw, u64, WriteU64RawBe, poll_write_u64_raw_be, OwnedWriteBuf64, to_be_bytes, InternalWriteU64RawBe);
        write_raw_future!(raw, i64, WriteI64RawLe, poll_write_i64_raw_le, OwnedWriteBuf64, to_le_bytes, InternalWriteI64RawLe);
        write_raw_future!(raw, i64, WriteI64RawBe, poll_write_i64_raw_be, OwnedWriteBuf64, to_be_bytes, InternalWriteI64RawBe);

        write_raw_future!(raw, u128, WriteU128RawLe, poll_write_u128_raw_le, OwnedWriteBuf128, to_le_bytes, InternalWriteU128RawLe);
        write_raw_future!(raw, u128, WriteU128RawBe, poll_write_u128_raw_be, OwnedWriteBuf128, to_be_bytes, InternalWriteU128RawBe);
        write_raw_future!(raw, i128, WriteI128RawLe, poll_write_i128_raw_le, OwnedWriteBuf128, to_le_bytes, InternalWriteI128RawLe);
        write_raw_future!(raw, i128, WriteI128RawBe, poll_write_i128_raw_be, OwnedWriteBuf128, to_be_bytes, InternalWriteI128RawBe);

        write_raw_future!(raw_size, WriteUsizeRawLe, u128, poll_write_usize_raw_le, to_le_bytes, InternalWriteUsizeRawLe);
        write_raw_future!(raw_size, WriteUsizeRawBe, u128, poll_write_usize_raw_be, to_be_bytes, InternalWriteUsizeRawBe);
        write_raw_future!(raw_size, WriteIsizeRawLe, i128, poll_write_isize_raw_le, to_le_bytes, InternalWriteIsizeRawLe);
        write_raw_future!(raw_size, WriteIsizeRawBe, i128, poll_write_isize_raw_be, to_be_bytes, InternalWriteIsizeRawBe);

        write_raw_future!(raw, f32, WriteF32RawLe, poll_write_f32_raw_le, OwnedWriteBuf32, to_le_bytes, InternalWriteF32RawLe);
        write_raw_future!(raw, f32, WriteF32RawBe, poll_write_f32_raw_be, OwnedWriteBuf32, to_be_bytes, InternalWriteF32RawBe);
        write_raw_future!(raw, f64, WriteF64RawLe, poll_write_f64_raw_le, OwnedWriteBuf64, to_le_bytes, InternalWriteF64RawLe);
        write_raw_future!(raw, f64, WriteF64RawBe, poll_write_f64_raw_be, OwnedWriteBuf64, to_be_bytes, InternalWriteF64RawBe);
    };
}
macro_rules! define_write_raw_poll {
    () => {
        write_raw_poll!(raw, poll_write_u8_raw, InternalWriteU8Raw);
        write_raw_poll!(raw, poll_write_i8_raw, InternalWriteI8Raw);

        write_raw_poll!(raw, poll_write_u16_raw_le, InternalWriteU16RawLe);
        write_raw_poll!(raw, poll_write_u16_raw_be, InternalWriteU16RawBe);
        write_raw_poll!(raw, poll_write_i16_raw_le, InternalWriteI16RawLe);
        write_raw_poll!(raw, poll_write_i16_raw_be, InternalWriteI16RawBe);

        write_raw_poll!(raw, poll_write_u32_raw_le, InternalWriteU32RawLe);
        write_raw_poll!(raw, poll_write_u32_raw_be, InternalWriteU32RawBe);
        write_raw_poll!(raw, poll_write_i32_raw_le, InternalWriteI32RawLe);
        write_raw_poll!(raw, poll_write_i32_raw_be, InternalWriteI32RawBe);

        write_raw_poll!(raw, poll_write_u64_raw_le, InternalWriteU64RawLe);
        write_raw_poll!(raw, poll_write_u64_raw_be, InternalWriteU64RawBe);
        write_raw_poll!(raw, poll_write_i64_raw_le, InternalWriteI64RawLe);
        write_raw_poll!(raw, poll_write_i64_raw_be, InternalWriteI64RawBe);

        write_raw_poll!(raw, poll_write_u128_raw_le, InternalWriteU128RawLe);
        write_raw_poll!(raw, poll_write_u128_raw_be, InternalWriteU128RawBe);
        write_raw_poll!(raw, poll_write_i128_raw_le, InternalWriteI128RawLe);
        write_raw_poll!(raw, poll_write_i128_raw_be, InternalWriteI128RawBe);

        write_raw_poll!(raw_size, poll_write_usize_raw_le, InternalWriteUsizeRawLe);
        write_raw_poll!(raw_size, poll_write_usize_raw_be, InternalWriteUsizeRawBe);
        write_raw_poll!(raw_size, poll_write_isize_raw_le, InternalWriteIsizeRawLe);
        write_raw_poll!(raw_size, poll_write_isize_raw_be, InternalWriteIsizeRawBe);

        write_raw_poll!(raw, poll_write_f32_raw_le, InternalWriteF32RawLe);
        write_raw_poll!(raw, poll_write_f32_raw_be, InternalWriteF32RawBe);
        write_raw_poll!(raw, poll_write_f64_raw_le, InternalWriteF64RawLe);
        write_raw_poll!(raw, poll_write_f64_raw_be, InternalWriteF64RawBe);
    };
}
macro_rules! define_write_raw_func {
    () => {
        write_raw_func!(raw, u8, write_u8_raw, WriteU8Raw, InternalWriteU8Raw);
        write_raw_func!(raw, i8, write_i8_raw, WriteI8Raw, InternalWriteI8Raw);

        write_raw_func!(raw, u16, write_u16_raw_le, WriteU16RawLe, InternalWriteU16RawLe);
        write_raw_func!(raw, u16, write_u16_raw_be, WriteU16RawBe, InternalWriteU16RawBe);
        write_raw_func!(raw, i16, write_i16_raw_le, WriteI16RawLe, InternalWriteI16RawLe);
        write_raw_func!(raw, i16, write_i16_raw_be, WriteI16RawBe, InternalWriteI16RawBe);

        write_raw_func!(raw, u32, write_u32_raw_le, WriteU32RawLe, InternalWriteU32RawLe);
        write_raw_func!(raw, u32, write_u32_raw_be, WriteU32RawBe, InternalWriteU32RawBe);
        write_raw_func!(raw, i32, write_i32_raw_le, WriteI32RawLe, InternalWriteI32RawLe);
        write_raw_func!(raw, i32, write_i32_raw_be, WriteI32RawBe, InternalWriteI32RawBe);

        write_raw_func!(raw, u64, write_u64_raw_le, WriteU64RawLe, InternalWriteU64RawLe);
        write_raw_func!(raw, u64, write_u64_raw_be, WriteU64RawBe, InternalWriteU64RawBe);
        write_raw_func!(raw, i64, write_i64_raw_le, WriteI64RawLe, InternalWriteI64RawLe);
        write_raw_func!(raw, i64, write_i64_raw_be, WriteI64RawBe, InternalWriteI64RawBe);

        write_raw_func!(raw, u128, write_u128_raw_le, WriteU128RawLe, InternalWriteU128RawLe);
        write_raw_func!(raw, u128, write_u128_raw_be, WriteU128RawBe, InternalWriteU128RawBe);
        write_raw_func!(raw, i128, write_i128_raw_le, WriteI128RawLe, InternalWriteI128RawLe);
        write_raw_func!(raw, i128, write_i128_raw_be, WriteI128RawBe, InternalWriteI128RawBe);

        write_raw_func!(raw_size, usize, write_usize_raw_le, WriteUsizeRawLe, u128, InternalWriteUsizeRawLe);
        write_raw_func!(raw_size, usize, write_usize_raw_be, WriteUsizeRawBe, u128, InternalWriteUsizeRawBe);
        write_raw_func!(raw_size, isize, write_isize_raw_le, WriteIsizeRawLe, i128, InternalWriteIsizeRawLe);
        write_raw_func!(raw_size, isize, write_isize_raw_be, WriteIsizeRawBe, i128, InternalWriteIsizeRawBe);

        write_raw_func!(raw, f32, write_f32_raw_le, WriteF32RawLe, InternalWriteF32RawLe);
        write_raw_func!(raw, f32, write_f32_raw_be, WriteF32RawBe, InternalWriteF32RawBe);
        write_raw_func!(raw, f64, write_f64_raw_le, WriteF64RawLe, InternalWriteF64RawLe);
        write_raw_func!(raw, f64, write_f64_raw_be, WriteF64RawBe, InternalWriteF64RawBe);
    };
}
define_write_raw_futures!();
