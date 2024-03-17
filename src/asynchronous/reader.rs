use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use pin_project_lite::pin_project;
use crate::asynchronous::{AsyncVariableReadable, ResettableFuture};
use crate::util::read_buf::*;

#[allow(unused_macros)]
macro_rules! read_wrap_future {
    (f $feature: meta, $future: ident, $inner_future: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                inner: $inner_future<'a, R>,
            }
        }
        #[$feature]
        impl<'a, R: ?Sized> ResettableFuture for $future<'a, R> {
            fn reset(self: Pin<&mut Self>) {
                let me = self.project();
                me.inner.reset();
            }
        }
    };
}
#[allow(unused_macros)]
macro_rules! read_wrap_func {
    (f $feature: meta, $future: ident, $func: ident, $inner_func: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { inner: self.$inner_func() }
        }
    };
}

/// AP means all-platform. This is used for usize/isize converting from u128/i128.
/// CP means current-platform. It reads usize/isize directly.
#[allow(unused_macros)]
macro_rules! read_size_ap_future {
    (f $feature: meta, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_wrap_future!(f $feature, $future, $inner_future);
        #[$feature]
        impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for $future<'a, R> {
            type Output = ::core::result::Result<$primitive, R::Error>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                self.project().inner.poll(cx).map_ok(|v| v as $primitive)
            }
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
        buf: Option<u8>,
    }
}
impl<'a, R: ?Sized> ResettableFuture for ReadSingle<'a, R> {
    fn reset(self: Pin<&mut Self>) {
        let me = self.project();
        *me.buf = None;
    }
}
impl<'a, R: AsyncVariableReadable + Unpin + ?Sized> Future for ReadSingle<'a, R> {
    type Output = Result<u8, R::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        if let Some(b) = me.buf.as_ref() { return Poll::Ready(Ok(*b)); }
        R::poll_read_single(Pin::new(&mut *me.reader), cx, me.buf)
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
impl<'a, R: ?Sized> ResettableFuture for ReadMore<'a, R> {
    fn reset(self: Pin<&mut Self>) {
        let me = self.project();
        me.buf.reset();
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
impl<'a, R: ?Sized> ResettableFuture for ReadBool<'a, R> {
    fn reset(self: Pin<&mut Self>) {
        let me = self.project();
        me.inner.reset();
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

include!("read_raw.rs");
include!("read_raw_size.rs");

include!("read_varint.rs");
include!("read_varint_size.rs");
include!("read_varint_long.rs");
include!("read_varint_long_size.rs");

include!("read_signed_varint.rs");
include!("read_signed_varint_size.rs");
include!("read_signed_varint_long.rs");
include!("read_signed_varint_long_size.rs");

include!("read_float_varint.rs");
include!("read_float_varint_long.rs");

#[cfg(feature = "async_vec_u8")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_vec_u8")))]
pin_project! {
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadVecU8<'a, R: ?Sized> {
        #[pin]
        inner: ReadUsizeVarintAp<'a, R>,
        buf: Option<OwnedReadBuf<alloc::vec::Vec<u8>>>,
    }
}
#[cfg(feature = "async_vec_u8")]
impl<'a, R: ?Sized> ResettableFuture for ReadVecU8<'a, R> {
    fn reset(self: Pin<&mut Self>) {
        let me = self.project();
        me.inner.reset();
        *me.buf = None;
    }
}
#[cfg(feature = "async_vec_u8")]
impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for ReadVecU8<'a, R> {
    type Output = Result<alloc::vec::Vec<u8>, R::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        let buf = match me.buf.as_mut() {
            None => {
                let size = ::core::task::ready!(me.inner.as_mut().poll(cx))?;
                *me.buf = Some(OwnedReadBuf::new(alloc::vec![0; size]));
                me.buf.as_mut().unwrap()
            }, Some(b) => b,
        };
        let mut ref_buf = buf.into();
        let res = R::poll_read_more(Pin::new(&mut me.inner.project().inner.project().inner.project().reader), cx, &mut ref_buf);
        let position = ref_buf.position();
        buf.set_position(position);
        ::core::task::ready!(res)?;
        Poll::Ready(Ok(buf.clone().into_inner()))
    }
}

pub trait AsyncVariableReader: AsyncVariableReadable {
    #[inline]
    fn read_single(&mut self) -> ReadSingle<Self> where Self: Unpin {
        ReadSingle { reader: self, buf: None }
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

    define_read_raw_func!();
    define_read_raw_size_func!();

    define_read_varint_func!();
    define_read_varint_size_func!();
    define_read_varint_long_func!();
    define_read_varint_long_size_func!();

    define_read_signed_varint_func!();
    define_read_signed_varint_size_func!();
    define_read_signed_varint_long_func!();
    define_read_signed_varint_long_size_func!();

    define_read_float_varint_func!();
    define_read_float_varint_long_func!();

    /// Note this future is not zero-cost,
    /// it will clone the inner vec buf when poll returns ready.
    ///
    /// You can use the example below instead.
    /// ```rust,ignore
    /// let len = self.read_usize_varint_ap().await?;
    /// let buf = vec![0; len];
    /// self.read_more(&mut buf).await?;
    /// ```
    /// Or you can simply call [`self.read_u8_vec_boxed`] instead.
    /// ```rust,ignore
    /// self.read_u8_vec_boxed().await?;
    /// ```
    #[cfg(feature = "async_vec_u8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_vec_u8")))]
    #[inline]
    #[deprecated(since = "3.0.0", note = "see docs for details")]
    fn read_u8_vec(&mut self) -> ReadVecU8<Self> where Self: Unpin {
        ReadVecU8 { inner: self.read_usize_varint_ap(), buf: None }
    }

    /// This future is not zero-cost.
    /// But it is more efficient than [`self.read_u8_vec`]
    /// when you need to read a large number of u8s.
    #[cfg(feature = "async_vec_u8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_vec_u8")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    fn read_u8_vec_boxed(&mut self) -> Pin<alloc::boxed::Box<dyn Future<Output = Result<alloc::vec::Vec<u8>, Self::Error>> + Send + '_>> where Self: Unpin + Send {
        alloc::boxed::Box::pin(async move {
            let length = self.read_usize_varint().await?;
            let mut bytes = alloc::vec![0; length];
            self.read_more(&mut bytes).await?;
            Ok(bytes)
        })
    }

