macro_rules! impl_read_buf {
    () => {
        #[inline]
        pub fn advance(&mut self, cnt: usize) {
            let new = self.filled.checked_add(cnt).expect("filled overflow");
            assert!(
                new <= self.buf.len(),
                "filled must not become larger than buf.len()"
            );
            self.filled = new;
        }

        #[inline]
        pub fn buf(&self) -> &[u8] {
            &self.buf
        }

        #[inline]
        pub fn buf_mut(&mut self) -> &mut [u8] {
            &mut self.buf
        }

        #[inline]
        pub fn filled(&self) -> usize {
            self.filled
        }

        #[inline]
        pub fn left(&self) -> usize {
            self.buf.len() - self.filled
        }

        #[inline]
        pub fn clear(&mut self) {
            self.filled = 0;
        }

        #[inline]
        pub fn set_filled(&mut self, filled: usize) {
            assert!(
                self.buf.len() >= filled,
                "filled must not become larger than buf.len()"
            );
            self.filled = filled;
        }

        #[inline]
        pub fn set(&mut self, val: u8) {
            assert!(
                self.left() >= 1,
                "left() must large than 1"
            );
            self.buf[self.filled] = val;
        }

        #[inline]
        pub fn put(&mut self, val: u8) {
            self.set(val);
            self.filled += 1;
        }

        #[inline]
        pub fn set_slice(&mut self, slice: &[u8]) {
            assert!(
                self.left() >= slice.len(),
                "buf.len() must fit in remaining()"
            );
            self.buf[self.filled..self.filled + slice.len()].copy_from_slice(slice);
        }

        #[inline]
        pub fn put_slice(&mut self, slice: &[u8]) {
            self.set_slice(slice);
            self.filled += slice.len();
        }
    };
}
#[cfg(feature = "bytes")]
#[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
macro_rules! impl_buf_mut {
    () => {
        #[inline]
        fn remaining_mut(&self) -> usize {
            self.left()
        }

        #[inline]
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.advance(cnt)
        }

        #[inline]
        fn chunk_mut(&mut self) -> &mut bytes::buf::UninitSlice {
            bytes::buf::UninitSlice::new(self.buf_mut())
        }
    };
}

#[derive(Debug)]
pub struct ReadBuf<'a> {
    buf: &'a mut [u8],
    filled: usize,
}

impl<'a> ReadBuf<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self {
            buf,
            filled: 0,
        }
    }
    impl_read_buf!();
}

#[cfg(feature = "bytes")]
#[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
unsafe impl<'a> bytes::BufMut for ReadBuf<'a> {
    impl_buf_mut!();
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl<'a, 'b> From<&'b mut ReadBuf<'a>> for tokio::io::ReadBuf<'b> {
    #[inline]
    fn from(value: &'b mut ReadBuf<'a>) -> Self {
        let mut buf = Self::new(value.buf);
        buf.set_filled(value.filled);
        buf
    }
}


macro_rules! impl_write_buf {
    () => {
        #[inline]
        pub fn skip(&mut self, cnt: usize) {
            let new = self.read.checked_add(cnt).expect("read overflow");
            assert!(
                new <= self.buf.len(),
                "read must not become larger than buf.len()"
            );
            self.read = new;
        }

        #[inline]
        pub fn buf(&self) -> &[u8] {
            &self.buf
        }

        #[inline]
        pub fn read(&self) -> usize {
            self.read
        }

        #[inline]
        pub fn left(&self) -> usize {
            self.buf.len() - self.read
        }

        #[inline]
        pub fn reset(&mut self) {
            self.read = 0;
        }

        #[inline]
        pub fn set_read(&mut self, read: usize) {
            assert!(
                self.buf.len() >= read,
                "read must not become larger than buf.len()"
            );
            self.read = read;
        }

        #[inline]
        pub fn get(&self) -> u8 {
            assert!(
                self.left() >= 1,
                "left() must large than 1"
            );
            self.buf[self.read]
        }

        #[inline]
        pub fn take(&mut self) -> u8 {
            let val = self.get();
            self.read += 1;
            val
        }

        #[inline]
        pub fn get_slice(&self, len: usize) -> &[u8] {
            assert!(
                self.left() >= len,
                "left() must large than len"
            );
            &self.buf[self.read..self.read + len]
        }

        #[inline]
        pub fn take_slice(&mut self, len: usize) -> &[u8] {
            assert!(
                self.left() >= len,
                "left() must large than len"
            );
            let slice = &self.buf[self.read..self.read + len];
            self.read += len;
            slice
        }
    };
}
#[cfg(feature = "bytes")]
#[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
macro_rules! impl_buf {
    () => {
        #[inline]
        fn remaining(&self) -> usize {
            self.left()
        }

        #[inline]
        fn chunk(&self) -> &[u8] {
            self.buf
        }

        #[inline]
        fn advance(&mut self, cnt: usize) {
            self.skip(cnt)
        }
    };
}

