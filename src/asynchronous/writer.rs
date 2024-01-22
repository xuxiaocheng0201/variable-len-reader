use std::future::Future;
use std::io::{Error, ErrorKind, Result};
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use pin_project_lite::pin_project;
use tokio::io::AsyncWrite;
use crate::asynchronous::AsyncVariableWritable;
use crate::util::bufs::*;

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct WriteSingle<'a, W: ?Sized> {
        #[pin]
        writer: &'a mut W,
        byte: u8,
    }
}
impl<'a, W: AsyncVariableWritable + Unpin> Future for WriteSingle<'a, W> {
    type Output = Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        W::poll_write_single(Pin::new(&mut *me.writer), cx, *me.byte)
    }
}

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct WriteMore<'a, W: ?Sized> {
        #[pin]
        writer: &'a mut W,
        buf: WriteBuf<'a>,
    }
}
impl<'a, W: AsyncVariableWritable + Unpin> Future for WriteMore<'a, W> {
    type Output = Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        W::poll_write_more(Pin::new(&mut *me.writer), cx, me.buf)
    }
}

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct WriteBool<'a, W: ?Sized> {
        #[pin]
        writer: &'a mut W,
        b: bool,
    }
}
impl<'a, W: AsyncVariableWritable + Unpin> Future for WriteBool<'a, W> {
    type Output = Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        W::poll_write_single(Pin::new(&mut *me.writer), cx, if *me.b { 1 } else { 0 })
    }
}

#[cfg(feature = "async_raw")]
macro_rules! write_raw_future {
    ($future: ident, $buf: ident) => {
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
                W::poll_write_more(Pin::new(&mut *me.writer), cx, &mut me.buf.into())
            }
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! write_raw_func {
    ($primitive: ty, $func: ident, $future: ident, $to: ident, $buf: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            let buf = $buf::new(<$primitive>::$to(num));
            $future { writer: self, buf }
        }
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! write_raw_size_future {
    ($future: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                buf: OwnedWriteBuf128,
            }
        }
        impl<'a, W: $crate::asynchronous::AsyncVariableWritable + Unpin> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                W::poll_write_more(Pin::new(&mut *me.writer), cx, &mut me.buf.into())
            }
        }
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! write_raw_size_func {
    ($primitive: ty, $func: ident, $future: ident, $to: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            let buf = OwnedWriteBuf128::new(u128::$to(num as u128));
            $future { writer: self, buf }
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_write_raw_futures {
    () => {
        write_raw_future!(WriteU8RawNe, OwnedWriteBuf8);
        write_raw_future!(WriteI8RawNe, OwnedWriteBuf8);

        write_raw_future!(WriteU16RawLe, OwnedWriteBuf16);
        write_raw_future!(WriteU16RawBe, OwnedWriteBuf16);
        write_raw_future!(WriteI16RawLe, OwnedWriteBuf16);
        write_raw_future!(WriteI16RawBe, OwnedWriteBuf16);

        write_raw_future!(WriteU32RawLe, OwnedWriteBuf32);
        write_raw_future!(WriteU32RawBe, OwnedWriteBuf32);
        write_raw_future!(WriteI32RawLe, OwnedWriteBuf32);
        write_raw_future!(WriteI32RawBe, OwnedWriteBuf32);

        write_raw_future!(WriteU64RawLe, OwnedWriteBuf64);
        write_raw_future!(WriteU64RawBe, OwnedWriteBuf64);
        write_raw_future!(WriteI64RawLe, OwnedWriteBuf64);
        write_raw_future!(WriteI64RawBe, OwnedWriteBuf64);

        write_raw_future!(WriteU128RawLe, OwnedWriteBuf128);
        write_raw_future!(WriteU128RawBe, OwnedWriteBuf128);
        write_raw_future!(WriteI128RawLe, OwnedWriteBuf128);
        write_raw_future!(WriteI128RawBe, OwnedWriteBuf128);

        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteUsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteUsizeRawBe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteIsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_future!(WriteIsizeRawBe);
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_write_raw_func {
    () => {
        write_raw_func!(u8, write_u8_raw, WriteU8RawNe, to_ne_bytes, OwnedWriteBuf8);
        write_raw_func!(i8, write_i8_raw, WriteI8RawNe, to_ne_bytes, OwnedWriteBuf8);

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
        write_raw_size_func!(usize, write_usize_raw_le, WriteUsizeRawLe, to_le_bytes);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_func!(usize, write_usize_raw_be, WriteUsizeRawBe, to_be_bytes);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_func!(isize, write_isize_raw_le, WriteIsizeRawLe, to_le_bytes);
        #[cfg(feature = "async_raw_size")]
        write_raw_size_func!(isize, write_isize_raw_be, WriteIsizeRawBe, to_be_bytes);
    };
}
#[cfg(feature = "async_raw")]
define_write_raw_futures!();

#[cfg(feature = "async_bools")]
macro_rules! write_bools_future {
    ($future: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                b: u8,
            }
        }
        impl<'a, W: $crate::asynchronous::AsyncVariableWritable + Unpin> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                W::poll_write_single(Pin::new(&mut *me.writer), cx, *me.b)
            }
        }
    };
}
#[cfg(feature = "async_bools")]
macro_rules! write_bools_func {
    ($func: ident, $future: ident, $n: literal) => {
        #[inline]
        fn $func(&mut self, bools: [bool; $n]) -> $future<Self> where Self: Unpin {
            let mut b = 0;
            for i in 0..$n {
                if bools[i] {
                    b |= 1 << i;
                }
            }
            $future { writer: self, b }
        }
    };
}
#[cfg(feature = "async_bools")]
macro_rules! define_write_bools_futures {
    () => {
        write_bools_future!(WriteBools2);
        write_bools_future!(WriteBools3);
        write_bools_future!(WriteBools4);
        write_bools_future!(WriteBools5);
        write_bools_future!(WriteBools6);
        write_bools_future!(WriteBools7);
        write_bools_future!(WriteBools8);
    };
}
#[cfg(feature = "async_bools")]
macro_rules! define_write_bools_func {
    () => {
        write_bools_func!(write_bools_2, WriteBools2, 2);
        write_bools_func!(write_bools_3, WriteBools3, 3);
        write_bools_func!(write_bools_4, WriteBools4, 4);
        write_bools_func!(write_bools_5, WriteBools5, 5);
        write_bools_func!(write_bools_6, WriteBools6, 6);
        write_bools_func!(write_bools_7, WriteBools7, 7);
        write_bools_func!(write_bools_8, WriteBools8, 8);
    };
}
#[cfg(feature = "async_bools")]
define_write_bools_futures!();

