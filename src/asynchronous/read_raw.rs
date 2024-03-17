macro_rules! read_raw_future {
    ($primitive: ty, $future: ident, $from: ident) => {
        read_raw_future!(f cfg(feature = "async_raw"), $primitive, $future, $from);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $from: ident) => {
        #[$feature]
        $crate::pin_project_lite::pin_project! {
            #[cfg_attr(docsrs, doc($feature))]
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
                buf: OwnedReadBuf<[u8; ::core::mem::size_of::<$primitive>()]>,
            }
        }
        #[$feature]
        impl<'a, R: ?Sized> ReaderFuture for $future<'a, R> {
            fn reset(self: Pin<&mut Self>) {
                let me = self.project();
                me.buf.reset();
            }
        }
        #[$feature]
        impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for $future<'a, R> {
            type Output = ::core::result::Result<$primitive, R::Error>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut me = self.project();
                let mut ref_buf = me.buf.into();
                let res = R::poll_read_more(Pin::new(&mut *me.reader), cx, &mut ref_buf);
                let position = ref_buf.position();
                me.buf.set_position(position);
                ::core::task::ready!(res)?;
                Poll::Ready(Ok(<$primitive>::$from(me.buf.clone().into_inner())))
            }
        }
    };
}
macro_rules! read_raw_func {
    ($primitive: ty, $func: ident, $future: ident) => {
        read_raw_func!(f cfg(feature = "async_raw"), $primitive, $func, $future);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $future: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, buf: OwnedReadBuf::new([0; ::core::mem::size_of::<$primitive>()]) }
        }
    };
}

macro_rules! define_read_raw_future {
    () => {
        read_raw_future!(u8, ReadU8Raw, from_ne_bytes);
        read_raw_future!(i8, ReadI8Raw, from_ne_bytes);

        read_raw_future!(u16, ReadU16RawLe, from_le_bytes);
        read_raw_future!(u16, ReadU16RawBe, from_be_bytes);
        read_raw_future!(i16, ReadI16RawLe, from_le_bytes);
        read_raw_future!(i16, ReadI16RawBe, from_be_bytes);

        read_raw_future!(u32, ReadU32RawLe, from_le_bytes);
        read_raw_future!(u32, ReadU32RawBe, from_be_bytes);
        read_raw_future!(i32, ReadI32RawLe, from_le_bytes);
        read_raw_future!(i32, ReadI32RawBe, from_be_bytes);

        read_raw_future!(u64, ReadU64RawLe, from_le_bytes);
        read_raw_future!(u64, ReadU64RawBe, from_be_bytes);
        read_raw_future!(i64, ReadI64RawLe, from_le_bytes);
        read_raw_future!(i64, ReadI64RawBe, from_be_bytes);

        read_raw_future!(u128, ReadU128RawLe, from_le_bytes);
        read_raw_future!(u128, ReadU128RawBe, from_be_bytes);
        read_raw_future!(i128, ReadI128RawLe, from_le_bytes);
        read_raw_future!(i128, ReadI128RawBe, from_be_bytes);

        read_raw_future!(f32, ReadF32RawLe, from_le_bytes);
        read_raw_future!(f32, ReadF32RawBe, from_be_bytes);
        read_raw_future!(f64, ReadF64RawLe, from_le_bytes);
        read_raw_future!(f64, ReadF64RawBe, from_be_bytes);
    };
}
macro_rules! define_read_raw_func {
    () => {
        read_raw_func!(u8, read_u8_raw, ReadU8Raw);
        read_raw_func!(i8, read_i8_raw, ReadI8Raw);

        read_raw_func!(u16, read_u16_raw_le, ReadU16RawLe);
        read_raw_func!(u16, read_u16_raw_be, ReadU16RawBe);
        read_raw_func!(i16, read_i16_raw_le, ReadI16RawLe);
        read_raw_func!(i16, read_i16_raw_be, ReadI16RawBe);

        read_raw_func!(u32, read_u32_raw_le, ReadU32RawLe);
        read_raw_func!(u32, read_u32_raw_be, ReadU32RawBe);
        read_raw_func!(i32, read_i32_raw_le, ReadI32RawLe);
        read_raw_func!(i32, read_i32_raw_be, ReadI32RawBe);

        read_raw_func!(u64, read_u64_raw_le, ReadU64RawLe);
        read_raw_func!(u64, read_u64_raw_be, ReadU64RawBe);
        read_raw_func!(i64, read_i64_raw_le, ReadI64RawLe);
        read_raw_func!(i64, read_i64_raw_be, ReadI64RawBe);

        read_raw_func!(u128, read_u128_raw_le, ReadU128RawLe);
        read_raw_func!(u128, read_u128_raw_be, ReadU128RawBe);
        read_raw_func!(i128, read_i128_raw_le, ReadI128RawLe);
        read_raw_func!(i128, read_i128_raw_be, ReadI128RawBe);

        read_raw_func!(f32, read_f32_raw_le, ReadF32RawLe);
        read_raw_func!(f32, read_f32_raw_be, ReadF32RawBe);
        read_raw_func!(f64, read_f64_raw_le, ReadF64RawLe);
        read_raw_func!(f64, read_f64_raw_be, ReadF64RawBe);
    };
}

define_read_raw_future!();
