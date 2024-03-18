test_func!(u8_raw_ne, read_u8_raw, write_u8_raw @a u8);
test_func!(i8_raw_ne, read_i8_raw, write_i8_raw @a i8);

test_func!(u16_raw_le, read_u16_raw_le, write_u16_raw_le @a u16);
test_func!(u16_raw_be, read_u16_raw_be, write_u16_raw_be @a u16);
test_func!(i16_raw_le, read_i16_raw_le, write_i16_raw_le @a i16);
test_func!(i16_raw_be, read_i16_raw_be, write_i16_raw_be @a i16);

test_func!(u32_raw_le, read_u32_raw_le, write_u32_raw_le @g u32);
test_func!(u32_raw_be, read_u32_raw_be, write_u32_raw_be @g u32);
test_func!(i32_raw_le, read_i32_raw_le, write_i32_raw_le @g i32);
test_func!(i32_raw_be, read_i32_raw_be, write_i32_raw_be @g i32);

test_func!(u64_raw_le, read_u64_raw_le, write_u64_raw_le @g u64);
test_func!(u64_raw_be, read_u64_raw_be, write_u64_raw_be @g u64);
test_func!(i64_raw_le, read_i64_raw_le, write_i64_raw_le @g i64);
test_func!(i64_raw_be, read_i64_raw_be, write_i64_raw_be @g i64);

test_func!(u128_raw_le, read_u128_raw_le, write_u128_raw_le @g u128);
test_func!(u128_raw_be, read_u128_raw_be, write_u128_raw_be @g u128);
test_func!(i128_raw_le, read_i128_raw_le, write_i128_raw_le @g i128);
test_func!(i128_raw_be, read_i128_raw_be, write_i128_raw_be @g i128);


test_func!(f32_raw_le, read_f32_raw_le, write_f32_raw_le @g f32);
test_func!(f32_raw_be, read_f32_raw_be, write_f32_raw_be @g f32);
test_func!(f64_raw_le, read_f64_raw_le, write_f64_raw_le @g f64);
test_func!(f64_raw_be, read_f64_raw_be, write_f64_raw_be @g f64);


test_func!(usize_raw_le, read_usize_raw_le, write_usize_raw_le @g usize);
test_func!(usize_raw_be, read_usize_raw_be, write_usize_raw_be @g usize);
test_func!(isize_raw_le, read_isize_raw_le, write_isize_raw_le @g isize);
test_func!(isize_raw_be, read_isize_raw_be, write_isize_raw_be @g isize);

test_func!(usize_raw_le_ap, read_usize_raw_le_ap, write_usize_raw_le_ap @g usize);
test_func!(usize_raw_be_ap, read_usize_raw_be_ap, write_usize_raw_be_ap @g usize);
test_func!(isize_raw_le_ap, read_isize_raw_le_ap, write_isize_raw_le_ap @g isize);
test_func!(isize_raw_be_ap, read_isize_raw_be_ap, write_isize_raw_be_ap @g isize);
