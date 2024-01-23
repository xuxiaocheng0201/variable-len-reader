#[cfg(feature = "async_varint")]
macro_rules! write_varint_future {
    ($primitive: ty, $future: ident, $internal: ident, $to: ident, $poll_func: ident, $buf: ident, $internal_struct: ident) => {
        write_varint_future!($primitive, $primitive, $future, $internal, $to, $poll_func, $buf, $internal_struct);
    };
    ($primitive: ty, $source: ty, $future: ident, $internal: ident, $to: ident, $poll_func: ident, $buf: ident, $internal_struct: ident) => {
        #[derive(Debug)]
        struct $internal_struct {
            value: $primitive,
            size: usize,
            inner_buf: $buf,
        }
        impl $internal_struct {
            fn new(num: $source) -> Self {
                let num = num as $primitive;
                const NUM_BITS: $internal = <$internal>::MAX >> 1;
                const SIGN_BIT: $internal = NUM_BITS + 1;
                const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
                let (value, buf) = if num >= SIGN_BIT as $primitive {
                    (num >> POS_OFFSET, $buf::new(<$internal>::$to(((num & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT)))
                } else {
                    (0, $buf::new(<$internal>::$to(num as $internal)))
                };
                Self { value, size: 0, inner_buf: buf }
            }
        }
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                internal: $internal_struct,
            }
        }
        impl<'a, W: $crate::asynchronous::AsyncVariableWritable + Unpin> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                W::$poll_func(Pin::new(&mut *me.writer), cx, me.internal)
            }
        }
    };
}
#[cfg(feature = "async_varint")]
macro_rules! write_varint_poll {
    ($primitive: ty, $poll_func: ident, $internal: ty, $to: ident, $poll_internal: ident, $internal_struct: ident) => {
        #[inline]
        fn $poll_func(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, internal: &mut $internal_struct) -> std::task::Poll<std::io::Result<usize>> {
            const NUM_BITS: $internal = <$internal>::MAX >> 1;
            const SIGN_BIT: $internal = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
            loop {
                internal.size += ready!(self.as_mut().$poll_internal(cx, &mut internal.inner_buf))?;
                if internal.value >= SIGN_BIT as $primitive {
                    internal.inner_buf.reset();
                    internal.inner_buf.set_buf(<$internal>::$to(((internal.value & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT));
                    internal.value >>= POS_OFFSET;
                } else {
                    if (internal.value == 0) {
                        return std::task::Poll::Ready(Ok(internal.size));
                    }
                    internal.inner_buf.reset();
                    internal.inner_buf.set_buf(<$internal>::$to(internal.value as $internal));
                    internal.value = 0;
                }
            }
        }
    };
}
#[cfg(feature = "async_varint")]
macro_rules! write_varint_func {
    ($primitive: ty, $func: ident, $future: ident, $internal_struct: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            $future { writer: self, internal: $internal_struct::new(num) }
        }
    };
}
#[cfg(feature = "async_varint_size")]
macro_rules! write_varint_size_future {
    ($future: ident, $internal: ident, $to: ident, $poll_func: ident, $buf: ident, $internal_struct: ident) => {
        write_varint_future!(u128, usize, $future, $internal, $to, $poll_func, $buf, $internal_struct);
    };
}
#[cfg(feature = "async_varint_size")]
macro_rules! write_varint_size_poll {
    ($poll_func: ident, $internal: ty, $to: ident, $poll_internal: ident, $internal_struct: ident) => {
        write_varint_poll!(u128, $poll_func, $internal, $to, $poll_internal, $internal_struct);
    };
}
#[cfg(feature = "async_varint_size")]
macro_rules! write_varint_size_func {
    ($func: ident, $future: ident, $internal_struct: ident) => {
        write_varint_func!(usize, $func, $future, $internal_struct);
    };
}
#[cfg(feature = "async_varint")]
macro_rules! define_write_varint_futures {
    () => {
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u8, WriteU8Varint, u8, to_ne_bytes, poll_write_u8_varint, OwnedWriteBuf8, InternalWriteU8Varint);

        write_varint_future!(u16, WriteU16Varint, u8, to_ne_bytes, poll_write_u16_varint, OwnedWriteBuf8, InternalWriteU16Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u16, WriteU16Varint2Le, u16, to_le_bytes, poll_write_u16_varint_2_le, OwnedWriteBuf16, InternalWriteU16Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u16, WriteU16Varint2Be, u16, to_be_bytes, poll_write_u16_varint_2_be, OwnedWriteBuf16, InternalWriteU16Varint2Be);

        write_varint_future!(u32, WriteU32Varint, u8, to_ne_bytes, poll_write_u32_varint, OwnedWriteBuf8, InternalWriteU32Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u32, WriteU32Varint2Le, u16, to_le_bytes, poll_write_u32_varint_2_le, OwnedWriteBuf16, InternalWriteU32Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u32, WriteU32Varint2Be, u16, to_be_bytes, poll_write_u32_varint_2_be, OwnedWriteBuf16, InternalWriteU32Varint2Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u32, WriteU32Varint4Le, u32, to_le_bytes, poll_write_u32_varint_4_le, OwnedWriteBuf32, InternalWriteU32Varint4Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u32, WriteU32Varint4Be, u32, to_be_bytes, poll_write_u32_varint_4_be, OwnedWriteBuf32, InternalWriteU32Varint4Be);

        write_varint_future!(u64, WriteU64Varint, u8, to_ne_bytes, poll_write_u64_varint, OwnedWriteBuf8, InternalWriteU64Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint2Le, u16, to_le_bytes, poll_write_u64_varint_2_le, OwnedWriteBuf16, InternalWriteU64Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint2Be, u16, to_be_bytes, poll_write_u64_varint_2_be, OwnedWriteBuf16, InternalWriteU64Varint2Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint4Le, u32, to_le_bytes, poll_write_u64_varint_4_le, OwnedWriteBuf32, InternalWriteU64Varint4Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint4Be, u32, to_be_bytes, poll_write_u64_varint_4_be, OwnedWriteBuf32, InternalWriteU64Varint4Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint8Le, u64, to_le_bytes, poll_write_u64_varint_8_le, OwnedWriteBuf64, InternalWriteU64Varint8Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint8Be, u64, to_be_bytes, poll_write_u64_varint_8_be, OwnedWriteBuf64, InternalWriteU64Varint8Be);

        write_varint_future!(u128, WriteU128Varint, u8, to_ne_bytes, poll_write_u128_varint, OwnedWriteBuf8, InternalWriteU128Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint2Le, u16, to_le_bytes, poll_write_u128_varint_2_le, OwnedWriteBuf16, InternalWriteU128Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint2Be, u16, to_be_bytes, poll_write_u128_varint_2_be, OwnedWriteBuf16, InternalWriteU128Varint2Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint4Le, u32, to_le_bytes, poll_write_u128_varint_4_le, OwnedWriteBuf32, InternalWriteU128Varint4Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint4Be, u32, to_be_bytes, poll_write_u128_varint_4_be, OwnedWriteBuf32, InternalWriteU128Varint4Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint8Le, u64, to_le_bytes, poll_write_u128_varint_8_le, OwnedWriteBuf64, InternalWriteU128Varint8Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint8Be, u64, to_be_bytes, poll_write_u128_varint_8_be, OwnedWriteBuf64, InternalWriteU128Varint8Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint16Le, u128, to_le_bytes, poll_write_u128_varint_16_le, OwnedWriteBuf128, InternalWriteU128Varint16Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint16Be, u128, to_be_bytes, poll_write_u128_varint_16_be, OwnedWriteBuf128, InternalWriteU128Varint16Be);
        
        #[cfg(feature = "async_varint_size")]
        write_varint_size_future!(WriteUsizeVarint, u8, to_ne_bytes, poll_write_usize_varint, OwnedWriteBuf8, InternalWriteUsizeVarint);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_future!(WriteUsizeVarint2Le, u16, to_le_bytes, poll_write_usize_varint_2_le, OwnedWriteBuf16, InternalWriteUsizeVarint2Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_future!(WriteUsizeVarint2Be, u16, to_be_bytes, poll_write_usize_varint_2_be, OwnedWriteBuf16, InternalWriteUsizeVarint2Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_future!(WriteUsizeVarint4Le, u32, to_le_bytes, poll_write_usize_varint_4_le, OwnedWriteBuf32, InternalWriteUsizeVarint4Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_future!(WriteUsizeVarint4Be, u32, to_be_bytes, poll_write_usize_varint_4_be, OwnedWriteBuf32, InternalWriteUsizeVarint4Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_future!(WriteUsizeVarint8Le, u64, to_le_bytes, poll_write_usize_varint_8_le, OwnedWriteBuf64, InternalWriteUsizeVarint8Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_future!(WriteUsizeVarint8Be, u64, to_be_bytes, poll_write_usize_varint_8_be, OwnedWriteBuf64, InternalWriteUsizeVarint8Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_future!(WriteUsizeVarint16Le, u128, to_le_bytes, poll_write_usize_varint_16_le, OwnedWriteBuf128, InternalWriteUsizeVarint16Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_future!(WriteUsizeVarint16Be, u128, to_be_bytes, poll_write_usize_varint_16_be, OwnedWriteBuf128, InternalWriteUsizeVarint16Be);
    };
}
#[cfg(feature = "async_varint")]
macro_rules! define_write_varint_poll {
    () => {
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u8, poll_write_u8_varint, u8, to_ne_bytes, poll_write_u8_raw, InternalWriteU8Varint);

        write_varint_poll!(u16, poll_write_u16_varint, u8, to_ne_bytes, poll_write_u8_raw, InternalWriteU16Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u16, poll_write_u16_varint_2_le, u16, to_le_bytes, poll_write_u16_raw_le, InternalWriteU16Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u16, poll_write_u16_varint_2_be, u16, to_be_bytes, poll_write_u16_raw_be, InternalWriteU16Varint2Be);

        write_varint_poll!(u32, poll_write_u32_varint, u8, to_ne_bytes, poll_write_u8_raw, InternalWriteU32Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u32, poll_write_u32_varint_2_le, u16, to_le_bytes, poll_write_u16_raw_le, InternalWriteU32Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u32, poll_write_u32_varint_2_be, u16, to_be_bytes, poll_write_u16_raw_be, InternalWriteU32Varint2Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u32, poll_write_u32_varint_4_le, u32, to_le_bytes, poll_write_u32_raw_le, InternalWriteU32Varint4Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u32, poll_write_u32_varint_4_be, u32, to_be_bytes, poll_write_u32_raw_be, InternalWriteU32Varint4Be);

        write_varint_poll!(u64, poll_write_u64_varint, u8, to_ne_bytes, poll_write_u8_raw, InternalWriteU64Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u64, poll_write_u64_varint_2_le, u16, to_le_bytes, poll_write_u16_raw_le, InternalWriteU64Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u64, poll_write_u64_varint_2_be, u16, to_be_bytes, poll_write_u16_raw_be, InternalWriteU64Varint2Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u64, poll_write_u64_varint_4_le, u32, to_le_bytes, poll_write_u32_raw_le, InternalWriteU64Varint4Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u64, poll_write_u64_varint_4_be, u32, to_be_bytes, poll_write_u32_raw_be, InternalWriteU64Varint4Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u64, poll_write_u64_varint_8_le, u64, to_le_bytes, poll_write_u64_raw_le, InternalWriteU64Varint8Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u64, poll_write_u64_varint_8_be, u64, to_be_bytes, poll_write_u64_raw_be, InternalWriteU64Varint8Be);

        write_varint_poll!(u128, poll_write_u128_varint, u8, to_ne_bytes, poll_write_u8_raw, InternalWriteU128Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u128, poll_write_u128_varint_2_le, u16, to_le_bytes, poll_write_u16_raw_le, InternalWriteU128Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u128, poll_write_u128_varint_2_be, u16, to_be_bytes, poll_write_u16_raw_be, InternalWriteU128Varint2Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u128, poll_write_u128_varint_4_le, u32, to_le_bytes, poll_write_u32_raw_le, InternalWriteU128Varint4Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u128, poll_write_u128_varint_4_be, u32, to_be_bytes, poll_write_u32_raw_be, InternalWriteU128Varint4Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u128, poll_write_u128_varint_8_le, u64, to_le_bytes, poll_write_u64_raw_le, InternalWriteU128Varint8Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u128, poll_write_u128_varint_8_be, u64, to_be_bytes, poll_write_u64_raw_be, InternalWriteU128Varint8Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u128, poll_write_u128_varint_16_le, u128, to_le_bytes, poll_write_u128_raw_le, InternalWriteU128Varint16Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_poll!(u128, poll_write_u128_varint_16_be, u128, to_be_bytes, poll_write_u128_raw_be, InternalWriteU128Varint16Be);

        #[cfg(feature = "async_varint_size")]
        write_varint_size_poll!(poll_write_usize_varint, u8, to_ne_bytes, poll_write_u8_raw, InternalWriteUsizeVarint);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_poll!(poll_write_usize_varint_2_le, u16, to_le_bytes, poll_write_u16_raw_le, InternalWriteUsizeVarint2Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_poll!(poll_write_usize_varint_2_be, u16, to_be_bytes, poll_write_u16_raw_be, InternalWriteUsizeVarint2Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_poll!(poll_write_usize_varint_4_le, u32, to_le_bytes, poll_write_u32_raw_le, InternalWriteUsizeVarint4Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_poll!(poll_write_usize_varint_4_be, u32, to_be_bytes, poll_write_u32_raw_be, InternalWriteUsizeVarint4Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_poll!(poll_write_usize_varint_8_le, u64, to_le_bytes, poll_write_u64_raw_le, InternalWriteUsizeVarint8Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_poll!(poll_write_usize_varint_8_be, u64, to_be_bytes, poll_write_u64_raw_be, InternalWriteUsizeVarint8Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_poll!(poll_write_usize_varint_16_le, u128, to_le_bytes, poll_write_u128_raw_le, InternalWriteUsizeVarint16Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_poll!(poll_write_usize_varint_16_be, u128, to_be_bytes, poll_write_u128_raw_be, InternalWriteUsizeVarint16Be);
    };
}
#[cfg(feature = "async_varint")]
macro_rules! define_write_varint_func {
    () => {
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u8, write_u8_varint, WriteU8Varint, InternalWriteU8Varint);

        write_varint_func!(u16, write_u16_varint, WriteU16Varint, InternalWriteU16Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u16, write_u16_varint_2_le, WriteU16Varint2Le, InternalWriteU16Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u16, write_u16_varint_2_be, WriteU16Varint2Be, InternalWriteU16Varint2Be);

        write_varint_func!(u32, write_u32_varint, WriteU32Varint, InternalWriteU32Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u32, write_u32_varint_2_le, WriteU32Varint2Le, InternalWriteU32Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u32, write_u32_varint_2_be, WriteU32Varint2Be, InternalWriteU32Varint2Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u32, write_u32_varint_4_le, WriteU32Varint4Le, InternalWriteU32Varint4Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u32, write_u32_varint_4_be, WriteU32Varint4Be, InternalWriteU32Varint4Be);

        write_varint_func!(u64, write_u64_varint, WriteU64Varint, InternalWriteU64Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_2_le, WriteU64Varint2Le, InternalWriteU64Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_2_be, WriteU64Varint2Be, InternalWriteU64Varint2Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_4_le, WriteU64Varint4Le, InternalWriteU64Varint4Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_4_be, WriteU64Varint4Be, InternalWriteU64Varint4Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_8_le, WriteU64Varint8Le, InternalWriteU64Varint8Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_8_be, WriteU64Varint8Be, InternalWriteU64Varint8Be);

        write_varint_func!(u128, write_u128_varint, WriteU128Varint, InternalWriteU128Varint);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_2_le, WriteU128Varint2Le, InternalWriteU128Varint2Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_2_be, WriteU128Varint2Be, InternalWriteU128Varint2Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_4_le, WriteU128Varint4Le, InternalWriteU128Varint4Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_4_be, WriteU128Varint4Be, InternalWriteU128Varint4Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_8_le, WriteU128Varint8Le, InternalWriteU128Varint8Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_8_be, WriteU128Varint8Be, InternalWriteU128Varint8Be);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_16_le, WriteU128Varint16Le, InternalWriteU128Varint16Le);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_16_be, WriteU128Varint16Be, InternalWriteU128Varint16Be);
        
        #[cfg(feature = "async_varint_size")]
        write_varint_size_func!(write_usize_varint, WriteUsizeVarint, InternalWriteUsizeVarint);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_func!(write_usize_varint_2_le, WriteUsizeVarint2Le, InternalWriteUsizeVarint2Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_func!(write_usize_varint_2_be, WriteUsizeVarint2Be, InternalWriteUsizeVarint2Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_func!(write_usize_varint_4_le, WriteUsizeVarint4Le, InternalWriteUsizeVarint4Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_func!(write_usize_varint_4_be, WriteUsizeVarint4Be, InternalWriteUsizeVarint4Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_func!(write_usize_varint_8_le, WriteUsizeVarint8Le, InternalWriteUsizeVarint8Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_func!(write_usize_varint_8_be, WriteUsizeVarint8Be, InternalWriteUsizeVarint8Be);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_func!(write_usize_varint_16_le, WriteUsizeVarint16Le, InternalWriteUsizeVarint16Le);
        #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
        write_varint_size_func!(write_usize_varint_16_be, WriteUsizeVarint16Be, InternalWriteUsizeVarint16Be);
    };
}
#[cfg(feature = "async_varint")]
define_write_varint_futures!();
