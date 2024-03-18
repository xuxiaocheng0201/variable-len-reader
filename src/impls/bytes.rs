use alloc::format;
use alloc::string::String;
use core::fmt::{Display, Formatter};
use bytes::{Buf, BufMut};
use crate::synchronous::{VariableReadable, VariableWritable};
use crate::synchronous::reader::VariableReader;
use crate::synchronous::writer::VariableWriter;

#[derive(Debug)]
pub enum ReadError {
    UnexpectedEof,
    InvalidData(String),
}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ReadError::UnexpectedEof => write!(f, "unexpected eof"),
            ReadError::InvalidData(e) => write!(f, "invalid data: {}", e),
        }
    }
}

#[cfg(feature = "std")] // FIXME: Use core::error::Error when it's stable.
impl std::error::Error for ReadError { }

#[cfg(feature = "std")]
#[cfg_attr(docsrs, cfg(feature = "std"))]
impl From<ReadError> for std::io::Error {
    fn from(value: ReadError) -> Self {
        match value {
            ReadError::UnexpectedEof => std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "unexpected eof"),
            ReadError::InvalidData(e) => std::io::Error::new(std::io::ErrorKind::InvalidData, e),
        }
    }
}

#[derive(Debug)]
pub enum WriteError {
    WriteZero,
}

impl Display for WriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            WriteError::WriteZero => write!(f, "not enough space"),
        }
    }
}

#[cfg(feature = "std")] // FIXME: Use core::error::Error when it's stable.
impl std::error::Error for WriteError { }

#[cfg(feature = "std")]
#[cfg_attr(docsrs, cfg(feature = "std"))]
impl From<WriteError> for std::io::Error {
    fn from(value: WriteError) -> Self {
        match value {
            WriteError::WriteZero => std::io::Error::new(std::io::ErrorKind::WriteZero, "not enough space"),
        }
    }
}


impl<R: Buf> VariableReadable for R {
    type Error = ReadError;

    fn read_single(&mut self) -> Result<u8, Self::Error> {
        if self.remaining() < 1 {
            Err(ReadError::UnexpectedEof)
        } else {
            Ok(self.get_u8())
        }
    }

    fn read_more(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        if self.remaining() < buf.len() {
            Err(ReadError::UnexpectedEof)
        } else {
            Ok(self.copy_to_slice(buf))
        }
    }
}

impl<R: Buf> VariableReader for R {
    #[inline]
    fn read_bool_error(func_name: &'static str, byte: u8) -> Self::Error {
        ReadError::InvalidData(format!("Invalid bool. value {} at {}.", byte, func_name))
    }

    #[cfg(feature = "sync_bools")]
    #[inline]
    fn read_bools_error(func_name: &'static str, byte: u8) -> Self::Error {
        ReadError::InvalidData(format!("Invalid bools. value {} at {}.", byte, func_name))
    }

    #[cfg(feature = "sync_varint")]
    #[inline]
    fn read_varint_error(func_name: &'static str, value: u128) -> Self::Error {
        ReadError::InvalidData(format!("Too long varint value. {} at {}.", value, func_name))
    }

    #[cfg(feature = "sync_string")]
    #[inline]
    fn read_string_error(_func_name: &'static str, error: alloc::string::FromUtf8Error) -> Self::Error {
        ReadError::InvalidData(format!("{:?}", error))
    }
}

impl<W: BufMut> VariableWritable for W {
    type Error = WriteError;

    fn write_single(&mut self, byte: u8) -> Result<(), Self::Error> {
        if self.remaining_mut() < 1 {
            Err(WriteError::WriteZero)
        } else {
            Ok(self.put_u8(byte))
        }
    }

    fn write_more(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        if self.remaining_mut() < buf.len() {
            Err(WriteError::WriteZero)
        } else {
            Ok(self.put_slice(buf))
        }
    }
}

impl<W: BufMut> VariableWriter for W { }