#[cfg(feature = "async_varint")]
macro_rules! write_varint_future {
    ($primitive: ty, $future: ident, $internal: ty, $to: ident, $buf: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                value: $primitive,
                size: usize,
                inner_buf: $buf,
            }
        }
        impl<'a, W: $crate::asynchronous::AsyncVariableWritable + Unpin> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                const NUM_BITS: $internal = <$internal>::MAX >> 1;
                const SIGN_BIT: $internal = NUM_BITS + 1;
                const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
                let mut me = self.project();
                loop {
                    (*me.size) += ready!(W::poll_write_more(Pin::new(&mut *me.writer), cx, &mut me.inner_buf.into()))?;
                    if (*me.value) >= SIGN_BIT as $primitive {
                        (*me.inner_buf) = $buf::new(<$internal>::$to((((*me.value) & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT));
                        (*me.value) >>= POS_OFFSET;
                    } else {
                        if (*me.value == 0) {
                            return std::task::Poll::Ready(Ok(*me.size));
                        }
                        (*me.inner_buf) = $buf::new(<$internal>::$to(((*me.value) & (NUM_BITS as $primitive)) as $internal));
                        *me.value = 0;
                    }
                }
            }
        }
    };
}
#[cfg(feature = "async_varint")]
macro_rules! write_varint_func {
    ($primitive: ty, $func: ident, $future: ident, $internal: ty, $to: ident, $buf: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            const NUM_BITS: $internal = <$internal>::MAX >> 1;
            const SIGN_BIT: $internal = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
            let (value, buf) = if num >= SIGN_BIT as $primitive {
                (num >> POS_OFFSET, $buf::new(<$internal>::$to(((num & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT)))
            } else {
                (0, $buf::new(<$internal>::$to(num as $internal)))
            };
            $future { writer: self, value, size: 0, inner_buf: buf }
        }
    };
}
#[cfg(feature = "async_varint")]
macro_rules! define_write_varint_futures {
    () => {
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u8, WriteU8Varint, u8, to_ne_bytes, OwnedWriteBuf8);

        write_varint_future!(u16, WriteU16Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u16, WriteU16Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u16, WriteU16Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);

        write_varint_future!(u32, WriteU32Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u32, WriteU32Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u32, WriteU32Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u32, WriteU32Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u32, WriteU32Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);

        write_varint_future!(u64, WriteU64Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint8Le, u64, to_le_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u64, WriteU64Varint8Be, u64, to_be_bytes, OwnedWriteBuf64);

        write_varint_future!(u128, WriteU128Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint8Le, u64, to_le_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint8Be, u64, to_be_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint16Le, u128, to_le_bytes, OwnedWriteBuf128);
        #[cfg(feature = "async_long_varint")]
        write_varint_future!(u128, WriteU128Varint16Be, u128, to_be_bytes, OwnedWriteBuf128);
    };
}
#[cfg(feature = "async_varint")]
macro_rules! define_write_varint_func {
    () => {
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u8, write_u8_varint, WriteU8Varint, u8, to_ne_bytes, OwnedWriteBuf8);

        write_varint_func!(u16, write_u16_varint, WriteU16Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u16, write_u16_varint_2_le, WriteU16Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u16, write_u16_varint_2_be, WriteU16Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);

        write_varint_func!(u32, write_u32_varint, WriteU32Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u32, write_u32_varint_2_le, WriteU32Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u32, write_u32_varint_2_be, WriteU32Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u32, write_u32_varint_4_le, WriteU32Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u32, write_u32_varint_4_be, WriteU32Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);

        write_varint_func!(u64, write_u64_varint, WriteU64Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_2_le, WriteU64Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_2_be, WriteU64Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_4_le, WriteU64Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_4_be, WriteU64Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_8_le, WriteU64Varint8Le, u64, to_le_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u64, write_u64_varint_8_be, WriteU64Varint8Be, u64, to_be_bytes, OwnedWriteBuf64);

        write_varint_func!(u128, write_u128_varint, WriteU128Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_2_le, WriteU128Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_2_be, WriteU128Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_4_le, WriteU128Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_4_be, WriteU128Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_8_le, WriteU128Varint8Le, u64, to_le_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_8_be, WriteU128Varint8Be, u64, to_be_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_16_le, WriteU128Varint16Le, u128, to_le_bytes, OwnedWriteBuf128);
        #[cfg(feature = "async_long_varint")]
        write_varint_func!(u128, write_u128_varint_16_be, WriteU128Varint16Be, u128, to_be_bytes, OwnedWriteBuf128);
    };
}
#[cfg(feature = "async_varint")]
define_write_varint_futures!();

