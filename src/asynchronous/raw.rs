macro_rules! read_raw_future {
    ($primitive: ty, $future: ident, $func: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: Unpin> where R: ?Sized {
                #[pin]
                reader: &'a mut R,
                #[pin]
                buf: [u8; std::mem::size_of::<$primitive>()],
                #[pin]
                read: usize,
            }
        }
        impl<'a, R: $crate::asynchronous::AsyncVariableReadable + Unpin + ?Sized> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$primitive>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                let mut buf = $crate::tokio::io::ReadBuf::new(&mut *me.buf);
                buf.advance(*me.read);
                match R::poll_read_more(Pin::new(&mut *me.reader), cx, &mut buf) {
                    std::task::Poll::Pending => {
                        *me.read = buf.filled().len();
                        std::task::Poll::Pending
                    },
                    std::task::Poll::Ready(e) => {
                        e?;
                        std::task::Poll::Ready(Ok(<$primitive>::$func(*me.buf)))
                    }
                }
            }
        }
    };
}
pub(crate) use read_raw_future;

macro_rules! read_raw_func {
    ($primitive: ty, $func: ident, $future: ident) => {
        #[inline]
        fn $func(&mut self) -> $future<Self> where Self: Unpin {
            const SIZE: usize = std::mem::size_of::<$primitive>();
            let buf = [0; SIZE];
            $future { reader: self, buf, read: 0 }
        }
    };
}
pub(crate) use read_raw_func;


