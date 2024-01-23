#[cfg(feature = "async_varint")]
macro_rules! read_varint_future {
    ($primitive: ty, $future: ident, $poll_func: ident, $buf: ident, $internal_struct: ident) => {
        #[derive(Debug)]
        struct $internal_struct {
            value: $primitive,
            position: usize,
            inner_buf: $buf,
        }
        impl $internal_struct {
            fn new() -> Self {
                Self { value: 0, position: 0, inner_buf: $buf::new() }
            }
        }
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
                internal: $internal_struct,
            }
        }
        impl<'a, R: $crate::asynchronous::AsyncVariableReadable + Unpin> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$primitive>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                R::$poll_func(Pin::new(&mut *me.reader), cx, me.internal)
            }
        }
    };
}
#[cfg(feature = "async_varint")]
macro_rules! read_varint_poll {
    ($primitive: ty, $func: ident, $internal: ty, $poll_func: ident, $poll_internal: ident, $internal_struct: ident) => {
        #[inline]
        fn $poll_func(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, internal: &mut $internal_struct) -> std::task::Poll<std::io::Result<$primitive>> {
            const SIZE: usize = std::mem::size_of::<$primitive>() << 3; // * 8
            const NUM_BITS: $internal = <$internal>::MAX >> 1;
            const SIGN_BIT: $internal = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
            loop {
                let current = ready!(self.as_mut().$poll_internal(cx, &mut internal.inner_buf))?;
                internal.inner_buf.clear();
                internal.value |= ((current & NUM_BITS) as $primitive) << internal.position;
                if current & SIGN_BIT == 0 {
                    return Poll::Ready(Ok(internal.value));
                }
                internal.position += POS_OFFSET;
                if internal.position >= SIZE {
                    return Poll::Ready(Err(Error::new(ErrorKind::InvalidData, format!("Varint {} in stream is too long.", stringify!($func)))));
                }
            }
        }
    };
}
#[cfg(feature = "async_varint")]
macro_rules! read_varint_func {
    ($func: ident, $future: ident, $internal_struct: ident) => {
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, internal: $internal_struct::new() }
        }
    };
}
#[cfg(feature = "async_varint")]
macro_rules! define_read_varint_futures {
    () => {
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u8, ReadU8Varint, poll_read_u8_varint, OwnedReadBuf8, InternalReadU8Varint);

        read_varint_future!(u16, ReadU16Varint, poll_read_u16_varint, OwnedReadBuf8, InternalReadU16Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u16, ReadU16Varint2Le, poll_read_u16_varint_2_le, OwnedReadBuf16, InternalReadU16Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u16, ReadU16Varint2Be, poll_read_u16_varint_2_be, OwnedReadBuf16, InternalReadU16Varint2Be);

        read_varint_future!(u32, ReadU32Varint, poll_read_u32_varint, OwnedReadBuf8, InternalReadU32Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u32, ReadU32Varint2Le, poll_read_u32_varint_2_le, OwnedReadBuf16, InternalReadU32Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u32, ReadU32Varint2Be, poll_read_u32_varint_2_be, OwnedReadBuf16, InternalReadU32Varint2Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u32, ReadU32Varint4Le, poll_read_u32_varint_4_le, OwnedReadBuf32, InternalReadU32Varint4Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u32, ReadU32Varint4Be, poll_read_u32_varint_4_be, OwnedReadBuf32, InternalReadU32Varint4Be);

        read_varint_future!(u64, ReadU64Varint, poll_read_u64_varint, OwnedReadBuf8, InternalReadU64Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint2Le, poll_read_u64_varint_2_le, OwnedReadBuf16, InternalReadU64Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint2Be, poll_read_u64_varint_2_be, OwnedReadBuf16, InternalReadU64Varint2Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint4Le, poll_read_u64_varint_4_le, OwnedReadBuf32, InternalReadU64Varint4Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint4Be, poll_read_u64_varint_4_be, OwnedReadBuf32, InternalReadU64Varint4Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint8Le, poll_read_u64_varint_8_le, OwnedReadBuf64, InternalReadU64Varint8Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint8Be, poll_read_u64_varint_8_be, OwnedReadBuf64, InternalReadU64Varint8Be);

        read_varint_future!(u128, ReadU128Varint, poll_read_u128_varint, OwnedReadBuf8, InternalReadU128Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint2Le, poll_read_u128_varint_2_le, OwnedReadBuf16, InternalReadU128Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint2Be, poll_read_u128_varint_2_be, OwnedReadBuf16, InternalReadU128Varint2Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint4Le, poll_read_u128_varint_4_le, OwnedReadBuf32, InternalReadU128Varint4Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint4Be, poll_read_u128_varint_4_be, OwnedReadBuf32, InternalReadU128Varint4Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint8Le, poll_read_u128_varint_8_le, OwnedReadBuf64, InternalReadU128Varint8Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint8Be, poll_read_u128_varint_8_be, OwnedReadBuf64, InternalReadU128Varint8Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint16Le, poll_read_u128_varint_16_le, OwnedReadBuf128, InternalReadU128Varint16Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint16Be, poll_read_u128_varint_16_be, OwnedReadBuf128, InternalReadU128Varint16Be);
    };
}
#[cfg(feature = "async_varint")]
macro_rules! define_read_varint_poll {
    () => {
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u8, read_u8_varint, u8, poll_read_u8_varint, poll_read_u8_raw, InternalReadU8Varint);

        read_varint_poll!(u16, read_u16_varint, u8, poll_read_u16_varint, poll_read_u8_raw, InternalReadU16Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u16, read_u16_varint_2_le, u16, poll_read_u16_varint_2_le, poll_read_u16_raw_le, InternalReadU16Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u16, read_u16_varint_2_be, u16, poll_read_u16_varint_2_be, poll_read_u16_raw_be, InternalReadU16Varint2Be);

        read_varint_poll!(u32, read_u32_varint, u8, poll_read_u32_varint, poll_read_u8_raw, InternalReadU32Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u32, read_u32_varint_2_le, u16, poll_read_u32_varint_2_le, poll_read_u16_raw_le, InternalReadU32Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u32, read_u32_varint_2_be, u16, poll_read_u32_varint_2_be, poll_read_u16_raw_be, InternalReadU32Varint2Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u32, read_u32_varint_4_le, u32, poll_read_u32_varint_4_le, poll_read_u32_raw_le, InternalReadU32Varint4Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u32, read_u32_varint_4_be, u32, poll_read_u32_varint_4_be, poll_read_u32_raw_be, InternalReadU32Varint4Be);

        read_varint_poll!(u64, read_u64_varint, u8, poll_read_u64_varint, poll_read_u8_raw, InternalReadU64Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u64, read_u64_varint_2_le, u16, poll_read_u64_varint_2_le, poll_read_u16_raw_le, InternalReadU64Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u64, read_u64_varint_2_be, u16, poll_read_u64_varint_2_be, poll_read_u16_raw_be, InternalReadU64Varint2Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u64, read_u64_varint_4_le, u32, poll_read_u64_varint_4_le, poll_read_u32_raw_le, InternalReadU64Varint4Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u64, read_u64_varint_4_be, u32, poll_read_u64_varint_4_be, poll_read_u32_raw_be, InternalReadU64Varint4Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u64, read_u64_varint_8_le, u64, poll_read_u64_varint_8_le, poll_read_u64_raw_le, InternalReadU64Varint8Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u64, read_u64_varint_8_be, u64, poll_read_u64_varint_8_be, poll_read_u64_raw_be, InternalReadU64Varint8Be);

        read_varint_poll!(u128, read_u128_varint, u8, poll_read_u128_varint, poll_read_u8_raw, InternalReadU128Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u128, read_u128_varint_2_le, u16, poll_read_u128_varint_2_le, poll_read_u16_raw_le, InternalReadU128Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u128, read_u128_varint_2_be, u16, poll_read_u128_varint_2_be, poll_read_u16_raw_be, InternalReadU128Varint2Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u128, read_u128_varint_4_le, u32, poll_read_u128_varint_4_le, poll_read_u32_raw_le, InternalReadU128Varint4Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u128, read_u128_varint_4_be, u32, poll_read_u128_varint_4_be, poll_read_u32_raw_be, InternalReadU128Varint4Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u128, read_u128_varint_8_le, u64, poll_read_u128_varint_8_le, poll_read_u64_raw_le, InternalReadU128Varint8Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u128, read_u128_varint_8_be, u64, poll_read_u128_varint_8_be, poll_read_u64_raw_be, InternalReadU128Varint8Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u128, read_u128_varint_16_le, u128, poll_read_u128_varint_16_le, poll_read_u128_raw_le, InternalReadU128Varint16Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_poll!(u128, read_u128_varint_16_be, u128, poll_read_u128_varint_16_be, poll_read_u128_raw_be, InternalReadU128Varint16Be);
    };
}
#[cfg(feature = "async_varint")]
macro_rules! define_read_varint_func {
    () => {
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u8_varint, ReadU8Varint, InternalReadU8Varint);

        read_varint_func!(read_u16_varint, ReadU16Varint, InternalReadU16Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u16_varint_2_le, ReadU16Varint2Le, InternalReadU16Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u16_varint_2_be, ReadU16Varint2Be, InternalReadU16Varint2Be);

        read_varint_func!(read_u32_varint, ReadU32Varint, InternalReadU32Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u32_varint_2_le, ReadU32Varint2Le, InternalReadU32Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u32_varint_2_be, ReadU32Varint2Be, InternalReadU32Varint2Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u32_varint_4_le, ReadU32Varint4Le, InternalReadU32Varint4Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u32_varint_4_be, ReadU32Varint4Be, InternalReadU32Varint4Be);

        read_varint_func!(read_u64_varint, ReadU64Varint, InternalReadU64Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_2_le, ReadU64Varint2Le, InternalReadU64Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_2_be, ReadU64Varint2Be, InternalReadU64Varint2Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_4_le, ReadU64Varint4Le, InternalReadU64Varint4Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_4_be, ReadU64Varint4Be, InternalReadU64Varint4Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_8_le, ReadU64Varint8Le, InternalReadU64Varint8Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_8_be, ReadU64Varint8Be, InternalReadU64Varint8Be);

        read_varint_func!(read_u128_varint, ReadU128Varint, InternalReadU128Varint);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_2_le, ReadU128Varint2Le, InternalReadU128Varint2Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_2_be, ReadU128Varint2Be, InternalReadU128Varint2Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_4_le, ReadU128Varint4Le, InternalReadU128Varint4Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_4_be, ReadU128Varint4Be, InternalReadU128Varint4Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_8_le, ReadU128Varint8Le, InternalReadU128Varint8Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_8_be, ReadU128Varint8Be, InternalReadU128Varint8Be);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_16_le, ReadU128Varint16Le, InternalReadU128Varint16Le);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_16_be, ReadU128Varint16Be, InternalReadU128Varint16Be);
    };
}
#[cfg(feature = "async_varint")]
define_read_varint_futures!();
