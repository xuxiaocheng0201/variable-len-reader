macro_rules! read_bools_future {
    ($future: ident, $poll_func: ident, $n: literal) => {
        #[cfg(feature = "async_bools")]
        $crate::pin_project_lite::pin_project! {
            #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
                byte: Option<u8>,
            }
        }
        #[cfg(feature = "async_bools")]
        impl<'a, R: $crate::AsyncVariableReadable + Unpin+ ?Sized> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<[bool; $n]>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let me = self.project();
                R::$poll_func(std::pin::Pin::new(&mut *me.reader), cx, me.byte)
            }
        }
    };
}
macro_rules! read_bools_poll {
    ($poll_func: ident, $n: literal) => {
        #[cfg(feature = "async_bools")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, byte: &mut Option<u8>) -> std::task::Poll<std::io::Result<[bool; $n]>> {
            const MAX: u8 = ((1 << ($n - 1)) - 1 << 1) + 1; // (1 << $n) - 1
            if let Some(bools) = b.as_ref() {
                return std::task::Poll::Ready(Ok(bools.clone()));
            }
            let b = match byte {
                Some(b) => b, None => {
                    let b = ready!(self.poll_read_single(cx))?;
                    byte.replace(b);
                    b
                }
            }
            if b > MAX {
                return std::task::Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid bools at {}.", stringify!($func)))));
            }
            let mut bools = [false; $n];
            for i in 0..$n {
                bools[i] = b & (1 << i) != 0;
            }
            b.
            std::task::Poll::Ready(Ok(bools))
        }
    };
}
macro_rules! read_bools_func {
    ($func: ident, $future: ident) => {
        #[cfg(feature = "async_bools")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, byte: None }
        }
    };
}
macro_rules! define_read_bools_futures {
    () => {
        read_bools_future!(ReadBools2, poll_read_bools_2, 2);
        read_bools_future!(ReadBools3, poll_read_bools_3, 3);
        read_bools_future!(ReadBools4, poll_read_bools_4, 4);
        read_bools_future!(ReadBools5, poll_read_bools_5, 5);
        read_bools_future!(ReadBools6, poll_read_bools_6, 6);
        read_bools_future!(ReadBools7, poll_read_bools_7, 7);
        read_bools_future!(ReadBools8, poll_read_bools_8, 8);
    };
}
macro_rules! define_read_bools_poll {
    () => {
        read_bools_poll!(poll_read_bools_2, 2);
        read_bools_poll!(poll_read_bools_3, 3);
        read_bools_poll!(poll_read_bools_4, 4);
        read_bools_poll!(poll_read_bools_5, 5);
        read_bools_poll!(poll_read_bools_6, 6);
        read_bools_poll!(poll_read_bools_7, 7);
        read_bools_poll!(poll_read_bools_8, 8);
    };
}
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
define_read_bools_futures!();
