use std::io::{Error, ErrorKind, Read, Result};
use crate::bufs::ReadBuf;
use crate::VariableReadable;

#[cfg(feature = "raw")]
macro_rules! read_raw {
    ($primitive: ty, $func: ident, $from: ident) => {
        #[inline]
        fn $func(&mut self) -> std::io::Result<$primitive> {
            const SIZE: usize = std::mem::size_of::<$primitive>();
            let mut bytes = [0; SIZE];
            self.read_more(&mut $crate::bufs::ReadBuf::new(&mut bytes))?;
            Ok(<$primitive>::$from(bytes))
        }
    };
}
#[cfg(feature = "raw")]
macro_rules! define_read_raw {
    () => {
        read_raw!(u8, read_u8_raw_ne, from_ne_bytes);
        read_raw!(i8, read_i8_raw_ne, from_ne_bytes);

        read_raw!(u16, read_u16_raw_le, from_le_bytes);
        read_raw!(u16, read_u16_raw_be, from_be_bytes);
        read_raw!(i16, read_i16_raw_le, from_le_bytes);
        read_raw!(i16, read_i16_raw_be, from_be_bytes);

        read_raw!(u32, read_u32_raw_le, from_le_bytes);
        read_raw!(u32, read_u32_raw_be, from_be_bytes);
        read_raw!(i32, read_i32_raw_le, from_le_bytes);
        read_raw!(i32, read_i32_raw_be, from_be_bytes);

        read_raw!(u64, read_u64_raw_le, from_le_bytes);
        read_raw!(u64, read_u64_raw_be, from_be_bytes);
        read_raw!(i64, read_i64_raw_le, from_le_bytes);
        read_raw!(i64, read_i64_raw_be, from_be_bytes);

        read_raw!(u128, read_u128_raw_le, from_le_bytes);
        read_raw!(u128, read_u128_raw_be, from_be_bytes);
        read_raw!(i128, read_i128_raw_le, from_le_bytes);
        read_raw!(i128, read_i128_raw_be, from_be_bytes);
    }
}

pub trait VariableReader: VariableReadable {
    #[inline]
    fn read_bool(&mut self) -> Result<bool> {
        match self.read_single()? {
            0 => Ok(false),
            1 => Ok(true),
            i => Err(Error::new(ErrorKind::InvalidData, format!("Invalid boolean value: {}", i))),
        }
    }

    #[cfg(feature = "raw")]
    define_read_raw!();

    // #[cfg(feature = "bools")]
    // bools::define_bools_read!();
    //
    // #[cfg(feature = "varint")]
    // varint::define_varint_read!();
    //
    // #[cfg(feature = "signed")]
    // signed::define_signed_read!();
    //
    // #[cfg(feature = "vec_u8")]
    // #[inline]
    // fn read_u8_vec(&mut self) -> std::io::Result<Vec<u8>> {
    //     let length = self.read_u128_varint()? as usize;
    //     let mut bytes = vec![0; length];
    //     self.read_more(&mut bytes)?;
    //     Ok(bytes)
    // }
    //
    // #[cfg(feature = "string")]
    // #[inline]
    // fn read_string(&mut self) -> std::io::Result<String> {
    //     match String::from_utf8(self.read_u8_vec()?) {
    //         Ok(s) => Ok(s),
    //         Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string())),
    //     }
    // }
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
    fn read_more(&mut self, buf: &mut ReadBuf<'_>) -> Result<()> {
        while buf.left() > 0 {
            let filled = buf.filled();
            let n = R::read(self, &mut buf.buf_mut()[filled..])?;
            buf.advance(n);
            if n == 0 {
                return Err(Error::new(ErrorKind::UnexpectedEof, "unexpected end of file"))
            }
        }
        Ok(())
    }
}
