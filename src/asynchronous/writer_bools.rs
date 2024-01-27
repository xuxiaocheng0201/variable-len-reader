macro_rules! write_bools_future {
    ($future: ident, $poll_func: ident, $struct_buf: ident, $n: literal) => {
        #[cfg(feature = "async_bools")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
        #[derive(Debug)]
        struct $struct_buf {
            b: Option<u8>,
        }
        #[cfg(feature = "async_bools")]
        impl $struct_buf {
            fn _handle(bools: [bool; $n]) -> u8 {
                let mut b = 0;
                for i in 0..$n {
                    if bools[i] {
                        b |= 1 << i;
                    }
                }
                b
            }
            #[inline]
            fn new(bools: [bool; $n]) -> Self {
                let b = $struct_buf::_handle(bools);
                Self { b: Some(b) }
            }
            #[inline]
            fn reset(&mut self, bools: [bool; $n]) {
                let b = $struct_buf::_handle(bools);
                self.b = Some(b);
            }
        }
        #[cfg(feature = "async_bools")]
        $crate::pin_project_lite::pin_project! {
            #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                inner: $struct_buf,
            }
        }
        #[cfg(feature = "async_bools")]
        impl<'a, W: $crate::AsyncVariableWritable + Unpin+ ?Sized> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                W::$poll_func(Pin::new(&mut *me.writer), cx, me.inner)
            }
        }
    };
}
macro_rules! write_bools_poll {
    ($poll_func:ident, $struct_buf: ident) => {
        #[cfg(feature = "async_bools")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
        fn $poll_func(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, inner: &mut $struct_buf) -> std::task::Poll<std::io::Result<usize>> {
            let b = match inner.b {
                Some(b) => b, None => { return std::task::Poll::Ready(Ok(0)); }
            };
            self.poll_write_single(cx, b)
        }
    };
}
macro_rules! write_bools_func {
    ($func: ident, $future: ident, $struct_buf: ident, $n: literal) => {
        #[cfg(feature = "async_bools")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async_bools")))]
        #[inline]
        fn $func(&mut self, bools: [bool; $n]) -> $future<Self> where Self: Unpin {
            $future { writer: self, inner: $struct_buf::new(bools) }
        }
    };
}
macro_rules! define_write_bools_futures {
    () => {
        write_bools_future!(WriteBools2, poll_write_bools_2, InternalWriteBools2, 2);
        write_bools_future!(WriteBools3, poll_write_bools_3, InternalWriteBools3, 3);
        write_bools_future!(WriteBools4, poll_write_bools_4, InternalWriteBools4, 4);
        write_bools_future!(WriteBools5, poll_write_bools_5, InternalWriteBools5, 5);
        write_bools_future!(WriteBools6, poll_write_bools_6, InternalWriteBools6, 6);
        write_bools_future!(WriteBools7, poll_write_bools_7, InternalWriteBools7, 7);
        write_bools_future!(WriteBools8, poll_write_bools_8, InternalWriteBools8, 8);
    };
}
macro_rules! define_write_bools_poll {
    () => {
        write_bools_poll!(poll_write_bools_2, InternalWriteBools2);
        write_bools_poll!(poll_write_bools_3, InternalWriteBools3);
        write_bools_poll!(poll_write_bools_4, InternalWriteBools4);
        write_bools_poll!(poll_write_bools_5, InternalWriteBools5);
        write_bools_poll!(poll_write_bools_6, InternalWriteBools6);
        write_bools_poll!(poll_write_bools_7, InternalWriteBools7);
        write_bools_poll!(poll_write_bools_8, InternalWriteBools8);
    };
}
macro_rules! define_write_bools_func {
    () => {
        write_bools_func!(write_bools_2, WriteBools2, InternalWriteBools2, 2);
        write_bools_func!(write_bools_3, WriteBools3, InternalWriteBools3, 3);
        write_bools_func!(write_bools_4, WriteBools4, InternalWriteBools4, 4);
        write_bools_func!(write_bools_5, WriteBools5, InternalWriteBools5, 5);
        write_bools_func!(write_bools_6, WriteBools6, InternalWriteBools6, 6);
        write_bools_func!(write_bools_7, WriteBools7, InternalWriteBools7, 7);
        write_bools_func!(write_bools_8, WriteBools8, InternalWriteBools8, 8);
    };
}
define_write_bools_futures!();
