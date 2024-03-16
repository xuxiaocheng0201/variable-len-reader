macro_rules! read_signed_varint {
    ($primitive: ty, $func: ident, $read_internal: ident) => {
        read_signed_varint!(f cfg(feature = "sync_signed_varint"), $primitive, $func, $read_internal);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $read_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> ::core::result::Result<$primitive, Self::Error> {
            use $crate::util::zigzag::Zigzag;
            self.$read_internal().map(|v| v.zigzag())
        }
    };
}
macro_rules! define_read_signed_varint {
    () => {
        read_signed_varint!(i16, read_i16_varint, read_u16_varint);
        read_signed_varint!(i32, read_i32_varint, read_u32_varint);
        read_signed_varint!(i64, read_i64_varint, read_u64_varint);
        read_signed_varint!(i128, read_i128_varint, read_u128_varint);
    };
}

#[cfg(all(feature = "sync_signed_varint", not(feature = "sync_varint")))]
compile_error!("developer error: please check Cargo.toml");
