use std::io::{Error, ErrorKind, Read, Write};
use crate::synchronous::reader::VariableReader;
use crate::synchronous::{VariableReadable, VariableWritable};
use crate::synchronous::writer::VariableWriter;

impl<R: Read> VariableReadable for R {
    type Error = Error;

    #[inline]
    fn read_single(&mut self) -> Result<u8, Self::Error> {
        let mut buf = [0];
        R::read_exact(self, &mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_more(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        R::read_exact(self, buf)
    }
}

impl<R: Read> VariableReader for R {
    #[inline]
    fn read_bool_error(func_name: &'static str, byte: u8) -> Self::Error {
        Error::new(ErrorKind::InvalidData, format!("Invalid bool. value {} at {}.", byte, func_name))
    }

    #[cfg(feature = "sync_bools")]
    #[inline]
    fn read_bools_error(func_name: &'static str, byte: u8) -> Self::Error {
        Error::new(ErrorKind::InvalidData, format!("Invalid bools. value {} at {}.", byte, func_name))
    }

    #[cfg(feature = "sync_varint")]
    #[inline]
    fn read_varint_error(func_name: &'static str, value: u128) -> Self::Error {
        Error::new(ErrorKind::InvalidData, format!("Too long varint value. {} at {}.", value, func_name))
    }

    #[cfg(feature = "sync_string")]
    #[inline]
    fn read_string_error(_func_name: &'static str, error: alloc::string::FromUtf8Error) -> Self::Error {
        Error::new(ErrorKind::InvalidData, error)
    }
}

impl<W: Write> VariableWritable for W {
    type Error = Error;

    #[inline]
    fn write_single(&mut self, byte: u8) -> Result<(), Self::Error> {
        W::write_all(self, &[byte])
    }

    #[inline]
    fn write_more(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        W::write_all(self, buf)
    }
}

impl<W: Write> VariableWriter for W { }
