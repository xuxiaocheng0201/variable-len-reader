macro_rules! read_bools_future {
    ($future: ident, $n: literal) => {
        read_bools_future!(f cfg(feature = "async_bools"), $future, $n);
    };
    (f $feature: meta, $future: ident, $n: literal) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                inner: ReadSingle<'a, R>,
            }
        }
        #[$feature]
        impl<'a, R: ?Sized> ResettableFuture for $future<'_, R> {
            fn reset(self: Pin<&mut Self>) {
                let me = self.project();
                me.inner.reset();
            }
        }
        #[$feature]
        impl<'a, R: AsyncVariableReader + Unpin + ?Sized> Future for $future<'a, R> {
            type Output = ::core::result::Result<[bool; $n], R::Error>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                self.project().inner.poll(cx).map(|r| r.and_then(|b| {
                    const MAX: u8 = ((1 << ($n - 1)) - 1 << 1) + 1; // (1 << $n) - 1
                    if b > MAX {
                        return Err(R::read_bools_error(stringify!($future), b));
                    }
                    let mut bools = [false; $n];
                    for i in 0..$n {
                        bools[i] = b & (1 << i) != 0;
                    }
                    Ok(bools)
                }))
            }
        }
    };
}
macro_rules! read_bools_func {
    ($func: ident, $future: ident) => {
        read_bools_func!(f cfg(feature = "async_bools"), $func, $future);
    };
    (f $feature: meta, $func: ident, $future: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { inner: self.read_single() }
        }
    };
}

macro_rules! define_read_bools_future {
    () => {
        read_bools_future!(ReadBools2, 2);
        read_bools_future!(ReadBools3, 3);
        read_bools_future!(ReadBools4, 4);
        read_bools_future!(ReadBools5, 5);
        read_bools_future!(ReadBools6, 6);
        read_bools_future!(ReadBools7, 7);
        read_bools_future!(ReadBools8, 8);
    };
}
macro_rules! define_read_bools_func {
    () => {
        #[cfg(feature = "async_bools")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
        fn read_bools_error(future_name: &'static str, byte: u8) -> Self::Error;

        read_bools_func!(read_bools_2, ReadBools2);
        read_bools_func!(read_bools_3, ReadBools3);
        read_bools_func!(read_bools_4, ReadBools4);
        read_bools_func!(read_bools_5, ReadBools5);
        read_bools_func!(read_bools_6, ReadBools6);
        read_bools_func!(read_bools_7, ReadBools7);
        read_bools_func!(read_bools_8, ReadBools8);
    };
}

define_read_bools_future!();
