test_func!(u8_varint, read_u8_varint, write_u8_varint @a u8);

test_func!(u16_varint, read_u16_varint, write_u16_varint @a u16);
test_func!(u16_varint_2_le, read_u16_varint_2_le, write_u16_varint_2_le @a u16);
test_func!(u16_varint_2_be, read_u16_varint_2_be, write_u16_varint_2_be @a u16);

test_func!(u32_varint, read_u32_varint, write_u32_varint @g u32);
test_func!(u32_varint_2_le, read_u32_varint_2_le, write_u32_varint_2_le @g u32);
test_func!(u32_varint_2_be, read_u32_varint_2_be, write_u32_varint_2_be @g u32);
test_func!(u32_varint_4_le, read_u32_varint_4_le, write_u32_varint_4_le @g u32);
test_func!(u32_varint_4_be, read_u32_varint_4_be, write_u32_varint_4_be @g u32);

test_func!(u64_varint, read_u64_varint, write_u64_varint @g u64);
test_func!(u64_varint_2_le, read_u64_varint_2_le, write_u64_varint_2_le @g u64);
test_func!(u64_varint_2_be, read_u64_varint_2_be, write_u64_varint_2_be @g u64);
test_func!(u64_varint_4_le, read_u64_varint_4_le, write_u64_varint_4_le @g u64);
test_func!(u64_varint_4_be, read_u64_varint_4_be, write_u64_varint_4_be @g u64);
test_func!(u64_varint_8_le, read_u64_varint_8_le, write_u64_varint_8_le @g u64);
test_func!(u64_varint_8_be, read_u64_varint_8_be, write_u64_varint_8_be @g u64);

test_func!(u128_varint, read_u128_varint, write_u128_varint @g u128);
test_func!(u128_varint_2_le, read_u128_varint_2_le, write_u128_varint_2_le @g u128);
test_func!(u128_varint_2_be, read_u128_varint_2_be, write_u128_varint_2_be @g u128);
test_func!(u128_varint_4_le, read_u128_varint_4_le, write_u128_varint_4_le @g u128);
test_func!(u128_varint_4_be, read_u128_varint_4_be, write_u128_varint_4_be @g u128);
test_func!(u128_varint_8_le, read_u128_varint_8_le, write_u128_varint_8_le @g u128);
test_func!(u128_varint_8_be, read_u128_varint_8_be, write_u128_varint_8_be @g u128);
test_func!(u128_varint_16_le, read_u128_varint_16_le, write_u128_varint_16_le @g u128);
test_func!(u128_varint_16_be, read_u128_varint_16_be, write_u128_varint_16_be @g u128);


test_func!(usize_varint, read_usize_varint, write_usize_varint @g usize);
test_func!(usize_varint_2_le, read_usize_varint_2_le, write_usize_varint_2_le @g usize);
test_func!(usize_varint_2_be, read_usize_varint_2_be, write_usize_varint_2_be @g usize);
test_func!(usize_varint_4_le, read_usize_varint_4_le, write_usize_varint_4_le @g usize);
test_func!(usize_varint_4_be, read_usize_varint_4_be, write_usize_varint_4_be @g usize);
test_func!(usize_varint_8_le, read_usize_varint_8_le, write_usize_varint_8_le @g usize);
test_func!(usize_varint_8_be, read_usize_varint_8_be, write_usize_varint_8_be @g usize);
test_func!(usize_varint_16_le, read_usize_varint_16_le, write_usize_varint_16_le @g usize);
test_func!(usize_varint_16_be, read_usize_varint_16_be, write_usize_varint_16_be @g usize);

test_func!(usize_varint_ap, read_usize_varint_ap, write_usize_varint_ap @g usize);
test_func!(usize_varint_2_le_ap, read_usize_varint_2_le_ap, write_usize_varint_2_le_ap @g usize);
test_func!(usize_varint_2_be_ap, read_usize_varint_2_be_ap, write_usize_varint_2_be_ap @g usize);
test_func!(usize_varint_4_le_ap, read_usize_varint_4_le_ap, write_usize_varint_4_le_ap @g usize);
test_func!(usize_varint_4_be_ap, read_usize_varint_4_be_ap, write_usize_varint_4_be_ap @g usize);
test_func!(usize_varint_8_le_ap, read_usize_varint_8_le_ap, write_usize_varint_8_le_ap @g usize);
test_func!(usize_varint_8_be_ap, read_usize_varint_8_be_ap, write_usize_varint_8_be_ap @g usize);
test_func!(usize_varint_16_le_ap, read_usize_varint_16_le_ap, write_usize_varint_16_le_ap @g usize);
test_func!(usize_varint_16_be_ap, read_usize_varint_16_be_ap, write_usize_varint_16_be_ap @g usize);
