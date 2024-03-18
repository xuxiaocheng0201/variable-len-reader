#![allow(deprecated)]

include!("func/value_generator.rs");

macro_rules! test_func {
    (@ $cursor: ident, $source: expr, $tester: ident, $writer_sync: ident, $writer_async: ident, |$a: ident, $b: ident| $eq: expr) => { {
        let source = $source;
        let a = {
            use variable_len_reader::synchronous::writer::VariableWriter;
            $cursor.set_position(0);
            $cursor.$writer_sync(source.clone()).unwrap();
            $cursor.get_ref().to_vec()
        };
        let b = {
            use variable_len_reader::asynchronous::writer::AsyncVariableWriter;
            $cursor.set_position(0);
            $cursor.$writer_async(source.clone()).await.unwrap();
            $cursor.get_ref().to_vec()
        };
        assert_eq!(a, b, "comparing failed. {:?} != {:?}. tester={}, source={:?}", a, b, stringify!($tester), source);
    } };
    ($tester: ident, $_reader: ident, $writer: ident @a $primitive: ty) => {
        #[tokio::test]
        async fn $tester() {
            let mut cursor = std::io::Cursor::new(Vec::new());
            for p in <$primitive>::MIN..=<$primitive>::MAX {
                test_func!(@ cursor, p, $tester, $writer, $writer, |a, b| a == b);
            }
        }
    };
    (S $tester: ident, $writer_sync: ident, $writer_async: ident @m $source: expr, |$a: ident, $b: ident| $eq: expr) => {
        #[tokio::test]
        async fn $tester() {
            let mut cursor = std::io::Cursor::new(Vec::new());
            for source in $source {
                test_func!(@ cursor, source, $tester, $writer_sync, $writer_async, |$a, $b| $eq);
            }
        }
    };
    (S $tester: ident, $writer_sync: ident, $writer_async: ident @m $source: expr) => {
        test_func!(S $tester, $writer_sync, $writer_async @m $source, |a, b| a == b);
    };
    ($tester: ident, $_reader: ident, $writer: ident @m $source: expr, |$a: ident, $b: ident| $eq: expr) => {
        test_func!(S $tester, $writer, $writer @m $source, |$a, $b| $eq);
    };
    ($tester: ident, $_reader: ident, $writer: ident @m $source: expr) => {
        test_func!(S $tester, $writer, $writer @m $source);
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

test_func!(S u8_vec, write_u8_vec, write_u8_vec_boxed @m test_u8_vec_values!());

test_func!(S string, write_string, write_string_boxed @m test_string_values!());
