use std::io::{Result, Write};
use crate::VariableWritable;

include!("writer_bools.rs");
include!("writer_raw.rs");
include!("writer_varint.rs");
include!("writer_signed.rs");

pub trait VariableWriter: VariableWritable {
    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<usize> {
        self.write_single(if b { 1 } else { 0 })
    }

    define_write_bools!();
    define_write_raw!();
    define_write_varint!();
    define_write_signed!();

    #[cfg(feature = "vec_u8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "vec_u8")))]
    #[inline]
    fn write_u8_vec(&mut self, message: &[u8]) -> Result<usize> {
        self.write_usize_varint(message.len())?;
        self.write_more(message)
    }

    #[cfg(feature = "string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "string")))]
    #[inline]
    fn write_string(&mut self, message: &str) -> Result<usize> {
        self.write_u8_vec(message.as_bytes())
    }
}

impl<W: VariableWritable> VariableWriter for W {
}

impl<W: Write> VariableWritable for W {
    #[inline]
    fn write_single(&mut self, byte: u8) -> Result<usize> {
        W::write_all(self, &[byte])?;
        Ok(1)
    }

    #[inline]
    fn write_more(&mut self, buf: &[u8]) -> Result<usize> {
        W::write_all(self, buf)?;
        Ok(buf.len())
    }
}
