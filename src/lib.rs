use std::io::{Read, Result, Write};

#[cfg(feature = "primitive")]
pub mod primitive;
#[cfg(feature = "variable-len")]
pub mod variable_len;
#[cfg(feature = "str")]
pub mod str;
#[cfg(feature = "bytes")]
mod bytes;

pub trait Readable {
    fn read(&mut self) -> Result<u8>;

    fn read_more(&mut self, buf: &mut [u8]) -> Result<()> {
        for i in 0..buf.len() {
            buf[i] = self.read()?;
        }
        Ok(())
    }
}

pub trait Writable {
    fn write(&mut self, byte: u8) -> Result<()>;

    fn write_more(&mut self, bytes: &[u8]) -> Result<()> {
        for i in 0..bytes.len() {
            self.write(bytes[i])?;
        }
        Ok(())
    }
}

impl Readable for dyn Read {
    fn read(&mut self) -> Result<u8> {
        let mut bytes = [0; 1];
        self.read_exact(&mut bytes)?;
        Ok(bytes[0])
    }

    fn read_more(&mut self, buf: &mut [u8]) -> Result<()> {
        self.read_exact(buf)
    }
}

impl Writable for dyn Write {
    fn write(&mut self, byte: u8) -> Result<()> {
        self.write_all(&[byte])
    }

    fn write_more(&mut self, bytes: &[u8]) -> Result<()> {
        self.write_all(bytes)
    }
}
