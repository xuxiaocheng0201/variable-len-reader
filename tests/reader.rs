#![allow(deprecated)]

include!("func/value_generator.rs");

macro_rules! test_func {
    (@ $cursor: ident, $source: expr, $tester: ident, $reader_sync: ident, $reader_async: ident, $writer_sync: ident, |$a: ident, $b: ident| $eq: expr) => { {
        let source = $source;
        {
            use variable_len_reader::synchronous::writer::VariableWriter;
            $cursor.set_position(0);
            $cursor.$writer_sync(source.clone()).unwrap();
        }
        let len = $cursor.position();
        {
            use variable_len_reader::asynchronous::reader::AsyncVariableReader;
            $cursor.set_position(0);
            let target = $cursor.$reader_async().await.expect(&format!("reading failed. async tester={}, source={:?}, len={}, buffer={:?}", stringify!($tester), source, len, $cursor));
            assert!({ let $a = source.clone(); let $b = target.clone(); $eq },
                "comparing failed. {:?} != {:?}. async tester={}. buffer: {:?}", source, target, stringify!($tester), $cursor);
            let pos = $cursor.position();
            assert_eq!(pos, len, "checking failed. {} != {}. async tester={}. source={:?}", pos,  len, stringify!($tester), source);
        }
        {
            use variable_len_reader::synchronous::reader::VariableReader;
            $cursor.set_position(0);
            let target = $cursor.$reader_sync().expect(&format!("reading failed. sync tester={}, source={:?}, len={}, buffer={:?}", stringify!($tester), source, len, $cursor));
            assert!({ let $a = source.clone(); let $b = target.clone(); $eq },
                "comparing failed. {:?} != {:?}. sync tester={}. buffer: {:?}", source, target, stringify!($tester), $cursor);
            let pos = $cursor.position();
            assert_eq!(pos, len, "checking failed. {} != {}. sync tester={}. source={:?}", pos,  len, stringify!($tester), source);
        }
    } };
    ($tester: ident, $reader: ident, $writer: ident @a $primitive: ty) => {
        #[tokio::test]
        async fn $tester() {
            let mut cursor = std::io::Cursor::new(Vec::new());
            for p in <$primitive>::MIN..=<$primitive>::MAX {
                test_func!(@ cursor, p, $tester, $reader, $reader, $writer, |a, b| a == b);
            }
        }
    };
    (S $tester: ident, $reader_sync: ident, $reader_async: ident, $writer_sync: ident @m $source: expr, |$a: ident, $b: ident| $eq: expr) => {
        #[tokio::test]
        async fn $tester() {
            let mut cursor = std::io::Cursor::new(Vec::new());
            for source in $source {
                test_func!(@ cursor, source, $tester, $reader_sync, $reader_async, $writer_sync, |$a, $b| $eq);
            }
        }
    };
    (S $tester: ident, $reader_sync: ident, $reader_async: ident, $writer_sync: ident @m $source: expr) => {
        test_func!(S $tester, $reader_sync, $reader_async, $writer_sync @m $source, |a, b| a == b);
    };
    ($tester: ident, $reader: ident, $writer: ident @m $source: expr, |$a: ident, $b: ident| $eq: expr) => {
        test_func!(S $tester, $reader, $reader, $writer @m $source, |$a, $b| $eq);
    };
    ($tester: ident, $reader: ident, $writer: ident @m $source: expr) => {
        test_func!(S $tester, $reader, $reader, $writer @m $source);
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

include!("func/string.rs");

test_func!(S u8_vec, read_u8_vec, read_u8_vec_boxed, write_u8_vec @m test_u8_vec_values!());

test_func!(S string, read_string, read_string_boxed, write_string @m test_string_values!());
