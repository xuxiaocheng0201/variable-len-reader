macro_rules! write_float_varint {
    ($primitive: ty, $func: ident, $write_internal: ident) => {
        write_float_varint!(f cfg(feature = "sync_float_varint"), $primitive, $func, $write_internal);
    };
    (f $feature: meta, $primitive: ty, $func: ident, $write_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self, value: $primitive) -> ::core::result::Result<usize, Self::Error> {
            self.$write_internal(value.to_bits())
        }
    };
}

macro_rules! define_write_float_varint {
    () => {
        write_float_varint!(f32, write_f32_varint, write_u32_varint);
        write_float_varint!(f64, write_f64_varint, write_u64_varint);
    };
}

#[cfg(all(feature = "sync_float_varint", not(feature = "sync_varint")))]
compile_error!("developer error: please check Cargo.toml");
