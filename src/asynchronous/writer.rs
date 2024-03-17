use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use pin_project_lite::pin_project;
use crate::asynchronous::AsyncVariableWritable;
use crate::util::write_buf::WriteBuf;

pub trait WriterFuture<'a, W: ?Sized, B> {
    fn new(writer: &'a mut W, buf: B) -> Self;
    fn reset(self: Pin<&mut Self>, buf: B);
}

macro_rules! write_wrap_future {
    (@$primitive: ty, $future: ident, $inner_future: ident, $bound: ident $(, $feature: meta)?) => {
        $(
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        )?
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                inner: $inner_future<'a, W>,
            }
        }
        $(
        #[$feature]
        )?
        impl<'a, W: ?Sized> WriterFuture<'a, W, $primitive> for $future<'a, W> {
            fn new(writer: &'a mut W, buf: $primitive) -> Self {
                Self { inner: $inner_future::new(writer, Self::_handle(buf)) }
            }
            fn reset(self: Pin<&mut Self>, buf: $primitive) {
                let me = self.project();
                me.inner.reset(Self::_handle(buf));
            }
        }
        $(
        #[$feature]
        )?
        impl<'a, W: $bound + Unpin + ?Sized> Future for $future<'a, W> {
            type Output = Result<(), W::Error>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                self.project().inner.poll(cx)
            }
        }
    };
    (f $feature: meta, $primitive: ty, $future: ident, $inner_future: ident) => {
        write_wrap_future!(@$primitive, $future, $inner_future, AsyncVariableWriter, $feature);
    };
    ($primitive: ty, $future: ident, $inner_future: ident able) => {
        write_wrap_future!(@$primitive, $future, $inner_future, AsyncVariableWritable);
    };
}
macro_rules! write_wrap_func {
    (@$primitive: ty, $future: ident, $func: ident $(, $feature: meta)?) => {
        $(
        #[$feature]
        )?
        #[inline]
        fn $func(&mut self, value: $primitive) -> $future<Self> where Self: Unpin {
            $future::new(self, value)
        }
    };
    (f $feature: meta, $primitive: ty, $future: ident, $func: ident) => {
        write_wrap_func!(@$primitive, $future, $func, $feature);
    };
    ($primitive: ty, $future: ident, $func: ident) => {
        write_wrap_func!(@$primitive, $future, $func);
    };
}

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct WriteSingle<'a, W: ?Sized> {
        #[pin]
        writer: &'a mut W,
        buf: Option<u8>,
    }
}
impl<'a, W: ?Sized> WriterFuture<'a, W, u8> for WriteSingle<'a, W> {
    fn new(writer: &'a mut W, buf: u8) -> Self {
        Self { writer, buf: Some(buf) }
    }
    fn reset(self: Pin<&mut Self>, buf: u8) {
        let me = self.project();
        *me.buf = Some(buf);
    }
}
impl<'a, W: AsyncVariableWritable + Unpin + ?Sized> Future for WriteSingle<'a, W> {
    type Output = Result<(), W::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        W::poll_write_single(Pin::new(&mut *me.writer), cx, me.buf)
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
impl<'a, W: ?Sized> WriterFuture<'a, W, WriteBuf<'a>> for WriteMore<'a, W> {
    fn new(writer: &'a mut W, buf: WriteBuf<'a>) -> Self {
        Self { writer, buf }
    }
    fn reset(self: Pin<&mut Self>, buf: WriteBuf<'a>) {
        let me = self.project();
        *me.buf = buf;
    }
}
impl<'a, W: AsyncVariableWritable + Unpin + ?Sized> Future for WriteMore<'a, W> {
    type Output = Result<(), W::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        W::poll_write_more(Pin::new(&mut *me.writer), cx, me.buf)
    }
}

#[cfg(feature = "bytes")]
#[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct WriteMoreBuf<'a, W: ?Sized, B> where B: bytes::Buf {
        #[pin]
        writer: &'a mut W,
        #[pin]
        buf: &'a mut B,
    }
}
#[cfg(feature = "bytes")]
impl<'a, W: AsyncVariableWritable + Unpin + ?Sized, B: bytes::Buf> Future for WriteMoreBuf<'a, W, B> {
    type Output = Result<(), W::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        W::poll_write_more_buf(Pin::new(&mut *me.writer), cx, &mut *me.buf)
    }
}


write_wrap_future!(bool, WriteBool, WriteSingle able);
impl<'a, W: ?Sized> WriteBool<'a, W> {
    fn _handle(buf: bool) -> u8 {
        if buf { 1 } else { 0 }
    }
}

include!("write_bools.rs");

// include!("writer_raw.rs");
// include!("writer_varint.rs");
// include!("writer_signed.rs");
// include!("writer_float.rs");

pub trait AsyncVariableWriter: AsyncVariableWritable {
    #[inline]
    fn write_single(&mut self, byte: u8) -> WriteSingle<Self> where Self: Unpin {
        WriteSingle::new(self, byte)
    }

    #[inline]
    fn write_more<'a>(&'a mut self, buf: &'a [u8]) -> WriteMore<Self> where Self: Unpin {
        WriteMore { writer: self, buf: WriteBuf::new(buf) }
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    #[inline]
    fn write_more_buf<'a, B: bytes::Buf>(&'a mut self, buf: &'a mut B) -> WriteMoreBuf<Self, B> where Self: Unpin {
        WriteMoreBuf { writer: self, buf }
    }


    write_wrap_func!(bool, WriteBool, write_bool);

    define_write_bools_func!();

    // define_write_raw_func!();
    // define_write_varint_func!();
    // define_write_signed_func!();
    // define_write_float_func!();
    //
    // #[cfg(feature = "async_vec_u8")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "async_vec_u8")))]
    // #[inline]
    // #[must_use = "futures do nothing unless you `.await` or poll them"]
    // fn write_u8_vec<'a>(&'a mut self, message: &'a [u8]) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin + Send {
    //     Box::pin(async move {
    //         let mut size = self.write_usize_varint(message.len()).await?;
    //         size += self.write_more(message).await?;
    //         Ok(size)
    //     })
    // }
    //
    // #[cfg(feature = "async_string")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    // #[inline]
    // #[must_use = "futures do nothing unless you `.await` or poll them"]
    // fn write_string<'a>(&'a mut self, message: &'a str) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin + Send {
    //     self.write_u8_vec(message.as_bytes())
    // }
}

impl<W: AsyncVariableWritable + ?Sized> AsyncVariableWriter for W {
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl<W: tokio::io::AsyncWrite + Unpin> AsyncVariableWritable for W {
    fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut Option<u8>) -> Poll<Result<(), Self::Error>> {
        match buf.as_ref() {
            None => Poll::Ready(Ok(())),
            Some(b) => W::poll_write(self, cx, &[*b]).map_ok(|_i| { buf.take(); () })
        }
    }

    fn poll_write_more(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut WriteBuf<'_>) -> Poll<Result<(), Self::Error>> {
        while buf.left() > 0 {
            let position = buf.position();
            let n = ready!(W::poll_write(self.as_mut(), cx, &buf.buf()[position..]))?;
            if n == 0 {
                return Poll::Ready(Err(tokio::io::Error::new(tokio::io::ErrorKind::WriteZero, "write 0 bytes")));
            }
            buf.skip(n);
        }
        Poll::Ready(Ok(()))
    }
}
