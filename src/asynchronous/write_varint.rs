macro_rules! write_varint_future {
    ($primitive: ty, $future: ident) => {
        write_varint_future!(f cfg(feature = "async_varint"), $primitive, $future, u8, WriteU8Raw);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        #[$feature]
        $crate::pin_project_lite::pin_project! {
            #[cfg_attr(docsrs, doc($feature))]
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                value: $primitive,
                position: usize,
                ok: bool,
                #[pin]
                inner: $inner_future<'a, W>,
            }
        }
        #[$feature]
        impl<'a, W: ?Sized> WriterFuture<'a, W, $primitive> for $future<'a, W> {
            fn new(writer: &'a mut W, buf: $primitive) -> Self {
                let (value, current, position) = Self::_extra(buf, 0);
                Self { value, position, ok: false, inner: $inner_future::new(writer, current) }
            }
            fn reset(self: Pin<&mut Self>, buf: $primitive) {
                let me = self.project();
                let (value, current, position) = Self::_extra(buf, 0);
                *me.value = value;
                *me.position = position;
                *me.ok = false;
                me.inner.reset(current);
            }
        }
        #[$feature]
        impl<'a, W: AsyncVariableWriter + Unpin + ?Sized> Future for $future<'a, W> {
            type Output = ::core::result::Result<(), W::Error>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut me = self.project();
                if *me.ok { return Poll::Ready(Ok(())); }
                loop {
                    ::core::task::ready!(me.inner.as_mut().poll(cx))?;
                    if *me.value == 0 {
                        *me.ok = true;
                        return Poll::Ready(Ok(()));
                    }
                    let (value, current, position) = Self::_extra(*me.value, *me.position);
                    *me.value = value;
                    *me.position = position;
                    me.inner.as_mut().reset(current);
                }
            }
        }
        #[$feature]
        #[allow(arithmetic_overflow)] // Safety: only used internally.
        impl<'a, W: ?Sized> $future<'a, W> {
            fn _extra(value: $primitive, position: usize) -> ($primitive, $internal, usize) {
                const NUM_BITS: $internal = <$internal>::MAX >> 1;
                const SIGN_BIT: $internal = NUM_BITS + 1;
                const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
                if value >= SIGN_BIT as $primitive {
                    (value >> POS_OFFSET, ((value & (NUM_BITS as $primitive)) as $internal) | SIGN_BIT, position + POS_OFFSET)
                } else {
                    (0, value as $internal, position + POS_OFFSET)
                }
            }
        }
    };
}
macro_rules! write_varint_func {
    ($primitive: ty, $future: ident, $func: ident) => {
        write_varint_func!(f cfg(feature = "async_varint"), $primitive, $future, $func);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $func: ident) => {
        write_wrap_func!(f $feature, $primitive, $future, $func);
    };
}

macro_rules! define_write_varint_future {
    () => {
        write_varint_future!(u16, WriteU16Varint);
        write_varint_future!(u32, WriteU32Varint);
        write_varint_future!(u64, WriteU64Varint);
        write_varint_future!(u128, WriteU128Varint);
    };
}
macro_rules! define_write_varint_func {
    () => {
        write_varint_func!(u16, WriteU16Varint, write_u16_varint);
        write_varint_func!(u32, WriteU32Varint, write_u32_varint);
        write_varint_func!(u64, WriteU64Varint, write_u64_varint);
        write_varint_func!(u128, WriteU128Varint, write_u128_varint);
    };
}

define_write_varint_future!();

#[cfg(all(feature = "async_varint", not(feature = "async_raw")))]
compile_error!("developer error: please check Cargo.toml");
