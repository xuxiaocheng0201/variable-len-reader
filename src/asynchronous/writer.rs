use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, ready};
use pin_project_lite::pin_project;
use tokio::io::AsyncWrite;
use crate::asynchronous::write_func;
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
impl<'a, W: AsyncVariableWritable + Unpin + ?Sized> Future for crate::asynchronous::WriteSingle<'a, W> {
    type Output = std::io::Result<usize>;

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
impl<'a, W: AsyncVariableWritable + Unpin + ?Sized> Future for crate::asynchronous::WriteMore<'a, W> {
    type Output = std::io::Result<usize>;

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
impl<'a, W: AsyncVariableWritable + Unpin + ?Sized> Future for crate::asynchronous::WriteBool<'a, W> {
    type Output = std::io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        W::poll_write_single(Pin::new(&mut *me.writer), cx, if *me.b { 1 } else { 0 })
    }
}

pub trait AsyncVariableWritable {
    fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, byte: u8) -> Poll<std::io::Result<usize>>;

    fn poll_write_more(mut self: Pin<&mut Self>, cx: &mut Context<'_>, bytes: &[u8]) -> Poll<std::io::Result<usize>> {
        for i in 0..bytes.len() {
            ready!(self.as_mut().poll_write_single(cx, bytes[i]))?;
        }
        Poll::Ready(Ok(bytes.len()))
    }

    #[inline]
    fn write_single(&mut self, byte: u8) -> crate::asynchronous::WriteSingle<Self> where Self: Unpin {
        crate::asynchronous::WriteSingle { writer: self, byte }
    }

    #[inline]
    fn write_more<'a>(&'a mut self, bytes: &'a [u8]) -> crate::asynchronous::WriteMore<Self> where Self: Unpin {
        crate::asynchronous::WriteMore { writer: self, bytes }
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
    fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, byte: u8) -> Poll<std::io::Result<usize>> {
        W::poll_write(self, cx, &[byte])
    }

    fn poll_write_more(self: Pin<&mut Self>, cx: &mut Context<'_>, bytes: &[u8]) -> Poll<std::io::Result<usize>> {
        W::poll_write(self, cx, bytes)
    }
}


macro_rules! write_func {
    ($b: ty, $func: ident, $future: ident) => {
        #[inline]
        fn $func(&mut self, b: $b) -> $future<Self> where Self: Unpin {
            $future { writer: self, b }
        }
    };
}
use write_func;