    #[cfg(feature = "async_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    fn read_string_error(future_name: &'static str, error: alloc::string::FromUtf8Error) -> Self::Error;

    #[cfg(feature = "async_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    fn read_string_boxed(&mut self) -> Pin<alloc::boxed::Box<dyn Future<Output = Result<alloc::string::String, Self::Error>> + Send + '_>> where Self: Unpin + Send {
        alloc::boxed::Box::pin(async move {
            match alloc::string::String::from_utf8(self.read_u8_vec_boxed().await?) {
                Ok(s) => Ok(s),
                Err(e) => Err(Self::read_string_error("ReadString", e)),
            }
        })
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl<R: tokio::io::AsyncRead + Unpin> AsyncVariableReadable for R {
    type Error = tokio::io::Error;

    fn poll_read_single(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut Option<u8>) -> Poll<Result<u8, Self::Error>> {
        if let Some(b) = buf.as_ref() { return Poll::Ready(Ok(*b)); }
        let mut b = [0];
        ::core::task::ready!(R::poll_read(self, cx, &mut tokio::io::ReadBuf::new(&mut b)))?;
        buf.replace(b[0]);
        Poll::Ready(Ok(b[0]))
    }

    fn poll_read_more(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<(), Self::Error>> {
        let origin = buf.left();
        if origin == 0 { return Poll::Ready(Ok(())); }
        let mut tokio_buf = buf.into();
        ::core::task::ready!(R::poll_read(self, cx, &mut tokio_buf))?;
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

    #[cfg(feature = "async_varint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_varint")))]
    #[inline]
    fn read_varint_error(future_name: &'static str, value: u128) -> Self::Error {
        tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, alloc::format!("Too long varint value. {} at future {}.", value, future_name))
    }

    #[cfg(feature = "async_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    #[inline]
    fn read_string_error(_future_name: &'static str, error: alloc::string::FromUtf8Error) -> Self::Error {
        tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, error)
    }
}
