#[cfg(feature = "async_bools")]
macro_rules! read_bools_future {
    ($future: ident, $n: literal) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
            }
        }
        impl<'a, R: $crate::asynchronous::AsyncVariableReadable + Unpin> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<[bool; $n]>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                const MAX: u8 = ((1 << ($n - 1)) - 1 << 1) + 1; // (1 << $n) - 1
                let mut me = self.project();
                let b = ready!(R::poll_read_single(Pin::new(&mut *me.reader), cx))?;
                if b > MAX {
                    return  Poll::Ready(Err(Error::new(ErrorKind::InvalidData, format!("Invalid bools at {}.", stringify!($func)))));
                }
                let mut bools = [false; $n];
                for i in 0..$n {
                    bools[i] = b & (1 << i) != 0;
                }
                Poll::Ready(Ok(bools))
            }
        }
    };
}
#[cfg(feature = "async_bools")]
macro_rules! read_bools_func {
    ($func: ident, $future: ident) => {
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self }
        }
    };
}
#[cfg(feature = "async_bools")]
macro_rules! define_read_bools_futures {
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
#[cfg(feature = "async_bools")]
macro_rules! define_read_bools_func {
    () => {
        read_bools_func!(read_bools_2, ReadBools2);
        read_bools_func!(read_bools_3, ReadBools3);
        read_bools_func!(read_bools_4, ReadBools4);
        read_bools_func!(read_bools_5, ReadBools5);
        read_bools_func!(read_bools_6, ReadBools6);
        read_bools_func!(read_bools_7, ReadBools7);
        read_bools_func!(read_bools_8, ReadBools8);
    };
}
#[cfg(feature = "async_bools")]
define_read_bools_futures!();
