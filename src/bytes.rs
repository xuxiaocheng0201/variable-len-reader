use std::io::{Error, ErrorKind, Result};
use bytes::{Buf, BufMut};
use crate::{Readable, Writable};

impl Readable for dyn Buf {
    fn read(&mut self) -> Result<u8> {
        if self.has_remaining() {
            Ok(self.get_u8())
        } else {
            Err(Error::new(ErrorKind::UnexpectedEof, "No more bytes."))
        }
    }

    fn read_more(&mut self, buf: &mut [u8]) -> Result<()> {
        if self.remaining() >= buf.len() {
            Ok(self.copy_to_slice(buf))
        } else {
            Err(Error::new(ErrorKind::UnexpectedEof, "Not enough bytes."))
        }
    }
}

impl Writable for dyn BufMut {
    fn write(&mut self, byte: u8) -> Result<()> {
        if self.has_remaining_mut() {
            Ok(self.put_u8(byte))
        } else {
            Err(Error::new(ErrorKind::UnexpectedEof, "No more space."))
        }
    }

    fn write_more(&mut self, bytes: &[u8]) -> Result<()> {
        if self.remaining_mut() >= bytes.len() {
            Ok(self.put_slice(bytes))
        } else {
            Err(Error::new(ErrorKind::UnexpectedEof, "Not enough space."))
        }
    }
}
