use std::future::Future;
use std::io::{Error, ErrorKind, Result};
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use pin_project_lite::pin_project;
use tokio::io::AsyncRead;
use crate::asynchronous::AsyncVariableReadable;
use crate::util::bufs::*;

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadSingle<'a, R: ?Sized> {
        #[pin]
        reader: &'a mut R,
    }
}
impl<'a, R: AsyncVariableReadable + Unpin> Future for ReadSingle<'a, R> {
    type Output = Result<u8>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        R::poll_read_single(Pin::new(&mut *me.reader), cx)
    }
}

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadMore<'a, R: ?Sized> {
        #[pin]
        reader: &'a mut R,
        buf: ReadBuf<'a>,
    }
}
impl<'a, R: AsyncVariableReadable + Unpin> Future for ReadMore<'a, R> {
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        R::poll_read_more(Pin::new(&mut *me.reader), cx, me.buf)
    }
}

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadBool<'a, R: ?Sized> {
        #[pin]
        reader: &'a mut R,
    }
}
impl<'a, R: AsyncVariableReadable + Unpin> Future for ReadBool<'a, R> {
    type Output = Result<bool>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        Poll::Ready(match ready!(R::poll_read_single(Pin::new(&mut *me.reader), cx))? {
            0 => Ok(false),
            1 => Ok(true),
            i => Err(Error::new(ErrorKind::InvalidData, format!("Invalid boolean value: {}", i))),
        })
    }
}

