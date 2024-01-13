#![allow(unused_imports, unused)] // TODO

use std::io::{Error, ErrorKind, Result};
// #[cfg(any(feature = "async_bools", feature = "async_raw", feature = "async_varint", feature = "async_signed", feature = "async_vec_u8", feature = "async_string"))]
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

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadSingle<'a, R: Unpin> where R: ?Sized {
        #[pin]
        reader: &'a mut R,
    }
}
impl<'a, R: AsyncVariableReadable + Unpin + ?Sized> Future for ReadSingle<'a, R> {
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
    pub struct ReadMore<'a, R: Unpin> where R: ?Sized {
        #[pin]
        reader: &'a mut R,
        #[pin]
        buf: ReadBuf<'a>,
    }
}
impl<'a, R: AsyncVariableReadable + Unpin + ?Sized> Future for ReadMore<'a, R> {
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        R::poll_read_more(Pin::new(&mut *me.reader), cx, &mut *me.buf)
    }
}

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadBool<'a, R: Unpin> where R: ?Sized {
        #[pin]
        reader: &'a mut R,
    }
}
impl<'a, R: AsyncVariableReadable + Unpin + ?Sized> Future for ReadBool<'a, R> {
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

pub trait AsyncVariableReadable {
    fn poll_read_single(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<u8>>;

    fn poll_read_more(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<()>> {
        for _ in 0..buf.remaining() {
            let b = ready!(self.as_mut().poll_read_single(cx))?;
            buf.put_slice(&[b]);
        }
        Poll::Ready(Ok(()))
    }

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

    // #[cfg(feature = "async_bools")]
    // bools::define_bools_read!();
    //
    // #[cfg(feature = "async_raw")]
    // raw::define_raw_read!();
    //
    // #[cfg(feature = "async_varint")]
    // varint::define_varint_read!();
    //
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

    #[inline]
    fn write_bool(&mut self, b: bool) -> WriteBool<Self> where Self: Unpin {
        WriteBool { writer: self, b }
    }

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

impl<R: AsyncRead + Unpin> AsyncVariableReadable for R {
    fn poll_read_single(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<u8>> {
        let mut buf = [0];
        ready!(R::poll_read(self, cx, &mut ReadBuf::new(&mut buf)))?;
        Poll::Ready(Ok(buf[0]))
    }

    fn poll_read_more(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<()>> {
        R::poll_read(self, cx, buf)
    }
}

impl<W: AsyncWrite + Unpin> AsyncVariableWritable for W {
    fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, byte: u8) -> Poll<Result<usize>> {
        W::poll_write(self, cx, &[byte])
    }

    fn poll_write_more(self: Pin<&mut Self>, cx: &mut Context<'_>, bytes: &[u8]) -> Poll<Result<usize>> {
        W::poll_write(self, cx, bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::asynchronous::AsyncVariableReadable;
    use crate::asynchronous::AsyncVariableWritable;

    #[tokio::test]
    async fn read_single() {
        let buf = [1, 2];
        let mut buf = buf.as_ref();
        let a = buf.read_single().await.unwrap();
        assert_eq!(a, 1);
        assert_eq!(buf, &[2]);
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
