macro_rules! write_signed_varint {
    ($primitive: ty, $func: ident, $write_internal: ident) => {
        write_signed_varint!(f cfg(feature = "sync_signed_varint"), $primitive, $func, $write_internal);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $write_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self, value: $primitive) -> ::core::result::Result<usize, Self::Error> {
            use $crate::util::zigzag::Zigzag;
            self.$write_internal(value.zigzag())
        }
    };
}
macro_rules! define_write_signed_varint {
    () => {
        write_signed_varint!(i16, write_i16_varint, write_u16_varint);
        write_signed_varint!(i32, write_i32_varint, write_u32_varint);
        write_signed_varint!(i64, write_i64_varint, write_u64_varint);
        write_signed_varint!(i128, write_i128_varint, write_u128_varint);
    };
}

#[cfg(all(feature = "sync_signed_varint", not(feature = "sync_varint")))]
compile_error!("developer error: please check Cargo.toml");