#[cfg(feature = "async_raw")]
macro_rules! read_raw_future {
    ($primitive: ty, $future: ident, $from: ident, $buf: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
                buf: $buf,
            }
        }
        impl<'a, R: $crate::asynchronous::AsyncVariableReadable + Unpin> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$primitive>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                ready!(R::poll_read_more(Pin::new(&mut *me.reader), cx, &mut me.buf.into()))?;
                Poll::Ready(Ok(<$primitive>::$from((*me.buf).into_inner())))
            }
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! read_raw_func {
    ($primitive: ty, $func: ident, $future: ident, $buf: ident) => {
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, buf: $buf::new() }
        }
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! read_raw_size_future {
    ($primitive: ty, $future: ident, $from: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
                buf: OwnedReadBuf128,
            }
        }
        impl<'a, R: $crate::asynchronous::AsyncVariableReadable + Unpin> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$primitive>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                ready!(R::poll_read_more(Pin::new(&mut *me.reader), cx, &mut me.buf.into()))?;
                Poll::Ready(Ok(u128::$from((*me.buf).into_inner()) as $primitive))
            }
        }
    };
}
#[cfg(feature = "async_raw_size")]
macro_rules! read_raw_func_size {
    ($func: ident, $future: ident) => {
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, buf: OwnedReadBuf128::new() }
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_read_raw_futures {
    () => {
        read_raw_future!(u8, ReadU8Raw, from_ne_bytes, OwnedReadBuf8);
        read_raw_future!(i8, ReadI8Raw, from_ne_bytes, OwnedReadBuf8);

        read_raw_future!(u16, ReadU16RawLe, from_le_bytes, OwnedReadBuf16);
        read_raw_future!(u16, ReadU16RawBe, from_be_bytes, OwnedReadBuf16);
        read_raw_future!(i16, ReadI16RawLe, from_le_bytes, OwnedReadBuf16);
        read_raw_future!(i16, ReadI16RawBe, from_be_bytes, OwnedReadBuf16);

        read_raw_future!(u32, ReadU32RawLe, from_le_bytes, OwnedReadBuf32);
        read_raw_future!(u32, ReadU32RawBe, from_be_bytes, OwnedReadBuf32);
        read_raw_future!(i32, ReadI32RawLe, from_le_bytes, OwnedReadBuf32);
        read_raw_future!(i32, ReadI32RawBe, from_be_bytes, OwnedReadBuf32);

        read_raw_future!(u64, ReadU64RawLe, from_le_bytes, OwnedReadBuf64);
        read_raw_future!(u64, ReadU64RawBe, from_be_bytes, OwnedReadBuf64);
        read_raw_future!(i64, ReadI64RawLe, from_le_bytes, OwnedReadBuf64);
        read_raw_future!(i64, ReadI64RawBe, from_be_bytes, OwnedReadBuf64);

        read_raw_future!(u128, ReadU128RawLe, from_le_bytes, OwnedReadBuf128);
        read_raw_future!(u128, ReadU128RawBe, from_be_bytes, OwnedReadBuf128);
        read_raw_future!(i128, ReadI128RawLe, from_le_bytes, OwnedReadBuf128);
        read_raw_future!(i128, ReadI128RawBe, from_be_bytes, OwnedReadBuf128);

        #[cfg(feature = "async_raw_size")]
        read_raw_size_future!(usize, ReadUsizeRawLe, from_le_bytes);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_future!(usize, ReadUsizeRawBe, from_be_bytes);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_future!(isize, ReadIsizeRawLe, from_le_bytes);
        #[cfg(feature = "async_raw_size")]
        read_raw_size_future!(isize, ReadIsizeRawBe, from_be_bytes);
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_read_raw_func {
    () => {
        read_raw_func!(u8, read_u8_raw, ReadU8Raw, OwnedReadBuf8);
        read_raw_func!(i8, read_i8_raw, ReadI8Raw, OwnedReadBuf8);

        read_raw_func!(u16, read_u16_raw_le, ReadU16RawLe, OwnedReadBuf16);
        read_raw_func!(u16, read_u16_raw_be, ReadU16RawBe, OwnedReadBuf16);
        read_raw_func!(i16, read_i16_raw_le, ReadI16RawLe, OwnedReadBuf16);
        read_raw_func!(i16, read_i16_raw_be, ReadI16RawBe, OwnedReadBuf16);

        read_raw_func!(u32, read_u32_raw_le, ReadU32RawLe, OwnedReadBuf32);
        read_raw_func!(u32, read_u32_raw_be, ReadU32RawBe, OwnedReadBuf32);
        read_raw_func!(i32, read_i32_raw_le, ReadI32RawLe, OwnedReadBuf32);
        read_raw_func!(i32, read_i32_raw_be, ReadI32RawBe, OwnedReadBuf32);

        read_raw_func!(u64, read_u64_raw_le, ReadU64RawLe, OwnedReadBuf64);
        read_raw_func!(u64, read_u64_raw_be, ReadU64RawBe, OwnedReadBuf64);
        read_raw_func!(i64, read_i64_raw_le, ReadI64RawLe, OwnedReadBuf64);
        read_raw_func!(i64, read_i64_raw_be, ReadI64RawBe, OwnedReadBuf64);

        read_raw_func!(u128, read_u128_raw_le, ReadU128RawLe, OwnedReadBuf128);
        read_raw_func!(u128, read_u128_raw_be, ReadU128RawBe, OwnedReadBuf128);
        read_raw_func!(i128, read_i128_raw_le, ReadI128RawLe, OwnedReadBuf128);
        read_raw_func!(i128, read_i128_raw_be, ReadI128RawBe, OwnedReadBuf128);

        #[cfg(feature = "async_raw_size")]
        read_raw_func_size!(read_usize_raw_le, ReadUsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        read_raw_func_size!(read_usize_raw_be, ReadUsizeRawBe);
        #[cfg(feature = "async_raw_size")]
        read_raw_func_size!(read_isize_raw_le, ReadIsizeRawLe);
        #[cfg(feature = "async_raw_size")]
        read_raw_func_size!(read_isize_raw_be, ReadIsizeRawBe);
    };
}
#[cfg(feature = "async_raw")]
define_read_raw_futures!();

#[cfg(feature = "async_bools")]
macro_rules! read_bools_future {
    ($future: ident, $n: literal) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
            }
        }
        impl<'a, R: $crate::asynchronous::AsyncVariableReadable + Unpin> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<[bool; $n]>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                const MAX: u8 = ((1 << ($n - 1)) - 1 << 1) + 1; // (1 << $n) - 1
                let mut me = self.project();
                let b = ready!(R::poll_read_single(Pin::new(&mut *me.reader), cx))?;
                if b > MAX {
                    return  Poll::Ready(Err(Error::new(ErrorKind::InvalidData, format!("Invalid bools at {}.", stringify!($func)))));
                }
                let mut bools = [false; $n];
                for i in 0..$n {
                    bools[i] = b & (1 << i) != 0;
                }
                Poll::Ready(Ok(bools))
            }
        }
    };
}
#[cfg(feature = "async_bools")]
macro_rules! read_bools_func {
    ($func: ident, $future: ident) => {
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self }
        }
    };
}
#[cfg(feature = "async_bools")]
macro_rules! define_read_bools_futures {
    () => {
        read_bools_future!(ReadBools2, 2);
        read_bools_future!(ReadBools3, 3);
        read_bools_future!(ReadBools4, 4);
        read_bools_future!(ReadBools5, 5);
        read_bools_future!(ReadBools6, 6);
        read_bools_future!(ReadBools7, 7);
        read_bools_future!(ReadBools8, 8);
    };
}
#[cfg(feature = "async_bools")]
macro_rules! define_read_bools_func {
    () => {
        read_bools_func!(read_bools_2, ReadBools2);
        read_bools_func!(read_bools_3, ReadBools3);
        read_bools_func!(read_bools_4, ReadBools4);
        read_bools_func!(read_bools_5, ReadBools5);
        read_bools_func!(read_bools_6, ReadBools6);
        read_bools_func!(read_bools_7, ReadBools7);
        read_bools_func!(read_bools_8, ReadBools8);
    };
}
#[cfg(feature = "async_bools")]
define_read_bools_futures!();

