macro_rules! read_bools {
    ($func: ident, $n: literal) => {
        read_bools!(f cfg(feature = "sync_bools"), $func, $n);
    };
    (f $feature: meta, $func: ident, $n: literal) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> ::core::result::Result<[bool; $n], Self::Error> {
            const MAX: u8 = ((1 << ($n - 1)) - 1 << 1) + 1; // (1 << $n) - 1 (Prevent compile error: `this arithmetic operation will overflow`)
            let b = self.read_single()?;
            if b > MAX {
                return Err(Self::read_bools_error(stringify!($func), b));
            }
            let mut bools = [false; $n];
            for i in 0..$n {
                bools[i] = b & (1 << i) != 0;
            }
            Ok(bools)
        }
    };
}
macro_rules! define_read_bools {
    () => {
        #[cfg(feature = "sync_bools")]
        #[cfg_attr(docsrs, doc(cfg(feature = "sync_bools")))]
        fn read_bools_error(func_name: &'static str, byte: u8) -> Self::Error;

        read_bools!(read_bools_2, 2);
        read_bools!(read_bools_3, 3);
        read_bools!(read_bools_4, 4);
        read_bools!(read_bools_5, 5);
        read_bools!(read_bools_6, 6);
        read_bools!(read_bools_7, 7);
        read_bools!(read_bools_8, 8);
    };
}
