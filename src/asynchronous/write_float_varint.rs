macro_rules! write_float_varint_future {
    ($primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_float_varint_future!(f cfg(feature = "async_float_varint"), $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_wrap_future!(f $feature, $primitive, $future, $inner_future);
        #[$feature]
        impl<'a, W: ?Sized> $future<'a, W> {
            fn _handle(value: $primitive) -> $internal {
                value.to_bits()
            }
        }
    };
}
macro_rules! write_float_varint_func {
    ($primitive: ty, $future: ident, $func: ident) => {
        write_float_varint_func!(f cfg(feature = "async_float_varint"), $primitive, $future, $func);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $func: ident) => {
        write_wrap_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_float_varint_future {
    () => {
        write_float_varint_future!(f32, WriteF32Varint, u32, WriteU32Varint);
        write_float_varint_future!(f64, WriteF64Varint, u64, WriteU64Varint);
    };
}
macro_rules! define_write_float_varint_func {
    () => {
        write_float_varint_func!(f32, WriteF32Varint, write_f32_varint);
        write_float_varint_func!(f64, WriteF64Varint, write_f64_varint);
    };
}

define_write_float_varint_future!();

#[cfg(all(feature = "async_float_varint", not(feature = "async_varint")))]
compile_error!("developer error: please check Cargo.toml");