#[cfg(feature = "async_varint")]
macro_rules! read_varint_future {
    ($primitive: ty, $future: ident, $func: ident, $internal: ty, $from: ident, $buf: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
                value: $primitive,
                position: usize,
                inner_buf: $buf,
            }
        }
        impl<'a, R: $crate::asynchronous::AsyncVariableReadable + Unpin> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$primitive>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                const SIZE: usize = std::mem::size_of::<$primitive>() << 3; // * 8
                const NUM_BITS: $internal = <$internal>::MAX >> 1;
                const SIGN_BIT: $internal = NUM_BITS + 1;
                const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
                let mut me = self.project();
                loop {
                    ready!(R::poll_read_more(Pin::new(&mut *me.reader), cx, &mut me.inner_buf.into()))?;
                    let current = <$internal>::$from((*me.inner_buf).into_inner());
                    me.inner_buf.clear();
                    (*me.value) |= ((current & NUM_BITS) as $primitive) << (*me.position);
                    if current & SIGN_BIT == 0 {
                        return Poll::Ready(Ok(*me.value));
                    }
                    (*me.position) += POS_OFFSET;
                    if (*me.position) >= SIZE {
                        return Poll::Ready(Err(Error::new(ErrorKind::InvalidData, format!("Varint {} in stream is too long.", stringify!($func)))));
                    }
                }
            }
        }
    };
}
#[cfg(feature = "async_varint")]
macro_rules! read_varint_func {
    ($func: ident, $future: ident, $buf: ident) => {
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, value: 0, position: 0, inner_buf: $buf::new() }
        }
    };
}
#[cfg(feature = "async_varint")]
macro_rules! define_read_varint_futures {
    () => {
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u8, ReadU8Varint, read_u8_varint, u8, from_ne_bytes, OwnedReadBuf8);

        read_varint_future!(u16, ReadU16Varint, read_u16_varint, u8, from_ne_bytes, OwnedReadBuf8);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u16, ReadU16Varint2Le, read_u16_varint_2_le, u16, from_le_bytes, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u16, ReadU16Varint2Be, read_u16_varint_2_be, u16, from_be_bytes, OwnedReadBuf16);

        read_varint_future!(u32, ReadU32Varint, read_u32_varint, u8, from_ne_bytes, OwnedReadBuf8);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u32, ReadU32Varint2Le, read_u32_varint_2_le, u16, from_le_bytes, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u32, ReadU32Varint2Be, read_u32_varint_2_be, u16, from_be_bytes, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u32, ReadU32Varint4Le, read_u32_varint_4_le, u32, from_le_bytes, OwnedReadBuf32);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u32, ReadU32Varint4Be, read_u32_varint_4_be, u32, from_be_bytes, OwnedReadBuf32);

        read_varint_future!(u64, ReadU64Varint, read_u64_varint, u8, from_ne_bytes, OwnedReadBuf8);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint2Le, read_u64_varint_2_le, u16, from_le_bytes, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint2Be, read_u64_varint_2_be, u16, from_be_bytes, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint4Le, read_u64_varint_4_le, u32, from_le_bytes, OwnedReadBuf32);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint4Be, read_u64_varint_4_be, u32, from_be_bytes, OwnedReadBuf32);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint8Le, read_u64_varint_8_le, u64, from_le_bytes, OwnedReadBuf64);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u64, ReadU64Varint8Be, read_u64_varint_8_be, u64, from_be_bytes, OwnedReadBuf64);

        read_varint_future!(u128, ReadU128Varint, read_u128_varint, u8, from_ne_bytes, OwnedReadBuf8);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint2Le, read_u128_varint_2_le, u16, from_le_bytes, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint2Be, read_u128_varint_2_be, u16, from_be_bytes, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint4Le, read_u128_varint_4_le, u32, from_le_bytes, OwnedReadBuf32);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint4Be, read_u128_varint_4_be, u32, from_be_bytes, OwnedReadBuf32);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint8Le, read_u128_varint_8_le, u64, from_le_bytes, OwnedReadBuf64);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint8Be, read_u128_varint_8_be, u64, from_be_bytes, OwnedReadBuf64);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint16Le, read_u128_varint_16_le, u128, from_le_bytes, OwnedReadBuf128);
        #[cfg(feature = "async_long_varint")]
        read_varint_future!(u128, ReadU128Varint16Be, read_u128_varint_16_be, u128, from_be_bytes, OwnedReadBuf128);
    };
}
#[cfg(feature = "async_varint")]
macro_rules! define_read_varint_func {
    () => {
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u8_varint, ReadU8Varint, OwnedReadBuf8);

        read_varint_func!(read_u16_varint, ReadU16Varint, OwnedReadBuf8);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u16_varint_2_le, ReadU16Varint2Le, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u16_varint_2_be, ReadU16Varint2Be, OwnedReadBuf16);

        read_varint_func!(read_u32_varint, ReadU32Varint, OwnedReadBuf8);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u32_varint_2_le, ReadU32Varint2Le, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u32_varint_2_be, ReadU32Varint2Be, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u32_varint_4_le, ReadU32Varint4Le, OwnedReadBuf32);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u32_varint_4_be, ReadU32Varint4Be, OwnedReadBuf32);

        read_varint_func!(read_u64_varint, ReadU64Varint, OwnedReadBuf8);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_2_le, ReadU64Varint2Le, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_2_be, ReadU64Varint2Be, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_4_le, ReadU64Varint4Le, OwnedReadBuf32);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_4_be, ReadU64Varint4Be, OwnedReadBuf32);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_8_le, ReadU64Varint8Le, OwnedReadBuf64);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u64_varint_8_be, ReadU64Varint8Be, OwnedReadBuf64);

        read_varint_func!(read_u128_varint, ReadU128Varint, OwnedReadBuf8);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_2_le, ReadU128Varint2Le, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_2_be, ReadU128Varint2Be, OwnedReadBuf16);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_4_le, ReadU128Varint4Le, OwnedReadBuf32);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_4_be, ReadU128Varint4Be, OwnedReadBuf32);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_8_le, ReadU128Varint8Le, OwnedReadBuf64);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_8_be, ReadU128Varint8Be, OwnedReadBuf64);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_16_le, ReadU128Varint16Le, OwnedReadBuf128);
        #[cfg(feature = "async_long_varint")]
        read_varint_func!(read_u128_varint_16_be, ReadU128Varint16Be, OwnedReadBuf128);
    };
}
#[cfg(feature = "async_varint")]
define_read_varint_futures!();

