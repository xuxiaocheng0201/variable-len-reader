use std::io::Result;
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use tokio::io::{ReadBuf};

#[cfg(feature = "async_bools")]
mod bools;
#[cfg(feature = "async_raw")]
mod raw;
#[cfg(feature = "async_varint")]
mod varint;
#[cfg(feature = "async_signed")]
mod signed;

mod reader;
pub use reader::*;

// mod writer;

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

#[cfg(test)]
mod tests {
    // use crate::asynchronous::AsyncVariableWritable;

    // #[tokio::test]
    // async fn write_single() {
    //     let mut buf = Vec::with_capacity(1);
    //     buf.write_single(1).await.unwrap();
    //     assert_eq!(&buf, &[1]);
    // }
    //
    // #[tokio::test]
    // async fn write_more() {
    //     let mut buf = Vec::with_capacity(2);
    //     buf.write_more(&[1, 2]).await.unwrap();
    //     assert_eq!(&buf, &[1, 2]);
    // }
}
