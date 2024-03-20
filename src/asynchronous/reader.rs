use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use pin_project_lite::pin_project;
use crate::asynchronous::AsyncVariableReadable;
use crate::util::read_buf::*;

pub trait ReaderFuture {
    fn reset(self: Pin<&mut Self>);
}

macro_rules! read_wrap_future {
    (@$future: ident, $inner_future: ident $(, $feature: meta)?) => {
        $(
        #[$feature]
        )?
        $crate::pin_project_lite::pin_project! {
            $(
            #[cfg_attr(docsrs, doc($feature))]
            )?
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                inner: $inner_future<'a, R>,
            }
        }
        $(
        #[$feature]
        )?
        impl<'a, R: ?Sized> ReaderFuture for $future<'a, R> {
            fn reset(self: Pin<&mut Self>) {
                let me = self.project();
                me.inner.reset();
            }
        }
    };
    (f $feature: meta, $future: ident, $inner_future: ident) => {
        read_wrap_future!(@$future, $inner_future, $feature);
    };
    ($future: ident, $inner_future: ident) => {
        read_wrap_future!(@$future, $inner_future);
    };
}
macro_rules! read_wrap_func {
    (@$future: ident, $func: ident, $inner_func: ident $(, $feature: meta)?) => {
        $(
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        )?
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { inner: self.$inner_func() }
        }
    };
    (f $feature: meta, $future: ident, $func: ident, $inner_func: ident) => {
		read_wrap_func!(@$future, $func, $inner_func, $feature);
	};
    ($future: ident, $func: ident, $inner_func: ident) => {
		read_wrap_func!(@$future, $func, $inner_func);
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
impl<'a, R: ?Sized> ReaderFuture for ReadSingle<'a, R> {
    fn reset(self: Pin<&mut Self>) {
        let me = self.project();
        *me.buf = None;
    }
}
impl<'a, R: AsyncVariableReadable + Unpin + ?Sized> Future for ReadSingle<'a, R> {
    type Output = Result<u8, R::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
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
impl<'a, R: ?Sized> ReaderFuture for ReadMore<'a, R> {
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
pin_project! {
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
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


read_wrap_future!(ReadBool, ReadSingle);
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

#[cfg(feature = "async_u8_vec")]
pin_project! {
    #[cfg_attr(docsrs, doc(cfg(feature = "async_u8_vec")))]
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadU8Vec<'a, R: ?Sized> {
        #[pin]
        inner: ReadUsizeVarintAp<'a, R>,
        buf: Option<OwnedReadBuf<alloc::vec::Vec<u8>>>,
    }
}
#[cfg(feature = "async_u8_vec")]
impl<'a, R: ?Sized> ReaderFuture for ReadU8Vec<'a, R> {
    fn reset(self: Pin<&mut Self>) {
        let me = self.project();
        me.inner.reset();
        *me.buf = None;
    }
}
#[cfg(feature = "async_u8_vec")]
impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for ReadU8Vec<'a, R> {
    type Output = Result<alloc::vec::Vec<u8>, R::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        let buf = match me.buf.as_mut() {
            None => {
                let size = core::task::ready!(me.inner.as_mut().poll(cx))?;
                *me.buf = Some(OwnedReadBuf::new(alloc::vec![0; size]));
                me.buf.as_mut().unwrap()
            }, Some(b) => b,
        };
        let mut ref_buf = buf.into();
        let res = R::poll_read_more(Pin::new(&mut me.inner.project().inner.project().inner.project().reader), cx, &mut ref_buf);
        let position = ref_buf.position();
        buf.set_position(position);
        core::task::ready!(res)?;
        Poll::Ready(Ok(buf.clone().into_inner()))
    }
}

read_wrap_future!(f cfg(feature = "async_string"), ReadString, ReadU8Vec);
#[cfg(feature = "async_string")]
impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for ReadString<'a, R> {
    type Output = Result<alloc::string::String, R::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().inner.poll(cx).map(|r| r.and_then(|v| {
            match alloc::string::String::from_utf8(v) {
                Ok(s) => Ok(s),
                Err(e) => Err(R::read_string_error("ReadString", e)),
            }
        }))
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

    /// You may call [bytes::BytesMut::limit] to prevent reading more data than needed.
    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    #[inline]
    fn read_more_buf<'a, B: bytes::BufMut>(&'a mut self, buf: &'a mut B) -> ReadMoreBuf<Self, B> where Self: Unpin {
        ReadMoreBuf { reader: self, buf }
    }


    fn read_bool_error(feature_name: &'static str, byte: u8) -> Self::Error;

    read_wrap_func!(ReadBool, read_bool, read_single);

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
    /// let len = reader.read_usize_varint_ap().await?;
    /// let buf = vec![0; len];
    /// reader.read_more(&mut buf).await?;
    /// ```
    /// Or you can simply call [Self::read_u8_vec_boxed] instead.
    /// ```rust,ignore
    /// reader.read_u8_vec_boxed().await?;
    /// ```
    ///
    /// Now you can call
    /// ```rust,ignore
    /// AsyncReaderHelper(&mut reader).help_read_u8_vec().await?;
    /// ```
    #[cfg(feature = "async_u8_vec")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_u8_vec")))]
    #[inline]
    #[deprecated(since = "3.0.0", note = "see docs for details")]
    fn read_u8_vec(&mut self) -> ReadU8Vec<Self> where Self: Unpin {
        ReadU8Vec { inner: self.read_usize_varint_ap(), buf: None }
    }

    /// This future is not zero-cost.
    /// But it is more efficient than [Self::read_u8_vec]
    /// when you need to read a large number of u8s.
    #[cfg(feature = "async_u8_vec")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_u8_vec")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    #[deprecated(since = "3.2.0", note = "use [AsyncReaderHelper::help_read_u8_vec] instead")]
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

    /// This future is based on [Self::read_u8_vec],
    /// which is not zero-cost and deprecated.
    ///
    /// Or you can simply call [Self::read_string_boxed] instead.
    /// ```rust,ignore
    /// reader.read_string_boxed().await?;
    /// ```
    ///
    /// Now you can call
    /// ```rust,ignore
    /// AsyncReaderHelper(&mut reader).help_read_string().await?;
    /// ```
    #[cfg(feature = "async_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    #[inline]
    #[deprecated(since = "3.0.0", note = "see docs for details")]
    #[allow(deprecated)]
    fn read_string(&mut self) -> ReadString<Self> where Self: Unpin {
        ReadString { inner: self.read_u8_vec() }
    }

    #[cfg(feature = "async_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    #[deprecated(since = "3.2.0", note = "use [AsyncReaderHelper::help_read_string] instead")]
    #[allow(deprecated)]
    fn read_string_boxed(&mut self) -> Pin<alloc::boxed::Box<dyn Future<Output = Result<alloc::string::String, Self::Error>> + Send + '_>> where Self: Unpin + Send {
        alloc::boxed::Box::pin(async move {
            match alloc::string::String::from_utf8(self.read_u8_vec_boxed().await?) {
                Ok(s) => Ok(s),
                Err(e) => Err(Self::read_string_error("ReadString", e)),
            }
        })
    }
}
