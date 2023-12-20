use std::io::{Error, ErrorKind, Read, Result, Write};
#[cfg(feature = "signed")]
use crate::zigzag::Zigzag;

#[cfg(feature = "async")]
pub extern crate async_trait;

#[cfg(feature = "bools")]
mod bools;
#[cfg(feature = "raw")]
mod raw;
#[cfg(feature = "varint")]
mod varint;
#[cfg(feature = "signed")]
mod signed;
#[cfg(any(feature = "signed", feature = "async_signed"))]
pub mod zigzag;
#[cfg(feature = "async")]
pub mod asynchronous;

pub trait VariableReadable {
    fn read(&mut self) -> Result<u8>;

    #[inline]
    fn read_bool(&mut self) -> Result<bool> {
        match self.read()? {
            0 => Ok(false),
            1 => Ok(true),
            i => Err(Error::new(ErrorKind::InvalidData, format!("Invalid boolean value: {}", i))),
        }
    }

    #[cfg(feature = "bools")]
    bools::define_bools_read!();

    fn read_more(&mut self, buf: &mut [u8]) -> Result<()> {
        for i in 0..buf.len() {
            buf[i] = self.read()?;
        }
        Ok(())
    }

    #[cfg(feature = "raw")]
    raw::define_raw_read!();

    #[cfg(feature = "varint")]
    varint::define_varint_read!();

    #[cfg(feature = "signed")]
    signed::define_signed_read!();

    #[cfg(feature = "vec_u8")]
    #[inline]
    fn read_u8_vec(&mut self) -> Result<Vec<u8>> {
        let length = self.read_u128_varint()? as usize;
        let mut bytes = vec![0; length];
        self.read_more(&mut bytes)?;
        Ok(bytes)
    }

    #[cfg(feature = "string")]
    #[inline]
    fn read_string(&mut self) -> Result<String> {
        match String::from_utf8(self.read_u8_vec()?) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string())),
        }
    }
}

pub trait VariableWritable {
    fn write(&mut self, byte: u8) -> Result<usize>;

    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<usize> {
        self.write(if b { 1 } else { 0 })
    }

    #[cfg(feature = "bools")]
    bools::define_bools_write!();

    fn write_more(&mut self, bytes: &[u8]) -> Result<usize> {
        for i in 0..bytes.len() {
            self.write(bytes[i])?;
        }
        Ok(bytes.len())
    }

    #[cfg(feature = "raw")]
    raw::define_raw_write!();

    #[cfg(feature = "varint")]
    varint::define_varint_write!();

    #[cfg(feature = "signed")]
    signed::define_signed_write!();

    #[cfg(feature = "vec_u8")]
    #[inline]
    fn write_u8_vec(&mut self, message: &[u8]) -> Result<usize> {
        self.write_u128_varint(message.len() as u128)?;
        self.write_more(message)
    }

    #[cfg(feature = "string")]
    #[inline]
    fn write_string(&mut self, message: &str) -> Result<usize> {
        self.write_u8_vec(message.as_bytes())
    }
}

impl<R: Read> VariableReadable for R {
    #[inline]
    fn read(&mut self) -> Result<u8> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_more(&mut self, buf: &mut [u8]) -> Result<()> {
        self.read_exact(buf)
    }
}

impl<W: Write> VariableWritable for W {
    #[inline]
    fn write(&mut self, byte: u8) -> Result<usize> {
        self.write_all(&[byte])?;
        Ok(1)
    }

    #[inline]
    fn write_more(&mut self, bytes: &[u8]) -> Result<usize> {
        self.write_all(bytes)?;
        Ok(bytes.len())
    }
}
