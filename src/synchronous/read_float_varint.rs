macro_rules! read_float_varint {
    ($primitive: ty, $func: ident, $read_internal: ident) => {
        read_float_varint!(f cfg(feature = "sync_float_varint"), $primitive, $func, $read_internal);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $read_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> ::core::result::Result<$primitive, Self::Error> {
            self.$read_internal().map(|v| <$primitive>::from_bits(v))
        }
    };
}

macro_rules! define_read_float_varint {
    () => {
        read_float_varint!(f32, read_f32_varint, read_u32_varint);
        read_float_varint!(f64, read_f64_varint, read_u64_varint);
    };
}

#[cfg(all(feature = "sync_float_varint", not(feature = "sync_varint")))]
compile_error!("developer error: please check Cargo.toml");
