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

#[cfg(all(feature = "std", feature = "sync_bools", test))]
mod test_read_bools {
    use crate::synchronous::tests::test_sync_read;

    test_sync_read!(test_read_bools_2, read_bools_2, [
        &[0x0], [false, false];
        &[0x1], [ true, false];
        &[0x2], [false,  true];
        &[0x3], [ true,  true];
    ]);

    test_sync_read!(test_read_bools_3, read_bools_3, [
        &[0x0], [false, false, false];
        &[0x1], [ true, false, false];
        &[0x2], [false,  true, false];
        &[0x3], [ true,  true, false];
        &[0x4], [false, false,  true];
        &[0x5], [ true, false,  true];
        &[0x6], [false,  true,  true];
        &[0x7], [ true,  true,  true];
    ]);

    test_sync_read!(test_read_bools_4, read_bools_4, [
        &[0x00], [false, false, false, false];
        &[0x01], [ true, false, false, false];
        &[0x02], [false,  true, false, false];
        &[0x03], [ true,  true, false, false];
        &[0x04], [false, false,  true, false];
        &[0x05], [ true, false,  true, false];
        &[0x06], [false,  true,  true, false];
        &[0x07], [ true,  true,  true, false];
        &[0x08], [false, false, false,  true];
        &[0x09], [ true, false, false,  true];
        &[0x0a], [false,  true, false,  true];
        &[0x0b], [ true,  true, false,  true];
        &[0x0c], [false, false,  true,  true];
        &[0x0d], [ true, false,  true,  true];
        &[0x0e], [false,  true,  true,  true];
        &[0x0f], [ true,  true,  true,  true];
    ]);

    // ...
}
