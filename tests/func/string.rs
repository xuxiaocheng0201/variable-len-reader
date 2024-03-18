macro_rules! test_u8_vec_values {
    ($value: expr, |$v: ident| $convert: expr) => { {
        let $v = &*$value.leak(); $convert // This leak is not safety, but this is just a test.
    } };
    (|$v: ident| $convert: expr) => { [
        test_u8_vec_values!(vec![1,2,3], |$v| $convert),
        test_u8_vec_values!(vec![5,4,3,2,1], |$v| $convert),
        test_u8_vec_values!(vec![1; 1024], |$v| $convert),
    ] };
    () => {
        test_u8_vec_values!(|v| v)
    };
}

macro_rules! test_string_values {
    ($value: expr, |$v: ident| $convert: expr) => { {
        let $v = $value; $convert
    } };
    (|$v: ident| $convert: expr) => { [
        test_string_values!("hello world!", |$v| $convert),
        test_string_values!(include_str!("func/varint.rs"), |$v| $convert), // a very long string.
        test_string_values!("一些非 ASCII 字符", |$v| $convert),
    ] };
    () => {
        test_string_values!(|v| v)
    };
}
