mod reader;
pub use reader::*;

// mod writer;
// pub use writer::*;

#[cfg(test)]
mod tests;

pub trait VariableReadable {
    type Error;

    fn read_single(&mut self) -> Result<u8, Self::Error>;

    fn read_more(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        for i in 0..buf.len() {
            buf[i] = self.read_single()?;
        }
        Ok(())
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    fn read_more_buf<B: bytes::BufMut>(&mut self, len: usize, buf: &mut B) -> Result<(), Self::Error> {
        let mut t = vec![0; len];
        self.read_more(&mut t)?;
        buf.put_slice(&t);
        Ok(())
    }
}

pub trait VariableWritable {
    type Error;

    fn write_single(&mut self, byte: u8) -> Result<usize, Self::Error>;

    fn write_more(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for i in 0..buf.len() {
            self.write_single(buf[i])?;
        }
        Ok(buf.len())
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    fn write_more_buf<B: bytes::Buf>(&mut self, buf: &mut B) -> Result<usize, Self::Error> {
        let mut len = 0;
        while buf.has_remaining() {
            let written = self.write_more(buf.chunk())?;
            buf.advance(written);
            len += written;
        }
        Ok(len)
    }
}
