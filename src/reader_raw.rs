macro_rules! read_raw {
    ($primitive: ty, $func: ident, $from: ident) => {
        #[cfg(feature = "sync_raw")]
        #[cfg_attr(docsrs, doc(cfg(feature = "sync_raw")))]
        #[inline]
        fn $func(&mut self) -> std::io::Result<$primitive> {
            const SIZE: usize = std::mem::size_of::<$primitive>();
            let mut bytes = [0; SIZE];
            self.read_more(&mut bytes)?;
            Ok(<$primitive>::$from(bytes))
        }
    };
}
macro_rules! read_raw_size {
    ($primitive: ty, $func: ident, $read_internal: ident) => {
        #[cfg(feature = "sync_raw_size")]
        #[cfg_attr(docsrs, doc(cfg(feature = "sync_raw_size")))]
        #[inline]
        fn $func(&mut self) -> Result<$primitive> {
            self.$read_internal().map(|v| v as $primitive)
        }
    };
}
macro_rules! define_read_raw {
    () => {
        read_raw!(u8, read_u8_raw, from_ne_bytes);
        read_raw!(i8, read_i8_raw, from_ne_bytes);

        read_raw!(u16, read_u16_raw_le, from_le_bytes);
        read_raw!(u16, read_u16_raw_be, from_be_bytes);
        read_raw!(i16, read_i16_raw_le, from_le_bytes);
        read_raw!(i16, read_i16_raw_be, from_be_bytes);

        read_raw!(u32, read_u32_raw_le, from_le_bytes);
        read_raw!(u32, read_u32_raw_be, from_be_bytes);
        read_raw!(i32, read_i32_raw_le, from_le_bytes);
        read_raw!(i32, read_i32_raw_be, from_be_bytes);

        read_raw!(u64, read_u64_raw_le, from_le_bytes);
        read_raw!(u64, read_u64_raw_be, from_be_bytes);
        read_raw!(i64, read_i64_raw_le, from_le_bytes);
        read_raw!(i64, read_i64_raw_be, from_be_bytes);

        read_raw!(u128, read_u128_raw_le, from_le_bytes);
        read_raw!(u128, read_u128_raw_be, from_be_bytes);
        read_raw!(i128, read_i128_raw_le, from_le_bytes);
        read_raw!(i128, read_i128_raw_be, from_be_bytes);

        read_raw_size!(usize, read_usize_raw_le, read_u128_raw_le);
        read_raw_size!(usize, read_usize_raw_be, read_u128_raw_be);
        read_raw_size!(isize, read_isize_raw_le, read_i128_raw_le);
        read_raw_size!(isize, read_isize_raw_be, read_i128_raw_be);

        read_raw!(f32, read_f32_raw_le, from_le_bytes);
        read_raw!(f32, read_f32_raw_be, from_be_bytes);
        read_raw!(f64, read_f64_raw_le, from_le_bytes);
        read_raw!(f64, read_f64_raw_be, from_be_bytes);
    }
}
