macro_rules! test_value_generator {
    (u8) => { test_value_generator!(u, u8) };
    (u16) => { test_value_generator!(u, u16) };
    (u32) => { test_value_generator!(u, u32) };
    (u64) => { test_value_generator!(u, u64) };
    (u128) => { test_value_generator!(u, u128) };
    (usize) => { test_value_generator!(u, usize) };
    (u, $primitive: ty) => {
        [0, 1, 2, <$primitive>::MAX - 1, <$primitive>::MAX,]
    };

    (i8) => { test_value_generator!(i, i8) };
    (i16) => { test_value_generator!(i, i16) };
    (i32) => { test_value_generator!(i, i32) };
    (i64) => { test_value_generator!(i, i64) };
    (i128) => { test_value_generator!(i, i128) };
    (isize) => { test_value_generator!(i, isize) };
    (i, $primitive: ty) => {
        [0, 1, 2, -1, -2, <$primitive>::MIN, <$primitive>::MIN + 1, <$primitive>::MAX - 1, <$primitive>::MAX,]
    };

    (f32) => { test_value_generator!(f, f32) };
    (f64) => { test_value_generator!(f, f64) };
    (f, $primitive: ty) => {
        [0.0, 1.0, 2.0, -1.0, -2.0, <$primitive>::MIN, <$primitive>::MIN + 1.0, <$primitive>::MAX - 1.0, <$primitive>::MAX,]
    };
}
