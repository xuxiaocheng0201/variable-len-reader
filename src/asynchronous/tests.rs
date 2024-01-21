macro_rules! test_func {
    ($tester: ident, $primitive: ty, $reader: ident, $writer: ident) => {
        #[tokio::test]
        async fn $tester() {
            let mut cursor = std::io::Cursor::new(Vec::new());
            for p in <$primitive>::MIN..=<$primitive>::MAX {
                cursor.set_position(0);
                cursor.$writer(p).await.expect(&format!("Failed to write {} at {}.", p, stringify!($tester)));
                cursor.set_position(0);
                let q = cursor.$reader().await.expect(&format!("Failed to read {} at {}.", p, stringify!($tester)));
                assert_eq!(p, q, "Not same: {} != {} at {}. bytes: {:?}", p, q, stringify!($tester), cursor.into_inner());
            }
        }
    };
}

#[cfg(feature = "raw")]
mod raw {
    use crate::asynchronous::{AsyncVariableReader, AsyncVariableWriter};

    test_func!(u8_ne, u8, read_u8_raw, write_u8_raw);
    test_func!(i8_ne, i8, read_i8_raw, write_i8_raw);
    test_func!(u16_le, u16, read_u16_raw_le, write_u16_raw_le);
    test_func!(u16_be, u16, read_u16_raw_be, write_u16_raw_be);
    test_func!(i16_le, i16, read_i16_raw_le, write_i16_raw_le);
    test_func!(i16_be, i16, read_i16_raw_be, write_i16_raw_be);
    // test_func!(u32_le, u32, read_u32_raw_le, write_u32_raw_le);
    // test_func!(u32_be, u32, read_u32_raw_be, write_u32_raw_be);
    // test_func!(i32_le, i32, read_i32_raw_le, write_i32_raw_le);
    // test_func!(i32_be, i32, read_i32_raw_be, write_i32_raw_be);
    // test_func!(u64_le, u64, read_u64_raw_le, write_u64_raw_le);
    // test_func!(u64_be, u64, read_u64_raw_be, write_u64_raw_be);
    // test_func!(i64_le, i64, read_i64_raw_le, write_i64_raw_le);
    // test_func!(i64_be, i64, read_i64_raw_be, write_i64_raw_be);
    // test_func!(u128_le, u128, read_u128_raw_le, write_u128_raw_le);
    // test_func!(u128_be, u128, read_u128_raw_be, write_u128_raw_be);
    // test_func!(i128_le, i128, read_i128_raw_le, write_i128_raw_le);
    // test_func!(i128_be, i128, read_i128_raw_be, write_i128_raw_be);
}
