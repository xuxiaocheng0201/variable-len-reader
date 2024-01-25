use std::io::{Error, ErrorKind, Read, Result};
use crate::VariableReadable;

include!("reader_bools.rs");
include!("reader_raw.rs");
include!("reader_varint.rs");
include!("reader_signed.rs");

pub trait VariableReader: VariableReadable {
    #[inline]
    fn read_bool(&mut self) -> Result<bool> {
        match self.read_single()? {
            0 => Ok(false),
            1 => Ok(true),
            i => Err(Error::new(ErrorKind::InvalidData, format!("Invalid boolean value: {}", i))),
        }
    }

    define_read_bools!();
    define_read_raw!();
    define_read_varint!();
    define_read_signed!();

    #[cfg(feature = "vec_u8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "vec_u8")))]
    #[inline]
    fn read_u8_vec(&mut self) -> Result<Vec<u8>> {
        let length = self.read_usize_varint()?;
        let mut bytes = vec![0; length];
        self.read_more(&mut bytes)?;
        Ok(bytes)
    }

    #[cfg(feature = "string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "string")))]
    #[inline]
    fn read_string(&mut self) -> Result<String> {
        match String::from_utf8(self.read_u8_vec()?) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }
    }
}

impl<R: VariableReadable> VariableReader for R {
}

impl<R: Read> VariableReadable for R {
    #[inline]
    fn read_single(&mut self) -> Result<u8> {
        let mut buf = [0];
        R::read_exact(self, &mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_more(&mut self, buf: &mut [u8]) -> Result<()> {
        R::read_exact(self, buf)
    }
}
