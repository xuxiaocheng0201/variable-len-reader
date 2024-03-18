test_func!(i8_varint, read_i8_varint, write_i8_varint @a i8);

test_func!(i16_varint, read_i16_varint, write_i16_varint @a i16);
test_func!(i16_varint_2_le, read_i16_varint_2_le, write_i16_varint_2_le @a i16);
test_func!(i16_varint_2_be, read_i16_varint_2_be, write_i16_varint_2_be @a i16);

test_func!(i32_varint, read_i32_varint, write_i32_varint @g i32);
test_func!(i32_varint_2_le, read_i32_varint_2_le, write_i32_varint_2_le @g i32);
test_func!(i32_varint_2_be, read_i32_varint_2_be, write_i32_varint_2_be @g i32);
test_func!(i32_varint_4_le, read_i32_varint_4_le, write_i32_varint_4_le @g i32);
test_func!(i32_varint_4_be, read_i32_varint_4_be, write_i32_varint_4_be @g i32);

test_func!(i64_varint, read_i64_varint, write_i64_varint @g i64);
test_func!(i64_varint_2_le, read_i64_varint_2_le, write_i64_varint_2_le @g i64);
test_func!(i64_varint_2_be, read_i64_varint_2_be, write_i64_varint_2_be @g i64);
test_func!(i64_varint_4_le, read_i64_varint_4_le, write_i64_varint_4_le @g i64);
test_func!(i64_varint_4_be, read_i64_varint_4_be, write_i64_varint_4_be @g i64);
test_func!(i64_varint_8_le, read_i64_varint_8_le, write_i64_varint_8_le @g i64);
test_func!(i64_varint_8_be, read_i64_varint_8_be, write_i64_varint_8_be @g i64);

test_func!(i128_varint, read_i128_varint, write_i128_varint @g i128);
test_func!(i128_varint_2_le, read_i128_varint_2_le, write_i128_varint_2_le @g i128);
test_func!(i128_varint_2_be, read_i128_varint_2_be, write_i128_varint_2_be @g i128);
test_func!(i128_varint_4_le, read_i128_varint_4_le, write_i128_varint_4_le @g i128);
test_func!(i128_varint_4_be, read_i128_varint_4_be, write_i128_varint_4_be @g i128);
test_func!(i128_varint_8_le, read_i128_varint_8_le, write_i128_varint_8_le @g i128);
test_func!(i128_varint_8_be, read_i128_varint_8_be, write_i128_varint_8_be @g i128);
test_func!(i128_varint_16_le, read_i128_varint_16_le, write_i128_varint_16_le @g i128);
test_func!(i128_varint_16_be, read_i128_varint_16_be, write_i128_varint_16_be @g i128);


test_func!(isize_varint, read_isize_varint, write_isize_varint @g isize);
test_func!(isize_varint_2_le, read_isize_varint_2_le, write_isize_varint_2_le @g isize);
test_func!(isize_varint_2_be, read_isize_varint_2_be, write_isize_varint_2_be @g isize);
test_func!(isize_varint_4_le, read_isize_varint_4_le, write_isize_varint_4_le @g isize);
test_func!(isize_varint_4_be, read_isize_varint_4_be, write_isize_varint_4_be @g isize);
test_func!(isize_varint_8_le, read_isize_varint_8_le, write_isize_varint_8_le @g isize);
test_func!(isize_varint_8_be, read_isize_varint_8_be, write_isize_varint_8_be @g isize);
test_func!(isize_varint_16_le, read_isize_varint_16_le, write_isize_varint_16_le @g isize);
test_func!(isize_varint_16_be, read_isize_varint_16_be, write_isize_varint_16_be @g isize);

test_func!(isize_varint_ap, read_isize_varint_ap, write_isize_varint_ap @g isize);
test_func!(isize_varint_2_le_ap, read_isize_varint_2_le_ap, write_isize_varint_2_le_ap @g isize);
test_func!(isize_varint_2_be_ap, read_isize_varint_2_be_ap, write_isize_varint_2_be_ap @g isize);
test_func!(isize_varint_4_le_ap, read_isize_varint_4_le_ap, write_isize_varint_4_le_ap @g isize);
test_func!(isize_varint_4_be_ap, read_isize_varint_4_be_ap, write_isize_varint_4_be_ap @g isize);
test_func!(isize_varint_8_le_ap, read_isize_varint_8_le_ap, write_isize_varint_8_le_ap @g isize);
test_func!(isize_varint_8_be_ap, read_isize_varint_8_be_ap, write_isize_varint_8_be_ap @g isize);
test_func!(isize_varint_16_le_ap, read_isize_varint_16_le_ap, write_isize_varint_16_le_ap @g isize);
test_func!(isize_varint_16_be_ap, read_isize_varint_16_be_ap, write_isize_varint_16_be_ap @g isize);
