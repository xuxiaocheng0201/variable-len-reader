macro_rules! read_varint_future {
    (varint, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_varint_future!(cfg(feature = "async_varint"), $primitive, $primitive, $future, $poll_func, $struct_buf, $internal_struct);
    };
    (long_varint, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_varint_future!(cfg(feature = "async_long_varint"), $primitive, $primitive, $future, $poll_func, $struct_buf, $internal_struct);
    };
    ($feature: meta, $primitive: ty, $target: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[derive(Debug)]
        struct $struct_buf {
            value: $primitive,
            position: usize,
            internal: $internal_struct,
        }
        #[$feature]
        impl $struct_buf {
            fn new() -> Self {
                Self { value: 0, position: 0, internal: $internal_struct::new() }
            }
            fn reset(&mut self) {
                self.value = 0;
                self.position = 0;
                self.internal.reset();
            }
        }
        #[$feature]
        $crate::pin_project_lite::pin_project! {
            #[cfg_attr(docsrs, doc($feature))]
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
        impl<'a, R: $crate::AsyncVariableReadable + Unpin + ?Sized> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$target>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                R::$poll_func(Pin::new(&mut *me.reader), cx, me.inner)
            }
        }
    };
}
macro_rules! read_varint_poll {
    (varint, $primitive: ty, $func: ident, $internal: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_varint_poll!(cfg(feature = "async_varint"), $primitive, $primitive, $func, $internal, $poll_func, $poll_internal, $struct_buf);
    };
    (long_varint, $primitive: ty, $func: ident, $internal: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_varint_poll!(cfg(feature = "async_long_varint"), $primitive, $primitive, $func, $internal, $poll_func, $poll_internal, $struct_buf);
    };
    ($feature: meta, $primitive: ty, $target: ty, $func: ident, $internal: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $poll_func(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, inner: &mut $struct_buf) -> std::task::Poll<std::io::Result<$target>> {
            const SIZE: usize = std::mem::size_of::<$primitive>() << 3; // * 8
            const NUM_BITS: $internal = <$internal>::MAX >> 1;
            const SIGN_BIT: $internal = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
            loop {
                let current = ready!(self.as_mut().$poll_internal(cx, &mut inner.internal))?;
                inner.value |= ((current & NUM_BITS) as $primitive) << inner.position;
                if current & SIGN_BIT == 0 {
                    return Poll::Ready(Ok(inner.value as $target));
                }
                inner.position += POS_OFFSET;
                if inner.position >= SIZE {
                    return Poll::Ready(Err(Error::new(ErrorKind::InvalidData, format!("Varint {} in stream is too long.", stringify!($func)))));
                }
                inner.internal.reset();
            }
        }
    };
}
macro_rules! read_varint_func {
    (varint, $func: ident, $future: ident, $struct_buf: ident) => {
        read_varint_func!(cfg(feature = "async_varint"), $func, $future, $struct_buf);
    };
    (long_varint, $func: ident, $future: ident, $struct_buf: ident) => {
        read_varint_func!(cfg(feature = "async_long_varint"), $func, $future, $struct_buf);
    };
    ($feature: meta, $func: ident, $future: ident, $struct_buf: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, inner: $struct_buf::new() }
        }
    };
}
macro_rules! read_varint_size_future {
    (varint, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_varint_size_future!(cfg(feature = "async_varint_size"), $future, $poll_func, $struct_buf, $internal_struct);
    };
    (long_varint, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_varint_size_future!(cfg(all(feature = "async_varint_size", feature = "async_long_varint")), $future, $poll_func, $struct_buf, $internal_struct);
    };
    ($feature: meta, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        read_varint_future!($feature, u128, usize, $future, $poll_func, $struct_buf, $internal_struct);
    };
}
macro_rules! read_varint_size_poll {
    (varint, $func: ident, $internal: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_varint_size_poll!(cfg(feature = "async_varint_size"), $func, $internal, $poll_func, $poll_internal, $struct_buf);
    };
    (long_varint, $func: ident, $internal: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_varint_size_poll!(cfg(all(feature = "async_varint_size", feature = "async_long_varint")), $func, $internal, $poll_func, $poll_internal, $struct_buf);
    };
    ($feature: meta, $func: ident, $internal: ty, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        read_varint_poll!($feature, u128, usize, $func, $internal, $poll_func, $poll_internal, $struct_buf);
    };
}
macro_rules! read_varint_size_func {
    (varint, $func: ident, $future: ident, $struct_buf: ident) => {
        read_varint_size_func!(cfg(feature = "async_varint_size"), $func, $future, $struct_buf);
    };
    (long_varint, $func: ident, $future: ident, $struct_buf: ident) => {
        read_varint_size_func!(cfg(all(feature = "async_varint_size", feature = "async_long_varint")), $func, $future, $struct_buf);
    };
    ($feature: meta, $func: ident, $future: ident, $struct_buf: ident) => {
        read_varint_func!($feature, $func, $future, $struct_buf);
    };
}
macro_rules! define_read_varint_futures {
    () => {
        read_varint_future!(long_varint, u8, ReadU8Varint, poll_read_u8_varint, InternalReadU8Varint, InternalReadU8Raw);

        read_varint_future!(varint, u16, ReadU16Varint, poll_read_u16_varint, InternalReadU16Varint, InternalReadU8Raw);
        read_varint_future!(long_varint, u16, ReadU16Varint2Le, poll_read_u16_varint_2_le, InternalReadU16Varint2Le, InternalReadU16RawLe);
        read_varint_future!(long_varint, u16, ReadU16Varint2Be, poll_read_u16_varint_2_be, InternalReadU16Varint2Be, InternalReadU16RawBe);

        read_varint_future!(varint, u32, ReadU32Varint, poll_read_u32_varint, InternalReadU32Varint, InternalReadU8Raw);
        read_varint_future!(long_varint, u32, ReadU32Varint2Le, poll_read_u32_varint_2_le, InternalReadU32Varint2Le, InternalReadU16RawLe);
        read_varint_future!(long_varint, u32, ReadU32Varint2Be, poll_read_u32_varint_2_be, InternalReadU32Varint2Be, InternalReadU16RawBe);
        read_varint_future!(long_varint, u32, ReadU32Varint4Le, poll_read_u32_varint_4_le, InternalReadU32Varint4Le, InternalReadU32RawLe);
        read_varint_future!(long_varint, u32, ReadU32Varint4Be, poll_read_u32_varint_4_be, InternalReadU32Varint4Be, InternalReadU32RawBe);

        read_varint_future!(varint, u64, ReadU64Varint, poll_read_u64_varint, InternalReadU64Varint, InternalReadU8Raw);
        read_varint_future!(long_varint, u64, ReadU64Varint2Le, poll_read_u64_varint_2_le, InternalReadU64Varint2Le, InternalReadU16RawLe);
        read_varint_future!(long_varint, u64, ReadU64Varint2Be, poll_read_u64_varint_2_be, InternalReadU64Varint2Be, InternalReadU16RawBe);
        read_varint_future!(long_varint, u64, ReadU64Varint4Le, poll_read_u64_varint_4_le, InternalReadU64Varint4Le, InternalReadU32RawLe);
        read_varint_future!(long_varint, u64, ReadU64Varint4Be, poll_read_u64_varint_4_be, InternalReadU64Varint4Be, InternalReadU32RawBe);
        read_varint_future!(long_varint, u64, ReadU64Varint8Le, poll_read_u64_varint_8_le, InternalReadU64Varint8Le, InternalReadU64RawLe);
        read_varint_future!(long_varint, u64, ReadU64Varint8Be, poll_read_u64_varint_8_be, InternalReadU64Varint8Be, InternalReadU64RawBe);

        read_varint_future!(varint, u128, ReadU128Varint, poll_read_u128_varint, InternalReadU128Varint, InternalReadU8Raw);
        read_varint_future!(long_varint, u128, ReadU128Varint2Le, poll_read_u128_varint_2_le, InternalReadU128Varint2Le, InternalReadU16RawLe);
        read_varint_future!(long_varint, u128, ReadU128Varint2Be, poll_read_u128_varint_2_be, InternalReadU128Varint2Be, InternalReadU16RawBe);
        read_varint_future!(long_varint, u128, ReadU128Varint4Le, poll_read_u128_varint_4_le, InternalReadU128Varint4Le, InternalReadU32RawLe);
        read_varint_future!(long_varint, u128, ReadU128Varint4Be, poll_read_u128_varint_4_be, InternalReadU128Varint4Be, InternalReadU32RawBe);
        read_varint_future!(long_varint, u128, ReadU128Varint8Le, poll_read_u128_varint_8_le, InternalReadU128Varint8Le, InternalReadU64RawLe);
        read_varint_future!(long_varint, u128, ReadU128Varint8Be, poll_read_u128_varint_8_be, InternalReadU128Varint8Be, InternalReadU64RawBe);
        read_varint_future!(long_varint, u128, ReadU128Varint16Le, poll_read_u128_varint_16_le, InternalReadU128Varint16Le, InternalReadU128RawLe);
        read_varint_future!(long_varint, u128, ReadU128Varint16Be, poll_read_u128_varint_16_be, InternalReadU128Varint16Be, InternalReadU128RawBe);

        read_varint_size_future!(varint, ReadUsizeVarint, poll_read_usize_varint, InternalReadUsizeVarint, InternalReadU8Raw);
        read_varint_size_future!(long_varint, ReadUsizeVarint2Le, poll_read_usize_varint_2_le, InternalReadUsizeVarint2Le, InternalReadU16RawLe);
        read_varint_size_future!(long_varint, ReadUsizeVarint2Be, poll_read_usize_varint_2_be, InternalReadUsizeVarint2Be, InternalReadU16RawBe);
        read_varint_size_future!(long_varint, ReadUsizeVarint4Le, poll_read_usize_varint_4_le, InternalReadUsizeVarint4Le, InternalReadU32RawLe);
        read_varint_size_future!(long_varint, ReadUsizeVarint4Be, poll_read_usize_varint_4_be, InternalReadUsizeVarint4Be, InternalReadU32RawBe);
        read_varint_size_future!(long_varint, ReadUsizeVarint8Le, poll_read_usize_varint_8_le, InternalReadUsizeVarint8Le, InternalReadU64RawLe);
        read_varint_size_future!(long_varint, ReadUsizeVarint8Be, poll_read_usize_varint_8_be, InternalReadUsizeVarint8Be, InternalReadU64RawBe);
        read_varint_size_future!(long_varint, ReadUsizeVarint16Le, poll_read_usize_varint_16_le, InternalReadUsizeVarint16Le, InternalReadU128RawLe);
        read_varint_size_future!(long_varint, ReadUsizeVarint16Be, poll_read_usize_varint_16_be, InternalReadUsizeVarint16Be, InternalReadU128RawBe);
    };
}
macro_rules! define_read_varint_poll {
    () => {
        read_varint_poll!(long_varint, u8, read_u8_varint, u8, poll_read_u8_varint, poll_read_u8_raw, InternalReadU8Varint);

        read_varint_poll!(varint, u16, read_u16_varint, u8, poll_read_u16_varint, poll_read_u8_raw, InternalReadU16Varint);
        read_varint_poll!(long_varint, u16, read_u16_varint_2_le, u16, poll_read_u16_varint_2_le, poll_read_u16_raw_le, InternalReadU16Varint2Le);
        read_varint_poll!(long_varint, u16, read_u16_varint_2_be, u16, poll_read_u16_varint_2_be, poll_read_u16_raw_be, InternalReadU16Varint2Be);

        read_varint_poll!(varint, u32, read_u32_varint, u8, poll_read_u32_varint, poll_read_u8_raw, InternalReadU32Varint);
        read_varint_poll!(long_varint, u32, read_u32_varint_2_le, u16, poll_read_u32_varint_2_le, poll_read_u16_raw_le, InternalReadU32Varint2Le);
        read_varint_poll!(long_varint, u32, read_u32_varint_2_be, u16, poll_read_u32_varint_2_be, poll_read_u16_raw_be, InternalReadU32Varint2Be);
        read_varint_poll!(long_varint, u32, read_u32_varint_4_le, u32, poll_read_u32_varint_4_le, poll_read_u32_raw_le, InternalReadU32Varint4Le);
        read_varint_poll!(long_varint, u32, read_u32_varint_4_be, u32, poll_read_u32_varint_4_be, poll_read_u32_raw_be, InternalReadU32Varint4Be);

        read_varint_poll!(varint, u64, read_u64_varint, u8, poll_read_u64_varint, poll_read_u8_raw, InternalReadU64Varint);
        read_varint_poll!(long_varint, u64, read_u64_varint_2_le, u16, poll_read_u64_varint_2_le, poll_read_u16_raw_le, InternalReadU64Varint2Le);
        read_varint_poll!(long_varint, u64, read_u64_varint_2_be, u16, poll_read_u64_varint_2_be, poll_read_u16_raw_be, InternalReadU64Varint2Be);
        read_varint_poll!(long_varint, u64, read_u64_varint_4_le, u32, poll_read_u64_varint_4_le, poll_read_u32_raw_le, InternalReadU64Varint4Le);
        read_varint_poll!(long_varint, u64, read_u64_varint_4_be, u32, poll_read_u64_varint_4_be, poll_read_u32_raw_be, InternalReadU64Varint4Be);
        read_varint_poll!(long_varint, u64, read_u64_varint_8_le, u64, poll_read_u64_varint_8_le, poll_read_u64_raw_le, InternalReadU64Varint8Le);
        read_varint_poll!(long_varint, u64, read_u64_varint_8_be, u64, poll_read_u64_varint_8_be, poll_read_u64_raw_be, InternalReadU64Varint8Be);

        read_varint_poll!(varint, u128, read_u128_varint, u8, poll_read_u128_varint, poll_read_u8_raw, InternalReadU128Varint);
        read_varint_poll!(long_varint, u128, read_u128_varint_2_le, u16, poll_read_u128_varint_2_le, poll_read_u16_raw_le, InternalReadU128Varint2Le);
        read_varint_poll!(long_varint, u128, read_u128_varint_2_be, u16, poll_read_u128_varint_2_be, poll_read_u16_raw_be, InternalReadU128Varint2Be);
        read_varint_poll!(long_varint, u128, read_u128_varint_4_le, u32, poll_read_u128_varint_4_le, poll_read_u32_raw_le, InternalReadU128Varint4Le);
        read_varint_poll!(long_varint, u128, read_u128_varint_4_be, u32, poll_read_u128_varint_4_be, poll_read_u32_raw_be, InternalReadU128Varint4Be);
        read_varint_poll!(long_varint, u128, read_u128_varint_8_le, u64, poll_read_u128_varint_8_le, poll_read_u64_raw_le, InternalReadU128Varint8Le);
        read_varint_poll!(long_varint, u128, read_u128_varint_8_be, u64, poll_read_u128_varint_8_be, poll_read_u64_raw_be, InternalReadU128Varint8Be);
        read_varint_poll!(long_varint, u128, read_u128_varint_16_le, u128, poll_read_u128_varint_16_le, poll_read_u128_raw_le, InternalReadU128Varint16Le);
        read_varint_poll!(long_varint, u128, read_u128_varint_16_be, u128, poll_read_u128_varint_16_be, poll_read_u128_raw_be, InternalReadU128Varint16Be);

        read_varint_size_poll!(varint, read_usize_varint, u8, poll_read_usize_varint, poll_read_u8_raw, InternalReadUsizeVarint);
        read_varint_size_poll!(long_varint, read_usize_varint_2_le, u16, poll_read_usize_varint_2_le, poll_read_u16_raw_le, InternalReadUsizeVarint2Le);
        read_varint_size_poll!(long_varint, read_usize_varint_2_be, u16, poll_read_usize_varint_2_be, poll_read_u16_raw_be, InternalReadUsizeVarint2Be);
        read_varint_size_poll!(long_varint, read_usize_varint_4_le, u32, poll_read_usize_varint_4_le, poll_read_u32_raw_le, InternalReadUsizeVarint4Le);
        read_varint_size_poll!(long_varint, read_usize_varint_4_be, u32, poll_read_usize_varint_4_be, poll_read_u32_raw_be, InternalReadUsizeVarint4Be);
        read_varint_size_poll!(long_varint, read_usize_varint_8_le, u64, poll_read_usize_varint_8_le, poll_read_u64_raw_le, InternalReadUsizeVarint8Le);
        read_varint_size_poll!(long_varint, read_usize_varint_8_be, u64, poll_read_usize_varint_8_be, poll_read_u64_raw_be, InternalReadUsizeVarint8Be);
        read_varint_size_poll!(long_varint, read_usize_varint_16_le, u128, poll_read_usize_varint_16_le, poll_read_u128_raw_le, InternalReadUsizeVarint16Le);
        read_varint_size_poll!(long_varint, read_usize_varint_16_be, u128, poll_read_usize_varint_16_be, poll_read_u128_raw_be, InternalReadUsizeVarint16Be);
    };
}
macro_rules! define_read_varint_func {
    () => {
        read_varint_func!(long_varint, read_u8_varint, ReadU8Varint, InternalReadU8Varint);

        read_varint_func!(varint, read_u16_varint, ReadU16Varint, InternalReadU16Varint);
        read_varint_func!(long_varint, read_u16_varint_2_le, ReadU16Varint2Le, InternalReadU16Varint2Le);
        read_varint_func!(long_varint, read_u16_varint_2_be, ReadU16Varint2Be, InternalReadU16Varint2Be);

        read_varint_func!(varint, read_u32_varint, ReadU32Varint, InternalReadU32Varint);
        read_varint_func!(long_varint, read_u32_varint_2_le, ReadU32Varint2Le, InternalReadU32Varint2Le);
        read_varint_func!(long_varint, read_u32_varint_2_be, ReadU32Varint2Be, InternalReadU32Varint2Be);
        read_varint_func!(long_varint, read_u32_varint_4_le, ReadU32Varint4Le, InternalReadU32Varint4Le);
        read_varint_func!(long_varint, read_u32_varint_4_be, ReadU32Varint4Be, InternalReadU32Varint4Be);

        read_varint_func!(varint, read_u64_varint, ReadU64Varint, InternalReadU64Varint);
        read_varint_func!(long_varint, read_u64_varint_2_le, ReadU64Varint2Le, InternalReadU64Varint2Le);
        read_varint_func!(long_varint, read_u64_varint_2_be, ReadU64Varint2Be, InternalReadU64Varint2Be);
        read_varint_func!(long_varint, read_u64_varint_4_le, ReadU64Varint4Le, InternalReadU64Varint4Le);
        read_varint_func!(long_varint, read_u64_varint_4_be, ReadU64Varint4Be, InternalReadU64Varint4Be);
        read_varint_func!(long_varint, read_u64_varint_8_le, ReadU64Varint8Le, InternalReadU64Varint8Le);
        read_varint_func!(long_varint, read_u64_varint_8_be, ReadU64Varint8Be, InternalReadU64Varint8Be);

        read_varint_func!(varint, read_u128_varint, ReadU128Varint, InternalReadU128Varint);
        read_varint_func!(long_varint, read_u128_varint_2_le, ReadU128Varint2Le, InternalReadU128Varint2Le);
        read_varint_func!(long_varint, read_u128_varint_2_be, ReadU128Varint2Be, InternalReadU128Varint2Be);
        read_varint_func!(long_varint, read_u128_varint_4_le, ReadU128Varint4Le, InternalReadU128Varint4Le);
        read_varint_func!(long_varint, read_u128_varint_4_be, ReadU128Varint4Be, InternalReadU128Varint4Be);
        read_varint_func!(long_varint, read_u128_varint_8_le, ReadU128Varint8Le, InternalReadU128Varint8Le);
        read_varint_func!(long_varint, read_u128_varint_8_be, ReadU128Varint8Be, InternalReadU128Varint8Be);
        read_varint_func!(long_varint, read_u128_varint_16_le, ReadU128Varint16Le, InternalReadU128Varint16Le);
        read_varint_func!(long_varint, read_u128_varint_16_be, ReadU128Varint16Be, InternalReadU128Varint16Be);

        read_varint_size_func!(varint, read_usize_varint, ReadUsizeVarint, InternalReadUsizeVarint);
        read_varint_size_func!(long_varint, read_usize_varint_2_le, ReadUsizeVarint2Le, InternalReadUsizeVarint2Le);
        read_varint_size_func!(long_varint, read_usize_varint_2_be, ReadUsizeVarint2Be, InternalReadUsizeVarint2Be);
        read_varint_size_func!(long_varint, read_usize_varint_4_le, ReadUsizeVarint4Le, InternalReadUsizeVarint4Le);
        read_varint_size_func!(long_varint, read_usize_varint_4_be, ReadUsizeVarint4Be, InternalReadUsizeVarint4Be);
        read_varint_size_func!(long_varint, read_usize_varint_8_le, ReadUsizeVarint8Le, InternalReadUsizeVarint8Le);
        read_varint_size_func!(long_varint, read_usize_varint_8_be, ReadUsizeVarint8Be, InternalReadUsizeVarint8Be);
        read_varint_size_func!(long_varint, read_usize_varint_16_le, ReadUsizeVarint16Le, InternalReadUsizeVarint16Le);
        read_varint_size_func!(long_varint, read_usize_varint_16_be, ReadUsizeVarint16Be, InternalReadUsizeVarint16Be);
    };
}
define_read_varint_futures!();
