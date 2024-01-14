use std::future::Future;
use std::io::{Error, ErrorKind, Result};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, ReadBuf};
use crate::asynchronous::AsyncVariableReadable;
use crate::asynchronous::raw::{define_read_raw_func, define_read_raw_futures};

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

#[cfg(feature = "async_raw")]
define_read_raw_futures!();

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

    // #[cfg(feature = "async_bools")]
    // bools::define_bools_read!();

    #[cfg(feature = "async_raw")]
    define_read_raw_func!();

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

impl<R: AsyncVariableReadable + ?Sized> AsyncVariableReader for R {
}

#[repr(transparent)]
pub struct AsyncVariableRead<R>(R) where R: AsyncRead;
impl<R: AsyncRead> Deref for AsyncVariableRead<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<R: AsyncRead> DerefMut for AsyncVariableRead<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<R: AsyncRead> From<R> for AsyncVariableRead<R> {
    fn from(value: R) -> Self {
        Self(value)
    }
}

impl<R: AsyncRead + Unpin> AsyncVariableReadable for AsyncVariableRead<R> {
    fn poll_read_single(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<u8>> {
        let mut buf = [0];
        ready!(R::poll_read(Pin::new(&mut self.0), cx, &mut ReadBuf::new(&mut buf)))?;
        Poll::Ready(Ok(buf[0]))
    }

    fn poll_read_more(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<()>> {
        R::poll_read(Pin::new(&mut self.0), cx, buf)
    }
}
