macro_rules! read_raw_future {
    ($primitive: ty, $future: ident, $func: ident) => {
        $crate::pin_project_lite::pin_project! {
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, R: ?Sized> {
                #[pin]
                reader: &'a mut R,
                buf: [u8; std::mem::size_of::<$primitive>()],
                read: usize,
            }
        }
        impl<'a, R: $crate::asynchronous::AsyncVariableReadable + Unpin + ?Sized> std::future::Future for $future<'a, R> {
            type Output = std::io::Result<$primitive>;

            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
                let mut me = self.project();
                let byte = ready!(R::poll_read_single(std::pin::Pin::new(&mut *me.reader), cx))?;
                (*me.buf)[*me.read] = byte;
                *me.read += 1;
                if *me.read >= (*me.buf).len() {
                    std::task::Poll::Ready(Ok(<$primitive>::$func(*me.buf)))
                } else {
                    cx.waker().clone().wake();
                    std::task::Poll::Pending
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
        $crate::asynchronous::raw::read_raw_future!(u8, ReadU8RawNe, from_ne_bytes);
        $crate::asynchronous::raw::read_raw_future!(i8, ReadI8RawNe, from_ne_bytes);

        $crate::asynchronous::raw::read_raw_future!(u16, ReadU16RawLe, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(u16, ReadU16RawBe, from_be_bytes);
        $crate::asynchronous::raw::read_raw_future!(i16, ReadI16RawLe, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(i16, ReadI16RawBe, from_be_bytes);

        $crate::asynchronous::raw::read_raw_future!(u32, ReadU32RawLe, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(u32, ReadU32RawBe, from_be_bytes);
        $crate::asynchronous::raw::read_raw_future!(i32, ReadI32RawLe, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(i32, ReadI32RawBe, from_be_bytes);

        $crate::asynchronous::raw::read_raw_future!(u64, ReadU64RawLe, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(u64, ReadU64RawBe, from_be_bytes);
        $crate::asynchronous::raw::read_raw_future!(i64, ReadI64RawLe, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(i64, ReadI64RawBe, from_be_bytes);

        $crate::asynchronous::raw::read_raw_future!(u128, ReadU128RawLe, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(u128, ReadU128RawBe, from_be_bytes);
        $crate::asynchronous::raw::read_raw_future!(i128, ReadI128RawLe, from_le_bytes);
        $crate::asynchronous::raw::read_raw_future!(i128, ReadI128RawBe, from_be_bytes);
    };
}
pub(crate) use define_read_raw_futures;

macro_rules! define_read_raw_func {
    () => {
        $crate::asynchronous::raw::read_raw_func!(u8, read_u8_raw_ne, ReadU8RawNe);
        $crate::asynchronous::raw::read_raw_func!(i8, read_i8_raw_ne, ReadI8RawNe);

        $crate::asynchronous::raw::read_raw_func!(u16, read_u16_raw_le, ReadU16RawLe);
        $crate::asynchronous::raw::read_raw_func!(u16, read_u16_raw_be, ReadU16RawBe);
        $crate::asynchronous::raw::read_raw_func!(i16, read_i16_raw_le, ReadI16RawLe);
        $crate::asynchronous::raw::read_raw_func!(i16, read_i16_raw_be, ReadI16RawBe);

        $crate::asynchronous::raw::read_raw_func!(u32, read_u32_raw_le, ReadU32RawLe);
        $crate::asynchronous::raw::read_raw_func!(u32, read_u32_raw_be, ReadU32RawBe);
        $crate::asynchronous::raw::read_raw_func!(i32, read_i32_raw_le, ReadI32RawLe);
        $crate::asynchronous::raw::read_raw_func!(i32, read_i32_raw_be, ReadI32RawBe);

        $crate::asynchronous::raw::read_raw_func!(u64, read_u64_raw_le, ReadU64RawLe);
        $crate::asynchronous::raw::read_raw_func!(u64, read_u64_raw_be, ReadU64RawBe);
        $crate::asynchronous::raw::read_raw_func!(i64, read_i64_raw_le, ReadI64RawLe);
        $crate::asynchronous::raw::read_raw_func!(i64, read_i64_raw_be, ReadI64RawBe);

        $crate::asynchronous::raw::read_raw_func!(u128, read_u128_raw_le, ReadU128RawLe);
        $crate::asynchronous::raw::read_raw_func!(u128, read_u128_raw_be, ReadU128RawBe);
        $crate::asynchronous::raw::read_raw_func!(i128, read_i128_raw_le, ReadI128RawLe);
        $crate::asynchronous::raw::read_raw_func!(i128, read_i128_raw_be, ReadI128RawBe);
    };
}
pub(crate) use define_read_raw_func;

#[cfg(test)]
mod read_tests {
    use std::time::Duration;
    use anyhow::Result;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::spawn;
    use tokio::task::JoinHandle;
    use tokio::time::sleep;
    use crate::asynchronous::AsyncVariableReader;

    #[tokio::test]
    async fn read_u8() -> Result<()> {
        assert_eq!([0].as_ref().read_u8_raw_ne().await?, 0);
        assert_eq!([1].as_ref().read_u8_raw_ne().await?, 1);
        assert_eq!([u8::MAX].as_ref().read_u8_raw_ne().await?, u8::MAX);
        Ok(())
    }

    #[tokio::test]
    async fn read_i8() -> Result<()> {
        assert_eq!([0].as_ref().read_i8_raw_ne().await?, 0);
        assert_eq!([1].as_ref().read_i8_raw_ne().await?, 1);
        assert_eq!([u8::MAX].as_ref().read_i8_raw_ne().await?, -1);
        Ok(())
    }

    #[tokio::test]
    async fn read_u16() -> Result<()> {
        assert_eq!([1, 2].as_ref().read_u16_raw_le().await?, 0x0201);
        assert_eq!([1, 2].as_ref().read_u16_raw_be().await?, 0x0102);
        Ok(())
    }

    #[tokio::test]
    async fn read_u16_twice() -> Result<()> {
        let server = TcpListener::bind("localhost:0").await?;
        let mut client = TcpStream::connect(server.local_addr()?).await?;
        let mut server = server.accept().await?.0;

        let _: JoinHandle<Result<()>> = spawn(async move {
            use tokio::io::AsyncWriteExt;
            server.write_all(&[2]).await?;
            sleep(Duration::from_millis(300)).await;
            server.write_all(&[1]).await?;
            Ok(())
        });
        assert_eq!(client.read_u16_raw_le().await?, 0x0102);
        Ok(())
    }
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
