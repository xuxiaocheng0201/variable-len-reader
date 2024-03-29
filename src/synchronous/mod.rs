pub mod reader;
pub mod writer;

pub trait VariableReadable {
    type Error;

    fn read_single(&mut self) -> Result<u8, Self::Error>;

    fn read_more(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        for i in 0..buf.len() {
            buf[i] = self.read_single()?;
        }
        Ok(())
    }

    /// You may call [bytes::BytesMut::limit] to prevent reading more data than needed.
    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    fn read_more_buf<B: bytes::BufMut>(&mut self, buf: &mut B) -> Result<(), Self::Error> {
        use bytes::BufMut;
        while buf.has_remaining_mut() {
            let chunk = buf.chunk_mut();
            let chunk = unsafe {&mut *core::ptr::slice_from_raw_parts_mut(chunk.as_mut_ptr(), chunk.len()) };
            self.read_more(chunk)?;
        }
        Ok(())
    }
}

pub trait VariableWritable {
    type Error;

    fn write_single(&mut self, byte: u8) -> Result<(), Self::Error>;

    fn write_more(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        for i in 0..buf.len() {
            self.write_single(buf[i])?;
        }
        Ok(())
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    fn write_more_buf<B: bytes::Buf>(&mut self, buf: &mut B) -> Result<(), Self::Error> {
        use bytes::Buf;
        while buf.has_remaining() {
            let chunk = buf.chunk();
            self.write_more(chunk)?;
            buf.advance(chunk.len());
        }
        Ok(())
    }
}