macro_rules! define_read_raw_futures {
    () => {
        $crate::asynchronous::raw::read_raw_future!(u8, ReadU8Ne, from_ne_bytes);
        $crate::asynchronous::raw::read_raw_future!(i8, ReadI8Ne, from_ne_bytes);

        $crate::asynchronous::raw::read_raw_future!(u16, ReadU16Le, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(u16, ReadU16Be, from_be_bytes);
        $crate::asynchronous::raw::read_raw_future!(i16, ReadI16Le, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(i16, ReadI16Be, from_be_bytes);

        $crate::asynchronous::raw::read_raw_future!(u32, ReadU32Le, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(u32, ReadU32Be, from_be_bytes);
        $crate::asynchronous::raw::read_raw_future!(i32, ReadI32Le, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(i32, ReadI32Be, from_be_bytes);

        $crate::asynchronous::raw::read_raw_future!(u64, ReadU64Le, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(u64, ReadU64Be, from_be_bytes);
        $crate::asynchronous::raw::read_raw_future!(i64, ReadI64Le, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(i64, ReadI64Be, from_be_bytes);

        $crate::asynchronous::raw::read_raw_future!(u128, ReadU128Le, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(u128, ReadU128Be, from_be_bytes);
        $crate::asynchronous::raw::read_raw_future!(i128, ReadI128Le, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(i128, ReadI128Be, from_be_bytes);
    };
}
pub(crate) use define_read_raw_futures;

macro_rules! define_read_raw_func {
    () => {
        $crate::asynchronous::raw::read_raw_func!(u8, read_u8_ne, ReadU8Ne);
        $crate::asynchronous::raw::read_raw_func!(u8, read_i8_ne, ReadI8Ne);

        // $crate::asynchronous::raw::read_raw_func!(read_u16_le, ReadU16Le);
        // $crate::asynchronous::raw::read_raw_func!(read_u16_be, ReadU16Be);
        // $crate::asynchronous::raw::read_raw_func!(read_i16_le, ReadI16Le);
        // $crate::asynchronous::raw::read_raw_func!(read_i16_be, ReadI16Be);
        //
        // $crate::asynchronous::raw::read_raw_func!(read_u32_le, ReadU32Le);
        // $crate::asynchronous::raw::read_raw_func!(read_u32_be, ReadU32Be);
        // $crate::asynchronous::raw::read_raw_func!(read_i32_le, ReadI32Le);
        // $crate::asynchronous::raw::read_raw_func!(read_i32_be, ReadI32Be);
        //
        // $crate::asynchronous::raw::read_raw_func!(read_u64_le, ReadU64Le);
        // $crate::asynchronous::raw::read_raw_func!(read_u64_be, ReadU64Be);
        // $crate::asynchronous::raw::read_raw_func!(read_i64_le, ReadI64Le);
        // $crate::asynchronous::raw::read_raw_func!(read_i64_be, ReadI64Be);
        //
        // $crate::asynchronous::raw::read_raw_func!(read_u128_le, ReadU128Le);
        // $crate::asynchronous::raw::read_raw_func!(read_u128_be, ReadU128Be);
        // $crate::asynchronous::raw::read_raw_func!(read_i128_le, ReadI128Le);
        // $crate::asynchronous::raw::read_raw_func!(read_i128_be, ReadI128Be);
    };
}
pub(crate) use define_read_raw_func;

#[cfg(test)]
mod read_tests {
    

    #[tokio::test]
    async fn read_u8() {

        // assert_eq!([0].as_mut().read_u8_ne().await.unwrap(), 0);
        // assert_eq!([1].as_mut().read_u8_ne().await.unwrap(), 1);
        // assert_eq!([u8::MAX].as_mut().read_u8_ne().await.unwrap(), u8::MAX);
    }

    #[tokio::test]
    async fn read_i8() {
        // assert_eq!([0].as_mut().read_i8_ne().await.unwrap(), 0);
        // assert_eq!([1].as_mut().read_i8_ne().await.unwrap(), 1);
        // assert_eq!([u8::MAX].as_mut().read_i8_ne().await.unwrap(), -1);
    }

    // #[tokio::test]
    // async fn read_u16() {
    //     assert_eq!([1, 2].as_ref().read_u16_le().await.unwrap(), 0x0201);
    //     assert_eq!([1, 2].as_ref().read_u16_be().await.unwrap(), 0x0102);
    // }
}


macro_rules! write_raw_future {
    ($primitive: ty, $future: ident, $func: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: Unpin> where W: ?Sized {
                #[pin]
                writer: &'a mut W,
                #[pin]
                b: $primitive,
            }
        }
        impl<'a, W: $crate::asynchronous::AsyncVariableWritable + Unpin + ?Sized> std::future::Future for $future<'a, W> {
            type Output = std::io::Result<usize>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                let bytes = <$primitive>::to_le_bytes(*me.b);
                ready!(R::poll_write_more(Pin::new(&mut *me.writer), cx, &bytes))?;
                std::task::Poll::Ready(Ok(<$primitive>::$func(bytes)))
        W::poll_write_single(Pin::new(&mut *me.writer), cx, *me.byte)
            }
        }

    };
}



// macro_rules! raw_write {
//     ($primitive: ty, $write_le: ident, $write_be: ident) => {
//         #[inline]
//         #[must_use = "futures do nothing unless you `.await` or poll them"]
//         fn $write_le(&mut self, num: $primitive) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin {
//             Box::pin(async move { self.write_more(&<$primitive>::to_le_bytes(num)).await })
//         }
//         #[inline]
//         #[must_use = "futures do nothing unless you `.await` or poll them"]
//         fn $write_be(&mut self, num: $primitive) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin {
//             Box::pin(async move { self.write_more(&<$primitive>::to_be_bytes(num)).await })
//         }
//     };
// }
// pub(crate) use raw_write;
//
// macro_rules! define_raw_write {
//     () => {
//         #[inline]
//         #[must_use = "futures do nothing unless you `.await` or poll them"]
//         fn write_u8_ne(&mut self, num: u8) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin {
//             Box::pin(async move { self.write_single(num.to_ne_bytes()[0]).await })
//         }
//         #[inline]
//         #[must_use = "futures do nothing unless you `.await` or poll them"]
//         fn write_i8_ne(&mut self, num: i8) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> where Self: Unpin {
//             Box::pin(async move { self.write_single(num.to_ne_bytes()[0]).await })
//         }
//         raw::raw_write!(u16, write_u16_le, write_u16_be);
//         raw::raw_write!(i16, write_i16_le, write_i16_be);
//         raw::raw_write!(u32, write_u32_le, write_u32_be);
//         raw::raw_write!(i32, write_i32_le, write_i32_be);
//         raw::raw_write!(u64, write_u64_le, write_u64_be);
//         raw::raw_write!(i64, write_i64_le, write_i64_be);
//         raw::raw_write!(u128, write_u128_le, write_u128_be);
//         raw::raw_write!(i128, write_i128_le, write_i128_be);
//     };
// }
// pub(crate) use define_raw_write;
