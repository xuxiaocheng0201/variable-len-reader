// #[cfg(all(feature = "std", feature = "sync"))]
// macro_rules! test_sync_read {
//     ($tester: ident, $reader: ident, [$($source: expr, $res: expr;)+]) => {
//         #[test]
//         fn $tester() {
//             let mut cursor = ::std::io::Cursor::new(::alloc::vec::Vec::new());
//             $(
//                 cursor.set_position(0);
//                 ::std::io::Write::write_all(&mut cursor, $source).unwrap();
//                 cursor.set_position(0);
//                 let res = $crate::synchronous::VariableReader::$reader(&mut cursor)
//                     .expect(&format!("reading failed. tester={}, res={:?}, source={:?}.", stringify!($tester), $res, $source));
//                 assert_eq!(res, $res, "comparing failed. tester={}, res={:?}, source={:?}.", stringify!($tester), $res, $source);
//             )+
//         }
//     };
// }

// #[cfg(feature = "sync_bools")]
// mod bools {
//     use crate::{VariableReader, VariableWriter};
//
//     macro_rules! test_func {
//         ($tester: ident, $reader: ident, $writer: ident, $n: literal) => {
//             #[test]
//             fn $tester() {
//                 let mut cursor = std::io::Cursor::new(Vec::new());
//                 const MAX: u8 = ((1 << ($n - 1)) - 1 << 1) + 1; // (1 << $n) - 1
//                 for n in 0..MAX {
//                     let mut p = [false; $n];
//                     for i in 0..$n {
//                         p[i] = n & (1 << i) != 0;
//                     }
//                     cursor.set_position(0);
//                     cursor.$writer(p).expect(&format!("Failed to write {:?} at {}.", p, stringify!($tester)));
//                     cursor.set_position(0);
//                     let q = cursor.$reader().expect(&format!("Failed to read {:?} at {}.", p, stringify!($tester)));
//                     assert_eq!(p, q, "Not same: {:?} != {:?} at {}. bytes: {:?}", p, q, stringify!($tester), cursor.into_inner());
//                 }
//             }
//         };
//     }
//
//     test_func!(bools_2, read_bools_2, write_bools_2, 2);
//     test_func!(bools_3, read_bools_3, write_bools_3, 3);
//     test_func!(bools_4, read_bools_4, write_bools_4, 4);
//     test_func!(bools_5, read_bools_5, write_bools_5, 5);
//     test_func!(bools_6, read_bools_6, write_bools_6, 6);
//     test_func!(bools_7, read_bools_7, write_bools_7, 7);
//     test_func!(bools_8, read_bools_8, write_bools_8, 8);
// }
//
// #[cfg(any(feature = "sync_raw", feature = "sync_varint", feature = "sync_signed_varint"))]
// macro_rules! test_func {
//     ($cursor: ident, $p: ident, $tester: ident, $reader: ident, $writer: ident) => {
//         $cursor.set_position(0);
//         $cursor.$writer($p).expect(&format!("Failed to write {} at {}.", $p, stringify!($tester)));
//         $cursor.set_position(0);
//         let q = $cursor.$reader().expect(&format!("Failed to read {} at {}.", $p, stringify!($tester)));
//         assert_eq!($p, q, "Not same: {} != {} at {}. bytes: {:?}", $p, q, stringify!($tester), $cursor.into_inner());
//     };
//     ($tester: ident, $primitive: ty, $reader: ident, $writer: ident) => {
//         #[test]
//         fn $tester() {
//             let mut cursor = std::io::Cursor::new(Vec::new());
//             for p in <$primitive>::MIN..=<$primitive>::MAX {
//                 test_func!(cursor, p, $tester, $reader, $writer);
//             }
//         }
//     };
//     ($tester: ident, $reader: ident, $writer: ident, $n: expr) => {
//         #[test]
//         fn $tester() {
//             let mut cursor = std::io::Cursor::new(Vec::new());
//             for n in $n {
//                  test_func!(cursor, n, $tester, $reader, $writer);
//             }
//         }
//     };
// }
//
// #[cfg(any(feature = "sync_raw", feature = "sync_float_varint"))]
// macro_rules! test_nan {
//     ($tester: ident, $reader: ident, $writer: ident, $nan: expr) => {
//         #[test]
//         fn $tester() {
//             let mut cursor = std::io::Cursor::new(Vec::new());
//             let n = $nan;
//             assert!(n.is_nan());
//             cursor.set_position(0);
//             cursor.$writer(n).expect(&format!("Failed to write at {}.", stringify!($tester)));
//             cursor.set_position(0);
//             let q = cursor.$reader().expect(&format!("Failed to read at {}.", stringify!($tester)));
//             assert!(q.is_nan())
//         }
//     };
// }
//
// #[cfg(feature = "sync_raw")]
// mod raw {
//     use crate::{VariableReader, VariableWriter};
//
//     test_func!(u8_ne, u8, read_u8_raw, write_u8_raw);
//     test_func!(i8_ne, i8, read_i8_raw, write_i8_raw);
//
//     test_func!(u16_le, u16, read_u16_raw_le, write_u16_raw_le);
//     test_func!(u16_be, u16, read_u16_raw_be, write_u16_raw_be);
//     test_func!(i16_le, i16, read_i16_raw_le, write_i16_raw_le);
//     test_func!(i16_be, i16, read_i16_raw_be, write_i16_raw_be);
//
//     // test_func!(u32_le, u32, read_u32_raw_le, write_u32_raw_le);
//     // test_func!(u32_be, u32, read_u32_raw_be, write_u32_raw_be);
//     // test_func!(i32_le, i32, read_i32_raw_le, write_i32_raw_le);
//     // test_func!(i32_be, i32, read_i32_raw_be, write_i32_raw_be);
//
//     // test_func!(u64_le, u64, read_u64_raw_le, write_u64_raw_le);
//     // test_func!(u64_be, u64, read_u64_raw_be, write_u64_raw_be);
//     // test_func!(i64_le, i64, read_i64_raw_le, write_i64_raw_le);
//     // test_func!(i64_be, i64, read_i64_raw_be, write_i64_raw_be);
//
//     // test_func!(u128_le, u128, read_u128_raw_le, write_u128_raw_le);
//     // test_func!(u128_be, u128, read_u128_raw_be, write_u128_raw_be);
//     // test_func!(i128_le, i128, read_i128_raw_le, write_i128_raw_le);
//     // test_func!(i128_be, i128, read_i128_raw_be, write_i128_raw_be);
//
//     #[cfg(feature = "sync_raw_size")]
//     test_func!(usize_le, read_usize_raw_le, write_usize_raw_le, [0, 1, 2, usize::MAX]);
//     #[cfg(feature = "sync_raw_size")]
//     test_func!(usize_be, read_usize_raw_be, write_usize_raw_be, [0, 1, 2, usize::MAX]);
//     #[cfg(feature = "sync_raw_size")]
//     test_func!(isize_le, read_isize_raw_le, write_isize_raw_le, [0, 1, 2, -1, -2, isize::MIN, isize::MAX]);
//     #[cfg(feature = "sync_raw_size")]
//     test_func!(isize_be, read_isize_raw_be, write_isize_raw_be, [0, 1, 2, -1, -2, isize::MIN, isize::MAX]);
//
//
//     test_func!(f32_raw_le, read_f32_raw_le, write_f32_raw_le, [f32::MIN, f32::MIN_POSITIVE, f32::MAX, f32::INFINITY, f32::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     test_func!(f32_raw_be, read_f32_raw_be, write_f32_raw_be, [f32::MIN, f32::MIN_POSITIVE, f32::MAX, f32::INFINITY, f32::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     test_func!(f64_raw_le, read_f64_raw_le, write_f64_raw_le, [f64::MIN, f64::MIN_POSITIVE, f64::MAX, f64::INFINITY, f64::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     test_func!(f64_raw_be, read_f64_raw_be, write_f64_raw_be, [f64::MIN, f64::MIN_POSITIVE, f64::MAX, f64::INFINITY, f64::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//
//     test_nan!(f32_raw_le_nan, read_f32_raw_le, write_f32_raw_le, f32::NAN);
//     test_nan!(f32_raw_be_nan, read_f32_raw_be, write_f32_raw_be, f32::NAN);
//     test_nan!(f64_raw_le_nan, read_f64_raw_le, write_f64_raw_le, f64::NAN);
//     test_nan!(f64_raw_be_nan, read_f64_raw_be, write_f64_raw_be, f64::NAN);
// }
//
// #[cfg(feature = "sync_varint")]
// mod varint {
//     use crate::{VariableReader, VariableWriter};
//
//     #[cfg(feature = "sync_varint_long")]
//     test_func!(u8_ne, u8, read_u8_varint, write_u8_varint);
//
//     test_func!(u16_ne, u16, read_u16_varint, write_u16_varint);
//     #[cfg(feature = "sync_varint_long")]
//     test_func!(u16_2_le, u16, read_u16_varint_2_le, write_u16_varint_2_le);
//     #[cfg(feature = "sync_varint_long")]
//     test_func!(u16_2_be, u16, read_u16_varint_2_be, write_u16_varint_2_be);
//
//     // test_func!(u32_ne, u32, read_u32_varint, write_u32_varint);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u32_2_le, u32, read_u32_varint_2_le, write_u32_varint_2_le);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u32_2_be, u32, read_u32_varint_2_be, write_u32_varint_2_be);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u32_4_le, u32, read_u32_varint_4_le, write_u32_varint_4_le);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u32_4_be, u32, read_u32_varint_4_be, write_u32_varint_4_be);
//
//     // test_func!(u64_ne, u64, read_u64_varint, write_u64_varint);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u64_2_le, u64, read_u64_varint_2_le, write_u64_varint_2_le);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u64_2_be, u64, read_u64_varint_2_be, write_u64_varint_2_be);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u64_4_le, u64, read_u64_varint_4_le, write_u64_varint_4_le);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u64_4_be, u64, read_u64_varint_4_be, write_u64_varint_4_be);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u64_8_le, u64, read_u64_varint_8_le, write_u64_varint_8_le);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u64_8_be, u64, read_u64_varint_8_be, write_u64_varint_8_be);
//
//     // test_func!(u128_ne, u128, read_u128_varint, write_u128_varint);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u128_2_le, u128, read_u128_varint_2_le, write_u128_varint_2_le);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u128_2_be, u128, read_u128_varint_2_be, write_u128_varint_2_be);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u128_4_le, u128, read_u128_varint_4_le, write_u128_varint_4_le);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u128_4_be, u128, read_u128_varint_4_be, write_u128_varint_4_be);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u128_8_le, u128, read_u128_varint_8_le, write_u128_varint_8_le);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u128_8_be, u128, read_u128_varint_8_be, write_u128_varint_8_be);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u128_16_le, u128, read_u128_varint_16_le, write_u128_varint_16_le);
//     // #[cfg(feature = "sync_varint_long")]
//     // test_func!(u128_16_be, u128, read_u128_varint_16_be, write_u128_varint_16_be);
//
//     #[cfg(feature = "sync_varint_size")]
//     test_func!(usize_ne, read_usize_varint, write_usize_varint, [0, 1, 2, usize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_varint_long"))]
//     test_func!(usize_2_le, read_usize_varint_2_le, write_usize_varint_2_le, [0, 1, 2, usize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_varint_long"))]
//     test_func!(usize_2_be, read_usize_varint_2_be, write_usize_varint_2_be, [0, 1, 2, usize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_varint_long"))]
//     test_func!(usize_4_le, read_usize_varint_4_le, write_usize_varint_4_le, [0, 1, 2, usize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_varint_long"))]
//     test_func!(usize_4_be, read_usize_varint_4_be, write_usize_varint_4_be, [0, 1, 2, usize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_varint_long"))]
//     test_func!(usize_8_le, read_usize_varint_8_le, write_usize_varint_8_le, [0, 1, 2, usize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_varint_long"))]
//     test_func!(usize_8_be, read_usize_varint_8_be, write_usize_varint_8_be, [0, 1, 2, usize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_varint_long"))]
//     test_func!(usize_16_le, read_usize_varint_16_le, write_usize_varint_16_le, [0, 1, 2, usize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_varint_long"))]
//     test_func!(usize_16_be, read_usize_varint_16_be, write_usize_varint_16_be, [0, 1, 2, usize::MAX]);
// }
//
// #[cfg(feature = "sync_signed_varint")]
// mod signed {
//     use crate::{VariableReader, VariableWriter};
//
//     #[cfg(feature = "sync_signed_varint_long")]
//     test_func!(i8_ne, i8, read_i8_varint, write_i8_varint);
//
//     test_func!(i16_ne, i16, read_i16_varint, write_i16_varint);
//     #[cfg(feature = "sync_signed_varint_long")]
//     test_func!(i16_2_le, i16, read_i16_varint_2_le, write_i16_varint_2_le);
//     #[cfg(feature = "sync_signed_varint_long")]
//     test_func!(i16_2_be, i16, read_i16_varint_2_be, write_i16_varint_2_be);
//
//     // test_func!(i32_ne, i32, read_i32_varint, write_i32_varint);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i32_2_le, i32, read_i32_varint_2_le, write_i32_varint_2_le);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i32_2_be, i32, read_i32_varint_2_be, write_i32_varint_2_be);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i32_4_le, i32, read_i32_varint_4_le, write_i32_varint_4_le);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i32_4_be, i32, read_i32_varint_4_be, write_i32_varint_4_be);
//
//     // test_func!(i64_ne, i64, read_i64_varint, write_i64_varint);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i64_2_le, i64, read_i64_varint_2_le, write_i64_varint_2_le);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i64_2_be, i64, read_i64_varint_2_be, write_i64_varint_2_be);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i64_4_le, i64, read_i64_varint_4_le, write_i64_varint_4_le);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i64_4_be, i64, read_i64_varint_4_be, write_i64_varint_4_be);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i64_8_le, i64, read_i64_varint_8_le, write_i64_varint_8_le);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i64_8_be, i64, read_i64_varint_8_be, write_i64_varint_8_be);
//
//     // test_func!(i128_ne, i128, read_i128_varint, write_i128_varint);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i128_2_le, i128, read_i128_varint_2_le, write_i128_varint_2_le);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i128_2_be, i128, read_i128_varint_2_be, write_i128_varint_2_be);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i128_4_le, i128, read_i128_varint_4_le, write_i128_varint_4_le);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i128_4_be, i128, read_i128_varint_4_be, write_i128_varint_4_be);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i128_8_le, i128, read_i128_varint_8_le, write_i128_varint_8_le);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i128_8_be, i128, read_i128_varint_8_be, write_i128_varint_8_be);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i128_16_le, i128, read_i128_varint_16_le, write_i128_varint_16_le);
//     // #[cfg(feature = "sync_signed_varint_long")]
//     // test_func!(i128_16_be, i128, read_i128_varint_16_be, write_i128_varint_16_be);
//
//     #[cfg(feature = "sync_varint_size")]
//     test_func!(isize_ne, read_isize_varint, write_isize_varint, [0, 1, 2, -1, -2, isize::MIN, isize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_signed_varint_long"))]
//     test_func!(isize_2_le, read_isize_varint_2_le, write_isize_varint_2_le, [0, 1, 2, -1, -2, isize::MIN, isize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_signed_varint_long"))]
//     test_func!(isize_2_be, read_isize_varint_2_be, write_isize_varint_2_be, [0, 1, 2, -1, -2, isize::MIN, isize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_signed_varint_long"))]
//     test_func!(isize_4_le, read_isize_varint_4_le, write_isize_varint_4_le, [0, 1, 2, -1, -2, isize::MIN, isize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_signed_varint_long"))]
//     test_func!(isize_4_be, read_isize_varint_4_be, write_isize_varint_4_be, [0, 1, 2, -1, -2, isize::MIN, isize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_signed_varint_long"))]
//     test_func!(isize_8_le, read_isize_varint_8_le, write_isize_varint_8_le, [0, 1, 2, -1, -2, isize::MIN, isize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_signed_varint_long"))]
//     test_func!(isize_8_be, read_isize_varint_8_be, write_isize_varint_8_be, [0, 1, 2, -1, -2, isize::MIN, isize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_signed_varint_long"))]
//     test_func!(isize_16_le, read_isize_varint_16_le, write_isize_varint_16_le, [0, 1, 2, -1, -2, isize::MIN, isize::MAX]);
//     #[cfg(all(feature = "sync_varint_size", feature = "sync_signed_varint_long"))]
//     test_func!(isize_16_be, read_isize_varint_16_be, write_isize_varint_16_be, [0, 1, 2, -1, -2, isize::MIN, isize::MAX]);
// }
//
// #[cfg(feature = "sync_float_varint")]
// mod float {
//     use crate::{VariableReader, VariableWriter};
//
//     test_func!(f32_ne, read_f32_varint, write_f32_varint, [f32::MIN, f32::MIN_POSITIVE, f32::MAX, f32::INFINITY, f32::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_func!(f32_2_le, read_f32_varint_2_le, write_f32_varint_2_le, [f32::MIN, f32::MIN_POSITIVE, f32::MAX, f32::INFINITY, f32::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_func!(f32_2_be, read_f32_varint_2_be, write_f32_varint_2_be, [f32::MIN, f32::MIN_POSITIVE, f32::MAX, f32::INFINITY, f32::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_func!(f32_4_le, read_f32_varint_4_le, write_f32_varint_4_le, [f32::MIN, f32::MIN_POSITIVE, f32::MAX, f32::INFINITY, f32::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_func!(f32_4_be, read_f32_varint_4_be, write_f32_varint_4_be, [f32::MIN, f32::MIN_POSITIVE, f32::MAX, f32::INFINITY, f32::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//
//     test_func!(f64_ne, read_f64_varint, write_f64_varint, [f64::MIN, f64::MIN_POSITIVE, f64::MAX, f64::INFINITY, f64::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_func!(f64_2_le, read_f64_varint_2_le, write_f64_varint_2_le, [f64::MIN, f64::MIN_POSITIVE, f64::MAX, f64::INFINITY, f64::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_func!(f64_2_be, read_f64_varint_2_be, write_f64_varint_2_be, [f64::MIN, f64::MIN_POSITIVE, f64::MAX, f64::INFINITY, f64::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_func!(f64_4_le, read_f64_varint_4_le, write_f64_varint_4_le, [f64::MIN, f64::MIN_POSITIVE, f64::MAX, f64::INFINITY, f64::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_func!(f64_4_be, read_f64_varint_4_be, write_f64_varint_4_be, [f64::MIN, f64::MIN_POSITIVE, f64::MAX, f64::INFINITY, f64::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_func!(f64_8_le, read_f64_varint_8_le, write_f64_varint_8_le, [f64::MIN, f64::MIN_POSITIVE, f64::MAX, f64::INFINITY, f64::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_func!(f64_8_be, read_f64_varint_8_be, write_f64_varint_8_be, [f64::MIN, f64::MIN_POSITIVE, f64::MAX, f64::INFINITY, f64::NEG_INFINITY, 0.0, 0.1, -0.1, 1.0, -1.0]);
//
//     test_nan!(f32_ne_nan, read_f32_varint, write_f32_varint, f32::NAN);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_nan!(f32_2_le_nan, read_f32_varint_2_le, write_f32_varint_2_le, f32::NAN);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_nan!(f32_2_be_nan, read_f32_varint_2_be, write_f32_varint_2_be, f32::NAN);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_nan!(f32_4_le_nan, read_f32_varint_4_le, write_f32_varint_4_le, f32::NAN);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_nan!(f32_4_be_nan, read_f32_varint_4_be, write_f32_varint_4_be, f32::NAN);
//
//     test_nan!(f64_ne_nan, read_f64_varint, write_f64_varint, f64::NAN);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_nan!(f64_2_le_nan, read_f64_varint_2_le, write_f64_varint_2_le, f64::NAN);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_nan!(f64_2_be_nan, read_f64_varint_2_be, write_f64_varint_2_be, f64::NAN);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_nan!(f64_4_le_nan, read_f64_varint_4_le, write_f64_varint_4_le, f64::NAN);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_nan!(f64_4_be_nan, read_f64_varint_4_be, write_f64_varint_4_be, f64::NAN);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_nan!(f64_8_le_nan, read_f64_varint_8_le, write_f64_varint_8_le, f64::NAN);
//     #[cfg(feature = "sync_float_varint_long")]
//     test_nan!(f64_8_be_nan, read_f64_varint_8_be, write_f64_varint_8_be, f64::NAN);
// }