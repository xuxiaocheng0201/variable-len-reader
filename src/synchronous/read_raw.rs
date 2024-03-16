macro_rules! read_raw {
    ($primitive: ty, $func: ident, $from: ident) => {
        read_raw!(f cfg(feature = "sync_raw"), $primitive, $func, $from);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $from: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> ::core::result::Result<$primitive, Self::Error> {
            const SIZE: usize = ::core::mem::size_of::<$primitive>();
            let mut bytes = [0; SIZE];
            self.read_more(&mut bytes)?;
            Ok(<$primitive>::$from(bytes))
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

        read_raw!(f32, read_f32_raw_le, from_le_bytes);
        read_raw!(f32, read_f32_raw_be, from_be_bytes);
        read_raw!(f64, read_f64_raw_le, from_le_bytes);
        read_raw!(f64, read_f64_raw_be, from_be_bytes);
    }
}
