use core::pin::Pin;
use core::task::{Context, Poll, ready};
use tokio::io::{AsyncRead, AsyncWrite, Error, ErrorKind};
use crate::asynchronous::{AsyncVariableReadable, AsyncVariableWritable};
use crate::asynchronous::reader::AsyncVariableReader;
use crate::asynchronous::writer::AsyncVariableWriter;
use crate::util::read_buf::ReadBuf;
use crate::util::write_buf::WriteBuf;

impl<R: AsyncRead + Unpin> AsyncVariableReadable for R {
    type Error = Error;

    fn poll_read_single(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut Option<u8>) -> Poll<Result<u8, Self::Error>> {
        if let Some(b) = buf.as_ref() { return Poll::Ready(Ok(*b)); }
        let mut b = [0];
        ready!(R::poll_read(self, cx, &mut tokio::io::ReadBuf::new(&mut b)))?;
        buf.replace(b[0]);
        Poll::Ready(Ok(b[0]))
    }

    fn poll_read_more(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<(), Self::Error>> {
        let origin = buf.left();
        if origin == 0 { return Poll::Ready(Ok(())); }
        let mut tokio_buf = buf.into();
        ready!(R::poll_read(self, cx, &mut tokio_buf))?;
        let filled = tokio_buf.filled().len();
        buf.set_position(filled);
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

impl<R: AsyncRead + Unpin> AsyncVariableReader for R {
    #[inline]
    fn read_bool_error(future_name: &'static str, byte: u8) -> Self::Error {
        Error::new(ErrorKind::InvalidData, alloc::format!("Invalid bool. value {} at future {}.", byte, future_name))
    }

    #[cfg(feature = "async_bools")]
    #[inline]
    fn read_bools_error(future_name: &'static str, byte: u8) -> Self::Error {
        Error::new(ErrorKind::InvalidData, alloc::format!("Invalid bools. value {} at future {}.", byte, future_name))
    }

    #[cfg(feature = "async_varint")]
    #[inline]
    fn read_varint_error(future_name: &'static str, value: u128) -> Self::Error {
        Error::new(ErrorKind::InvalidData, alloc::format!("Too long varint value. {} at future {}.", value, future_name))
    }

    #[cfg(feature = "async_string")]
    #[inline]
    fn read_string_error(_future_name: &'static str, error: alloc::string::FromUtf8Error) -> Self::Error {
        Error::new(ErrorKind::InvalidData, error)
    }
}

impl<W: AsyncWrite + Unpin> AsyncVariableWritable for W {
    type Error = Error;

    fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut Option<u8>) -> Poll<Result<(), Self::Error>> {
        match buf.as_ref() {
            None => Poll::Ready(Ok(())),
            Some(b) => W::poll_write(self, cx, &[*b]).map_ok(|_i| { *buf = None; () })
        }
    }

    fn poll_write_more(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut WriteBuf<'_>) -> Poll<Result<(), Self::Error>> {
        while buf.left() > 0 {
            let position = buf.position();
            let n = core::task::ready!(W::poll_write(self.as_mut(), cx, &buf.buf()[position..]))?;
            if n == 0 {
                return Poll::Ready(Err(Error::new(ErrorKind::WriteZero, "write 0 bytes")));
            }
            buf.skip(n);
        }
        Poll::Ready(Ok(()))
    }
}

impl<W: AsyncWrite + Unpin> AsyncVariableWriter for W { }
