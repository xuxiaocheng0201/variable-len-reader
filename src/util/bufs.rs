macro_rules! impl_read_buf {
    () => {
        pub fn advance(&mut self, cnt: usize) {
            let new = self.filled.checked_add(cnt).expect("filled overflow");
            assert!(
                new <= self.buf.len(),
                "filled must not become larger than buf.len()"
            );
            self.filled = new;
        }

        pub fn buf(&self) -> &[u8] {
            &self.buf
        }

        pub fn buf_mut(&mut self) -> &mut [u8] {
            &mut self.buf
        }

        pub fn filled(&self) -> usize {
            self.filled
        }

        pub fn left(&self) -> usize {
            self.buf.len() - self.filled
        }

        pub fn set(&mut self, val: u8) {
            assert!(
                self.left() >= 1,
                "left() must large than 1"
            );
            self.buf[self.filled] = val;
        }

        pub fn put(&mut self, val: u8) {
            self.set(val);
            self.filled += 1;
        }

        pub fn set_slice(&mut self, slice: &[u8]) {
            assert!(
                self.left() >= slice.len(),
                "buf.len() must fit in remaining()"
            );
            self.buf[self.filled..self.filled + slice.len()].copy_from_slice(slice);
        }

        pub fn put_slice(&mut self, slice: &[u8]) {
            self.set_slice(slice);
            self.filled += slice.len();
        }
    };
}
#[cfg(feature = "bytes")]
macro_rules! impl_buf_mut {
    () => {
        fn remaining_mut(&self) -> usize {
            self.left()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.advance(cnt)
        }

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
    pub fn new(buf: &'a mut [u8]) -> ReadBuf<'a> {
        ReadBuf {
            buf,
            filled: 0,
        }
    }
    impl_read_buf!();
}

#[cfg(feature = "bytes")]
unsafe impl<'a> bytes::BufMut for ReadBuf<'a> {
    impl_buf_mut!();
}

#[cfg(feature = "async")] // Tokio dep
impl<'a, 'b> From<&'b mut ReadBuf<'a>> for tokio::io::ReadBuf<'b> {
    fn from(value: &'b mut ReadBuf<'a>) -> Self {
        let mut buf = Self::new(value.buf);
        buf.advance(value.filled);
        buf
    }
}


macro_rules! impl_write_buf {
    () => {
        pub fn skip(&mut self, cnt: usize) {
            let new = self.read.checked_add(cnt).expect("read overflow");
            assert!(
                new <= self.buf.len(),
                "read must not become larger than buf.len()"
            );
            self.read = new;
        }

        pub fn buf(&self) -> &[u8] {
            &self.buf
        }

        pub fn read(&self) -> usize {
            self.read
        }

        pub fn left(&self) -> usize {
            self.buf.len() - self.read
        }

        pub fn get(&self) -> u8 {
            assert!(
                self.left() >= 1,
                "left() must large than 1"
            );
            self.buf[self.read]
        }

        pub fn take(&mut self) -> u8 {
            let val = self.get();
            self.read += 1;
            val
        }

        pub fn get_slice(&self, len: usize) -> &[u8] {
            assert!(
                self.left() >= len,
                "left() must large than len"
            );
            &self.buf[self.read..self.read + len]
        }

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
macro_rules! impl_buf {
    () => {
        fn remaining(&self) -> usize {
            self.left()
        }

        fn chunk(&self) -> &[u8] {
            self.buf
        }

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
    pub fn new(buf: &'a [u8]) -> WriteBuf<'a> {
        WriteBuf {
            buf,
            read: 0,
        }
    }
    impl_write_buf!();
}

#[cfg(feature = "bytes")]
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
            pub fn new() -> $name {
                $name {
                    buf: [0; $n],
                    filled: 0,
                }
            }
            pub fn into_inner(&self) -> [u8; $n] {
                self.buf
            }
            impl_read_buf!();
        }
        #[cfg(feature = "bytes")]
        unsafe impl bytes::BufMut for $name {
            impl_buf_mut!();
        }
        impl<'a> From<&'a mut $name> for ReadBuf<'a> {
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
            pub fn new(buf: [u8; $n]) -> $name {
                $name {
                    buf,
                    read: 0,
                }
            }
            pub fn into_inner(&self) -> [u8; $n] {
                self.buf
            }
            pub fn buf_mut(&mut self) -> &mut [u8] {
                &mut self.buf
            }
            impl_write_buf!();
        }
        #[cfg(feature = "bytes")]
        impl bytes::Buf for $name {
            impl_buf!();
        }
        impl<'a> From<&'a mut $name> for WriteBuf<'a> {
            fn from(value: &'a mut $name) -> Self {
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
