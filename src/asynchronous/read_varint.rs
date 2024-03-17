macro_rules! read_varint_future {
    ($primitive: ty, $future: ident) => {
        read_varint_future!(f cfg(feature = "async_varint"), $primitive, $future, u8, ReadU8Raw);
    };
    (f $feature: meta, $primitive: ty, $future: ident, $internal: ty, $inner_future: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                value: $primitive,
                position: usize,
                ok: Option<bool>,
                #[pin]
                inner: $inner_future<'a, R>,
            }
        }
        #[$feature]
        impl<'a, R: ?Sized> ResettableFuture for $future<'a, R> {
            fn reset(self: Pin<&mut Self>) {
                let me = self.project();
                *me.value = 0;
                *me.position = 0;
                *me.ok = None;
                me.inner.reset();
            }
        }
        #[$feature]
        impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for $future<'a, R> {
            type Output = ::core::result::Result<$primitive, R::Error>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut me = self.project();
                if let Some(ok) = me.ok.as_ref() {
                    return Poll::Ready(if *ok { Ok(*me.value) } else { Err(R::read_varint_error(stringify!($future), *me.value as u128)) });
                }
                const SIZE: usize = ::core::mem::size_of::<$primitive>() << 3; // * 8
                const NUM_BITS: $internal = <$internal>::MAX >> 1;
                const SIGN_BIT: $internal = NUM_BITS + 1;
                const POS_OFFSET: usize = (<$internal>::BITS - 1) as usize;
                loop {
                    let current = ::core::task::ready!(me.inner.as_mut().poll(cx))?;
                    *me.value |= ((current & NUM_BITS) as $primitive) << *me.position;
                    if current & SIGN_BIT == 0 {
                        *me.ok = Some(true);
                        return Poll::Ready(Ok(*me.value));
                    }
                    *me.position += POS_OFFSET;
                    if *me.position >= SIZE {
                        *me.ok = Some(false);
                        return Poll::Ready(Err(R::read_varint_error(stringify!($future), *me.value as u128)));
                    }
                    me.inner.as_mut().reset();
                }
            }
        }
    };
}
macro_rules! read_varint_func {
    ($func: ident, $future: ident) => {
        read_varint_func!(f cfg(feature = "async_varint"), $func, $future, read_u8_raw);
    };
    (f $feature: meta, $future: ident, $func: ident, $inner_func: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { value: 0, position: 0, ok: None, inner: self.$inner_func() }
        }
    };
}

macro_rules! define_read_varint_future {
    () => {
        read_varint_future!(u16, ReadU16Varint);
        read_varint_future!(u32, ReadU32Varint);
        read_varint_future!(u64, ReadU64Varint);
        read_varint_future!(u128, ReadU128Varint);
    };
}
macro_rules! define_read_varint_func {
    () => {
        #[cfg(feature = "async_varint")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_varint")))]
        fn read_varint_error(future_name: &'static str, value: u128) -> Self::Error;

        read_varint_func!(ReadU16Varint, read_u16_varint);
        read_varint_func!(ReadU32Varint, read_u32_varint);
        read_varint_func!(ReadU64Varint, read_u64_varint);
        read_varint_func!(ReadU128Varint, read_u128_varint);
    };
}

define_read_varint_future!();

#[cfg(all(feature = "async_varint", not(feature = "async_raw")))]
compile_error!("developer error: please check Cargo.toml");
