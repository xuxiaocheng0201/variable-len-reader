use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use pin_project_lite::pin_project;
use crate::asynchronous::AsyncVariableWritable;
use crate::util::write_buf::*;

pub trait WriterFuture<'a, W: ?Sized, B> {
    fn new(writer: &'a mut W, buf: B) -> Self;
    fn reset(self: Pin<&mut Self>, buf: B);
}

macro_rules! write_wrap_future {
    (@$primitive: ty, $future: ident, $inner_future: ident, $bound: ident $(, $feature: meta)?) => {
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
        #[cfg_attr(docsrs, doc($feature))]
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

/// AP means all-platform. This is used for usize/isize converting to u128/i128.
/// CP means current-platform. It writes usize/isize directly.
#[allow(unused_macros)]
macro_rules! write_size_ap_future {
    (f $feature: meta, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_wrap_future!(f $feature, $primitive, $future, $inner_future);
        #[$feature]
        impl<'a, W: ?Sized> $future<'a, W> {
            fn _handle(value: $primitive) -> $internal {
                value as $internal
            }
        }
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
pin_project! {
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
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

include!("write_raw.rs");
include!("write_raw_size.rs");

include!("write_varint.rs");
include!("write_varint_size.rs");
include!("write_varint_long.rs");
include!("write_varint_long_size.rs");

include!("write_signed_varint.rs");
include!("write_signed_varint_size.rs");
include!("write_signed_varint_long.rs");
include!("write_signed_varint_long_size.rs");

include!("write_float_varint.rs");
include!("write_float_varint_long.rs");

#[cfg(feature = "async_u8_vec")]
pin_project! {
    #[cfg_attr(docsrs, doc(cfg(feature = "async_u8_vec")))]
    #[derive(Debug)]
    #[project(!Unpin)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct WriteU8Vec<'a, W: ?Sized> {
        #[pin]
        inner: WriteUsizeVarintAp<'a, W>,
        buf: Option<OwnedWriteBuf<alloc::vec::Vec<u8>>>,
    }
}
#[cfg(feature = "async_u8_vec")]
impl<'a, W: ?Sized> WriterFuture<'a, W, alloc::vec::Vec<u8>> for WriteU8Vec<'a, W> {
    fn new(writer: &'a mut W, buf: alloc::vec::Vec<u8>) -> Self {
        Self { inner: WriteUsizeVarintAp::new(writer, buf.len()), buf: Some(OwnedWriteBuf::new(buf)) }
    }

    fn reset(self: Pin<&mut Self>, buf: alloc::vec::Vec<u8>) {
        let me = self.project();
        me.inner.reset(buf.len());
        *me.buf = Some(OwnedWriteBuf::new(buf));
    }
}
#[cfg(feature = "async_u8_vec")]
impl<'a, W: AsyncVariableWriter + Unpin + ?Sized> Future for WriteU8Vec<'a, W> {
    type Output = Result<(), W::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        let buf = match me.buf.as_mut() {
            None => return Poll::Ready(Ok(())), Some(b) => b
        };
        core::task::ready!(me.inner.as_mut().poll(cx))?;
        let mut ref_buf = buf.into();
        let res = W::poll_write_more(Pin::new(&mut me.inner.project().inner.project().inner.project().writer), cx, &mut ref_buf);
        let position = ref_buf.position();
        buf.set_position(position);
        core::task::ready!(res)?;
        *me.buf = None;
        Poll::Ready(Ok(()))
    }
}

write_wrap_future!(f cfg(feature = "async_string"), alloc::string::String, WriteString, WriteU8Vec);
#[cfg(feature = "async_string")]
impl<'a, W: ?Sized> WriteString<'a, W> {
    fn _handle(value: alloc::string::String) -> alloc::vec::Vec<u8> {
        value.into_bytes()
    }
}

pub trait AsyncVariableWriter: AsyncVariableWritable {
    #[inline]
    fn write_single(&mut self, byte: u8) -> WriteSingle<Self> where Self: Unpin {
        WriteSingle::new(self, byte)
    }

    #[inline]
    fn write_more<'a>(&'a mut self, buf: &'a [u8]) -> WriteMore<'a, Self> where Self: Unpin {
        WriteMore { writer: self, buf: WriteBuf::new(buf) }
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    #[inline]
    fn write_more_buf<'a, B: bytes::Buf>(&'a mut self, buf: &'a mut B) -> WriteMoreBuf<'a, Self, B> where Self: Unpin {
        WriteMoreBuf { writer: self, buf }
    }


    write_wrap_func!(bool, WriteBool, write_bool);

    define_write_bools_func!();

    define_write_raw_func!();
    define_write_raw_size_func!();

    define_write_varint_func!();
    define_write_varint_size_func!();
    define_write_varint_long_func!();
    define_write_varint_long_size_func!();

    define_write_signed_varint_func!();
    define_write_signed_varint_size_func!();
    define_write_signed_varint_long_func!();
    define_write_signed_varint_long_size_func!();

    define_write_float_varint_func!();
    define_write_float_long_func!();

    /// This method consumes the vec.
    ///
    /// You can use the example instead.
    /// ```rust,ignore
    /// writer.write_usize_varint_ap(value.len()).await?;
    /// writer.write_more(value).await?;
    /// ```
    /// Or you can simply call [Self::write_u8_vec_boxed] instead.
    /// ```rust,ignore
    /// writer.write_u8_vec_boxed(&value).await?;
    /// ```
    ///
    /// Now you can call
    /// ```rust,ignore
    /// AsyncWriterHelper(&mut writer).help_write_u8_vec().await?;
    /// ```
    #[cfg(feature = "async_u8_vec")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_u8_vec")))]
    #[inline]
    fn write_u8_vec(&mut self, value: alloc::vec::Vec<u8>) -> WriteU8Vec<Self> where Self: Unpin {
        WriteU8Vec::new(self, value)
    }

    /// This future is not zero-cost.
    /// But it borrows the vec, different from [Self::write_u8_vec].
    #[cfg(feature = "async_u8_vec")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_u8_vec")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    #[deprecated(since = "3.2.0", note = "use [AsyncWriterHelper::help_write_u8_vec] instead")]
    fn write_u8_vec_boxed<'a>(&'a mut self, value: &'a [u8]) -> Pin<alloc::boxed::Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> where Self: Unpin + Send {
        alloc::boxed::Box::pin(async move {
            self.write_usize_varint_ap(value.len()).await?;
            self.write_more(value).await?;
            Ok(())
        })
    }

    /// This method is based on [Self::write_u8_vec],
    /// which consumes the value.
    ///
    /// Or you can simply call [Self::write_string_boxed] instead.
    ///
    /// Now you can call
    /// ```rust,ignore
    /// AsyncWriterHelper(&mut writer).help_write_string().await?;
    /// ```
    #[cfg(feature = "async_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    #[inline]
    fn write_string(&mut self, value: alloc::string::String) -> WriteString<Self> where Self: Unpin {
        WriteString::new(self, value)
    }

    #[cfg(feature = "async_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    #[inline]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    #[deprecated(since = "3.2.0", note = "use [AsyncWriterHelper::help_write_string] instead")]
    #[allow(deprecated)]
    fn write_string_boxed<'a>(&'a mut self, value: &'a str) -> Pin<alloc::boxed::Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> where Self: Unpin + Send {
        self.write_u8_vec_boxed(value.as_bytes())
    }
}
