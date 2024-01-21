use std::io::{Error, ErrorKind, Result, Write};
use crate::bufs::WriteBuf;
use crate::VariableWritable;

macro_rules! write_raw {
    ($primitive: ty, $func: ident, $to: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> std::io::Result<usize> {
            self.write_more(&mut $crate::bufs::WriteBuf::new(&<$primitive>::$to(num)))
        }
    };
}
macro_rules! define_write_raw {
    () => {
        write_raw!(u8, write_u8_raw_ne, to_ne_bytes);
        write_raw!(i8, write_i8_raw_ne, to_ne_bytes);

        write_raw!(u16, write_u16_raw_le, to_le_bytes);
        write_raw!(u16, write_u16_raw_be, to_be_bytes);
        write_raw!(i16, write_i16_raw_le, to_le_bytes);
        write_raw!(i16, write_i16_raw_be, to_be_bytes);

        write_raw!(u32, write_u32_raw_le, to_le_bytes);
        write_raw!(u32, write_u32_raw_be, to_be_bytes);
        write_raw!(i32, write_i32_raw_le, to_le_bytes);
        write_raw!(i32, write_i32_raw_be, to_be_bytes);

        write_raw!(u64, write_u64_raw_le, to_le_bytes);
        write_raw!(u64, write_u64_raw_be, to_be_bytes);
        write_raw!(i64, write_i64_raw_le, to_le_bytes);
        write_raw!(i64, write_i64_raw_be, to_be_bytes);

        write_raw!(u128, write_u128_raw_le, to_le_bytes);
        write_raw!(u128, write_u128_raw_be, to_be_bytes);
        write_raw!(i128, write_i128_raw_le, to_le_bytes);
        write_raw!(i128, write_i128_raw_be, to_be_bytes);
    };
}

pub trait VariableWriter: VariableWritable {
    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<usize> {
        self.write_single(if b { 1 } else { 0 })
    }

    #[cfg(feature = "raw")]
    define_write_raw!();

    // #[cfg(feature = "bools")]
    // bools::define_bools_write!();
    //
    // #[cfg(feature = "varint")]
    // varint::define_varint_write!();
    //
    // #[cfg(feature = "signed")]
    // signed::define_signed_write!();
    //
    // #[cfg(feature = "vec_u8")]
    // #[inline]
    // fn write_u8_vec(&mut self, message: &[u8]) -> std::io::Result<usize> {
    //     self.write_u128_varint(message.len() as u128)?;
    //     self.write_more(message)
    // }
    //
    // #[cfg(feature = "string")]
    // #[inline]
    // fn write_string(&mut self, message: &str) -> std::io::Result<usize> {
    //     self.write_u8_vec(message.as_bytes())
    // }
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
    fn write_more(&mut self, buf: &mut WriteBuf<'_>) -> Result<usize> {
        while buf.left() > 0 {
            let read = buf.read();
            let n = W::write(self, &buf.buf()[read..])?;
            buf.skip(n);
            if n == 0 {
                return Err(Error::new(ErrorKind::WriteZero, "failed to write whole buffer"));
            }
        }
        Ok(buf.buf().len())
    }
}

