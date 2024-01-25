#[cfg(feature = "bools")]
#[cfg_attr(docsrs, doc(cfg(feature = "bools")))]
macro_rules! read_bools {
    ($func: ident, $n: literal) => {
        #[inline]
        fn $func(&mut self) -> Result<[bool; $n]> {
            const MAX: u8 = ((1 << ($n - 1)) - 1 << 1) + 1; // (1 << $n) - 1 (Prevent `this arithmetic operation will overflow`)
            let b = self.read_single()?;
            if b > MAX {
                return Err(Error::new(ErrorKind::InvalidData, format!("Invalid bools at {}.", stringify!($func))));
            }
            let mut bools = [false; $n];
            for i in 0..$n {
                bools[i] = b & (1 << i) != 0;
            }
            Ok(bools)
        }
    };
}
#[cfg(feature = "bools")]
#[cfg_attr(docsrs, doc(cfg(feature = "bools")))]
macro_rules! define_read_bools {
    () => {
        read_bools!(read_bools_2, 2);
        read_bools!(read_bools_3, 3);
        read_bools!(read_bools_4, 4);
        read_bools!(read_bools_5, 5);
        read_bools!(read_bools_6, 6);
        read_bools!(read_bools_7, 7);
        read_bools!(read_bools_8, 8);
    };
}
