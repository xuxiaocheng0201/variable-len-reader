use std::io::{Error, ErrorKind, Read, Result};
use crate::util::bufs::ReadBuf;
use crate::VariableReadable;

#[cfg(feature = "raw")]
macro_rules! read_raw {
    ($primitive: ty, $func: ident, $from: ident) => {
        #[inline]
        fn $func(&mut self) -> std::io::Result<$primitive> {
            const SIZE: usize = std::mem::size_of::<$primitive>();
            let mut bytes = [0; SIZE];
            self.read_more(&mut $crate::util::bufs::ReadBuf::new(&mut bytes))?;
            Ok(<$primitive>::$from(bytes))
        }
    };
}
#[cfg(feature = "raw")]
macro_rules! define_read_raw {
    () => {
        read_raw!(u8, read_u8_raw, from_ne_bytes);
        read_raw!(i8, read_i8_raw, from_ne_bytes);

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

        #[cfg(feature = "raw_size")]
        #[inline]
        fn read_usize_raw_le(&mut self) -> Result<usize> {
            self.read_u128_raw_le().map(|v| v as usize)
        }
        #[cfg(feature = "raw_size")]
        #[inline]
        fn read_usize_raw_be(&mut self) -> Result<usize> {
            self.read_u128_raw_be().map(|v| v as usize)
        }
        #[cfg(feature = "raw_size")]
        #[inline]
        fn read_isize_raw_le(&mut self) -> Result<isize> {
            self.read_i128_raw_le().map(|v| v as isize)
        }
        #[cfg(feature = "raw_size")]
        #[inline]
        fn read_isize_raw_be(&mut self) -> Result<isize> {
            self.read_i128_raw_be().map(|v| v as isize)
        }
    }
}

#[cfg(feature = "bools")]
macro_rules! read_bools {
    ($func: ident, $n: literal) => {
        #[inline]
        fn $func(&mut self) -> Result<[bool; $n]> {
            const MAX: u8 = ((1 << ($n - 1)) - 1 << 1) + 1; // (1 << $n) - 1 (Prevent `this arithmetic operation will overflow`)
            let b = self.read_single()?;
            if b > MAX {
                return Err(Error::new(ErrorKind::InvalidData, format!("Invalid bools at {}.", stringify!($func))));
            }
            let mut bools = [false; $n];
            for i in 0..$n {
                bools[i] = b & (1 << i) != 0;
            }
            Ok(bools)
        }
    };
}
#[cfg(feature = "bools")]
macro_rules! define_read_bools {
    () => {
        read_bools!(read_bools_2, 2);
        read_bools!(read_bools_3, 3);
        read_bools!(read_bools_4, 4);
        read_bools!(read_bools_5, 5);
        read_bools!(read_bools_6, 6);
        read_bools!(read_bools_7, 7);
        read_bools!(read_bools_8, 8);
    };
}

#[cfg(feature = "varint")]
macro_rules! read_varint {
    ($primitive: ty, $func: ident, $internal: ty, $read_internal: ident) => {
        fn $func(&mut self) -> std::io::Result<$primitive> {
            const SIZE: usize = std::mem::size_of::<$primitive>() << 3; // * 8
            const NUM_BITS: $internal = <$internal>::MAX >> 1;
            const SIGN_BIT: $internal = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
            let mut value = 0;
            let mut position = 0;
            loop {
                let current = self.$read_internal()?;
                value |= ((current & NUM_BITS) as $primitive) << position;
                if current & SIGN_BIT == 0 {
                    break;
                }
                position += POS_OFFSET;
                if position >= SIZE {
                    return Err(Error::new(ErrorKind::InvalidData, format!("Varint {} in stream is too long.", stringify!($func))));
                }
            }
            Ok(value)
        }
    };
}
#[cfg(feature = "varint")]
macro_rules! define_read_varint {
    () => {
        #[cfg(feature = "long_varint")]
        read_varint!(u8, read_u8_varint, u8, read_u8_raw);

        read_varint!(u16, read_u16_varint, u8, read_u8_raw);
        #[cfg(feature = "long_varint")]
        read_varint!(u16, read_u16_varint_2_le, u16, read_u16_raw_le);
        #[cfg(feature = "long_varint")]
        read_varint!(u16, read_u16_varint_2_be, u16, read_u16_raw_be);

        read_varint!(u32, read_u32_varint, u8, read_u8_raw);
        #[cfg(feature = "long_varint")]
        read_varint!(u32, read_u32_varint_2_le, u16, read_u16_raw_le);
        #[cfg(feature = "long_varint")]
        read_varint!(u32, read_u32_varint_2_be, u16, read_u16_raw_be);
        #[cfg(feature = "long_varint")]
        read_varint!(u32, read_u32_varint_4_le, u32, read_u32_raw_le);
        #[cfg(feature = "long_varint")]
        read_varint!(u32, read_u32_varint_4_be, u32, read_u32_raw_be);

        read_varint!(u64, read_u64_varint, u8, read_u8_raw);
        #[cfg(feature = "long_varint")]
        read_varint!(u64, read_u64_varint_2_le, u16, read_u16_raw_le);
        #[cfg(feature = "long_varint")]
        read_varint!(u64, read_u64_varint_2_be, u16, read_u16_raw_be);
        #[cfg(feature = "long_varint")]
        read_varint!(u64, read_u64_varint_4_le, u32, read_u32_raw_le);
        #[cfg(feature = "long_varint")]
        read_varint!(u64, read_u64_varint_4_be, u32, read_u32_raw_be);
        #[cfg(feature = "long_varint")]
        read_varint!(u64, read_u64_varint_8_le, u64, read_u64_raw_le);
        #[cfg(feature = "long_varint")]
        read_varint!(u64, read_u64_varint_8_be, u64, read_u64_raw_be);

        read_varint!(u128, read_u128_varint, u8, read_u8_raw);
        #[cfg(feature = "long_varint")]
        read_varint!(u128, read_u128_varint_2_le, u16, read_u16_raw_le);
        #[cfg(feature = "long_varint")]
        read_varint!(u128, read_u128_varint_2_be, u16, read_u16_raw_be);
        #[cfg(feature = "long_varint")]
        read_varint!(u128, read_u128_varint_4_le, u32, read_u32_raw_le);
        #[cfg(feature = "long_varint")]
        read_varint!(u128, read_u128_varint_4_be, u32, read_u32_raw_be);
        #[cfg(feature = "long_varint")]
        read_varint!(u128, read_u128_varint_8_le, u64, read_u64_raw_le);
        #[cfg(feature = "long_varint")]
        read_varint!(u128, read_u128_varint_8_be, u64, read_u64_raw_be);
        #[cfg(feature = "long_varint")]
        read_varint!(u128, read_u128_varint_16_le, u128, read_u128_raw_le);
        #[cfg(feature = "long_varint")]
        read_varint!(u128, read_u128_varint_16_be, u128, read_u128_raw_be);

        #[cfg(feature = "varint_size")]
        #[inline]
        fn read_usize_varint(&mut self) -> Result<usize> {
            self.read_u128_varint().map(|v| v as usize)
        }
    };
}

