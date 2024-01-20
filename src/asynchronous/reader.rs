use std::future::Future;
use std::io::{Error, ErrorKind, Result};
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
        R::poll_read_more(Pin::new(&mut *me.reader), cx, &mut me.buf)
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

impl<R: AsyncRead + Unpin> AsyncVariableReadable for R {
    fn poll_read_single(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<u8>> {
        let mut buf = [0];
        ready!(R::poll_read(self, cx, &mut ReadBuf::new(&mut buf)))?;
        Poll::Ready(Ok(buf[0]))
    }

    fn poll_read_more(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<()>> {
        let origin = buf.remaining();
        ready!(R::poll_read(self, cx, buf))?;
        if buf.remaining() == 0 {
            Poll::Ready(Ok(()))
        } else if buf.remaining() == origin {
            Poll::Ready(Err(Error::new(ErrorKind::UnexpectedEof, "read 0 byte")))
        } else {
            cx.waker().clone().wake();
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use anyhow::Result;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::spawn;
    use tokio::task::JoinHandle;
    use tokio::time::sleep;
    use crate::asynchronous::AsyncVariableReader;

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
        let server = TcpListener::bind("localhost:0").await?;
        let mut client = TcpStream::connect(server.local_addr()?).await?;
        let mut server = server.accept().await?.0;

        let mut buf = [0, 0];
        let _: JoinHandle<Result<()>> = spawn(async move {
            use tokio::io::AsyncWriteExt;
            server.write_all(&[1]).await?;
            sleep(Duration::from_millis(300)).await;
            server.write_all(&[2]).await?;
            Ok(())
        });
        client.read_more(buf.as_mut()).await?;
        assert_eq!(buf, [1, 2]);
        Ok(())
    }
}
