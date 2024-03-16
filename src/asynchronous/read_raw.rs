macro_rules! read_raw_future {
    ($primitive: ty, $future: ident, $poll_func: ident, $buf: ident, $struct_buf: ident) => {
        read_raw_future!(f cfg(feature = "async_raw"), $primitive, $future, $poll_func, $buf, $struct_buf);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $poll_func: ident, $buf: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
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
        #[$feature]
        impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for $future<'a, R> {
            type Output = ::core::result::Result<$primitive, R::Error>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut me = self.project();
                R::$poll_func(Pin::new(&mut *me.reader), cx, me.inner)
            }
        }
    };
}
macro_rules! read_raw_func {
    ($primitive: ty, $func: ident, $future: ident, $poll_func: ident, $from: ident, $struct_buf: ident) => {
        read_raw_func!(f cfg(feature = "async_raw"), $primitive, $primitive, $func, $future, $poll_func, $from, $struct_buf);
    };
    (f $feature: meta, $primitive: ty, $target: ty, $func: ident, $future: ident, $poll_func: ident, $from: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        fn $poll_func(self: Pin<&mut Self>, cx: &mut Context<'_>, inner: &mut $struct_buf) -> Poll<::core::result::Result<$target, Self::Error>> {
            let mut ref_buf = (&mut inner.buf).into(); // TODO: optimise
            let res = self.poll_read_more(cx, &mut ref_buf);
            let filled = ref_buf.filled();
            inner.buf.set_filled(filled);
            ready!(res)?;
            Poll::Ready(Ok(<$primitive>::$from(inner.buf.into_inner()) as $target))
        }
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, inner: $struct_buf::new() }
        }
    };
}

macro_rules! define_read_raw_future {
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

        read_raw_future!(f32, ReadF32RawLe, poll_read_f32_raw_le, OwnedReadBuf32, InternalReadF32RawLe);
        read_raw_future!(f32, ReadF32RawBe, poll_read_f32_raw_be, OwnedReadBuf32, InternalReadF32RawBe);
        read_raw_future!(f64, ReadF64RawLe, poll_read_f64_raw_le, OwnedReadBuf64, InternalReadF64RawLe);
        read_raw_future!(f64, ReadF64RawBe, poll_read_f64_raw_be, OwnedReadBuf64, InternalReadF64RawBe);
    };
}
macro_rules! define_read_raw_func {
    () => {
        read_raw_func!(u8, read_u8_raw, ReadU8Raw, poll_read_u8_raw, from_ne_bytes, InternalReadU8Raw);
        read_raw_func!(i8, read_i8_raw, ReadI8Raw, poll_read_i8_raw, from_ne_bytes, InternalReadI8Raw);

        read_raw_func!(u16, read_u16_raw_le, ReadU16RawLe, poll_read_u16_raw_le, from_le_bytes, InternalReadU16RawLe);
        read_raw_func!(u16, read_u16_raw_be, ReadU16RawBe, poll_read_u16_raw_be, from_be_bytes, InternalReadU16RawBe);
        read_raw_func!(i16, read_i16_raw_le, ReadI16RawLe, poll_read_i16_raw_le, from_le_bytes, InternalReadI16RawLe);
        read_raw_func!(i16, read_i16_raw_be, ReadI16RawBe, poll_read_i16_raw_be, from_be_bytes, InternalReadI16RawBe);

        read_raw_func!(u32, read_u32_raw_le, ReadU32RawLe, poll_read_u32_raw_le, from_le_bytes, InternalReadU32RawLe);
        read_raw_func!(u32, read_u32_raw_be, ReadU32RawBe, poll_read_u32_raw_be, from_be_bytes, InternalReadU32RawBe);
        read_raw_func!(i32, read_i32_raw_le, ReadI32RawLe, poll_read_i32_raw_le, from_le_bytes, InternalReadI32RawLe);
        read_raw_func!(i32, read_i32_raw_be, ReadI32RawBe, poll_read_i32_raw_be, from_be_bytes, InternalReadI32RawBe);

        read_raw_func!(u64, read_u64_raw_le, ReadU64RawLe, poll_read_u64_raw_le, from_le_bytes, InternalReadU64RawLe);
        read_raw_func!(u64, read_u64_raw_be, ReadU64RawBe, poll_read_u64_raw_be, from_be_bytes, InternalReadU64RawBe);
        read_raw_func!(i64, read_i64_raw_le, ReadI64RawLe, poll_read_i64_raw_le, from_le_bytes, InternalReadI64RawLe);
        read_raw_func!(i64, read_i64_raw_be, ReadI64RawBe, poll_read_i64_raw_be, from_be_bytes, InternalReadI64RawBe);

        read_raw_func!(u128, read_u128_raw_le, ReadU128RawLe, poll_read_u128_raw_le, from_le_bytes, InternalReadU128RawLe);
        read_raw_func!(u128, read_u128_raw_be, ReadU128RawBe, poll_read_u128_raw_be, from_be_bytes, InternalReadU128RawBe);
        read_raw_func!(i128, read_i128_raw_le, ReadI128RawLe, poll_read_i128_raw_le, from_le_bytes, InternalReadI128RawLe);
        read_raw_func!(i128, read_i128_raw_be, ReadI128RawBe, poll_read_i128_raw_be, from_be_bytes, InternalReadI128RawBe);

        read_raw_func!(f32, read_f32_raw_le, ReadF32RawLe, poll_read_f32_raw_le, from_le_bytes, InternalReadF32RawLe);
        read_raw_func!(f32, read_f32_raw_be, ReadF32RawBe, poll_read_f32_raw_be, from_be_bytes, InternalReadF32RawBe);
        read_raw_func!(f64, read_f64_raw_le, ReadF64RawLe, poll_read_f64_raw_le, from_le_bytes, InternalReadF64RawLe);
        read_raw_func!(f64, read_f64_raw_be, ReadF64RawBe, poll_read_f64_raw_be, from_be_bytes, InternalReadF64RawBe);
    };
}

define_read_raw_future!();
