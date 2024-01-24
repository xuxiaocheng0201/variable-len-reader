#[cfg(feature = "async_bools")]
macro_rules! write_bools_future {
    ($future: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                b: u8,
            }
        }
        impl<'a, W: $crate::AsyncVariableWritable + Unpin+ ?Sized> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                W::poll_write_single(Pin::new(&mut *me.writer), cx, *me.b)
            }
        }
    };
}
#[cfg(feature = "async_bools")]
macro_rules! write_bools_func {
    ($func: ident, $future: ident, $n: literal) => {
        #[inline]
        fn $func(&mut self, bools: [bool; $n]) -> $future<Self> where Self: Unpin {
            let mut b = 0;
            for i in 0..$n {
                if bools[i] {
                    b |= 1 << i;
                }
            }
            $future { writer: self, b }
        }
    };
}
#[cfg(feature = "async_bools")]
macro_rules! define_write_bools_futures {
    () => {
        write_bools_future!(WriteBools2);
        write_bools_future!(WriteBools3);
        write_bools_future!(WriteBools4);
        write_bools_future!(WriteBools5);
        write_bools_future!(WriteBools6);
        write_bools_future!(WriteBools7);
        write_bools_future!(WriteBools8);
    };
}
#[cfg(feature = "async_bools")]
macro_rules! define_write_bools_func {
    () => {
        write_bools_func!(write_bools_2, WriteBools2, 2);
        write_bools_func!(write_bools_3, WriteBools3, 3);
        write_bools_func!(write_bools_4, WriteBools4, 4);
        write_bools_func!(write_bools_5, WriteBools5, 5);
        write_bools_func!(write_bools_6, WriteBools6, 6);
        write_bools_func!(write_bools_7, WriteBools7, 7);
        write_bools_func!(write_bools_8, WriteBools8, 8);
    };
}
#[cfg(feature = "async_bools")]
define_write_bools_futures!();
