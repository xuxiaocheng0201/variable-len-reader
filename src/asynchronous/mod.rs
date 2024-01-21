use std::io::Result;
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use crate::util::bufs::{ReadBuf, WriteBuf};

// #[cfg(feature = "async_bools")]
// mod bools;
// #[cfg(feature = "async_raw")]
// mod raw;
// #[cfg(feature = "async_varint")]
// mod varint;
// #[cfg(feature = "async_signed")]
// mod signed;

mod reader;
pub use reader::*;

mod writer;
pub use writer::*;

pub trait AsyncVariableReadable {
    fn poll_read_single(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<u8>>;

    fn poll_read_more(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<()>> {
        while buf.left() > 0 {
            buf.put(ready!(self.as_mut().poll_read_single(cx))?);
        }
        Poll::Ready(Ok(()))
    }
}

pub trait AsyncVariableWritable {
    fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, byte: u8) -> Poll<Result<usize>>;

    fn poll_write_more(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut WriteBuf<'_>) -> Poll<Result<usize>> {
        while buf.left() > 0 {
            ready!(self.as_mut().poll_write_single(cx, buf.get()))?;
            buf.skip(1);
        }
        Poll::Ready(Ok(buf.buf().len()))
    }
}

#[cfg(test)]
mod channel {
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tokio::sync::mpsc::{Receiver, Sender};
    use tokio::sync::mpsc::error::{TryRecvError, TrySendError};
    use crate::asynchronous::{AsyncVariableReadable, AsyncVariableWritable};

    pub(crate) struct SenderWriter<T>(pub Sender<T>);
    pub(crate) struct ReceiverReader<T>(pub Receiver<T>);

    impl AsyncVariableWritable for SenderWriter<u8> {
        fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, byte: u8) -> Poll<std::io::Result<usize>> {
            self.0.try_send(byte).map_or_else(|e| match e {
                TrySendError::Full(_) => {
                    cx.waker().wake_by_ref(); // TODO: transfer handle into self.0
                    Poll::Pending
                }
                TrySendError::Closed(_) => {
                    Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "channel disconnected")))
                }
            }, |()| Poll::Ready(Ok(1)))
        }
    }

    impl AsyncVariableReadable for ReceiverReader<u8> {
        fn poll_read_single(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<u8>> {
            self.0.try_recv().map_or_else(|e| match e {
                TryRecvError::Empty => {
                    cx.waker().wake_by_ref(); // TODO: transfer handle into self.0
                    Poll::Pending
                }
                TryRecvError::Disconnected => {
                    Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "channel disconnected")))
                }
            }, |v| Poll::Ready(Ok(v)))
        }
    }
}
