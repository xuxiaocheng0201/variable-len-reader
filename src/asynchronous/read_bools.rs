macro_rules! read_bools_future {
    ($future: ident, $n: literal) => {
        read_bools_future!(f cfg(feature = "async_bools"), $future, $n);
    };
    (f $feature: meta, $future: ident, $n: literal) => {
        read_wrap_future!(f $feature, $future, ReadSingle);
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
    ($future: ident, $func: ident) => {
        read_bools_func!(f cfg(feature = "async_bools"), $future, $func);
    };
    (f $feature: meta, $future: ident, $func: ident) => {
        read_wrap_func!(f $feature, $future, $func, read_single);
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

        read_bools_func!(ReadBools2, read_bools_2);
        read_bools_func!(ReadBools3, read_bools_3);
        read_bools_func!(ReadBools4, read_bools_4);
        read_bools_func!(ReadBools5, read_bools_5);
        read_bools_func!(ReadBools6, read_bools_6);
        read_bools_func!(ReadBools7, read_bools_7);
        read_bools_func!(ReadBools8, read_bools_8);
    };
}

define_read_bools_future!();
