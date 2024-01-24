#[cfg(feature = "async_bools")]
mod bools {
    use crate::{AsyncVariableReader, AsyncVariableWriter};

    macro_rules! test_func {
        ($tester: ident, $reader: ident, $writer: ident, $n: literal) => {
            #[tokio::test]
            async fn $tester() {
                let mut cursor = std::io::Cursor::new(Vec::new());
                const MAX: u8 = ((1 << ($n - 1)) - 1 << 1) + 1; // (1 << $n) - 1
                for n in 0..MAX {
                    let mut p = [false; $n];
                    for i in 0..$n {
                        p[i] = n & (1 << i) != 0;
                    }
                    cursor.set_position(0);
                    cursor.$writer(p).await.expect(&format!("Failed to write {:?} at {}.", p, stringify!($tester)));
                    cursor.set_position(0);
                    let q = cursor.$reader().await.expect(&format!("Failed to read {:?} at {}.", p, stringify!($tester)));
                    assert_eq!(p, q, "Not same: {:?} != {:?} at {}. bytes: {:?}", p, q, stringify!($tester), cursor.into_inner());
                }
            }
        };
    }

    test_func!(bools_2, read_bools_2, write_bools_2, 2);
    test_func!(bools_3, read_bools_3, write_bools_3, 3);
    test_func!(bools_4, read_bools_4, write_bools_4, 4);
    test_func!(bools_5, read_bools_5, write_bools_5, 5);
    test_func!(bools_6, read_bools_6, write_bools_6, 6);
    test_func!(bools_7, read_bools_7, write_bools_7, 7);
    test_func!(bools_8, read_bools_8, write_bools_8, 8);
}

#[cfg(any(feature = "async_raw", feature = "async_varint", feature = "async_signed"))]
macro_rules! test_func {
    ($cursor: ident, $p: ident, $tester: ident, $reader: ident, $writer: ident) => {
        $cursor.set_position(0);
        $cursor.$writer($p).await.expect(&format!("Failed to write {} at {}.", $p, stringify!($tester)));
        $cursor.set_position(0);
        let q = $cursor.$reader().await.expect(&format!("Failed to read {} at {}.", $p, stringify!($tester)));
        assert_eq!($p, q, "Not same: {} != {} at {}. bytes: {:?}", $p, q, stringify!($tester), $cursor.into_inner());
    };
    ($tester: ident, $primitive: ty, $reader: ident, $writer: ident) => {
        #[tokio::test]
        async fn $tester() {
            let mut cursor = std::io::Cursor::new(Vec::new());
            for p in <$primitive>::MIN..=<$primitive>::MAX {
                test_func!(cursor, p, $tester, $reader, $writer);
            }
        }
    };
    ($tester: ident, $reader: ident, $writer: ident, [$($n: expr,)+]) => {
        #[tokio::test]
        async fn $tester() {
            let mut cursor = std::io::Cursor::new(Vec::new());
            $({
                 let n = $n;
                 test_func!(cursor, n, $tester, $reader, $writer);
            })+
        }
    };
}

#[cfg(feature = "async_raw")]
mod raw {
    use crate::{AsyncVariableReader, AsyncVariableWriter};

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

    #[cfg(feature = "async_raw_size")]
    test_func!(usize_le, read_usize_raw_le, write_usize_raw_le, [0, 1, 2, usize::MAX,]);
    #[cfg(feature = "async_raw_size")]
    test_func!(usize_be, read_usize_raw_be, write_usize_raw_be, [0, 1, 2, usize::MAX,]);
    #[cfg(feature = "async_raw_size")]
    test_func!(isize_le, read_isize_raw_le, write_isize_raw_le, [0, 1, 2, -1, -2, isize::MIN, isize::MAX,]);
    #[cfg(feature = "async_raw_size")]
    test_func!(isize_be, read_isize_raw_be, write_isize_raw_be, [0, 1, 2, -1, -2, isize::MIN, isize::MAX,]);
}