#[cfg(feature = "async_signed")]
macro_rules! write_signed_future {
    ($primitive: ty, $future: ident, $internal: ty, $to: ident, $buf: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                value: $primitive,
                size: usize,
                inner_buf: $buf,
            }
        }
        impl<'a, W: $crate::asynchronous::AsyncVariableWritable + Unpin> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                const NUM_BITS: $internal = <$internal>::MAX >> 1;
                const SIGN_BIT: $internal = NUM_BITS + 1;
                const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
                let mut me = self.project();
                loop {
                    (*me.size) += ready!(W::poll_write_more(Pin::new(&mut *me.writer), cx, &mut me.inner_buf.into()))?;
                    if (*me.value) >= SIGN_BIT as $primitive {
                        (*me.inner_buf) = $buf::new(<$internal>::$to((((*me.value) & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT));
                        (*me.value) >>= POS_OFFSET;
                    } else {
                        if (*me.value == 0) {
                            return std::task::Poll::Ready(Ok(*me.size));
                        }
                        (*me.inner_buf) = $buf::new(<$internal>::$to(((*me.value) & (NUM_BITS as $primitive)) as $internal));
                        *me.value = 0;
                    }
                }
            }
        }
    };
} // Completely same to `write_varint_future`.
#[cfg(feature = "async_signed")]
macro_rules! write_signed_func {
    ($source: ty, $primitive: ty, $func: ident, $future: ident, $internal: ty, $to: ident, $buf: ident) => {
        #[inline]
        fn $func(&mut self, num: $source) -> $future<Self> where Self: Unpin {
            use $crate::util::zigzag::Zigzag;
            let num = num.zigzag();
            const NUM_BITS: $internal = <$internal>::MAX >> 1;
            const SIGN_BIT: $internal = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
            let (value, buf) = if num >= SIGN_BIT as $primitive {
                (num >> POS_OFFSET, $buf::new(<$internal>::$to(((num & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT)))
            } else {
                (0, $buf::new(<$internal>::$to(num as $internal)))
            };
            $future { writer: self, value, size: 0, inner_buf: buf }
        }
    };
}
#[cfg(feature = "async_signed")]
macro_rules! define_write_signed_futures {
    () => {
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u8, WriteI8Varint, u8, to_ne_bytes, OwnedWriteBuf8);

        write_signed_future!(u16, WriteI16Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u16, WriteI16Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u16, WriteI16Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);

        write_signed_future!(u32, WriteI32Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u32, WriteI32Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u32, WriteI32Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u32, WriteI32Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u32, WriteI32Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);

        write_signed_future!(u64, WriteI64Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u64, WriteI64Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u64, WriteI64Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u64, WriteI64Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u64, WriteI64Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u64, WriteI64Varint8Le, u64, to_le_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u64, WriteI64Varint8Be, u64, to_be_bytes, OwnedWriteBuf64);

        write_signed_future!(u128, WriteI128Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u128, WriteI128Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u128, WriteI128Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u128, WriteI128Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u128, WriteI128Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u128, WriteI128Varint8Le, u64, to_le_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u128, WriteI128Varint8Be, u64, to_be_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u128, WriteI128Varint16Le, u128, to_le_bytes, OwnedWriteBuf128);
        #[cfg(feature = "async_long_signed")]
        write_signed_future!(u128, WriteI128Varint16Be, u128, to_be_bytes, OwnedWriteBuf128);
    };
}
#[cfg(feature = "async_signed")]
macro_rules! define_write_signed_func {
    () => {
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i8, u8, write_i8_varint, WriteI8Varint, u8, to_ne_bytes, OwnedWriteBuf8);

        write_signed_func!(i16, u16, write_i16_varint, WriteI16Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i16, u16, write_i16_varint_2_le, WriteI16Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i16, u16, write_i16_varint_2_be, WriteI16Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);

        write_signed_func!(i32, u32, write_i32_varint, WriteI32Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i32, u32, write_i32_varint_2_le, WriteI32Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i32, u32, write_i32_varint_2_be, WriteI32Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i32, u32, write_i32_varint_4_le, WriteI32Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i32, u32, write_i32_varint_4_be, WriteI32Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);

        write_signed_func!(i64, u64, write_i64_varint, WriteI64Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, u64, write_i64_varint_2_le, WriteI64Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, u64, write_i64_varint_2_be, WriteI64Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, u64, write_i64_varint_4_le, WriteI64Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, u64, write_i64_varint_4_be, WriteI64Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, u64, write_i64_varint_8_le, WriteI64Varint8Le, u64, to_le_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i64, u64, write_i64_varint_8_be, WriteI64Varint8Be, u64, to_be_bytes, OwnedWriteBuf64);

        write_signed_func!(i128, u128, write_i128_varint, WriteI128Varint, u8, to_ne_bytes, OwnedWriteBuf8);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, u128, write_i128_varint_2_le, WriteI128Varint2Le, u16, to_le_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, u128, write_i128_varint_2_be, WriteI128Varint2Be, u16, to_be_bytes, OwnedWriteBuf16);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, u128, write_i128_varint_4_le, WriteI128Varint4Le, u32, to_le_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, u128, write_i128_varint_4_be, WriteI128Varint4Be, u32, to_be_bytes, OwnedWriteBuf32);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, u128, write_i128_varint_8_le, WriteI128Varint8Le, u64, to_le_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, u128, write_i128_varint_8_be, WriteI128Varint8Be, u64, to_be_bytes, OwnedWriteBuf64);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, u128, write_i128_varint_16_le, WriteI128Varint16Le, u128, to_le_bytes, OwnedWriteBuf128);
        #[cfg(feature = "async_long_signed")]
        write_signed_func!(i128, u128, write_i128_varint_16_be, WriteI128Varint16Be, u128, to_be_bytes, OwnedWriteBuf128);
    };
}
#[cfg(feature = "async_signed")]
define_write_signed_futures!();

