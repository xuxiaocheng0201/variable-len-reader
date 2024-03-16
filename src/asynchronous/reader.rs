use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, ready};
use pin_project_lite::pin_project;
use crate::asynchronous::AsyncVariableReadable;
use crate::util::read_buf::*;

/// AP means all-platform. This is used for usize/isize converting from u128/i128.
/// CP means current-platform. It reads usize/isize directly.
#[allow(unused_macros)]
macro_rules! read_size_ap_feature {
    (f $feature: meta, $primitive: ty, $future: ident, $inner_future: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        $crate::pin_project_lite::pin_project! {
            #[derive(::core::fmt::Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                inner: $inner_future<'a, R>,
            }
        }
        #[$feature]
        impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for $future<'a, R> {
            type Output = ::core::result::Result<$primitive, R::Error>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                self.project().inner.poll(cx).map_ok(|v| v as $primitive)
            }
        }
    };
}
#[allow(unused_macros)]
macro_rules! read_size_ap_func {
    (f $feature: meta, $future: ident, $func: ident, $inner_func: ident) => {
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { inner: self.inner_func() }
        }
    };
}

pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadSingle<'a, R: ?Sized> {
        #[pin]
        reader: &'a mut R,
    }
}
impl<'a, R: AsyncVariableReadable + Unpin + ?Sized> Future for ReadSingle<'a, R> {
    type Output = Result<u8, R::Error>;

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
impl<'a, R: AsyncVariableReadable + Unpin + ?Sized> Future for ReadMore<'a, R> {
    type Output = Result<(), R::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        R::poll_read_more(Pin::new(&mut *me.reader), cx, me.buf)
    }
}

#[cfg(feature = "bytes")]
#[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadMoreBuf<'a, R: ?Sized, B> where B: bytes::BufMut {
        #[pin]
        reader: &'a mut R,
        #[pin]
        buf: &'a mut B,
    }
}
#[cfg(feature = "bytes")]
#[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
impl<'a, R: AsyncVariableReadable + Unpin + ?Sized, B: bytes::BufMut> Future for ReadMoreBuf<'a, R, B> {
    type Output = Result<(), R::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        R::poll_read_more_buf(Pin::new(&mut *me.reader), cx, &mut *me.buf)
    }
}


pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadBool<'a, R: ?Sized> {
        #[pin]
        inner: ReadSingle<'a, R>,
    }
}
impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for ReadBool<'a, R> {
    type Output = Result<bool, R::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().inner.poll(cx).map(|r| r.and_then(|b| match b {
            0 => Ok(false),
            1 => Ok(true),
            b => Err(R::read_bool_error("ReadBool", b)),
        }))
    }
}

include!("read_bools.rs");

// include!("read_raw.rs");
// include!("read_raw_size.rs");

// include!("reader_varint.rs");
// include!("reader_signed.rs");
// include!("reader_float.rs");

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
    fn read_more_buf<'a, B: bytes::BufMut>(&'a mut self, buf: &'a mut B) -> ReadMoreBuf<Self, B> where Self: Unpin {
        ReadMoreBuf { reader: self, buf }
    }


    fn read_bool_error(feature_name: &'static str, byte: u8) -> Self::Error;

    #[inline]
    fn read_bool(&mut self) -> ReadBool<Self> where Self: Unpin {
        ReadBool { inner: self.read_single() }
    }

    define_read_bools_func!();

    // define_read_raw_func!();

    // define_read_varint_func!();
    // define_read_signed_func!();
    // define_read_float_func!();

    // #[cfg(feature = "async_vec_u8")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "async_vec_u8")))]
    // #[inline]
    // #[must_use = "futures do nothing unless you `.await` or poll them"]
    // fn read_u8_vec(&mut self) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + '_>> where Self: Unpin + Send {
    //     Box::pin(async move {
    //         let length = self.read_usize_varint().await?;
    //         let mut bytes = vec![0; length];
    //         self.read_more(&mut bytes).await?;
    //         Ok(bytes)
    //     })
    // }
    //
    // #[cfg(feature = "async_string")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    // #[inline]
    // #[must_use = "futures do nothing unless you `.await` or poll them"]
    // fn read_string(&mut self) -> Pin<Box<dyn Future<Output = Result<String>> + Send + '_>> where Self: Unpin + Send {
    //     Box::pin(async move {
    //         match String::from_utf8(self.read_u8_vec().await?) {
    //             Ok(s) => Ok(s),
    //             Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string())),
    //         }
    //     })
    // }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl<R: tokio::io::AsyncRead + Unpin> AsyncVariableReadable for R {
    type Error = tokio::io::Error;

    fn poll_read_single(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<u8, Self::Error>> {
        let mut buf = [0];
        ready!(R::poll_read(self, cx, &mut tokio::io::ReadBuf::new(&mut buf)))?;
        Poll::Ready(Ok(buf[0]))
    }

    fn poll_read_more(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<(), Self::Error>> {
        let origin = buf.left();
        if origin == 0 {
            return Poll::Ready(Ok(()));
        }
        let mut tokio_buf = buf.into();
        ready!(R::poll_read(self, cx, &mut tokio_buf))?;
        let filled = tokio_buf.filled().len();
        buf.set_position(filled);
        let left = buf.left();
        if left == 0 {
            Poll::Ready(Ok(()))
        } else if left == origin {
            Poll::Ready(Err(tokio::io::Error::new(tokio::io::ErrorKind::UnexpectedEof, "read 0 byte")))
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl<R: tokio::io::AsyncRead + Unpin> AsyncVariableReader for R {
    #[inline]
    fn read_bool_error(future_name: &'static str, byte: u8) -> Self::Error {
        tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, alloc::format!("Invalid bool. value {} at future {}.", byte, future_name))
    }

    #[cfg(feature = "async_bools")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
    #[inline]
    fn read_bools_error(future_name: &'static str, byte: u8) -> Self::Error {
        tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, alloc::format!("Invalid bools. value {} at future {}.", byte, future_name))
    }
}
