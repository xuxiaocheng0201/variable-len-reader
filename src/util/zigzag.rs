pub trait Zigzag<T> {
    fn zigzag(self) -> T;
}

impl Zigzag<u8> for i8 {
    #[inline]
    fn zigzag(self) -> u8 {
        ((self << 1) ^ (self >> 7)) as u8
    }
}

impl Zigzag<i8> for u8 {
    #[inline]
    fn zigzag(self) -> i8 {
        ((self >> 1) as i8) ^ (-((self & 1) as i8))
    }
}

impl Zigzag<u16> for i16 {
    #[inline]
    fn zigzag(self) -> u16 {
        ((self << 1) ^ (self >> 15)) as u16
    }
}

impl Zigzag<i16> for u16 {
    #[inline]
    fn zigzag(self) -> i16 {
        ((self >> 1) as i16) ^ (-((self & 1) as i16))
    }
}

impl Zigzag<u32> for i32 {
    #[inline]
    fn zigzag(self) -> u32 {
        ((self << 1) ^ (self >> 31)) as u32
    }
}

impl Zigzag<i32> for u32 {
    #[inline]
    fn zigzag(self) -> i32 {
        ((self >> 1) as i32) ^ (-((self & 1) as i32))
    }
}

impl Zigzag<u64> for i64 {
    #[inline]
    fn zigzag(self) -> u64 {
        ((self << 1) ^ (self >> 63)) as u64
    }
}

impl Zigzag<i64> for u64 {
    #[inline]
    fn zigzag(self) -> i64 {
        ((self >> 1) as i64) ^ (-((self & 1) as i64))
    }
}

impl Zigzag<u128> for i128 {
    #[inline]
    fn zigzag(self) -> u128 {
        ((self << 1) ^ (self >> 127)) as u128
    }
}

impl Zigzag<i128> for u128 {
    #[inline]
    fn zigzag(self) -> i128 {
        ((self >> 1) as i128) ^ (-((self & 1) as i128))
    }
}

impl Zigzag<usize> for isize {
    #[inline]
    fn zigzag(self) -> usize {
        ((self << 1) ^ (self >> (usize::BITS - 1))) as usize
    }
}

impl Zigzag<isize> for usize {
    #[inline]
    fn zigzag(self) -> isize {
        ((self >> 1) as isize) ^ (-((self & 1) as isize))
    }
}
