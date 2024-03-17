macro_rules! read_signed_varint_future {
    ($primitive: ty, $future: ident, $inner_future: ident) => {
        read_signed_varint_future!(f cfg(feature = "async_signed_varint"), $primitive, $future, $inner_future);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $inner_future: ident) => {
        read_wrap_future!(f $feature, $future, $inner_future);
        #[$feature]
        impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for $future<'a, R> {
            type Output = ::core::result::Result<$primitive, R::Error>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                use $crate::util::zigzag::Zigzag;
                self.project().inner.poll(cx).map_ok(|v| v.zigzag())
            }
        }
    };
}
macro_rules! read_signed_varint_func {
    ($future: ident, $func: ident, $inner_func: ident) => {
        read_signed_varint_func!(f cfg(feature = "async_signed_varint"), $future, $func, $inner_func);
    };
    (f $feature: meta, $future: ident, $func: ident, $inner_func: ident) => {
        read_wrap_func!(f $feature, $future, $func, $inner_func);
    };
}

macro_rules! define_read_signed_varint_future {
    () => {
        read_signed_varint_future!(i16, ReadI16Varint, ReadU16Varint);
        read_signed_varint_future!(i32, ReadI32Varint, ReadU32Varint);
        read_signed_varint_future!(i64, ReadI64Varint, ReadU64Varint);
        read_signed_varint_future!(i128, ReadI128Varint, ReadU128Varint);
    };
}
macro_rules! define_read_signed_varint_func {
    () => {
        read_signed_varint_func!(ReadI16Varint, read_i16_varint, read_u16_varint);
        read_signed_varint_func!(ReadI32Varint, read_i32_varint, read_u32_varint);
        read_signed_varint_func!(ReadI64Varint, read_i64_varint, read_u64_varint);
        read_signed_varint_func!(ReadI128Varint, read_i128_varint, read_u128_varint);
    };
}

define_read_signed_varint_future!();

#[cfg(all(feature = "async_signed_varint", not(feature = "async_varint")))]
compile_error!("developer error: please check Cargo.toml");