pub trait AsyncVariableReader: AsyncVariableReadable {
    #[inline]
    fn read_single(&mut self) -> ReadSingle<Self> where Self: Unpin {
        ReadSingle { reader: self }
    }

    #[inline]
    fn read_more<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadMore<Self> where Self: Unpin {
        ReadMore { reader: self, buf: ReadBuf::new(buf) }
    }

    #[inline]
    fn read_bool(&mut self) -> ReadBool<Self> where Self: Unpin {
        ReadBool { reader: self }
    }

    #[cfg(feature = "async_raw")]
    define_read_raw_func!();

    #[cfg(feature = "async_bools")]
    define_read_bools_func!();

    #[cfg(feature = "async_varint")]
    define_read_varint_func!();

    // #[cfg(feature = "async_signed")]
    // signed::define_signed_read!();
    //
    // #[cfg(feature = "async_vec_u8")]
    // #[inline]
    // async fn read_u8_vec(&mut self) -> Result<Vec<u8>> where Self: Unpin {
    //     let length = self.read_u128_varint().await? as usize;
    //     let mut bytes = vec![0; length];
    //     self.read_more(&mut bytes).await?;
    //     Ok(bytes)
    // }
    //
    // #[cfg(feature = "async_string")]
    // #[inline]
    // async fn read_string(&mut self) -> Result<String> where Self: Unpin {
    //     match String::from_utf8(self.read_u8_vec().await?) {
    //         Ok(s) => Ok(s),
    //         Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string())),
    //     }
    // }
}

