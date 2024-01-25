use std::future::Future;
use std::io::{Error, ErrorKind, Result};
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use pin_project_lite::pin_project;
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
impl<'a, R: AsyncVariableReadable + Unpin+ ?Sized> Future for ReadSingle<'a, R> {
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
impl<'a, R: AsyncVariableReadable + Unpin+ ?Sized> Future for ReadMore<'a, R> {
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
impl<'a, R: AsyncVariableReadable + Unpin+ ?Sized> Future for ReadBool<'a, R> {
    type Output = Result<bool>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        R::poll_read_bool(Pin::new(&mut *me.reader), cx)
    }
}

include!("reader_bools.rs");
include!("reader_raw.rs");
include!("reader_varint.rs");
include!("reader_signed.rs");

trait InternalAsyncVariableReader: AsyncVariableReader {
    fn poll_read_bool(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<bool>> {
        Poll::Ready(match ready!(self.poll_read_single(cx))? {
            0 => Ok(false),
            1 => Ok(true),
            i => Err(Error::new(ErrorKind::InvalidData, format!("Invalid boolean value: {}", i))),
        })
    }

    #[cfg(feature = "async_raw")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_raw")))]
    define_read_raw_poll!();

    #[cfg(feature = "async_varint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_varint")))]
    define_read_varint_poll!();

    #[cfg(feature = "async_signed")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_signed")))]
    define_read_signed_poll!();
}

impl<R: AsyncVariableReader + ?Sized> InternalAsyncVariableReader for R {
}


pub trait AsyncVariableReader: AsyncVariableReadable {
    #[inline]
    fn read_single(&mut self) -> ReadSingle<Self> where Self: Unpin {
        ReadSingle { reader: self }
    }

    #[inline]
    fn read_more<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadMore<Self> where Self: Unpin {
        ReadMore { reader: self, buf: ReadBuf::new(buf) }
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    fn read_more_buf<'a, B: bytes::BufMut>(&'a mut self, buf: &'a mut B) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> where Self: Unpin + Send {
        Box::pin(async move {
            while buf.has_remaining_mut() {
                let slice = buf.chunk_mut();
                let len = slice.len();
                let mut t = vec![0; len];
                self.read_more(&mut t).await?;
                slice.copy_from_slice(&t);
                // SAFETY: we just filled `slice` with `len` bytes from `t`.
                unsafe { bytes::BufMut::advance_mut(buf, len); }
            }
            Ok(())
        })
    }

    #[inline]
    fn read_bool(&mut self) -> ReadBool<Self> where Self: Unpin {
        ReadBool { reader: self }
    }

    #[cfg(feature = "async_bools")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
    define_read_bools_func!();

    #[cfg(feature = "async_raw")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_raw")))]
    define_read_raw_func!();

    #[cfg(feature = "async_varint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_varint")))]
    define_read_varint_func!();

    #[cfg(feature = "async_signed")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_signed")))]
    define_read_signed_func!();

    #[cfg(feature = "async_vec_u8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_vec_u8")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    fn read_u8_vec(&mut self) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + '_>> where Self: Unpin + Send {
        Box::pin(async move {
            let length = self.read_usize_varint().await?;
            let mut bytes = vec![0; length];
            self.read_more(&mut bytes).await?;
            Ok(bytes)
        })
    }

    #[cfg(feature = "async_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    fn read_string(&mut self) -> Pin<Box<dyn Future<Output = Result<String>> + Send + '_>> where Self: Unpin + Send {
        Box::pin(async move {
            match String::from_utf8(self.read_u8_vec().await?) {
                Ok(s) => Ok(s),
                Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string())),
            }
        })
    }
}

impl<R: AsyncVariableReadable + ?Sized> AsyncVariableReader for R {
}


impl<R: tokio::io::AsyncRead + Unpin> AsyncVariableReadable for R {
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
