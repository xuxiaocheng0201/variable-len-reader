macro_rules! signed_read {
    ($primitive: ty, $read_signed: ident, $read_varint: ident) => {
        #[inline]
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        fn $read_signed(&mut self) -> Pin<Box<dyn Future<Output = Result<$primitive>> + Send + '_>> {
            Box::pin(async move { Ok(self.$read_varint().await?.zigzag()) })
        }
    };
}
pub(crate) use signed_read;

macro_rules! define_signed_read {
    () => {
        signed::signed_read!(i16, read_i16_varint, read_u16_varint);

        signed::signed_read!(i32, read_i32_varint, read_u32_varint);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i32, read_i32_varint_2_le, read_u32_varint_2_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i32, read_i32_varint_2_be, read_u32_varint_2_be);

        signed::signed_read!(i64, read_i64_varint, read_u64_varint);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i64, read_i64_varint_2_le, read_u64_varint_2_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i64, read_i64_varint_2_be, read_u64_varint_2_be);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i64, read_i64_varint_4_le, read_u64_varint_4_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i64, read_i64_varint_4_be, read_u64_varint_4_be);

        signed::signed_read!(i128, read_i128_varint, read_u128_varint);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i128, read_i128_varint_2_le, read_u128_varint_2_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i128, read_i128_varint_2_be, read_u128_varint_2_be);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i128, read_i128_varint_4_le, read_u128_varint_4_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i128, read_i128_varint_4_be, read_u128_varint_4_be);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i128, read_i128_varint_8_le, read_u128_varint_8_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_read!(i128, read_i128_varint_8_be, read_u128_varint_8_be);
    };
}
pub(crate) use define_signed_read;

macro_rules! signed_write {
    ($primitive: ty, $write_signed: ident, $write_varint: ident) => {
        #[inline]
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        fn $write_signed(&mut self, num: $primitive) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> {
            Box::pin(async move { self.$write_varint(num.zigzag()).await })
        }
    };
}
pub(crate) use signed_write;

macro_rules! define_signed_write {
    () => {
        signed::signed_write!(i16, write_i16_varint, write_u16_varint);

        signed::signed_write!(i32, write_i32_varint, write_u32_varint);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i32, write_i32_varint_2_le, write_u32_varint_2_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i32, write_i32_varint_2_be, write_u32_varint_2_be);

        signed::signed_write!(i64, write_i64_varint, write_u64_varint);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i64, write_i64_varint_2_le, write_u64_varint_2_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i64, write_i64_varint_2_be, write_u64_varint_2_be);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i64, write_i64_varint_4_le, write_u64_varint_4_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i64, write_i64_varint_4_be, write_u64_varint_4_be);

        signed::signed_write!(i128, write_i128_varint, write_u128_varint);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i128, write_i128_varint_2_le, write_u128_varint_2_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i128, write_i128_varint_2_be, write_u128_varint_2_be);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i128, write_i128_varint_4_le, write_u128_varint_4_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i128, write_i128_varint_4_be, write_u128_varint_4_be);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i128, write_i128_varint_8_le, write_u128_varint_8_le);
        #[cfg(feature = "async_long_signed")]
        signed::signed_write!(i128, write_i128_varint_8_be, write_u128_varint_8_be);
    };
}
pub(crate) use define_signed_write;
