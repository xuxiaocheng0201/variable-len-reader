use std::future::Future;
use std::io::{Error, ErrorKind, Result};
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use pin_project_lite::pin_project;
use tokio::io::AsyncWrite;
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
impl<'a, W: AsyncVariableWritable + Unpin> Future for WriteSingle<'a, W> {
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
impl<'a, W: AsyncVariableWritable + Unpin> Future for WriteMore<'a, W> {
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
impl<'a, W: AsyncVariableWritable + Unpin> Future for WriteBool<'a, W> {
    type Output = Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        W::poll_write_single(Pin::new(&mut *me.writer), cx, if *me.b { 1 } else { 0 })
    }
}

#[cfg(feature = "async_raw")]
macro_rules! write_raw_future {
    ($future: ident, $buf: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: Unpin> where W: ?Sized {
                #[pin]
                writer: &'a mut W,
                buf: $buf,
            }
        }
        impl<'a, W: $crate::asynchronous::AsyncVariableWritable + Unpin + ?Sized> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                W::poll_write_more(Pin::new(&mut *me.writer), cx, &mut me.buf.into())
            }
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! write_raw_func {
    ($primitive: ty, $func: ident, $future: ident, $to: ident, $buf: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> $future<Self> where Self: Unpin {
            let buf = $buf::new(<$primitive>::$to(num));
            $future { writer: self, buf }
        }
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_write_raw_futures {
    () => {
        write_raw_future!(WriteU8RawNe, OwnedWriteBuf8);
        write_raw_future!(WriteI8RawNe, OwnedWriteBuf8);

        write_raw_future!(WriteU16RawLe, OwnedWriteBuf16);
        write_raw_future!(WriteU16RawBe, OwnedWriteBuf16);
        write_raw_future!(WriteI16RawLe, OwnedWriteBuf16);
        write_raw_future!(WriteI16RawBe, OwnedWriteBuf16);

        write_raw_future!(WriteU32RawLe, OwnedWriteBuf32);
        write_raw_future!(WriteU32RawBe, OwnedWriteBuf32);
        write_raw_future!(WriteI32RawLe, OwnedWriteBuf32);
        write_raw_future!(WriteI32RawBe, OwnedWriteBuf32);

        write_raw_future!(WriteU64RawLe, OwnedWriteBuf64);
        write_raw_future!(WriteU64RawBe, OwnedWriteBuf64);
        write_raw_future!(WriteI64RawLe, OwnedWriteBuf64);
        write_raw_future!(WriteI64RawBe, OwnedWriteBuf64);

        write_raw_future!(WriteU128RawLe, OwnedWriteBuf128);
        write_raw_future!(WriteU128RawBe, OwnedWriteBuf128);
        write_raw_future!(WriteI128RawLe, OwnedWriteBuf128);
        write_raw_future!(WriteI128RawBe, OwnedWriteBuf128);
    };
}
#[cfg(feature = "async_raw")]
macro_rules! define_read_raw_func {
    () => {
        write_raw_func!(u8, write_u8_raw, WriteU8RawNe, to_ne_bytes, OwnedWriteBuf8);
        write_raw_func!(i8, write_i8_raw, WriteI8RawNe, to_ne_bytes, OwnedWriteBuf8);

        write_raw_func!(u16, write_u16_raw_le, WriteU16RawLe, to_le_bytes, OwnedWriteBuf16);
        write_raw_func!(u16, write_u16_raw_be, WriteU16RawBe, to_be_bytes, OwnedWriteBuf16);
        write_raw_func!(i16, write_i16_raw_le, WriteI16RawLe, to_le_bytes, OwnedWriteBuf16);
        write_raw_func!(i16, write_i16_raw_be, WriteI16RawBe, to_be_bytes, OwnedWriteBuf16);

        write_raw_func!(u32, write_u32_raw_le, WriteU32RawLe, to_le_bytes, OwnedWriteBuf32);
        write_raw_func!(u32, write_u32_raw_be, WriteU32RawBe, to_be_bytes, OwnedWriteBuf32);
        write_raw_func!(i32, write_i32_raw_le, WriteI32RawLe, to_le_bytes, OwnedWriteBuf32);
        write_raw_func!(i32, write_i32_raw_be, WriteI32RawBe, to_be_bytes, OwnedWriteBuf32);

        write_raw_func!(u64, write_u64_raw_le, WriteU64RawLe, to_le_bytes, OwnedWriteBuf64);
        write_raw_func!(u64, write_u64_raw_be, WriteU64RawBe, to_be_bytes, OwnedWriteBuf64);
        write_raw_func!(i64, write_i64_raw_le, WriteI64RawLe, to_le_bytes, OwnedWriteBuf64);
        write_raw_func!(i64, write_i64_raw_be, WriteI64RawBe, to_be_bytes, OwnedWriteBuf64);

        write_raw_func!(u128, write_u128_raw_le, WriteU128RawLe, to_le_bytes, OwnedWriteBuf128);
        write_raw_func!(u128, write_u128_raw_be, WriteU128RawBe, to_be_bytes, OwnedWriteBuf128);
        write_raw_func!(i128, write_i128_raw_le, WriteI128RawLe, to_le_bytes, OwnedWriteBuf128);
        write_raw_func!(i128, write_i128_raw_be, WriteI128RawBe, to_be_bytes, OwnedWriteBuf128);
    };
}
#[cfg(feature = "async_raw")]
define_write_raw_futures!();

pub trait AsyncVariableWriter: AsyncVariableWritable {
    #[inline]
    fn write_single(&mut self, byte: u8) -> WriteSingle<Self> where Self: Unpin {
        WriteSingle { writer: self, byte }
    }

    #[inline]
    fn write_more<'a>(&'a mut self, buf: &'a [u8]) -> WriteMore<Self> where Self: Unpin {
        WriteMore { writer: self, buf: WriteBuf::new(buf) }
    }

    #[inline]
    fn write_bool(&mut self, b: bool) -> WriteBool<Self> where Self: Unpin {
        WriteBool { writer: self, b }
    }

    #[cfg(feature = "async_raw")]
    define_read_raw_func!();

//     #[cfg(feature = "async_bools")]
//     bools::define_bools_write!();

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

impl<W: AsyncVariableWritable + ?Sized> AsyncVariableWriter for W {
}

impl<W: AsyncWrite + Unpin> AsyncVariableWritable for W {
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
}
