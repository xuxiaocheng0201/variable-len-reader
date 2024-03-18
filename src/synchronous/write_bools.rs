macro_rules! write_bools {
    ($func: ident, $n: literal) => {
        write_bools!(f cfg(feature = "sync_bools"), $func, $n);
    };
    (f $feature: meta, $func: ident, $n: literal) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self, bools: [bool; $n]) -> ::core::result::Result<usize, Self::Error> {
            let mut b = 0;
            for i in 0..$n {
                if bools[i] {
                    b |= 1 << i;
                }
            }
            self.write_single(b)
        }
    };
}

macro_rules! define_write_bools {
    () => {
        write_bools!(write_bools_2, 2);
        write_bools!(write_bools_3, 3);
        write_bools!(write_bools_4, 4);
        write_bools!(write_bools_5, 5);
        write_bools!(write_bools_6, 6);
        write_bools!(write_bools_7, 7);
        write_bools!(write_bools_8, 8);
    };
}
