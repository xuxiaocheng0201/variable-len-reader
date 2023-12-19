use std::io::{Error, ErrorKind, Read, Result, Write};

pub trait VariableReadable: varint_rs::VarintReader {
    fn read_more(&mut self, buf: &mut [u8]) -> Result<()> {
        for i in 0..buf.len() {
            buf[i] = self.read()?;
        }
        Ok(())
    }

    #[cfg(feature = "vec_u8")]
    fn read_u8_vec(&mut self) -> Result<Vec<u8>> {
        let length = self.read_u128_varint()? as usize;
        let mut bytes = vec![0; length];
        self.read_more(&mut bytes)?;
        Ok(bytes)
    }

    #[cfg(feature = "string")]
    fn read_string(&mut self) -> Result<String> {
        match String::from_utf8(self.read_u8_vec()?) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string())),
        }
    }
}

pub trait VariableWritable: varint_rs::VarintWriter {
    fn write_more(&mut self, bytes: &[u8]) -> Result<()> {
        for i in 0..bytes.len() {
            self.write(bytes[i])?;
        }
        Ok(())
    }

    #[cfg(feature = "vec_u8")]
    fn write_u8_vec(&mut self, message: &[u8]) -> Result<()> {
        self.write_u128_varint(message.len() as u128)?;
        self.write_more(message)
    }

    #[cfg(feature = "string")]
    fn write_string(&mut self, message: &str) -> Result<()> {
        self.write_u8_vec(message.as_bytes())
    }
}

impl<R: Read> VariableReadable for R {
    #[inline]
    fn read_more(&mut self, buf: &mut [u8]) -> Result<()> {
        self.read_exact(buf)
    }
}

impl<W: Write> VariableWritable for W {
    #[inline]
    fn write_more(&mut self, bytes: &[u8]) -> Result<()> {
        self.write_all(bytes)
    }
}
