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

#[cfg(all(feature = "std", feature = "sync_raw", test))]
mod test_read_raw {
    use crate::synchronous::tests::test_sync_read;

    test_sync_read!(test_read_u8_raw, read_u8_raw, [
        &[0x00], 0; &[0x01], 1;
        &[0x7f], 127; &[0x80], 128;
        &[0xfe], 254; &[0xff], 255;
    ]);
    test_sync_read!(test_read_i8_raw, read_i8_raw, [
        &[0x00], 0; &[0x01], 1;
        &[0x7f], 127; &[0x80], -128;
        &[0xfe], -2; &[0xff], -1;
    ]);

    test_sync_read!(test_read_u16_raw_be, read_u16_raw_be, [
        &[0x00, 0x00], 0; &[0x00, 0x01], 1;
        &[0x7f, 0xff], 32767; &[0x80, 0x00], 32768;
        &[0xff, 0xfe], 65534; &[0xff, 0xff], 65535;
    ]);
    test_sync_read!(test_read_u16_raw_le, read_u16_raw_le, [
        &[0x00, 0x00], 0; &[0x01, 0x00], 1;
        &[0xff, 0x7f], 32767; &[0x00, 0x80], 32768;
        &[0xfe, 0xff], 65534; &[0xff, 0xff], 65535;
    ]);
    test_sync_read!(test_read_i16_raw_be, read_i16_raw_be, [
        &[0x00, 0x00], 0; &[0x00, 0x01], 1;
        &[0x7f, 0xff], 32767; &[0x80, 0x00], -32768;
        &[0xff, 0xfe], -2; &[0xff, 0xff], -1;
    ]);
    test_sync_read!(test_read_i16_raw_le, read_i16_raw_le, [
        &[0x00, 0x00], 0; &[0x01, 0x00], 1;
        &[0xff, 0x7f], 32767; &[0x00, 0x80], -32768;
        &[0xfe, 0xff], -2; &[0xff, 0xff], -1;
    ]);

    // ...

    test_sync_read!(test_read_f32_raw_le, read_f32_raw_le, [
        &[0x00, 0x00, 0x00, 0x00], 0.0;
        &[0x00, 0x00, 0x80, 0x3f], 1.0;
        &[0x00, 0x00, 0x80, 0xbf], -1.0;
        &[0x00, 0x00, 0x80, 0x7f], f32::INFINITY;
        &[0x00, 0x00, 0x80, 0xff], f32::NEG_INFINITY;
        // &[0x00, 0x00, 0xc0, 0x7f], f32::NAN;
    ]);

    // ...
}
