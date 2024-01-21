
pub(crate) use read_raw_future;

pub(crate) use read_raw_func;

pub(crate) use define_read_raw_futures;

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
        assert_eq!([0].as_ref().read_u8_raw().await?, 0);
        assert_eq!([1].as_ref().read_u8_raw().await?, 1);
        assert_eq!([u8::MAX].as_ref().read_u8_raw().await?, u8::MAX);
        Ok(())
    }

    #[tokio::test]
    async fn read_i8() -> Result<()> {
        assert_eq!([0].as_ref().read_i8_raw().await?, 0);
        assert_eq!([1].as_ref().read_i8_raw().await?, 1);
        assert_eq!([u8::MAX].as_ref().read_i8_raw().await?, -1);
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