#[cfg(feature = "async_varint")]
mod varint {
    // use crate::{AsyncVariableReader, AsyncVariableWriter};
    //
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u8_ne, u8, read_u8_varint, write_u8_varint);
    //
    // test_func!(u16_ne, u16, read_u16_varint, write_u16_varint);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u16_2_le, u16, read_u16_varint_2_le, write_u16_varint_2_le);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u16_2_be, u16, read_u16_varint_2_be, write_u16_varint_2_be);

    // test_func!(u32_ne, u32, read_u32_varint, write_u32_varint);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u32_2_le, u32, read_u32_varint_2_le, write_u32_varint_2_le);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u32_2_be, u32, read_u32_varint_2_be, write_u32_varint_2_be);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u32_4_le, u32, read_u32_varint_4_le, write_u32_varint_4_le);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u32_4_be, u32, read_u32_varint_4_be, write_u32_varint_4_be);
    //
    // test_func!(u64_ne, u64, read_u64_varint, write_u64_varint);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u64_2_le, u64, read_u64_varint_2_le, write_u64_varint_2_le);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u64_2_be, u64, read_u64_varint_2_be, write_u64_varint_2_be);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u64_4_le, u64, read_u64_varint_4_le, write_u64_varint_4_le);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u64_4_be, u64, read_u64_varint_4_be, write_u64_varint_4_be);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u64_8_le, u64, read_u64_varint_8_le, write_u64_varint_8_le);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u64_8_be, u64, read_u64_varint_8_be, write_u64_varint_8_be);
    //
    // test_func!(u128_ne, u128, read_u128_varint, write_u128_varint);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u128_2_le, u128, read_u128_varint_2_le, write_u128_varint_2_le);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u128_2_be, u128, read_u128_varint_2_be, write_u128_varint_2_be);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u128_4_le, u128, read_u128_varint_4_le, write_u128_varint_4_le);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u128_4_be, u128, read_u128_varint_4_be, write_u128_varint_4_be);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u128_8_le, u128, read_u128_varint_8_le, write_u128_varint_8_le);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u128_8_be, u128, read_u128_varint_8_be, write_u128_varint_8_be);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u128_16_le, u128, read_u128_varint_16_le, write_u128_varint_16_le);
    // #[cfg(feature = "async_long_varint")]
    // test_func!(u128_16_be, u128, read_u128_varint_16_be, write_u128_varint_16_be);

    // #[cfg(feature = "async_varint_size")]
    // test_func!(usize_ne, read_usize_varint, write_usize_varint, [0, 1, 2, usize::MAX,]);
    // #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
    // test_func!(usize_2_le, read_usize_varint_2_le, write_usize_varint_2_le, [0, 1, 2, usize::MAX,]);
    // #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
    // test_func!(usize_2_be, read_usize_varint_2_be, write_usize_varint_2_be, [0, 1, 2, usize::MAX,]);
    // #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
    // test_func!(usize_4_le, read_usize_varint_4_le, write_usize_varint_4_le, [0, 1, 2, usize::MAX,]);
    // #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
    // test_func!(usize_4_be, read_usize_varint_4_be, write_usize_varint_4_be, [0, 1, 2, usize::MAX,]);
    // #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
    // test_func!(usize_8_le, read_usize_varint_8_le, write_usize_varint_8_le, [0, 1, 2, usize::MAX,]);
    // #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
    // test_func!(usize_8_be, read_usize_varint_8_be, write_usize_varint_8_be, [0, 1, 2, usize::MAX,]);
    // #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
    // test_func!(usize_16_le, read_usize_varint_16_le, write_usize_varint_16_le, [0, 1, 2, usize::MAX,]);
    // #[cfg(all(feature = "async_varint_size", feature = "async_long_varint"))]
    // test_func!(usize_16_be, read_usize_varint_16_be, write_usize_varint_16_be, [0, 1, 2, usize::MAX,]);
}

#[cfg(feature = "async_signed")]
mod signed {
    // use crate::{AsyncVariableReader, AsyncVariableWriter};
    //
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i8_ne, i8, read_i8_varint, write_i8_varint);
    //
    // test_func!(i16_ne, i16, read_i16_varint, write_i16_varint);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i16_2_le, i16, read_i16_varint_2_le, write_i16_varint_2_le);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i16_2_be, i16, read_i16_varint_2_be, write_i16_varint_2_be);

    // test_func!(i32_ne, i32, read_i32_varint, write_i32_varint);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i32_2_le, i32, read_i32_varint_2_le, write_i32_varint_2_le);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i32_2_be, i32, read_i32_varint_2_be, write_i32_varint_2_be);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i32_4_le, i32, read_i32_varint_4_le, write_i32_varint_4_le);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i32_4_be, i32, read_i32_varint_4_be, write_i32_varint_4_be);
    //
    // test_func!(i64_ne, i64, read_i64_varint, write_i64_varint);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i64_2_le, i64, read_i64_varint_2_le, write_i64_varint_2_le);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i64_2_be, i64, read_i64_varint_2_be, write_i64_varint_2_be);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i64_4_le, i64, read_i64_varint_4_le, write_i64_varint_4_le);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i64_4_be, i64, read_i64_varint_4_be, write_i64_varint_4_be);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i64_8_le, i64, read_i64_varint_8_le, write_i64_varint_8_le);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i64_8_be, i64, read_i64_varint_8_be, write_i64_varint_8_be);
    //
    // test_func!(i128_ne, i128, read_i128_varint, write_i128_varint);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i128_2_le, i128, read_i128_varint_2_le, write_i128_varint_2_le);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i128_2_be, i128, read_i128_varint_2_be, write_i128_varint_2_be);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i128_4_le, i128, read_i128_varint_4_le, write_i128_varint_4_le);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i128_4_be, i128, read_i128_varint_4_be, write_i128_varint_4_be);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i128_8_le, i128, read_i128_varint_8_le, write_i128_varint_8_le);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i128_8_be, i128, read_i128_varint_8_be, write_i128_varint_8_be);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i128_16_le, i128, read_i128_varint_16_le, write_i128_varint_16_le);
    // #[cfg(feature = "async_long_signed")]
    // test_func!(i128_16_be, i128, read_i128_varint_16_be, write_i128_varint_16_be);
}
