use std::io::{Error, ErrorKind, Read, Result, Write};
use crate::primitive::*;

macro_rules! variable_len_util {
    ($primitive: ident, $length: literal, $read_variable: ident, $write_variable: ident, $cause: literal,
    $read: ident, $write: ident, $inside_type: ident, $num_bits: literal, $next_bit: literal, $offset_position: literal) => {
        pub fn $read_variable(source: &mut impl Read) -> Result<$primitive> {
            let mut value = 0;
            let mut position = 0;
            loop {
                let current = $read(source)?;
                value |= ((current & $num_bits) as $primitive) << position;
                if current & $next_bit == 0 {
                    break;
                }
                position += $offset_position;
                if position >= $length {
                    return Err(Error::new(ErrorKind::InvalidData, format!("Variable {} in stream is too long.", $cause)));
                }
            }
            Ok(value)
        }
        pub fn $write_variable(target: &mut impl Write, message: $primitive) -> Result<usize> {
            let mut size = 0;
            let mut value = message;
            while value >> $offset_position > 0 {
                size += $write(target, ((value & $num_bits) as $inside_type) | $next_bit)?;
                value >>= $offset_position;
            }
            size += $write(target, (value & $num_bits) as $inside_type)?;
            Ok(size)
        }
    };
}
macro_rules! variable_len_utils {
    ($primitive: ident, $length: literal, $read_variable: ident, $write_variable: ident, $cause: literal, u8) => {
        variable_len_util!($primitive, $length, $read_variable, $write_variable, $cause, read_u8, write_u8, u8, 0x7f, 0x80, 7);
    };
    ($primitive: ident, $length: literal, $read_variable: ident, $write_variable: ident, $cause: literal, u16) => {
        variable_len_util!($primitive, $length, $read_variable, $write_variable, $cause, read_u16, write_u16, u16, 0x7fff, 0x8000, 15);
    };
    ($primitive: ident, $length: literal, $read_variable: ident, $write_variable: ident, $cause: literal, u16_be) => {
        variable_len_util!($primitive, $length, $read_variable, $write_variable, $cause, read_u16_be, write_u16_be, u16, 0x7fff, 0x8000, 15);
    };
    ($primitive: ident, $length: literal, $read_variable: ident, $write_variable: ident, $cause: literal, u32) => {
        variable_len_util!($primitive, $length, $read_variable, $write_variable, $cause, read_u32, write_u32, u32, 0x7fffffff, 0x80000000, 31);
    };
    ($primitive: ident, $length: literal, $read_variable: ident, $write_variable: ident, $cause: literal, u32_be) => {
        variable_len_util!($primitive, $length, $read_variable, $write_variable, $cause, read_u32_be, write_u32_be, u32, 0x7fffffff, 0x80000000, 31);
    };
    ($primitive: ident, $length: literal, $read_variable: ident, $write_variable: ident, $cause: literal, u64) => {
        variable_len_util!($primitive, $length, $read_variable, $write_variable, $cause, read_u64, write_u64, u64, 0x7fffffffffffffff, 0x8000000000000000, 63);
    };
    ($primitive: ident, $length: literal, $read_variable: ident, $write_variable: ident, $cause: literal, u64_be) => {
        variable_len_util!($primitive, $length, $read_variable, $write_variable, $cause, read_u64_be, write_u64_be, u64, 0x7fffffffffffffff, 0x8000000000000000, 63);
    };
}
variable_len_utils!(u16, 16, read_variable_u16, write_variable_u16, "u16", u8);
variable_len_utils!(u32, 32, read_variable_u32, write_variable_u32, "u32", u8);
variable_len_utils!(u32, 32, read_variable2_u32, write_variable2_u32, "u32(2)", u16);
variable_len_utils!(u32, 32, read_variable2_u32_be, write_variable2_u32_be, "u32(2be)", u16_be);
variable_len_utils!(u64, 64, read_variable_u64, write_variable_u64, "u64", u8);
variable_len_utils!(u64, 64, read_variable2_u64, write_variable2_u64, "u64(2)", u16);
variable_len_utils!(u64, 64, read_variable2_u64_be, write_variable2_u64_be, "u64(2be)", u16_be);
variable_len_utils!(u64, 64, read_variable4_u64, write_variable4_u64, "u64(4)", u32);
variable_len_utils!(u64, 64, read_variable4_u64_be, write_variable4_u64_be, "u64(4be)", u32_be);
variable_len_utils!(u128, 128, read_variable_u128, write_variable_u128, "u128", u8);
variable_len_utils!(u128, 128, read_variable2_u128, write_variable2_u128, "u128(2)", u16);
variable_len_utils!(u128, 128, read_variable2_u128_be, write_variable2_u128_be, "u128(2be)", u16_be);
variable_len_utils!(u128, 128, read_variable4_u128, write_variable4_u128, "u128(4)", u32);
variable_len_utils!(u128, 128, read_variable4_u128_be, write_variable4_u128_be, "u128(4be)", u32_be);
variable_len_utils!(u128, 128, read_variable8_u128, write_variable8_u128, "u128(8)", u64);
variable_len_utils!(u128, 128, read_variable8_u128_be, write_variable8_u128_be, "u128(8be)", u64_be);