pub trait AsyncVariableWriter: AsyncVariableWritable {
    #[inline]
    fn write_single(&mut self, byte: u8) -> WriteSingle<Self> where Self: Unpin {
        WriteSingle { writer: self, byte }
    }

    #[inline]
    fn write_more<'a>(&'a mut self, buf: &'a [u8]) -> WriteMore<Self> where Self: Unpin {
        WriteMore { writer: self, buf: WriteBuf::new(buf) }
    }

    #[inline]
    fn write_bool(&mut self, b: bool) -> WriteBool<Self> where Self: Unpin {
        WriteBool { writer: self, b }
    }

    #[cfg(feature = "async_raw")]
    define_write_raw_func!();

    #[cfg(feature = "async_bools")]
    define_write_bools_func!();

    #[cfg(feature = "async_varint")]
    define_write_varint_func!();

    #[cfg(feature = "async_signed")]
    define_write_signed_func!();

//     #[cfg(feature = "async_vec_u8")]
//     #[inline]
//     async fn write_u8_vec(&mut self, message: &[u8]) -> Result<usize> {
//         let mut size = self.write_u128_varint(message.len() as u128).await?;
//         size += self.write_more(message).await?;
//         Ok(size)
//     }
//
//     #[cfg(feature = "async_string")]
//     #[inline]
//     async fn write_string(&mut self, message: &str) -> Result<usize> {
//         self.write_u8_vec(message.as_bytes()).await
//     }
}

