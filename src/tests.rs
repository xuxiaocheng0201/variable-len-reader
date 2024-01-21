mod raw {
    use crate::{VariableReader, VariableWriter};

    macro_rules! raw_test {
        ($tester: ident, $primitive: ty, $reader: ident, $writer: ident) => {
            #[test]
            fn $tester() {
                let mut cursor = std::io::Cursor::new(Vec::new());
                for p in <$primitive>::MIN..=<$primitive>::MAX {
                    cursor.set_position(0);
                    cursor.$writer(p).expect(&format!("Failed to write {} at {}.", p, stringify!($tester)));
                    cursor.set_position(0);
                    let q = cursor.$reader().expect(&format!("Failed to read {} at {}.", p, stringify!($tester)));
                    assert_eq!(p, q, "Not same: {} != {} at {}. bytes: {:?}", p, q, stringify!($tester), cursor.into_inner());
                }
            }
        };
    }

    raw_test!(u8_ne, u8, read_u8_raw_ne, write_u8_raw_ne);
    raw_test!(i8_ne, i8, read_i8_raw_ne, write_i8_raw_ne);
    raw_test!(u16_le, u16, read_u16_raw_le, write_u16_raw_le);
    raw_test!(u16_be, u16, read_u16_raw_be, write_u16_raw_be);
    raw_test!(i16_le, i16, read_i16_raw_le, write_i16_raw_le);
    raw_test!(i16_be, i16, read_i16_raw_be, write_i16_raw_be);
    // raw_test!(u32_le, u32, read_u32_raw_le, write_u32_raw_le);
    // raw_test!(u32_be, u32, read_u32_raw_be, write_u32_raw_be);
    // raw_test!(i32_le, i32, read_i32_raw_le, write_i32_raw_le);
    // raw_test!(i32_be, i32, read_i32_raw_be, write_i32_raw_be);
    // raw_test!(u64_le, u64, read_u64_raw_le, write_u64_raw_le);
    // raw_test!(u64_be, u64, read_u64_raw_be, write_u64_raw_be);
    // raw_test!(i64_le, i64, read_i64_raw_le, write_i64_raw_le);
    // raw_test!(i64_be, i64, read_i64_raw_be, write_i64_raw_be);
    // raw_test!(u128_le, u128, read_u128_raw_le, write_u128_raw_le);
    // raw_test!(u128_be, u128, read_u128_raw_be, write_u128_raw_be);
    // raw_test!(i128_le, i128, read_i128_raw_le, write_i128_raw_le);
    // raw_test!(i128_be, i128, read_i128_raw_be, write_i128_raw_be);
}
