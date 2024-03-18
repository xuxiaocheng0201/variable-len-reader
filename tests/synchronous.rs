include!("func/value_generator.rs");

macro_rules! test_func {
    (@ $cursor: ident, $source: expr, $tester: ident, $reader: ident, $writer: ident, |$a: ident, $b: ident| $eq: expr) => { {
        use variable_len_reader::synchronous::reader::VariableReader;
        use variable_len_reader::synchronous::writer::VariableWriter;
        $cursor.set_position(0);
        let source = $source;
        $cursor.$writer(source).expect(&format!("writing failed. tester={}, source={:?}", stringify!($tester), source));
        let len = $cursor.position();
        $cursor.set_position(0);
        let target = $cursor.$reader().expect(&format!("reading failed. tester={}, source={:?}, len={}, buffer={:?}", stringify!($tester), source, len, $cursor));
        assert!({ let $a = source.clone(); let $b = target.clone(); $eq },
            "comparing failed. {:?} != {:?}. tester={}. buffer: {:?}", source, target, stringify!($tester), $cursor);
        let pos = $cursor.position();
        assert_eq!(pos, len, "checking failed. {} != {}. tester={}. source={:?}", pos,  len, stringify!($tester), source);
    } };
    ($tester: ident, $reader: ident, $writer: ident @a $primitive: ty) => {
        #[test]
        fn $tester() {
            let mut cursor = std::io::Cursor::new(Vec::new());
            for p in <$primitive>::MIN..=<$primitive>::MAX {
                test_func!(@ cursor, p, $tester, $reader, $writer, |a, b| a == b);
            }
        }
    };
    ($tester: ident, $reader: ident, $writer: ident @m $source: expr, |$a: ident, $b: ident| $eq: expr) => {
        #[test]
        fn $tester() {
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

include!("func/string.rs");

test_func!(u8_vec, read_u8_vec, write_u8_vec @m test_u8_vec_values!(), |a, b| a.as_slice() == b.as_slice());

test_func!(string, read_string, write_string @m test_string_values!());