impl<W: AsyncVariableWritable + ?Sized> AsyncVariableWriter for W {
}

impl<W: AsyncWrite + Unpin> AsyncVariableWritable for W {
    fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, byte: u8) -> Poll<Result<usize>> {
        W::poll_write(self, cx, &[byte])
    }

    fn poll_write_more(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut WriteBuf<'_>) -> Poll<Result<usize>> {
        while buf.left() > 0 {
            let read = buf.read();
            let n = ready!(W::poll_write(self.as_mut(), cx, &buf.buf()[read..]))?;
            buf.skip(n);
            if n == 0 {
                return Poll::Ready(Err(Error::new(ErrorKind::WriteZero, "failed to write whole buffer")));
            }
        }
        Poll::Ready(Ok(buf.buf().len()))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use anyhow::Result;
    use tokio::spawn;
    use tokio::sync::mpsc::channel;
    use tokio::task::JoinHandle;
    use tokio::time::sleep;
    use crate::asynchronous::AsyncVariableWriter;
    use crate::asynchronous::channel::SenderWriter;

    #[tokio::test]
    async fn write_single() -> Result<()> {
        let mut buf = Vec::with_capacity(1);
        buf.write_single(1).await?;
        assert_eq!(&buf, &[1]);
        Ok(())
    }

    #[tokio::test]
    async fn write_more() -> Result<()> {
        let mut buf = Vec::with_capacity(2);
        buf.write_more(&[1, 2]).await?;
        assert_eq!(&buf, &[1, 2]);
        Ok(())
    }

    #[tokio::test]
    async fn write_more_twice() -> Result<()> {
        let (sender, mut receiver) = channel(1);
        let mut sender = SenderWriter(sender);
        let j: JoinHandle<Result<()>> = spawn(async move {
            assert_eq!(receiver.recv().await, Some(1));
            sleep(Duration::from_millis(300)).await;
            assert_eq!(receiver.recv().await, Some(2));
            Ok(())
        });
        sender.write_more(&[1, 2]).await?;
        j.await??;
        Ok(())
    }
}
