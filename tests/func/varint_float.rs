test_func!(f32_varint, read_f32_varint, write_f32_varint @g f32);
test_func!(f32_varint_2_le, read_f32_varint_2_le, write_f32_varint_2_le @g f32);
test_func!(f32_varint_2_be, read_f32_varint_2_be, write_f32_varint_2_be @g f32);
test_func!(f32_varint_4_le, read_f32_varint_4_le, write_f32_varint_4_le @g f32);
test_func!(f32_varint_4_be, read_f32_varint_4_be, write_f32_varint_4_be @g f32);

test_func!(f64_varint, read_f64_varint, write_f64_varint @g f64);
test_func!(f64_varint_2_le, read_f64_varint_2_le, write_f64_varint_2_le @g f64);
test_func!(f64_varint_2_be, read_f64_varint_2_be, write_f64_varint_2_be @g f64);
test_func!(f64_varint_4_le, read_f64_varint_4_le, write_f64_varint_4_le @g f64);
test_func!(f64_varint_4_be, read_f64_varint_4_be, write_f64_varint_4_be @g f64);
test_func!(f64_varint_8_le, read_f64_varint_8_le, write_f64_varint_8_le @g f64);
test_func!(f64_varint_8_be, read_f64_varint_8_be, write_f64_varint_8_be @g f64);
