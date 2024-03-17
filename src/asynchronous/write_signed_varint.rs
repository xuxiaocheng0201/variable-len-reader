macro_rules! write_signed_varint_future {
    ($primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_signed_varint_future!(f cfg(feature = "async_signed_varint"), $primitive, $future, $internal, $inner_future);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        write_wrap_future!(f $feature, $primitive, $future, $inner_future);
        #[$feature]
        impl<'a, W: ?Sized> $future<'a, W> {
            fn _handle(value: $primitive) -> $internal {
                use $crate::util::zigzag::Zigzag;
                value.zigzag()
            }
        }
    };
}
macro_rules! write_signed_varint_func {
    ($primitive: ty, $future: ident, $func: ident) => {
        write_signed_varint_func!(f cfg(feature = "async_signed_varint"), $primitive, $future, $func);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $func: ident) => {
        write_wrap_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_signed_varint_future {
    () => {
        write_signed_varint_future!(i16, WriteI16Varint, u16, WriteU16Varint);
        write_signed_varint_future!(i32, WriteI32Varint, u32, WriteU32Varint);
        write_signed_varint_future!(i64, WriteI64Varint, u64, WriteU64Varint);
        write_signed_varint_future!(i128, WriteI128Varint, u128, WriteU128Varint);
    };
}
macro_rules! define_write_signed_varint_func {
    () => {
        write_signed_varint_func!(i16, WriteI16Varint, write_i16_varint);
        write_signed_varint_func!(i32, WriteI32Varint, write_i32_varint);
        write_signed_varint_func!(i64, WriteI64Varint, write_i64_varint);
        write_signed_varint_func!(i128, WriteI128Varint, write_i128_varint);
    };
}

define_write_signed_varint_future!();

#[cfg(all(feature = "async_signed_varint", not(feature = "async_varint")))]
compile_error!("developer error: please check Cargo.toml");
