use std::future::Future;
use std::io::{Error, ErrorKind, Result};
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use pin_project_lite::pin_project;
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
impl<'a, W: AsyncVariableWritable + Unpin+ ?Sized> Future for WriteSingle<'a, W> {
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
impl<'a, W: AsyncVariableWritable + Unpin+ ?Sized> Future for WriteMore<'a, W> {
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
impl<'a, W: AsyncVariableWritable + Unpin+ ?Sized> Future for WriteBool<'a, W> {
    type Output = Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        W::poll_write_bool(Pin::new(&mut *me.writer), cx, *me.b)
    }
}

include!("writer_bools.rs");
include!("writer_raw.rs");
include!("writer_varint.rs");
include!("writer_signed.rs");

trait InternalAsyncVariableWriter: AsyncVariableWriter {
    fn poll_write_bool(self: Pin<&mut Self>, cx: &mut Context<'_>, b: bool) -> Poll<Result<usize>> {
        self.poll_write_single(cx, if b { 1 } else { 0 })
    }

    define_write_bools_poll!();
    define_write_raw_poll!();
    define_write_varint_poll!();
    define_write_signed_poll!();
}

impl<R: AsyncVariableWriter + ?Sized> InternalAsyncVariableWriter for R {
}


#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub trait AsyncVariableWriter: AsyncVariableWritable {
    #[inline]
    fn write_single(&mut self, byte: u8) -> WriteSingle<Self> where Self: Unpin {
        WriteSingle { writer: self, byte }
    }

    #[inline]
    fn write_more<'a>(&'a mut self, buf: &'a [u8]) -> WriteMore<Self> where Self: Unpin {
        WriteMore { writer: self, buf: WriteBuf::new(buf) }
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    fn write_more_buf<'a, B: bytes::Buf + Send>(&'a mut self, message: &'a mut B) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin + Send {
        Box::pin(async move {
            let mut len = 0;
            while message.has_remaining() {
                let written = self.write_more(message.chunk()).await?;
                message.advance(written);
                len += written;
            }
            Ok(len)
        })
    }

    #[inline]
    fn write_bool(&mut self, b: bool) -> WriteBool<Self> where Self: Unpin {
        WriteBool { writer: self, b }
    }

    define_write_bools_func!();
    define_write_raw_func!();
    define_write_varint_func!();
    define_write_signed_func!();

    #[cfg(feature = "async_vec_u8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_vec_u8")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    fn write_u8_vec<'a>(&'a mut self, message: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin + Send {
        Box::pin(async move {
            let mut size = self.write_usize_varint(message.len()).await?;
            size += self.write_more(message).await?;
            Ok(size)
        })
    }

    #[cfg(feature = "async_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    fn write_string<'a>(&'a mut self, message: &'a str) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin + Send {
        self.write_u8_vec(message.as_bytes())
    }
}

impl<W: AsyncVariableWritable + ?Sized> AsyncVariableWriter for W {
}


impl<W: tokio::io::AsyncWrite + Unpin> AsyncVariableWritable for W {
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

    #[tokio::test]
    async fn write_buf() -> Result<()> {
        use bytes::Bytes;
        let mut buf = Vec::with_capacity(2);
        buf.write_more_buf(&mut Bytes::copy_from_slice(&[1, 2])).await?;
        assert_eq!(&buf, &[1, 2]);
        Ok(())
    }

    #[tokio::test]
    async fn write_buf_slice() -> Result<()> {
        use bytes::{Buf, Bytes};
        let mut buf = Vec::with_capacity(2);
        buf.write_more_buf(&mut Bytes::copy_from_slice(&[1]).chain(Bytes::copy_from_slice(&[2]))).await?;
        assert_eq!(&buf, &[1, 2]);
        Ok(())
    }
}
