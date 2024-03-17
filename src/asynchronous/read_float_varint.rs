macro_rules! read_float_varint_future {
    ($primitive: ty, $future: ident, $inner_future: ident) => {
        read_float_varint_future!(f cfg(feature = "async_float_varint"), $primitive, $future, $inner_future);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_wrap_future!(f $feature, $future, $inner_future);
        #[$feature]
        impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for $future<'a, R> {
            type Output = ::core::result::Result<$primitive, R::Error>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                self.project().inner.poll(cx).map_ok(|v| <$primitive>::from_bits(v))
            }
        }
    };
}
macro_rules! read_float_varint_func {
    ($future: ident, $func: ident, $inner_func: ident) => {
        read_float_varint_func!(f cfg(feature = "async_float_varint"), $future, $func, $inner_func);
    };
    (f $feature: meta, $future: ident, $func: ident, $inner_func: ident) => {
        read_wrap_func!(f $feature, $future, $func, $inner_func);
    };
}

macro_rules! define_read_float_varint_future {
    () => {
        read_float_varint_future!(f32, ReadF32Varint, ReadU32Varint);
        read_float_varint_future!(f64, ReadF64Varint, ReadU64Varint);
    };
}
macro_rules! define_read_float_varint_func {
    () => {
        read_float_varint_func!(ReadF32Varint, read_f32_varint, read_u32_varint);
        read_float_varint_func!(ReadF64Varint, read_f64_varint, read_u64_varint);
    };
}

define_read_float_varint_future!();

#[cfg(all(feature = "async_float_varint", not(feature = "async_varint")))]
compile_error!("developer error: please check Cargo.toml");
