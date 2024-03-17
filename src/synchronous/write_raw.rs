macro_rules! write_raw {
    ($primitive: ty, $func: ident, $to: ident) => {
        write_raw!(f cfg(feature = "sync_raw"), $primitive, $func, $to);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $to: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self, value: $primitive) -> ::core::result::Result<usize, Self::Error> {
            self.write_more(&<$primitive>::$to(value))
        }
    };
}
macro_rules! define_write_raw {
    () => {
        write_raw!(u8, write_u8_raw, to_ne_bytes);
        write_raw!(i8, write_i8_raw, to_ne_bytes);

        write_raw!(u16, write_u16_raw_le, to_le_bytes);
        write_raw!(u16, write_u16_raw_be, to_be_bytes);
        write_raw!(i16, write_i16_raw_le, to_le_bytes);
        write_raw!(i16, write_i16_raw_be, to_be_bytes);

        write_raw!(u32, write_u32_raw_le, to_le_bytes);
        write_raw!(u32, write_u32_raw_be, to_be_bytes);
        write_raw!(i32, write_i32_raw_le, to_le_bytes);
        write_raw!(i32, write_i32_raw_be, to_be_bytes);

        write_raw!(u64, write_u64_raw_le, to_le_bytes);
        write_raw!(u64, write_u64_raw_be, to_be_bytes);
        write_raw!(i64, write_i64_raw_le, to_le_bytes);
        write_raw!(i64, write_i64_raw_be, to_be_bytes);

        write_raw!(u128, write_u128_raw_le, to_le_bytes);
        write_raw!(u128, write_u128_raw_be, to_be_bytes);
        write_raw!(i128, write_i128_raw_le, to_le_bytes);
        write_raw!(i128, write_i128_raw_be, to_be_bytes);

        write_raw!(f32, write_f32_raw_le, to_le_bytes);
        write_raw!(f32, write_f32_raw_be, to_be_bytes);
        write_raw!(f64, write_f64_raw_le, to_le_bytes);
        write_raw!(f64, write_f64_raw_be, to_be_bytes);
    };
}
