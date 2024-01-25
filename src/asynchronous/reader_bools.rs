macro_rules! read_bools_future {
    ($future: ident, $poll_func: ident, $struct_buf: ident, $n: literal) => {
        #[cfg(feature = "async_bools")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
        #[derive(Debug)]
        struct $struct_buf {
            byte: Option<u8>,
        }
        #[cfg(feature = "async_bools")]
        impl $struct_buf {
            fn new() -> Self {
                Self { byte: None }
            }
            fn reset(&mut self) {
                self.byte = None;
            }
        }
        #[cfg(feature = "async_bools")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
                inner: $struct_buf,
            }
        }
        #[cfg(feature = "async_bools")]
        impl<'a, R: $crate::AsyncVariableReadable + Unpin+ ?Sized> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<[bool; $n]>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                R::$poll_func(std::pin::Pin::new(&mut *me.reader), cx, me.inner)
            }
        }
    };
}
macro_rules! read_bools_poll {
    ($poll_func: ident, $struct_buf: ident, $n: literal) => {
        #[cfg(feature = "async_bools")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, inner: &mut $struct_buf) -> std::task::Poll<std::io::Result<[bool; $n]>> {
            const MAX: u8 = ((1 << ($n - 1)) - 1 << 1) + 1; // (1 << $n) - 1
            let b = match inner.byte {
                Some(b) => b, None => {
                    let b = ready!(self.poll_read_single(cx))?;
                    inner.byte.replace(b);
                    b
                },
            };
            if b > MAX {
                return std::task::Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid bools at {}.", stringify!($func)))));
            }
            let mut bools = [false; $n];
            for i in 0..$n {
                bools[i] = b & (1 << i) != 0;
            }
            std::task::Poll::Ready(Ok(bools))
        }
    };
}
macro_rules! read_bools_func {
    ($func: ident, $future: ident, $struct_buf: ident) => {
        #[cfg(feature = "async_bools")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            $future { reader: self, inner: $struct_buf::new() }
        }
    };
}
macro_rules! define_read_bools_futures {
    () => {
        read_bools_future!(ReadBools2, poll_read_bools_2, InternalReadBools2, 2);
        read_bools_future!(ReadBools3, poll_read_bools_3, InternalReadBools3, 3);
        read_bools_future!(ReadBools4, poll_read_bools_4, InternalReadBools4, 4);
        read_bools_future!(ReadBools5, poll_read_bools_5, InternalReadBools5, 5);
        read_bools_future!(ReadBools6, poll_read_bools_6, InternalReadBools6, 6);
        read_bools_future!(ReadBools7, poll_read_bools_7, InternalReadBools7, 7);
        read_bools_future!(ReadBools8, poll_read_bools_8, InternalReadBools8, 8);
    };
}
macro_rules! define_read_bools_poll {
    () => {
        read_bools_poll!(poll_read_bools_2, InternalReadBools2, 2);
        read_bools_poll!(poll_read_bools_3, InternalReadBools3, 3);
        read_bools_poll!(poll_read_bools_4, InternalReadBools4, 4);
        read_bools_poll!(poll_read_bools_5, InternalReadBools5, 5);
        read_bools_poll!(poll_read_bools_6, InternalReadBools6, 6);
        read_bools_poll!(poll_read_bools_7, InternalReadBools7, 7);
        read_bools_poll!(poll_read_bools_8, InternalReadBools8, 8);
    };
}
macro_rules! define_read_bools_func {
    () => {
        read_bools_func!(read_bools_2, ReadBools2, InternalReadBools2);
        read_bools_func!(read_bools_3, ReadBools3, InternalReadBools3);
        read_bools_func!(read_bools_4, ReadBools4, InternalReadBools4);
        read_bools_func!(read_bools_5, ReadBools5, InternalReadBools5);
        read_bools_func!(read_bools_6, ReadBools6, InternalReadBools6);
        read_bools_func!(read_bools_7, ReadBools7, InternalReadBools7);
        read_bools_func!(read_bools_8, ReadBools8, InternalReadBools8);
    };
}
define_read_bools_futures!();