impl<R: AsyncVariableReadable + ?Sized> AsyncVariableReader for R {
}

impl<R: AsyncRead + Unpin> AsyncVariableReadable for R {
    fn poll_read_single(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<u8>> {
        let mut buf = [0];
        ready!(R::poll_read(self, cx, &mut tokio::io::ReadBuf::new(&mut buf)))?;
        Poll::Ready(Ok(buf[0]))
    }

    fn poll_read_more(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<()>> {
        let origin = buf.left();
        if origin == 0 {
            return Poll::Ready(Ok(()));
        }
        let mut tokio_buf = buf.into();
        ready!(R::poll_read(self, cx, &mut tokio_buf))?;
        let remaining = tokio_buf.remaining();
        buf.advance(origin - remaining);
        let left = buf.left();
        if left == 0 {
            Poll::Ready(Ok(()))
        } else if left == origin {
            Poll::Ready(Err(Error::new(ErrorKind::UnexpectedEof, "read 0 byte")))
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
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
    use crate::asynchronous::AsyncVariableReader;
    use crate::asynchronous::channel::ReceiverReader;

    #[tokio::test]
    async fn read_single() -> Result<()> {
        let buf = [1u8, 2];
        let mut buf = buf.as_ref();
        let a = buf.read_single().await?;
        assert_eq!(a, 1);
        assert_eq!(buf, &[2]);
        Ok(())
    }

    #[tokio::test]
    async fn read_more() -> Result<()> {
        let buf = [1, 2];
        let mut buf = buf.as_ref();
        let mut a = [0, 0];
        buf.read_more(&mut a).await?;
        assert_eq!(a, [1, 2]);
        Ok(())
    }

    #[tokio::test]
    async fn read_more_twice() -> Result<()> {
        let (sender, receiver) = channel(1);
        let mut receiver = ReceiverReader(receiver);

        let j: JoinHandle<Result<()>> = spawn(async move {
            sender.send(1).await?;
            sleep(Duration::from_millis(300)).await;
            sender.send(2).await?;
            Ok(())
        });
        let mut buf = [0, 0];
        receiver.read_more(buf.as_mut()).await?;
        assert_eq!(buf, [1, 2]);
        j.await??;
        Ok(())
    }
}