#[derive(Debug)]
pub struct WriteBuf<'a> {
    buf: &'a [u8],
    read: usize,
}

impl<'a> WriteBuf<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            read: 0,
        }
    }
    impl_write_buf!();
}

#[cfg(feature = "bytes")]
#[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
impl<'a> bytes::Buf for WriteBuf<'a> {
    impl_buf!();
}


#[macro_export]
macro_rules! define_read_buf {
    ($name: ident, $n: expr) => {
        #[derive(Debug)]
        pub struct $name {
            buf: [u8; $n],
            filled: usize,
        }
        impl $name {
            #[inline]
            pub fn new() -> Self {
                Self {
                    buf: [0; $n],
                    filled: 0,
                }
            }
            #[inline]
            pub fn into_inner(&self) -> [u8; $n] {
                self.buf
            }
            impl_read_buf!();
        }
        #[cfg(feature = "bytes")]
        #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
        unsafe impl bytes::BufMut for $name {
            impl_buf_mut!();
        }
        impl<'a> From<&'a mut $name> for ReadBuf<'a> {
            #[inline]
            fn from(value: &'a mut $name) -> Self {
                Self {
                    buf: &mut value.buf,
                    filled: value.filled,
                }
            }
        }
    };
}

define_read_buf!(OwnedReadBuf8, std::mem::size_of::<u8>());
define_read_buf!(OwnedReadBuf16, std::mem::size_of::<u16>());
define_read_buf!(OwnedReadBuf32, std::mem::size_of::<u32>());
define_read_buf!(OwnedReadBuf64, std::mem::size_of::<u64>());
define_read_buf!(OwnedReadBuf128, std::mem::size_of::<u128>());


#[macro_export]
macro_rules! define_write_buf {
    ($name: ident, $n: expr) => {
        #[derive(Debug)]
        pub struct $name {
            buf: [u8; $n],
            read: usize,
        }
        impl $name {
            #[inline]
            pub fn new(buf: [u8; $n]) -> Self {
                Self {
                    buf,
                    read: 0,
                }
            }
            #[inline]
            pub fn into_inner(&self) -> [u8; $n] {
                self.buf
            }
            #[inline]
            pub fn buf_mut(&mut self) -> &mut [u8] {
                &mut self.buf
            }
            #[inline]
            pub fn set_buf(&mut self, buf: [u8; $n]) {
                self.buf = buf;
            }
            impl_write_buf!();
        }
        #[cfg(feature = "bytes")]
        #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
        impl bytes::Buf for $name {
            impl_buf!();
        }
        impl<'a> From<&'a $name> for WriteBuf<'a> {
            #[inline]
            fn from(value: &'a $name) -> Self {
                Self {
                    buf: &value.buf,
                    read: value.read,
                }
            }
        }
    };
}

define_write_buf!(OwnedWriteBuf8, std::mem::size_of::<u8>());
define_write_buf!(OwnedWriteBuf16, std::mem::size_of::<u16>());
define_write_buf!(OwnedWriteBuf32, std::mem::size_of::<u32>());
define_write_buf!(OwnedWriteBuf64, std::mem::size_of::<u64>());
define_write_buf!(OwnedWriteBuf128, std::mem::size_of::<u128>());
