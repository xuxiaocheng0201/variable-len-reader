


// macro_rules! raw_write {
//     ($primitive: ty, $write_le: ident, $write_be: ident) => {
//         #[inline]
//         #[must_use = "futures do nothing unless you `.await` or poll them"]
//         fn $write_le(&mut self, num: $primitive) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin {
//             Box::pin(async move { self.write_more(&<$primitive>::to_le_bytes(num)).await })
//         }
//         #[inline]
//         #[must_use = "futures do nothing unless you `.await` or poll them"]
//         fn $write_be(&mut self, num: $primitive) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin {
//             Box::pin(async move { self.write_more(&<$primitive>::to_be_bytes(num)).await })
//         }
//     };
// }
// pub(crate) use raw_write;
//
// macro_rules! define_raw_write {
//     () => {
//         #[inline]
//         #[must_use = "futures do nothing unless you `.await` or poll them"]
//         fn write_u8_ne(&mut self, num: u8) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin {
//             Box::pin(async move { self.write_single(num.to_ne_bytes()[0]).await })
//         }
//         #[inline]
//         #[must_use = "futures do nothing unless you `.await` or poll them"]
//         fn write_i8_ne(&mut self, num: i8) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin {
//             Box::pin(async move { self.write_single(num.to_ne_bytes()[0]).await })
//         }
//         raw::raw_write!(u16, write_u16_le, write_u16_be);
//         raw::raw_write!(i16, write_i16_le, write_i16_be);
//         raw::raw_write!(u32, write_u32_le, write_u32_be);
//         raw::raw_write!(i32, write_i32_le, write_i32_be);
//         raw::raw_write!(u64, write_u64_le, write_u64_be);
//         raw::raw_write!(i64, write_i64_le, write_i64_be);
//         raw::raw_write!(u128, write_u128_le, write_u128_be);
//         raw::raw_write!(i128, write_i128_le, write_i128_be);
//     };
// }
// pub(crate) use define_raw_write;
