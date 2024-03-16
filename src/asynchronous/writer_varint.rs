macro_rules! write_varint_future {
    (varint, $primitive: ty, $future: ident, $internal: ident, $to: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_varint_future!(cfg(feature = "async_varint"), $primitive, $primitive, $future, $internal, $to, $poll_func, $struct_buf, $internal_struct);
    };
    (long_varint, $primitive: ty, $future: ident, $internal: ident, $to: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_varint_future!(cfg(feature = "async_long_varint"), $primitive, $primitive, $future, $internal, $to, $poll_func, $struct_buf, $internal_struct);
    };
    ($feature: meta, $primitive: ty, $source: ty, $future: ident, $internal: ident, $to: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[derive(Debug)]
        struct $struct_buf {
            value: $primitive,
            size: usize,
            internal: $internal_struct,
        }
        #[$feature]
        impl $struct_buf {
            fn _handle(num: $primitive) -> ($primitive, $internal) {
                const NUM_BITS: $internal = <$internal>::MAX >> 1;
                const SIGN_BIT: $internal = NUM_BITS + 1;
                const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
                if num >= SIGN_BIT as $primitive {
                    (num >> POS_OFFSET, ((num & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT)
                } else {
                    (0, num as $internal)
                }
            }
            fn new(num: $source) -> Self {
                let (value, internal) = $struct_buf::_handle(num as $primitive);
                Self { value, size: 0, internal: $internal_struct::new(internal) }
            }
            fn reset(&mut self, num: $source) {
                let (value, internal) = $struct_buf::_handle(num as $primitive);
                self.value = value;
                self.size = 0;
                self.internal.reset(internal);
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
        impl<'a, W: $crate::AsyncVariableWritable + Unpin + ?Sized> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                W::$poll_func(Pin::new(&mut *me.writer), cx, me.inner)
            }
        }
    };
}
macro_rules! write_varint_poll {
    (varint, $primitive: ty, $poll_func: ident, $internal: ty, $poll_internal: ident, $struct_buf: ident) => {
        write_varint_poll!(cfg(feature = "async_varint"), $primitive, $poll_func, $internal, $poll_internal, $struct_buf);
    };
    (long_varint, $primitive: ty, $poll_func: ident, $internal: ty, $poll_internal: ident, $struct_buf: ident) => {
        write_varint_poll!(cfg(feature = "async_long_varint"), $primitive, $poll_func, $internal, $poll_internal, $struct_buf);
    };
    ($feature: meta, $primitive: ty, $poll_func: ident, $internal: ty, $poll_internal: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $poll_func(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, inner: &mut $struct_buf) -> std::task::Poll<std::io::Result<usize>> {
            const NUM_BITS: $internal = <$internal>::MAX >> 1;
            const SIGN_BIT: $internal = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
            loop {
                inner.size += ready!(self.as_mut().$poll_internal(cx, &mut inner.internal))?;
                if inner.value >= SIGN_BIT as $primitive {
                    inner.internal.reset(((inner.value & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT);
                    inner.value >>= POS_OFFSET;
                } else {
                    if (inner.value == 0) {
                        return std::task::Poll::Ready(Ok(inner.size));
                    }
                    inner.internal.reset(inner.value as $internal);
                    inner.value = 0;
                }
            }
        }
    };
}
macro_rules! write_varint_func {
    (varint, $primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        write_varint_func!(cfg(feature = "async_varint"), $primitive, $func, $future, $struct_buf);
    };
    (long_varint, $primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        write_varint_func!(cfg(feature = "async_long_varint"), $primitive, $func, $future, $struct_buf);
    };
    ($feature: meta, $primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            $future { writer: self, inner: $struct_buf::new(num) }
        }
    };
}
macro_rules! write_varint_size_future {
    (varint, $future: ident, $internal: ident, $to: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_varint_size_future!(cfg(feature = "async_varint_size"), $future, $internal, $to, $poll_func, $struct_buf, $internal_struct);
    };
    (long_varint, $future: ident, $internal: ident, $to: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_varint_size_future!(cfg(all(feature = "async_varint_size", feature = "async_long_varint")), $future, $internal, $to, $poll_func, $struct_buf, $internal_struct);
    };
    ($feature: meta, $future: ident, $internal: ident, $to: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_varint_future!($feature, u128, usize, $future, $internal, $to, $poll_func, $struct_buf, $internal_struct);
    };
}
macro_rules! write_varint_size_poll {
    (varint, $poll_func: ident, $internal: ty, $poll_internal: ident, $struct_buf: ident) => {
        write_varint_size_poll!(cfg(feature = "async_varint_size"), $poll_func, $internal, $poll_internal, $struct_buf);
    };
    (long_varint, $poll_func: ident, $internal: ty, $poll_internal: ident, $struct_buf: ident) => {
        write_varint_size_poll!(cfg(all(feature = "async_varint_size", feature = "async_long_varint")), $poll_func, $internal, $poll_internal, $struct_buf);
    };
    ($feature: meta, $poll_func: ident, $internal: ty, $poll_internal: ident, $struct_buf: ident) => {
        write_varint_poll!($feature, u128, $poll_func, $internal, $poll_internal, $struct_buf);
    };
}
macro_rules! write_varint_size_func {
    (varint, $func: ident, $future: ident, $struct_buf: ident) => {
        write_varint_size_func!(cfg(feature = "async_varint_size"), $func, $future, $struct_buf);
    };
    (long_varint, $func: ident, $future: ident, $struct_buf: ident) => {
        write_varint_size_func!(cfg(all(feature = "async_varint_size", feature = "async_long_varint")), $func, $future, $struct_buf);
    };
    ($feature: meta, $func: ident, $future: ident, $struct_buf: ident) => {
        write_varint_func!($feature, usize, $func, $future, $struct_buf);
    };
}
macro_rules! define_write_varint_futures {
    () => {
        write_varint_future!(long_varint, u8, WriteU8Varint, u8, to_ne_bytes, poll_write_u8_varint, InternalWriteU8Varint, InternalWriteU8Raw);

        write_varint_future!(varint, u16, WriteU16Varint, u8, to_ne_bytes, poll_write_u16_varint, InternalWriteU16Varint, InternalWriteU8Raw);
        write_varint_future!(long_varint, u16, WriteU16Varint2Le, u16, to_le_bytes, poll_write_u16_varint_2_le, InternalWriteU16Varint2Le, InternalWriteU16RawLe);
        write_varint_future!(long_varint, u16, WriteU16Varint2Be, u16, to_be_bytes, poll_write_u16_varint_2_be, InternalWriteU16Varint2Be, InternalWriteU16RawBe);

        write_varint_future!(varint, u32, WriteU32Varint, u8, to_ne_bytes, poll_write_u32_varint, InternalWriteU32Varint, InternalWriteU8Raw);
        write_varint_future!(long_varint, u32, WriteU32Varint2Le, u16, to_le_bytes, poll_write_u32_varint_2_le, InternalWriteU32Varint2Le, InternalWriteU16RawLe);
        write_varint_future!(long_varint, u32, WriteU32Varint2Be, u16, to_be_bytes, poll_write_u32_varint_2_be, InternalWriteU32Varint2Be, InternalWriteU16RawBe);
        write_varint_future!(long_varint, u32, WriteU32Varint4Le, u32, to_le_bytes, poll_write_u32_varint_4_le, InternalWriteU32Varint4Le, InternalWriteU32RawLe);
        write_varint_future!(long_varint, u32, WriteU32Varint4Be, u32, to_be_bytes, poll_write_u32_varint_4_be, InternalWriteU32Varint4Be, InternalWriteU32RawBe);

        write_varint_future!(varint, u64, WriteU64Varint, u8, to_ne_bytes, poll_write_u64_varint, InternalWriteU64Varint, InternalWriteU8Raw);
        write_varint_future!(long_varint, u64, WriteU64Varint2Le, u16, to_le_bytes, poll_write_u64_varint_2_le, InternalWriteU64Varint2Le, InternalWriteU16RawLe);
        write_varint_future!(long_varint, u64, WriteU64Varint2Be, u16, to_be_bytes, poll_write_u64_varint_2_be, InternalWriteU64Varint2Be, InternalWriteU16RawBe);
        write_varint_future!(long_varint, u64, WriteU64Varint4Le, u32, to_le_bytes, poll_write_u64_varint_4_le, InternalWriteU64Varint4Le, InternalWriteU32RawLe);
        write_varint_future!(long_varint, u64, WriteU64Varint4Be, u32, to_be_bytes, poll_write_u64_varint_4_be, InternalWriteU64Varint4Be, InternalWriteU32RawBe);
        write_varint_future!(long_varint, u64, WriteU64Varint8Le, u64, to_le_bytes, poll_write_u64_varint_8_le, InternalWriteU64Varint8Le, InternalWriteU64RawLe);
        write_varint_future!(long_varint, u64, WriteU64Varint8Be, u64, to_be_bytes, poll_write_u64_varint_8_be, InternalWriteU64Varint8Be, InternalWriteU64RawBe);

        write_varint_future!(varint, u128, WriteU128Varint, u8, to_ne_bytes, poll_write_u128_varint, InternalWriteU128Varint, InternalWriteU8Raw);
        write_varint_future!(long_varint, u128, WriteU128Varint2Le, u16, to_le_bytes, poll_write_u128_varint_2_le, InternalWriteU128Varint2Le, InternalWriteU16RawLe);
        write_varint_future!(long_varint, u128, WriteU128Varint2Be, u16, to_be_bytes, poll_write_u128_varint_2_be, InternalWriteU128Varint2Be, InternalWriteU16RawBe);
        write_varint_future!(long_varint, u128, WriteU128Varint4Le, u32, to_le_bytes, poll_write_u128_varint_4_le, InternalWriteU128Varint4Le, InternalWriteU32RawLe);
        write_varint_future!(long_varint, u128, WriteU128Varint4Be, u32, to_be_bytes, poll_write_u128_varint_4_be, InternalWriteU128Varint4Be, InternalWriteU32RawBe);
        write_varint_future!(long_varint, u128, WriteU128Varint8Le, u64, to_le_bytes, poll_write_u128_varint_8_le, InternalWriteU128Varint8Le, InternalWriteU64RawLe);
        write_varint_future!(long_varint, u128, WriteU128Varint8Be, u64, to_be_bytes, poll_write_u128_varint_8_be, InternalWriteU128Varint8Be, InternalWriteU64RawBe);
        write_varint_future!(long_varint, u128, WriteU128Varint16Le, u128, to_le_bytes, poll_write_u128_varint_16_le, InternalWriteU128Varint16Le, InternalWriteU128RawLe);
        write_varint_future!(long_varint, u128, WriteU128Varint16Be, u128, to_be_bytes, poll_write_u128_varint_16_be, InternalWriteU128Varint16Be, InternalWriteU128RawBe);

        write_varint_size_future!(varint, WriteUsizeVarint, u8, to_ne_bytes, poll_write_usize_varint, InternalWriteUsizeVarint, InternalWriteU8Raw);
        write_varint_size_future!(long_varint, WriteUsizeVarint2Le, u16, to_le_bytes, poll_write_usize_varint_2_le, InternalWriteUsizeVarint2Le, InternalWriteU16RawLe);
        write_varint_size_future!(long_varint, WriteUsizeVarint2Be, u16, to_be_bytes, poll_write_usize_varint_2_be, InternalWriteUsizeVarint2Be, InternalWriteU16RawBe);
        write_varint_size_future!(long_varint, WriteUsizeVarint4Le, u32, to_le_bytes, poll_write_usize_varint_4_le, InternalWriteUsizeVarint4Le, InternalWriteU32RawLe);
        write_varint_size_future!(long_varint, WriteUsizeVarint4Be, u32, to_be_bytes, poll_write_usize_varint_4_be, InternalWriteUsizeVarint4Be, InternalWriteU32RawBe);
        write_varint_size_future!(long_varint, WriteUsizeVarint8Le, u64, to_le_bytes, poll_write_usize_varint_8_le, InternalWriteUsizeVarint8Le, InternalWriteU64RawLe);
        write_varint_size_future!(long_varint, WriteUsizeVarint8Be, u64, to_be_bytes, poll_write_usize_varint_8_be, InternalWriteUsizeVarint8Be, InternalWriteU64RawBe);
        write_varint_size_future!(long_varint, WriteUsizeVarint16Le, u128, to_le_bytes, poll_write_usize_varint_16_le, InternalWriteUsizeVarint16Le, InternalWriteU128RawLe);
        write_varint_size_future!(long_varint, WriteUsizeVarint16Be, u128, to_be_bytes, poll_write_usize_varint_16_be, InternalWriteUsizeVarint16Be, InternalWriteU128RawBe);
    };
}
macro_rules! define_write_varint_poll {
    () => {
        write_varint_poll!(long_varint, u8, poll_write_u8_varint, u8, poll_write_u8_raw, InternalWriteU8Varint);

        write_varint_poll!(varint, u16, poll_write_u16_varint, u8, poll_write_u8_raw, InternalWriteU16Varint);
        write_varint_poll!(long_varint, u16, poll_write_u16_varint_2_le, u16, poll_write_u16_raw_le, InternalWriteU16Varint2Le);
        write_varint_poll!(long_varint, u16, poll_write_u16_varint_2_be, u16, poll_write_u16_raw_be, InternalWriteU16Varint2Be);

        write_varint_poll!(varint, u32, poll_write_u32_varint, u8, poll_write_u8_raw, InternalWriteU32Varint);
        write_varint_poll!(long_varint, u32, poll_write_u32_varint_2_le, u16, poll_write_u16_raw_le, InternalWriteU32Varint2Le);
        write_varint_poll!(long_varint, u32, poll_write_u32_varint_2_be, u16, poll_write_u16_raw_be, InternalWriteU32Varint2Be);
        write_varint_poll!(long_varint, u32, poll_write_u32_varint_4_le, u32, poll_write_u32_raw_le, InternalWriteU32Varint4Le);
        write_varint_poll!(long_varint, u32, poll_write_u32_varint_4_be, u32, poll_write_u32_raw_be, InternalWriteU32Varint4Be);

        write_varint_poll!(varint, u64, poll_write_u64_varint, u8, poll_write_u8_raw, InternalWriteU64Varint);
        write_varint_poll!(long_varint, u64, poll_write_u64_varint_2_le, u16, poll_write_u16_raw_le, InternalWriteU64Varint2Le);
        write_varint_poll!(long_varint, u64, poll_write_u64_varint_2_be, u16, poll_write_u16_raw_be, InternalWriteU64Varint2Be);
        write_varint_poll!(long_varint, u64, poll_write_u64_varint_4_le, u32, poll_write_u32_raw_le, InternalWriteU64Varint4Le);
        write_varint_poll!(long_varint, u64, poll_write_u64_varint_4_be, u32, poll_write_u32_raw_be, InternalWriteU64Varint4Be);
        write_varint_poll!(long_varint, u64, poll_write_u64_varint_8_le, u64, poll_write_u64_raw_le, InternalWriteU64Varint8Le);
        write_varint_poll!(long_varint, u64, poll_write_u64_varint_8_be, u64, poll_write_u64_raw_be, InternalWriteU64Varint8Be);

        write_varint_poll!(varint, u128, poll_write_u128_varint, u8, poll_write_u8_raw, InternalWriteU128Varint);
        write_varint_poll!(long_varint, u128, poll_write_u128_varint_2_le, u16, poll_write_u16_raw_le, InternalWriteU128Varint2Le);
        write_varint_poll!(long_varint, u128, poll_write_u128_varint_2_be, u16, poll_write_u16_raw_be, InternalWriteU128Varint2Be);
        write_varint_poll!(long_varint, u128, poll_write_u128_varint_4_le, u32, poll_write_u32_raw_le, InternalWriteU128Varint4Le);
        write_varint_poll!(long_varint, u128, poll_write_u128_varint_4_be, u32, poll_write_u32_raw_be, InternalWriteU128Varint4Be);
        write_varint_poll!(long_varint, u128, poll_write_u128_varint_8_le, u64, poll_write_u64_raw_le, InternalWriteU128Varint8Le);
        write_varint_poll!(long_varint, u128, poll_write_u128_varint_8_be, u64, poll_write_u64_raw_be, InternalWriteU128Varint8Be);
        write_varint_poll!(long_varint, u128, poll_write_u128_varint_16_le, u128, poll_write_u128_raw_le, InternalWriteU128Varint16Le);
        write_varint_poll!(long_varint, u128, poll_write_u128_varint_16_be, u128, poll_write_u128_raw_be, InternalWriteU128Varint16Be);

        write_varint_size_poll!(varint, poll_write_usize_varint, u8, poll_write_u8_raw, InternalWriteUsizeVarint);
        write_varint_size_poll!(long_varint, poll_write_usize_varint_2_le, u16, poll_write_u16_raw_le, InternalWriteUsizeVarint2Le);
        write_varint_size_poll!(long_varint, poll_write_usize_varint_2_be, u16, poll_write_u16_raw_be, InternalWriteUsizeVarint2Be);
        write_varint_size_poll!(long_varint, poll_write_usize_varint_4_le, u32, poll_write_u32_raw_le, InternalWriteUsizeVarint4Le);
        write_varint_size_poll!(long_varint, poll_write_usize_varint_4_be, u32, poll_write_u32_raw_be, InternalWriteUsizeVarint4Be);
        write_varint_size_poll!(long_varint, poll_write_usize_varint_8_le, u64, poll_write_u64_raw_le, InternalWriteUsizeVarint8Le);
        write_varint_size_poll!(long_varint, poll_write_usize_varint_8_be, u64, poll_write_u64_raw_be, InternalWriteUsizeVarint8Be);
        write_varint_size_poll!(long_varint, poll_write_usize_varint_16_le, u128, poll_write_u128_raw_le, InternalWriteUsizeVarint16Le);
        write_varint_size_poll!(long_varint, poll_write_usize_varint_16_be, u128, poll_write_u128_raw_be, InternalWriteUsizeVarint16Be);
    };
}
macro_rules! define_write_varint_func {
    () => {
        write_varint_func!(long_varint, u8, write_u8_varint, WriteU8Varint, InternalWriteU8Varint);

        write_varint_func!(varint, u16, write_u16_varint, WriteU16Varint, InternalWriteU16Varint);
        write_varint_func!(long_varint, u16, write_u16_varint_2_le, WriteU16Varint2Le, InternalWriteU16Varint2Le);
        write_varint_func!(long_varint, u16, write_u16_varint_2_be, WriteU16Varint2Be, InternalWriteU16Varint2Be);

        write_varint_func!(varint, u32, write_u32_varint, WriteU32Varint, InternalWriteU32Varint);
        write_varint_func!(long_varint, u32, write_u32_varint_2_le, WriteU32Varint2Le, InternalWriteU32Varint2Le);
        write_varint_func!(long_varint, u32, write_u32_varint_2_be, WriteU32Varint2Be, InternalWriteU32Varint2Be);
        write_varint_func!(long_varint, u32, write_u32_varint_4_le, WriteU32Varint4Le, InternalWriteU32Varint4Le);
        write_varint_func!(long_varint, u32, write_u32_varint_4_be, WriteU32Varint4Be, InternalWriteU32Varint4Be);

        write_varint_func!(varint, u64, write_u64_varint, WriteU64Varint, InternalWriteU64Varint);
        write_varint_func!(long_varint, u64, write_u64_varint_2_le, WriteU64Varint2Le, InternalWriteU64Varint2Le);
        write_varint_func!(long_varint, u64, write_u64_varint_2_be, WriteU64Varint2Be, InternalWriteU64Varint2Be);
        write_varint_func!(long_varint, u64, write_u64_varint_4_le, WriteU64Varint4Le, InternalWriteU64Varint4Le);
        write_varint_func!(long_varint, u64, write_u64_varint_4_be, WriteU64Varint4Be, InternalWriteU64Varint4Be);
        write_varint_func!(long_varint, u64, write_u64_varint_8_le, WriteU64Varint8Le, InternalWriteU64Varint8Le);
        write_varint_func!(long_varint, u64, write_u64_varint_8_be, WriteU64Varint8Be, InternalWriteU64Varint8Be);

        write_varint_func!(varint, u128, write_u128_varint, WriteU128Varint, InternalWriteU128Varint);
        write_varint_func!(long_varint, u128, write_u128_varint_2_le, WriteU128Varint2Le, InternalWriteU128Varint2Le);
        write_varint_func!(long_varint, u128, write_u128_varint_2_be, WriteU128Varint2Be, InternalWriteU128Varint2Be);
        write_varint_func!(long_varint, u128, write_u128_varint_4_le, WriteU128Varint4Le, InternalWriteU128Varint4Le);
        write_varint_func!(long_varint, u128, write_u128_varint_4_be, WriteU128Varint4Be, InternalWriteU128Varint4Be);
        write_varint_func!(long_varint, u128, write_u128_varint_8_le, WriteU128Varint8Le, InternalWriteU128Varint8Le);
        write_varint_func!(long_varint, u128, write_u128_varint_8_be, WriteU128Varint8Be, InternalWriteU128Varint8Be);
        write_varint_func!(long_varint, u128, write_u128_varint_16_le, WriteU128Varint16Le, InternalWriteU128Varint16Le);
        write_varint_func!(long_varint, u128, write_u128_varint_16_be, WriteU128Varint16Be, InternalWriteU128Varint16Be);

        write_varint_size_func!(varint, write_usize_varint, WriteUsizeVarint, InternalWriteUsizeVarint);
        write_varint_size_func!(long_varint, write_usize_varint_2_le, WriteUsizeVarint2Le, InternalWriteUsizeVarint2Le);
        write_varint_size_func!(long_varint, write_usize_varint_2_be, WriteUsizeVarint2Be, InternalWriteUsizeVarint2Be);
        write_varint_size_func!(long_varint, write_usize_varint_4_le, WriteUsizeVarint4Le, InternalWriteUsizeVarint4Le);
        write_varint_size_func!(long_varint, write_usize_varint_4_be, WriteUsizeVarint4Be, InternalWriteUsizeVarint4Be);
        write_varint_size_func!(long_varint, write_usize_varint_8_le, WriteUsizeVarint8Le, InternalWriteUsizeVarint8Le);
        write_varint_size_func!(long_varint, write_usize_varint_8_be, WriteUsizeVarint8Be, InternalWriteUsizeVarint8Be);
        write_varint_size_func!(long_varint, write_usize_varint_16_le, WriteUsizeVarint16Le, InternalWriteUsizeVarint16Le);
        write_varint_size_func!(long_varint, write_usize_varint_16_be, WriteUsizeVarint16Be, InternalWriteUsizeVarint16Be);
    };
}
define_write_varint_futures!();
