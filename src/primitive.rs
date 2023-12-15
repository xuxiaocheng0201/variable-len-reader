use std::io::Result;
use crate::{Readable, Writable};

pub fn read_u8(source: &mut impl Readable) -> Result<u8> {
    source.read()
}
pub fn write_u8(target: &mut impl Writable, message: u8) -> Result<usize> {
    target.write(message)?;
    Ok(1)
}

pub fn read_bool(source: &mut impl Readable) -> Result<bool> {
    Ok(read_u8(source)? != 0)
}
pub fn write_bool(target: &mut impl Writable, message: bool) -> Result<usize> {
    write_u8(target, if message { 1 } else { 0 })
}

macro_rules! primitive_util {
    ($primitive: ident, $length: literal, $read: ident, $read_be: ident, $write: ident, $write_be: ident) => {
        pub fn $read(source: &mut impl Readable) -> Result<$primitive> {
            let mut bytes = [0; $length];
            source.read_more(&mut bytes)?;
            Ok($primitive::from_le_bytes(bytes))
        }
        pub fn $read_be(source: &mut impl Readable) -> Result<$primitive> {
            let mut bytes = [0; $length];
            source.read_more(&mut bytes)?;
            Ok($primitive::from_be_bytes(bytes))
        }
        pub fn $write(target: &mut impl Writable, message: $primitive) -> Result<usize> {
            let m = $primitive::to_le_bytes(message);
            target.write_more(&m)?;
            Ok(m.len())
        }
        pub fn $write_be(target: &mut impl Writable, message: $primitive) -> Result<usize> {
            let m = $primitive::to_be_bytes(message);
            target.write_more(&m)?;
            Ok(m.len())
        }
    };
}
primitive_util!(i8, 1, read_i8, read_i8_be, write_i8, write_i8_be);
primitive_util!(u16, 2, read_u16, read_u16_be, write_u16, write_u16_be);
primitive_util!(i16, 2, read_i16, read_i16_be, write_i16, write_i16_be);
primitive_util!(u32, 4, read_u32, read_u32_be, write_u32, write_u32_be);
primitive_util!(i32, 4, read_i32, read_i32_be, write_i32, write_i32_be);
primitive_util!(u64, 8, read_u64, read_u64_be, write_u64, write_u64_be);
primitive_util!(i64, 8, read_i64, read_i64_be, write_i64, write_i64_be);
primitive_util!(u128, 16, read_u128, read_u128_be, write_u128, write_u128_be);
primitive_util!(i128, 16, read_i128, read_i128_be, write_i128, write_i128_be);
