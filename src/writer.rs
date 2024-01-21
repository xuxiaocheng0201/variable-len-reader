use std::io::{Error, ErrorKind, Result, Write};
use crate::util::bufs::WriteBuf;
use crate::VariableWritable;

#[cfg(feature = "raw")]
macro_rules! write_raw {
    ($primitive: ty, $func: ident, $to: ident) => {
        #[inline]
        fn $func(&mut self, num: $primitive) -> std::io::Result<usize> {
            self.write_more(&mut $crate::util::bufs::WriteBuf::new(&<$primitive>::$to(num)))
        }
    };
}
#[cfg(feature = "raw")]
macro_rules! define_write_raw {
    () => {
        write_raw!(u8, write_u8_raw, to_ne_bytes);
        write_raw!(i8, write_i8_raw, to_ne_bytes);

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

#[cfg(feature = "bools")]
macro_rules! write_bools {
    ($func: ident, $n: literal) => {
        #[inline]
        fn $func(&mut self, bools: [bool; $n]) -> Result<usize> {
            let mut b = 0;
            for i in 0..$n {
                if bools[i] {
                    b |= 1 << i;
                }
            }
            self.write_single(b)
        }
    };
}
#[cfg(feature = "bools")]
macro_rules! define_write_bools {
    () => {
        write_bools!(write_bools_2, 2);
        write_bools!(write_bools_3, 3);
        write_bools!(write_bools_4, 4);
        write_bools!(write_bools_5, 5);
        write_bools!(write_bools_6, 6);
        write_bools!(write_bools_7, 7);
        write_bools!(write_bools_8, 8);
    };
}

#[cfg(feature = "varint")]
macro_rules! write_varint {
    ($primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        fn $func(&mut self, num: $primitive) -> std::io::Result<usize> {
            const NUM_BITS: $internal = <$internal>::MAX >> 1;
            const SIGN_BIT: $internal = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
            let mut size = 0;
            let mut value = num;
            while value >= SIGN_BIT as $primitive {
                size += self.$write_internal(((value & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT)?;
                value >>= POS_OFFSET;
            }
            size += self.$write_internal((value & (NUM_BITS as $primitive)) as $internal)?;
            Ok(size)
        }
    };
}
#[cfg(feature = "varint")]
macro_rules! define_write_varint {
    () => {
        #[cfg(feature = "long_varint")]
        write_varint!(u8, write_u8_varint, u8, write_u8_raw);

        write_varint!(u16, write_u16_varint, u8, write_u8_raw);
        #[cfg(feature = "long_varint")]
        write_varint!(u16, write_u16_varint_2_le, u16, write_u16_raw_le);
        #[cfg(feature = "long_varint")]
        write_varint!(u16, write_u16_varint_2_be, u16, write_u16_raw_be);

        write_varint!(u32, write_u32_varint, u8, write_u8_raw);
        #[cfg(feature = "long_varint")]
        write_varint!(u32, write_u32_varint_2_le, u16, write_u16_raw_le);
        #[cfg(feature = "long_varint")]
        write_varint!(u32, write_u32_varint_2_be, u16, write_u16_raw_be);
        #[cfg(feature = "long_varint")]
        write_varint!(u32, write_u32_varint_4_le, u32, write_u32_raw_le);
        #[cfg(feature = "long_varint")]
        write_varint!(u32, write_u32_varint_4_be, u32, write_u32_raw_be);

        write_varint!(u64, write_u64_varint, u8, write_u8_raw);
        #[cfg(feature = "long_varint")]
        write_varint!(u64, write_u64_varint_2_le, u16, write_u16_raw_le);
        #[cfg(feature = "long_varint")]
        write_varint!(u64, write_u64_varint_2_be, u16, write_u16_raw_be);
        #[cfg(feature = "long_varint")]
        write_varint!(u64, write_u64_varint_4_le, u32, write_u32_raw_le);
        #[cfg(feature = "long_varint")]
        write_varint!(u64, write_u64_varint_4_be, u32, write_u32_raw_be);
        #[cfg(feature = "long_varint")]
        write_varint!(u64, write_u64_varint_8_le, u64, write_u64_raw_le);
        #[cfg(feature = "long_varint")]
        write_varint!(u64, write_u64_varint_8_be, u64, write_u64_raw_be);

        write_varint!(u128, write_u128_varint, u8, write_u8_raw);
        #[cfg(feature = "long_varint")]
        write_varint!(u128, write_u128_varint_2_le, u16, write_u16_raw_le);
        #[cfg(feature = "long_varint")]
        write_varint!(u128, write_u128_varint_2_be, u16, write_u16_raw_be);
        #[cfg(feature = "long_varint")]
        write_varint!(u128, write_u128_varint_4_le, u32, write_u32_raw_le);
        #[cfg(feature = "long_varint")]
        write_varint!(u128, write_u128_varint_4_be, u32, write_u32_raw_be);
        #[cfg(feature = "long_varint")]
        write_varint!(u128, write_u128_varint_8_le, u64, write_u64_raw_le);
        #[cfg(feature = "long_varint")]
        write_varint!(u128, write_u128_varint_8_be, u64, write_u64_raw_be);
        #[cfg(feature = "long_varint")]
        write_varint!(u128, write_u128_varint_16_le, u128, write_u128_raw_le);
        #[cfg(feature = "long_varint")]
        write_varint!(u128, write_u128_varint_16_be, u128, write_u128_raw_be);
    };
}

#[cfg(feature = "signed")]
macro_rules! write_signed {
    ($primitive: ty, $func: ident, $write_internal: ident) => {
        fn $func(&mut self, num: $primitive) -> std::io::Result<usize> {
            use $crate::util::zigzag::Zigzag;
            self.$write_internal(num.zigzag())
        }
    };
}
#[cfg(feature = "signed")]
macro_rules! define_write_signed {
    () => {
        #[cfg(feature = "long_signed")]
        write_signed!(i8, write_i8_varint, write_u8_varint);

        write_signed!(i16, write_i16_varint, write_u16_varint);
        #[cfg(feature = "long_signed")]
        write_signed!(i16, write_i16_varint_2_le, write_u16_varint_2_le);
        #[cfg(feature = "long_signed")]
        write_signed!(i16, write_i16_varint_2_be, write_u16_varint_2_be);

        write_signed!(i32, write_i32_varint, write_u32_varint);
        #[cfg(feature = "long_signed")]
        write_signed!(i32, write_i32_varint_2_le, write_u32_varint_2_le);
        #[cfg(feature = "long_signed")]
        write_signed!(i32, write_i32_varint_2_be, write_u32_varint_2_be);
        #[cfg(feature = "long_signed")]
        write_signed!(i32, write_i32_varint_4_le, write_u32_varint_4_le);
        #[cfg(feature = "long_signed")]
        write_signed!(i32, write_i32_varint_4_be, write_u32_varint_4_be);

        write_signed!(i64, write_i64_varint, write_u64_varint);
        #[cfg(feature = "long_signed")]
        write_signed!(i64, write_i64_varint_2_le, write_u64_varint_2_le);
        #[cfg(feature = "long_signed")]
        write_signed!(i64, write_i64_varint_2_be, write_u64_varint_2_be);
        #[cfg(feature = "long_signed")]
        write_signed!(i64, write_i64_varint_4_le, write_u64_varint_4_le);
        #[cfg(feature = "long_signed")]
        write_signed!(i64, write_i64_varint_4_be, write_u64_varint_4_be);
        #[cfg(feature = "long_signed")]
        write_signed!(i64, write_i64_varint_8_le, write_u64_varint_8_le);
        #[cfg(feature = "long_signed")]
        write_signed!(i64, write_i64_varint_8_be, write_u64_varint_8_be);

        write_signed!(i128, write_i128_varint, write_u128_varint);
        #[cfg(feature = "long_signed")]
        write_signed!(i128, write_i128_varint_2_le, write_u128_varint_2_le);
        #[cfg(feature = "long_signed")]
        write_signed!(i128, write_i128_varint_2_be, write_u128_varint_2_be);
        #[cfg(feature = "long_signed")]
        write_signed!(i128, write_i128_varint_4_le, write_u128_varint_4_le);
        #[cfg(feature = "long_signed")]
        write_signed!(i128, write_i128_varint_4_be, write_u128_varint_4_be);
        #[cfg(feature = "long_signed")]
        write_signed!(i128, write_i128_varint_8_le, write_u128_varint_8_le);
        #[cfg(feature = "long_signed")]
        write_signed!(i128, write_i128_varint_8_be, write_u128_varint_8_be);
        #[cfg(feature = "long_signed")]
        write_signed!(i128, write_i128_varint_16_le, write_u128_varint_16_le);
        #[cfg(feature = "long_signed")]
        write_signed!(i128, write_i128_varint_16_be, write_u128_varint_16_be);
    };
}

pub trait VariableWriter: VariableWritable {
    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<usize> {
        self.write_single(if b { 1 } else { 0 })
    }

    #[cfg(feature = "raw")]
    define_write_raw!();

    #[cfg(feature = "bools")]
    define_write_bools!();

    #[cfg(feature = "varint")]
    define_write_varint!();

    #[cfg(feature = "signed")]
    define_write_signed!();

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

