#![allow(unused_imports, unused)] // TODO

use std::io::{Error, ErrorKind, Result};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
#[cfg(feature = "async_signed")]
use crate::zigzag::Zigzag;

#[cfg(feature = "async_bools")]
mod bools;
#[cfg(feature = "async_raw")]
mod raw;
#[cfg(feature = "async_varint")]
mod varint;
#[cfg(feature = "async_signed")]
mod signed;
mod reader;

pub trait AsyncVariableReadable {
    fn poll_read_single(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<u8>>;

    fn poll_read_more(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<()>> {
        for _ in 0..buf.remaining() {
            let b = ready!(self.as_mut().poll_read_single(cx))?;
            buf.put_slice(&[b]);
        }
        Poll::Ready(Ok(()))
    }
}

pub use reader::*;


macro_rules! write_func {
    ($b: ty, $func: ident, $future: ident) => {
        #[inline]
        fn $func(&mut self, b: $b) -> $future<Self> where Self: Unpin {
            $future { writer: self, b }
        }
    };
}
use write_func;

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct WriteSingle<'a, W: Unpin> where W: ?Sized {
        #[pin]
        writer: &'a mut W,
        #[pin]
        byte: u8,
    }
}
impl<'a, W: AsyncVariableWritable + Unpin + ?Sized> Future for WriteSingle<'a, W> {
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
    pub struct WriteMore<'a, W: Unpin> where W: ?Sized {
        #[pin]
        writer: &'a mut W,
        #[pin]
        bytes: &'a [u8],
    }
}
impl<'a, W: AsyncVariableWritable + Unpin + ?Sized> Future for WriteMore<'a, W> {
    type Output = Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        W::poll_write_more(Pin::new(&mut *me.writer), cx, *me.bytes)
    }
}

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct WriteBool<'a, W: Unpin> where W: ?Sized {
        #[pin]
        writer: &'a mut W,
        #[pin]
        b: bool,
    }
}
impl<'a, W: AsyncVariableWritable + Unpin + ?Sized> Future for WriteBool<'a, W> {
    type Output = Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        W::poll_write_single(Pin::new(&mut *me.writer), cx, if *me.b { 1 } else { 0 })
    }
}

pub trait AsyncVariableWritable {
    fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, byte: u8) -> Poll<Result<usize>>;

    fn poll_write_more(mut self: Pin<&mut Self>, cx: &mut Context<'_>, bytes: &[u8]) -> Poll<Result<usize>> {
        for i in 0..bytes.len() {
            ready!(self.as_mut().poll_write_single(cx, bytes[i]))?;
        }
        Poll::Ready(Ok(bytes.len()))
    }

    #[inline]
    fn write_single(&mut self, byte: u8) -> WriteSingle<Self> where Self: Unpin {
        WriteSingle { writer: self, byte }
    }

    #[inline]
    fn write_more<'a>(&'a mut self, bytes: &'a [u8]) -> WriteMore<Self> where Self: Unpin {
        WriteMore { writer: self, bytes }
    }

    write_func!(bool, write_bool, WriteBool);

//     #[cfg(feature = "async_bools")]
//     bools::define_bools_write!();
//
//     #[cfg(feature = "async_raw")]
//     raw::define_raw_write!();
//
//     #[cfg(feature = "async_varint")]
//     varint::define_varint_write!();
//
//     #[cfg(feature = "async_signed")]
//     signed::define_signed_write!();
//
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

impl<W: AsyncWrite + Unpin> AsyncVariableWritable for W {
    fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, byte: u8) -> Poll<Result<usize>> {
        W::poll_write(self, cx, &[byte])
    }

    fn poll_write_more(self: Pin<&mut Self>, cx: &mut Context<'_>, bytes: &[u8]) -> Poll<Result<usize>> {
        W::poll_write(self, cx, bytes)
    }
}

impl AsyncVariableReadable for std::sync::mpsc::Receiver<u8> {
    fn poll_read_single(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<u8>> {
        todo!()
    }

    fn poll_read_more(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<()>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::sync::mpsc::channel;
    use crate::asynchronous::AsyncVariableRead;
    use crate::asynchronous::AsyncVariableReadable;
    use crate::asynchronous::AsyncVariableReader;
    use crate::asynchronous::AsyncVariableWritable;

    #[tokio::test]
    async fn read_single() {
        let mut buf = [1u8, 2];
        let mut buf: AsyncVariableRead<&[u8]> = buf.as_ref().into();
        let a = buf.read_single().await.unwrap();
        assert_eq!(a, 1);
        assert_eq!(buf.deref(), &[2]);
    }

    #[tokio::test]
    async fn read_more() {
        let buf = [1, 2];
        let mut buf = buf.as_ref();
        let mut a = [0, 0];
        buf.read_more(&mut a).await.unwrap();
        assert_eq!(a, [1, 2]);
    }

    #[tokio::test]
    async fn read_more_slice() {
        let (receiver, sender) = channel::<u8>();
    }

    #[tokio::test]
    async fn write_single() {
        let mut buf = Vec::with_capacity(1);
        buf.write_single(1).await.unwrap();
        assert_eq!(&buf, &[1]);
    }

    #[tokio::test]
    async fn write_more() {
        let mut buf = Vec::with_capacity(2);
        buf.write_more(&[1, 2]).await.unwrap();
        assert_eq!(&buf, &[1, 2]);
    }
}
