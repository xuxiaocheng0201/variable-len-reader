include!("func/value_generator.rs");

macro_rules! test_func {
    (@ $cursor: ident, $source: expr, $tester: ident, $reader: ident, $writer: ident, |$a: ident, $b: ident| $eq: expr) => { {
        use variable_len_reader::asynchronous::reader::AsyncVariableReader;
        use variable_len_reader::asynchronous::writer::AsyncVariableWriter;
        $cursor.set_position(0);
        let source = $source;
        $cursor.$writer(source).await.expect(&format!("writing failed. tester={}, source={:?}", stringify!($tester), source));
        let len = $cursor.position();
        $cursor.set_position(0);
        let target = $cursor.$reader().await.expect(&format!("reading failed. tester={}, source={:?}, len={}, buffer={:?}", stringify!($tester), source, len, $cursor));
        assert!({ let $a = source.clone(); let $b = target.clone(); $eq },
            "comparing failed. {:?} != {:?}. tester={}. buffer: {:?}", source, target, stringify!($tester), $cursor);
        let pos = $cursor.position();
        assert_eq!(pos, len, "checking failed. {} != {}. tester={}. source={:?}", pos,  len, stringify!($tester), source);
    } };
    ($tester: ident, $reader: ident, $writer: ident @a $primitive: ty) => {
        #[tokio::test]
        async fn $tester() {
            let mut cursor = std::io::Cursor::new(Vec::new());
            for p in <$primitive>::MIN..=<$primitive>::MAX {
                test_func!(@ cursor, p, $tester, $reader, $writer, |a, b| a == b);
            }
        }
    };
    ($tester: ident, $reader: ident, $writer: ident @m $source: expr, |$a: ident, $b: ident| $eq: expr) => {
        #[tokio::test]
        async fn $tester() {
            let mut cursor = std::io::Cursor::new(Vec::new());
            for source in $source {
                test_func!(@ cursor, source, $tester, $reader, $writer, |$a, $b| $eq);
            }
        }
    };
    ($tester: ident, $reader: ident, $writer: ident @m $source: expr) => {
        test_func!($tester, $reader, $writer @m $source, |a, b| a == b);
    };
    ($tester: ident, $reader: ident, $writer: ident @g $primitive: ident) => {
        test_func!($tester, $reader, $writer @m test_value_generator!($primitive));
    };
}

include!("func/bools.rs");

include!("func/raw.rs");

include!("func/varint.rs");
include!("func/varint_signed.rs");
include!("func/varint_float.rs");

test_func!(u8_vec, read_u8_vec_boxed, write_u8_vec_boxed @m [
    &vec![1,2,3],
    &vec![5,4,3,2,1],
], |a, b| a.as_slice() == b.as_slice());

test_func!(string, read_string_boxed, write_string_boxed @m [
    "hello world!",
    include_str!("func/varint.rs") /*a very long string*/,
    "一些非 ASCII 字符",
]);


// #[cfg(all(test, feature = "tokio"))]
// mod tests {
//     use std::time::Duration;
//     use anyhow::Result;
//     use tokio::spawn;
//     use tokio::sync::mpsc::channel;
//     use tokio::task::JoinHandle;
//     use tokio::time::sleep;
//     use crate::asynchronous::AsyncVariableReader;
//     use crate::asynchronous::channel::ReceiverReader;
//
//     #[tokio::test]
//     async fn read_single() -> Result<()> {
//         let buf = [1u8, 2];
//         let mut buf = buf.as_ref();
//         let a = buf.read_single().await?;
//         assert_eq!(a, 1);
//         assert_eq!(buf, &[2]);
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn read_more() -> Result<()> {
//         let buf = [1, 2];
//         let mut buf = buf.as_ref();
//         let mut a = [0, 0];
//         buf.read_more(&mut a).await?;
//         assert_eq!(a, [1, 2]);
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn read_more_twice() -> Result<()> {
//         let (sender, receiver) = channel(1);
//         let mut receiver = ReceiverReader(receiver);
//
//         let j: JoinHandle<Result<()>> = spawn(async move {
//             sender.send(1).await?;
//             sleep(Duration::from_millis(300)).await;
//             sender.send(2).await?;
//             Ok(())
//         });
//         let mut buf = [0, 0];
//         receiver.read_more(buf.as_mut()).await?;
//         assert_eq!(buf, [1, 2]);
//         j.await??;
//         Ok(())
//     }
//
//     #[tokio::test]
//     #[cfg(feature = "bytes")]
//     async fn read_buf() -> Result<()> {
//         use bytes::BytesMut;
//         let mut a = BytesMut::with_capacity(2);
//         [1, 2].as_ref().read_more_buf(2, &mut a).await?;
//         assert_eq!(&a[0..], &[1, 2]);
//         Ok(())
//     }
//
//     #[tokio::test]
//     #[cfg(feature = "bytes")]
//     async fn read_buf_slice() -> Result<()> {
//         use bytes::{BufMut, BytesMut};
//         let mut a = BytesMut::with_capacity(1).chain_mut(BytesMut::with_capacity(1));
//         [1, 2].as_ref().read_more_buf(2, &mut a).await?;
//         assert_eq!(&a.into_inner().0[0..], &[1, 2]); // TODO: optimise?
//         Ok(())
//     }
// }

// #[cfg(all(test, feature = "tokio"))]
// mod tests {
//     use std::time::Duration;
//     use anyhow::Result;
//     use tokio::spawn;
//     use tokio::sync::mpsc::channel;
//     use tokio::task::JoinHandle;
//     use tokio::time::sleep;
//     use crate::asynchronous::AsyncVariableWriter;
//     use crate::asynchronous::channel::SenderWriter;
//
//     #[tokio::test]
//     async fn write_single() -> Result<()> {
//         let mut buf = Vec::with_capacity(1);
//         buf.write_single(1).await?;
//         assert_eq!(&buf, &[1]);
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn write_more() -> Result<()> {
//         let mut buf = Vec::with_capacity(2);
//         buf.write_more(&[1, 2]).await?;
//         assert_eq!(&buf, &[1, 2]);
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn write_more_twice() -> Result<()> {
//         let (sender, mut receiver) = channel(1);
//         let mut sender = SenderWriter(sender);
//         let j: JoinHandle<Result<()>> = spawn(async move {
//             assert_eq!(receiver.recv().await, Some(1));
//             sleep(Duration::from_millis(300)).await;
//             assert_eq!(receiver.recv().await, Some(2));
//             Ok(())
//         });
//         sender.write_more(&[1, 2]).await?;
//         j.await??;
//         Ok(())
//     }
//
//     #[tokio::test]
//     #[cfg(feature = "bytes")]
//     async fn write_buf() -> Result<()> {
//         use bytes::Bytes;
//         let mut buf = Vec::with_capacity(2);
//         buf.write_more_buf(&mut Bytes::from_static(&[1, 2])).await?;
//         assert_eq!(&buf, &[1, 2]);
//         Ok(())
//     }
//
//     #[tokio::test]
//     #[cfg(feature = "bytes")]
//     async fn write_buf_slice() -> Result<()> {
//         use bytes::{Buf, Bytes};
//         let mut buf = Vec::with_capacity(2);
//         buf.write_more_buf(&mut Bytes::from_static(&[1]).chain(Bytes::from_static(&[2]))).await?;
//         assert_eq!(&buf, &[1, 2]);
//         Ok(())
//     }
// }