#[cfg(feature = "signed")]
macro_rules! read_signed {
    ($primitive: ty, $func: ident, $read_internal: ident) => {
        fn $func(&mut self) -> Result<$primitive> {
            use $crate::util::zigzag::Zigzag;
            self.$read_internal().map(|v| v.zigzag())
        }
    };
}
#[cfg(feature = "signed")]
macro_rules! define_read_signed {
    () => {
        #[cfg(feature = "long_signed")]
        read_signed!(i8, read_i8_varint, read_u8_varint);

        read_signed!(i16, read_i16_varint, read_u16_varint);
        #[cfg(feature = "long_signed")]
        read_signed!(i16, read_i16_varint_2_le, read_u16_varint_2_le);
        #[cfg(feature = "long_signed")]
        read_signed!(i16, read_i16_varint_2_be, read_u16_varint_2_be);

        read_signed!(i32, read_i32_varint, read_u32_varint);
        #[cfg(feature = "long_signed")]
        read_signed!(i32, read_i32_varint_2_le, read_u32_varint_2_le);
        #[cfg(feature = "long_signed")]
        read_signed!(i32, read_i32_varint_2_be, read_u32_varint_2_be);
        #[cfg(feature = "long_signed")]
        read_signed!(i32, read_i32_varint_4_le, read_u32_varint_4_le);
        #[cfg(feature = "long_signed")]
        read_signed!(i32, read_i32_varint_4_be, read_u32_varint_4_be);

        read_signed!(i64, read_i64_varint, read_u64_varint);
        #[cfg(feature = "long_signed")]
        read_signed!(i64, read_i64_varint_2_le, read_u64_varint_2_le);
        #[cfg(feature = "long_signed")]
        read_signed!(i64, read_i64_varint_2_be, read_u64_varint_2_be);
        #[cfg(feature = "long_signed")]
        read_signed!(i64, read_i64_varint_4_le, read_u64_varint_4_le);
        #[cfg(feature = "long_signed")]
        read_signed!(i64, read_i64_varint_4_be, read_u64_varint_4_be);
        #[cfg(feature = "long_signed")]
        read_signed!(i64, read_i64_varint_8_le, read_u64_varint_8_le);
        #[cfg(feature = "long_signed")]
        read_signed!(i64, read_i64_varint_8_be, read_u64_varint_8_be);

        read_signed!(i128, read_i128_varint, read_u128_varint);
        #[cfg(feature = "long_signed")]
        read_signed!(i128, read_i128_varint_2_le, read_u128_varint_2_le);
        #[cfg(feature = "long_signed")]
        read_signed!(i128, read_i128_varint_2_be, read_u128_varint_2_be);
        #[cfg(feature = "long_signed")]
        read_signed!(i128, read_i128_varint_4_le, read_u128_varint_4_le);
        #[cfg(feature = "long_signed")]
        read_signed!(i128, read_i128_varint_4_be, read_u128_varint_4_be);
        #[cfg(feature = "long_signed")]
        read_signed!(i128, read_i128_varint_8_le, read_u128_varint_8_le);
        #[cfg(feature = "long_signed")]
        read_signed!(i128, read_i128_varint_8_be, read_u128_varint_8_be);
        #[cfg(feature = "long_signed")]
        read_signed!(i128, read_i128_varint_16_le, read_u128_varint_16_le);
        #[cfg(feature = "long_signed")]
        read_signed!(i128, read_i128_varint_16_be, read_u128_varint_16_be);

        #[cfg(feature = "varint_size")]
        #[inline]
        fn read_isize_varint(&mut self) -> Result<isize> {
            self.read_i128_varint().map(|v| v as isize)
        }
    };
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

    #[cfg(feature = "bools")]
    define_read_bools!();

    #[cfg(feature = "varint")]
    define_read_varint!();

    #[cfg(feature = "signed")]
    define_read_signed!();

    #[cfg(feature = "vec_u8")]
    #[inline]
    fn read_u8_vec(&mut self) -> Result<Vec<u8>> {
        let length = self.read_usize_varint()?;
        let mut bytes = vec![0; length];
        self.read_more(&mut ReadBuf::new(&mut bytes))?;
        Ok(bytes)
    }

    #[cfg(feature = "string")]
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
