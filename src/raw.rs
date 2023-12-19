macro_rules! raw_read {
    ($primitive: ty, $read_le: ident, $read_be: ident) => {
        #[inline]
        fn $read_le(&mut self) -> Result<$primitive> {
            const SIZE: usize = std::mem::size_of::<$primitive>();
            let mut bytes = [0; SIZE];
            self.read_more(&mut bytes)?;
            Ok(<$primitive>::from_le_bytes(bytes))
        }
        #[inline]
        fn $read_be(&mut self) -> Result<$primitive> {
            const SIZE: usize = std::mem::size_of::<$primitive>();
            let mut bytes = [0; SIZE];
            self.read_more(&mut bytes)?;
            Ok(<$primitive>::from_be_bytes(bytes))
        }
    };
}
pub(crate) use raw_read;

macro_rules! define_raw_read {
    () => {
        #[inline]
        fn read_u8_ne(&mut self) -> Result<u8> {
            Ok(u8::from_ne_bytes([self.read()?]))
        }
        #[inline]
        fn read_i8_ne(&mut self) -> Result<i8> {
            Ok(i8::from_ne_bytes([self.read()?]))
        }
        raw::raw_read!(u16, read_u16_le, read_u16_be);
        raw::raw_read!(i16, read_i16_le, read_i16_be);
        raw::raw_read!(u32, read_u32_le, read_u32_be);
        raw::raw_read!(i32, read_i32_le, read_i32_be);
        raw::raw_read!(u64, read_u64_le, read_u64_be);
        raw::raw_read!(i64, read_i64_le, read_i64_be);
        raw::raw_read!(u128, read_u128_le, read_u128_be);
        raw::raw_read!(i128, read_i128_le, read_i128_be);
    };
}
pub(crate) use define_raw_read;

macro_rules! raw_write {
    ($primitive: ty, $write_le: ident, $write_be: ident) => {
        #[inline]
        fn $write_le(&mut self, num: $primitive) -> Result<usize> {
            self.write_more(&<$primitive>::to_le_bytes(num))
        }
        #[inline]
        fn $write_be(&mut self, num: $primitive) -> Result<usize> {
            self.write_more(&<$primitive>::to_be_bytes(num))
        }
    };
}
pub(crate) use raw_write;

macro_rules! define_raw_write {
    () => {
        #[inline]
        fn write_u8_ne(&mut self, num: u8) -> Result<usize> {
            self.write(num.to_ne_bytes()[0])
        }
        #[inline]
        fn write_i8_ne(&mut self, num: i8) -> Result<usize> {
            self.write(num.to_ne_bytes()[0])
        }
        raw::raw_write!(u16, write_u16_le, write_u16_be);
        raw::raw_write!(i16, write_i16_le, write_i16_be);
        raw::raw_write!(u32, write_u32_le, write_u32_be);
        raw::raw_write!(i32, write_i32_le, write_i32_be);
        raw::raw_write!(u64, write_u64_le, write_u64_be);
        raw::raw_write!(i64, write_i64_le, write_i64_be);
        raw::raw_write!(u128, write_u128_le, write_u128_be);
        raw::raw_write!(i128, write_i128_le, write_i128_be);
    };
}
pub(crate) use define_raw_write;

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use crate::{VariableReadable, VariableWritable};

    macro_rules! raw_test_0 {
        ($tester: ident, $primitive: ty, $reader: ident, $writer: ident, $num: expr) => {
            let p = $num;
            let mut cursor = Cursor::new(Vec::new());
            cursor.$writer(p).expect(&format!("Failed to write {} at {}.", p, stringify!($tester)));
            cursor.set_position(0);
            let q = cursor.$reader().expect(&format!("Failed to read {} at {}.", p, stringify!($tester)));
            assert_eq!(p, q, "Not same: {} != {} at {}. bytes: {:?}", p, q, stringify!($tester), cursor.into_inner());
        };
    }

    macro_rules! raw_test {
        ($tester: ident, $primitive: ty, $reader: ident, $writer: ident) => {
            #[test]
            fn $tester() {
                // Test first.
                raw_test_0!($tester, $primitive, $reader, $writer, <$primitive>::MIN);
                raw_test_0!($tester, $primitive, $reader, $writer, <$primitive>::MAX);
                raw_test_0!($tester, $primitive, $reader, $writer, 0);
                for p in <$primitive>::MIN..=<$primitive>::MAX {
                    raw_test_0!($tester, $primitive, $reader, $writer, p);
                }
            }
        };
    }

    raw_test!(u8_ne, u8, read_u8_ne, write_u8_ne);
    raw_test!(i8_ne, i8, read_i8_ne, write_i8_ne);
    raw_test!(u16_le, u16, read_u16_le, write_u16_le);
    raw_test!(u16_be, u16, read_u16_be, write_u16_be);
    raw_test!(i16_le, i16, read_i16_le, write_i16_le);
    raw_test!(i16_be, i16, read_i16_be, write_i16_be);
    // raw_test!(u32_le, u32, read_u32_le, write_u32_le);
    // raw_test!(u32_be, u32, read_u32_be, write_u32_be);
    // raw_test!(i32_le, i32, read_i32_le, write_i32_le);
    // raw_test!(i32_be, i32, read_i32_be, write_i32_be);
    // raw_test!(u64_le, u64, read_u64_le, write_u64_le);
    // raw_test!(u64_be, u64, read_u64_be, write_u64_be);
    // raw_test!(i64_le, i64, read_i64_le, write_i64_le);
    // raw_test!(i64_be, i64, read_i64_be, write_i64_be);
    // raw_test!(u128_le, u128, read_u128_le, write_u128_le);
    // raw_test!(u128_be, u128, read_u128_be, write_u128_be);
    // raw_test!(i128_le, i128, read_i128_le, write_i128_le);
    // raw_test!(i128_be, i128, read_i128_be, write_i128_be);
}
