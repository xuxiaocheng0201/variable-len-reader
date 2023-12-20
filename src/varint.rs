#[cfg(feature = "varint")]
macro_rules! varint_read {
    ($primitive: ty, $read_varint: ident, $inside_type: ty, $read_raw: ident) => {
        fn $read_varint(&mut self) -> Result<$primitive> {
            const SIZE: usize = std::mem::size_of::<$primitive>() << 3;// * 8
            const NUM_BITS: $inside_type = <$inside_type>::MAX >> 1;
            const SIGN_BIT: $inside_type = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$inside_type>::BITS - 1) as usize;
            let mut value = 0;
            let mut position = 0;
            loop {
                let current = self.$read_raw()?;
                value |= ((current & NUM_BITS) as $primitive) << position;
                if current & SIGN_BIT == 0 {
                    break;
                }
                position += POS_OFFSET;
                if position >= SIZE {
                    return Err(Error::new(ErrorKind::InvalidData, format!("Varint {} in stream is too long.", stringify!($read_varint))));
                }
            }
            Ok(value)
        }
    };
}
#[cfg(feature = "varint")]
pub(crate) use varint_read;

macro_rules! define_varint_read {
    () => {
        varint::varint_read!(u16, read_u16_varint, u8, read_u8_ne);
        varint::varint_read!(u32, read_u32_varint, u8, read_u8_ne);
        varint::varint_read!(u32, read_u32_varint_2_le, u16, read_u16_le);
        varint::varint_read!(u32, read_u32_varint_2_be, u16, read_u16_be);
        varint::varint_read!(u64, read_u64_varint, u8, read_u8_ne);
        varint::varint_read!(u64, read_u64_varint_2_le, u16, read_u16_le);
        varint::varint_read!(u64, read_u64_varint_2_be, u16, read_u16_be);
        varint::varint_read!(u64, read_u64_varint_4_le, u32, read_u32_le);
        varint::varint_read!(u64, read_u64_varint_4_be, u32, read_u32_be);
        varint::varint_read!(u128, read_u128_varint, u8, read_u8_ne);
        varint::varint_read!(u128, read_u128_varint_2_le, u16, read_u16_le);
        varint::varint_read!(u128, read_u128_varint_2_be, u16, read_u16_be);
        varint::varint_read!(u128, read_u128_varint_4_le, u32, read_u32_le);
        varint::varint_read!(u128, read_u128_varint_4_be, u32, read_u32_be);
        varint::varint_read!(u128, read_u128_varint_8_le, u64, read_u64_le);
        varint::varint_read!(u128, read_u128_varint_8_be, u64, read_u64_be);
    };
}
pub(crate) use define_varint_read;

#[cfg(feature = "varint")]
macro_rules! varint_write {
    ($primitive: ty, $write_varint: ident, $inside_type: ty, $write_raw: ident) => {
        fn $write_varint(&mut self, num: $primitive) -> Result<usize> {
            const NUM_BITS: $inside_type = <$inside_type>::MAX >> 1;
            const SIGN_BIT: $inside_type = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$inside_type>::BITS - 1) as usize;
            let mut size = 0;
            let mut value = num;
            while value >= SIGN_BIT as $primitive {
                size += self.$write_raw(((value & (NUM_BITS as $primitive)) as $inside_type) | SIGN_BIT)?;
                value >>= POS_OFFSET;
            }
            size += self.$write_raw((value & (NUM_BITS as $primitive)) as $inside_type)?;
            Ok(size)
        }
    };
}
#[cfg(feature = "varint")]
pub(crate) use varint_write;

