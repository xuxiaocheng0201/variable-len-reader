macro_rules! impl_read_buf {
    () => {
        #[inline]
        fn len(&self) -> usize {
            self.buf.as_ref().len()
        }

        #[inline]
        pub fn buf(&self) -> &[u8] {
            self.buf.as_ref()
        }

        #[inline]
        pub fn buf_mut(&mut self) -> &mut [u8] {
            self.buf.as_mut()
        }

        #[inline]
        pub fn position(&self) -> usize {
            self.position
        }

        #[inline]
        pub fn advance(&mut self, cnt: usize) {
            let new = self.position.checked_add(cnt).expect("advance overflow");
            assert!(
                self.len() >= new,
                "position ({}) must not become larger than buf.len ({})",
                new, self.len()
            );
            self.position = new;
        }

        #[inline]
        pub fn left(&self) -> usize {
            self.len() - self.position
        }

        #[inline]
        pub fn reset(&mut self) {
            self.position = 0;
        }

        #[inline]
        pub fn set_position(&mut self, position: usize) {
            assert!(
                self.len() >= position,
                "position ({}) must not become larger than buf.len ({})",
                position, self.len()
            );
            self.position = position;
        }


        #[inline]
        pub fn set(&mut self, val: u8) {
            self.buf.as_mut()[self.position] = val;
        }

        #[inline]
        pub fn put(&mut self, val: u8) {
            assert!(
                self.left() >= 1,
                "no more writable bytes in buffer"
            );
            self.set(val);
            self.position += 1;
        }

        #[inline]
        pub fn set_slice(&mut self, slice: &[u8]) {
            assert!(
                self.left() >= slice.len(),
                "not enough writable bytes in buffer"
            );
            self.buf.as_mut()[self.position..self.position + slice.len()].copy_from_slice(slice);
        }

        #[inline]
        pub fn put_slice(&mut self, slice: &[u8]) {
            self.set_slice(slice);
            self.position += slice.len();
        }
    };
}
#[cfg(feature = "bytes")]
macro_rules! impl_bytes_buf {
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
    position: usize,
}

impl<'a> ReadBuf<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self {
            buf,
            position: 0,
        }
    }

    impl_read_buf!();
}

#[cfg(feature = "bytes")]
#[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
unsafe impl<'a> bytes::BufMut for ReadBuf<'a> {
    impl_bytes_buf!();
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl<'a, 'b> From<&'b mut ReadBuf<'a>> for tokio::io::ReadBuf<'b> {
    #[inline]
    fn from(value: &'b mut ReadBuf<'a>) -> Self {
        let mut buf = Self::new(value.buf);
        buf.set_filled(value.position);
        buf
    }
}


/// The type parameter `B` should be `[u8, $n]`.
#[derive(Debug, Clone)]
pub struct OwnedReadBuf<B: AsRef<[u8]> + AsMut<[u8]>> {
    buf: B,
    position: usize
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> OwnedReadBuf<B> {
    pub fn new(buf: B) -> Self {
        Self {
            buf,
            position: 0,
        }
    }

    pub fn into_inner(self) -> B {
        self.buf
    }

    impl_read_buf!();
}

#[cfg(feature = "bytes")]
#[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
unsafe impl<B: AsRef<[u8]> + AsMut<[u8]>> bytes::BufMut for OwnedReadBuf<B> {
    impl_bytes_buf!();
}

impl<'a, B: AsRef<[u8]> + AsMut<[u8]>> From<&'a mut OwnedReadBuf<B>> for ReadBuf<'a> { // TODO: guard
    #[inline]
    fn from(value: &'a mut OwnedReadBuf<B>) -> Self {
        let mut buf = Self::new(value.buf.as_mut());
        buf.set_position(value.position);
        buf
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl<'a, B: AsRef<[u8]> + AsMut<[u8]>> From<&'a mut OwnedReadBuf<B>> for tokio::io::ReadBuf<'a> {
    #[inline]
    fn from(value: &'a mut OwnedReadBuf<B>) -> Self {
        let mut buf = Self::new(value.buf.as_mut());
        buf.set_filled(value.position);
        buf
    }
}


#[cfg(test)]
fn __owned_read_buf_u8_array() {
    let _ = OwnedReadBuf::<[u8; 16]>::new([0; 16]);
}
#[cfg(all(test, feature = "alloc"))]
fn __owned_read_buf_u8_vec() {
    use alloc::vec::Vec;
    let _ = OwnedReadBuf::<Vec<u8>>::new(Vec::new());
}
