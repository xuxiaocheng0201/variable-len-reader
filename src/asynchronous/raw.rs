macro_rules! raw_read {
    ($primitive: ty, $read_le: ident, $read_be: ident) => {
        #[inline]
        fn $read_le(&mut self) -> Pin<Box<dyn Future<Output = Result<$primitive>> + Send + '_>> {
            const SIZE: usize = std::mem::size_of::<$primitive>();
            Box::pin(async move {
                let mut bytes = [0; SIZE];
                self.read_more(&mut bytes).await?;
                Ok(<$primitive>::from_le_bytes(bytes))
            })
        }
        #[inline]
        fn $read_be(&mut self) -> Pin<Box<dyn Future<Output = Result<$primitive>> + Send + '_>> {
            const SIZE: usize = std::mem::size_of::<$primitive>();
            Box::pin(async move {
                let mut bytes = [0; SIZE];
                self.read_more(&mut bytes).await?;
                Ok(<$primitive>::from_be_bytes(bytes))
            })
        }
    };
}
pub(crate) use raw_read;

macro_rules! define_raw_read {
    () => {
        #[inline]
        fn read_u8_ne(&mut self) -> Pin<Box<dyn Future<Output = Result<u8>> + Send + '_>> {
            Box::pin(async move { Ok(u8::from_ne_bytes([self.read().await?])) })
        }
        #[inline]
        fn read_i8_ne(&mut self) -> Pin<Box<dyn Future<Output = Result<i8>> + Send + '_>> {
            Box::pin(async move { Ok(i8::from_ne_bytes([self.read().await?])) })
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
        fn $write_le(&mut self, num: $primitive) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> {
            Box::pin(async move { self.write_more(&<$primitive>::to_le_bytes(num)).await })
        }
        #[inline]
        fn $write_be(&mut self, num: $primitive) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> {
            Box::pin(async move { self.write_more(&<$primitive>::to_be_bytes(num)).await })
        }
    };
}
pub(crate) use raw_write;

macro_rules! define_raw_write {
    () => {
        #[inline]
        fn write_u8_ne(&mut self, num: u8) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> {
            Box::pin(async move { self.write(num.to_ne_bytes()[0]).await })
        }
        #[inline]
        fn write_i8_ne(&mut self, num: i8) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> {
            Box::pin(async move { self.write(num.to_ne_bytes()[0]).await })
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
