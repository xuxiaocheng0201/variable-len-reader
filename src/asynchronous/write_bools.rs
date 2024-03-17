macro_rules! write_bools_future {
    ($future: ident, $n: literal) => {
        write_bools_future!(f cfg(feature = "async_bools"), $future, WriteSingle, $n);
    };
    (f $feature: meta, $future: ident, $inner_future: ident, $n: literal) => {
        write_wrap_future!(f $feature, [bool; $n], $future, $inner_future);
        #[$feature]
        impl<'a, W: ?Sized> $future<'a, W> {
            fn _handle(bools: [bool; $n]) -> u8 {
                let mut b = 0;
                for i in 0..$n {
                    if bools[i] {
                        b |= 1 << i;
                    }
                }
                b
            }
        }
    };
}
macro_rules! write_bools_func {
    ($future: ident, $func: ident, $n: literal) => {
        write_bools_func!(f cfg(feature = "async_bools"), $future, $func, $n);
    };
    (f $feature: meta, $future: ident, $func: ident, $n: literal) => {
        write_wrap_func!(f $feature, [bool; $n], $future, $func);
    };
}

macro_rules! define_write_bools_future {
    () => {
        write_bools_future!(WriteBools2, 2);
        write_bools_future!(WriteBools3, 3);
        write_bools_future!(WriteBools4, 4);
        write_bools_future!(WriteBools5, 5);
        write_bools_future!(WriteBools6, 6);
        write_bools_future!(WriteBools7, 7);
        write_bools_future!(WriteBools8, 8);
    };
}
macro_rules! define_write_bools_func {
    () => {
        write_bools_func!(WriteBools2, write_bools_2, 2);
        write_bools_func!(WriteBools3, write_bools_3, 3);
        write_bools_func!(WriteBools4, write_bools_4, 4);
        write_bools_func!(WriteBools5, write_bools_5, 5);
        write_bools_func!(WriteBools6, write_bools_6, 6);
        write_bools_func!(WriteBools7, write_bools_7, 7);
        write_bools_func!(WriteBools8, write_bools_8, 8);
    };
}

define_write_bools_future!();
