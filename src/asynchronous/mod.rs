use core::pin::Pin;
use core::task::{Context, Poll, ready};
use crate::util::read_buf::ReadBuf;
use crate::util::write_buf::WriteBuf;

pub mod reader;
pub mod writer;
#[cfg(feature = "async_string")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
pub mod helper;

pub trait AsyncVariableReadable {
    type Error;

    fn poll_read_single(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut Option<u8>) -> Poll<Result<u8, Self::Error>>;

    fn poll_read_more(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<Result<(), Self::Error>> {
        while buf.left() > 0 {
            buf.put(ready!(self.as_mut().poll_read_single(cx, &mut None))?);
        }
        Poll::Ready(Ok(()))
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    #[inline]
    fn poll_read_more_buf<'a, B: bytes::BufMut>(mut self: Pin<&mut Self>, cx: &mut Context<'_>, bytes: &'a mut B) -> Poll<Result<(), Self::Error>> {
        use bytes::BufMut;
        while bytes.has_remaining_mut() {
            let chunk = bytes.chunk_mut();
            let chunk = unsafe {&mut *core::ptr::slice_from_raw_parts_mut(chunk.as_mut_ptr(), chunk.len()) };
            let mut buf = ReadBuf::new(chunk);
            let res = self.as_mut().poll_read_more(cx, &mut buf);
            let position = buf.position();
            unsafe { bytes.advance_mut(position); }
            ready!(res)?;
        }
        Poll::Ready(Ok(()))
    }
}

pub trait AsyncVariableWritable {
    type Error;

    fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut Option<u8>) -> Poll<Result<(), Self::Error>>;

    fn poll_write_more(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut WriteBuf<'_>) -> Poll<Result<(), Self::Error>> {
        while buf.left() > 0 {
            ready!(self.as_mut().poll_write_single(cx, &mut Some(buf.get())))?;
            buf.skip(1);
        }
        Poll::Ready(Ok(()))
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    #[inline]
    fn poll_write_more_buf<'a, B: bytes::Buf>(mut self: Pin<&mut Self>, cx: &mut Context<'_>, bytes: &'a mut B) -> Poll<Result<(), Self::Error>> {
        use bytes::Buf;
        while bytes.has_remaining() {
            let chunk = bytes.chunk();
            let mut buf = WriteBuf::new(chunk);
            let res = self.as_mut().poll_write_more(cx, &mut buf);
            let position = buf.position();
            bytes.advance(position);
            ready!(res)?;
        }
        Poll::Ready(Ok(()))
    }
}