macro_rules! define_varint_write {
    () => {
        varint::varint_write!(u16, write_u16_varint, u8, write_u8_ne);
        varint::varint_write!(u32, write_u32_varint, u8, write_u8_ne);
        varint::varint_write!(u32, write_u32_varint_2_le, u16, write_u16_le);
        varint::varint_write!(u32, write_u32_varint_2_be, u16, write_u16_be);
        varint::varint_write!(u64, write_u64_varint, u8, write_u8_ne);
        varint::varint_write!(u64, write_u64_varint_2_le, u16, write_u16_le);
        varint::varint_write!(u64, write_u64_varint_2_be, u16, write_u16_be);
        varint::varint_write!(u64, write_u64_varint_4_le, u32, write_u32_le);
        varint::varint_write!(u64, write_u64_varint_4_be, u32, write_u32_be);
        varint::varint_write!(u128, write_u128_varint, u8, write_u8_ne);
        varint::varint_write!(u128, write_u128_varint_2_le, u16, write_u16_le);
        varint::varint_write!(u128, write_u128_varint_2_be, u16, write_u16_be);
        varint::varint_write!(u128, write_u128_varint_4_le, u32, write_u32_le);
        varint::varint_write!(u128, write_u128_varint_4_be, u32, write_u32_be);
        varint::varint_write!(u128, write_u128_varint_8_le, u64, write_u64_le);
        varint::varint_write!(u128, write_u128_varint_8_be, u64, write_u64_be);
    };
}
pub(crate) use define_varint_write;

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use crate::{VariableReadable, VariableWritable};

    macro_rules! varint_test_0 {
        ($tester: ident, $primitive: ty, $reader: ident, $writer: ident, $num: expr) => {
            let p = $num;
            let mut cursor = Cursor::new(Vec::new());
            cursor.$writer(p).expect(&format!("Failed to write {} at {}.", p, stringify!($tester)));
            cursor.set_position(0);
            let q = cursor.$reader().expect(&format!("Failed to read {} at {}.", p, stringify!($tester)));
            assert_eq!(p, q, "Not same: {} != {} at {}. bytes: {:?}", p, q, stringify!($tester), cursor.into_inner());
        };
    }

    macro_rules! varint_test {
        ($tester: ident, $primitive: ty, $reader: ident, $writer: ident) => {
            #[test]
            fn $tester() {
                // Test first.
                varint_test_0!($tester, $primitive, $reader, $writer, <$primitive>::MIN);
                varint_test_0!($tester, $primitive, $reader, $writer, <$primitive>::MAX);
                varint_test_0!($tester, $primitive, $reader, $writer, 0);
                for p in <$primitive>::MIN..=<$primitive>::MAX {
                    varint_test_0!($tester, $primitive, $reader, $writer, p);
                }
            }
        };
    }

    varint_test!(u16_ne, u16, read_u16_varint, write_u16_varint);
    // varint_test!(u32_ne, u32, read_u32_varint, write_u32_varint);
    // varint_test!(u32_2_le, u32, read_u32_varint_2_le, write_u32_varint_2_le);
    // varint_test!(u32_2_be, u32, read_u32_varint_2_be, write_u32_varint_2_be);
    // varint_test!(u64_ne, u64, read_u64_varint, write_u64_varint);
    // varint_test!(u64_2_le, u64, read_u64_varint_2_le, write_u64_varint_2_le);
    // varint_test!(u64_2_be, u64, read_u64_varint_2_be, write_u64_varint_2_be);
    // varint_test!(u64_4_le, u64, read_u64_varint_4_le, write_u64_varint_4_le);
    // varint_test!(u64_4_be, u64, read_u64_varint_4_be, write_u64_varint_4_be);
    // varint_test!(u128_ne, u128, read_u128_varint, write_u128_varint);
    // varint_test!(u128_2_le, u128, read_u128_varint_2_le, write_u128_varint_2_le);
    // varint_test!(u128_2_be, u128, read_u128_varint_2_be, write_u128_varint_2_be);
    // varint_test!(u128_4_le, u128, read_u128_varint_4_le, write_u128_varint_4_le);
    // varint_test!(u128_4_be, u128, read_u128_varint_4_be, write_u128_varint_4_be);
    // varint_test!(u128_8_le, u128, read_u128_varint_8_le, write_u128_varint_8_le);
    // varint_test!(u128_8_be, u128, read_u128_varint_8_be, write_u128_varint_8_be);
}
