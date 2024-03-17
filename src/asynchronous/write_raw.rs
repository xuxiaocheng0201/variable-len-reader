macro_rules! write_raw_future {
    ($primitive: ty, $future: ident, $to: ident) => {
        write_raw_future!(f cfg(feature = "async_raw"), $primitive, $future, $to);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $to: ident) => {
        #[$feature]
        $crate::pin_project_lite::pin_project! {
            #[cfg_attr(docsrs, doc($feature))]
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                buf: OwnedWriteBuf<[u8; ::core::mem::size_of::<$primitive>()]>,
            }
        }
        #[$feature]
        impl<'a, W: ?Sized> WriterFuture<'a, W, $primitive> for $future<'a, W> {
            fn new(writer: &'a mut W, buf: $primitive) -> Self {
                Self { writer, buf: OwnedWriteBuf::new(Self::_handle(buf)) }
            }
            fn reset(self: Pin<&mut Self>, buf: $primitive) {
                let me = self.project();
                *me.buf = OwnedWriteBuf::new(Self::_handle(buf));
            }
        }
        #[$feature]
        impl<'a, W: AsyncVariableWriter + Unpin + ?Sized> Future for $future<'a, W> {
            type Output = ::core::result::Result<(), W::Error>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut me = self.project();
                let mut ref_buf = me.buf.into();
                let res = W::poll_write_more(Pin::new(&mut *me.writer), cx, &mut ref_buf);
                let position = ref_buf.position();
                me.buf.set_position(position);
                res
            }
        }
        #[$feature]
        impl<'a, W: ?Sized> $future<'a, W> {
            fn _handle(value: $primitive) -> [u8; ::core::mem::size_of::<$primitive>()] {
                <$primitive>::$to(value)
            }
        }
    };
}
macro_rules! write_raw_func {
    ($primitive: ty, $future: ident, $func: ident) => {
        write_raw_func!(f cfg(feature = "async_raw"), $primitive, $future, $func);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $func: ident) => {
        write_wrap_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_raw_future {
    () => {
        write_raw_future!(u8, WriteU8Raw, to_ne_bytes);
        write_raw_future!(i8, WriteI8Raw, to_ne_bytes);

        write_raw_future!(u16, WriteU16RawLe, to_le_bytes);
        write_raw_future!(u16, WriteU16RawBe, to_be_bytes);
        write_raw_future!(i16, WriteI16RawLe, to_le_bytes);
        write_raw_future!(i16, WriteI16RawBe, to_be_bytes);

        write_raw_future!(u32, WriteU32RawLe, to_le_bytes);
        write_raw_future!(u32, WriteU32RawBe, to_be_bytes);
        write_raw_future!(i32, WriteI32RawLe, to_le_bytes);
        write_raw_future!(i32, WriteI32RawBe, to_be_bytes);

        write_raw_future!(u64, WriteU64RawLe, to_le_bytes);
        write_raw_future!(u64, WriteU64RawBe, to_be_bytes);
        write_raw_future!(i64, WriteI64RawLe, to_le_bytes);
        write_raw_future!(i64, WriteI64RawBe, to_be_bytes);

        write_raw_future!(u128, WriteU128RawLe, to_le_bytes);
        write_raw_future!(u128, WriteU128RawBe, to_be_bytes);
        write_raw_future!(i128, WriteI128RawLe, to_le_bytes);
        write_raw_future!(i128, WriteI128RawBe, to_be_bytes);

        write_raw_future!(f32, WriteF32RawLe, to_le_bytes);
        write_raw_future!(f32, WriteF32RawBe, to_be_bytes);
        write_raw_future!(f64, WriteF64RawLe, to_le_bytes);
        write_raw_future!(f64, WriteF64RawBe, to_be_bytes);
    };
}
macro_rules! define_write_raw_func {
    () => {
        write_raw_func!(u8, WriteU8Raw, write_u8_raw);
        write_raw_func!(i8, WriteI8Raw, write_i8_raw);

        write_raw_func!(u16, WriteU16RawLe, write_u16_raw_le);
        write_raw_func!(u16, WriteU16RawBe, write_u16_raw_be);
        write_raw_func!(i16, WriteI16RawLe, write_i16_raw_le);
        write_raw_func!(i16, WriteI16RawBe, write_i16_raw_be);

        write_raw_func!(u32, WriteU32RawLe, write_u32_raw_le);
        write_raw_func!(u32, WriteU32RawBe, write_u32_raw_be);
        write_raw_func!(i32, WriteI32RawLe, write_i32_raw_le);
        write_raw_func!(i32, WriteI32RawBe, write_i32_raw_be);

        write_raw_func!(u64, WriteU64RawLe, write_u64_raw_le);
        write_raw_func!(u64, WriteU64RawBe, write_u64_raw_be);
        write_raw_func!(i64, WriteI64RawLe, write_i64_raw_le);
        write_raw_func!(i64, WriteI64RawBe, write_i64_raw_be);

        write_raw_func!(u128, WriteU128RawLe, write_u128_raw_le);
        write_raw_func!(u128, WriteU128RawBe, write_u128_raw_be);
        write_raw_func!(i128, WriteI128RawLe, write_i128_raw_le);
        write_raw_func!(i128, WriteI128RawBe, write_i128_raw_be);

        write_raw_func!(f32, WriteF32RawLe, write_f32_raw_le);
        write_raw_func!(f32, WriteF32RawBe, write_f32_raw_be);
        write_raw_func!(f64, WriteF64RawLe, write_f64_raw_le);
        write_raw_func!(f64, WriteF64RawBe, write_f64_raw_be);
    };
}

define_write_raw_future!();
