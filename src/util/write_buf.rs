macro_rules! impl_write_buf {
    () => {
        #[inline]
        pub fn len(&self) -> usize {
            self.buf.as_ref().len()
        }

        #[inline]
        pub fn buf(&self) -> &[u8] {
            self.buf.as_ref()
        }

        #[inline]
        pub fn position(&self) -> usize {
            self.position
        }

        #[inline]
        pub fn skip(&mut self, cnt: usize) {
            let new = self.position.checked_add(cnt).expect("skip overflow");
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
        pub fn get(&self) -> u8 {
            self.buf.as_ref()[self.position]
        }

        #[inline]
        pub fn take(&mut self) -> u8 {
            assert!(
                self.left() >= 1,
                "no more readable bytes in buffer"
            );
            let val = self.get();
            self.position += 1;
            val
        }

        #[inline]
        pub fn get_slice(&self, len: usize) -> &[u8] {
            assert!(
                self.left() >= len,
                "not enough readable bytes in buffer"
            );
            &self.buf.as_ref()[self.position..self.position + len]
        }

        #[inline]
        pub fn take_slice(&mut self, len: usize) -> &[u8] {
            assert!(
                self.left() >= len,
                "not enough readable bytes in buffer"
            );
            let slice = &self.buf.as_ref()[self.position..self.position + len];
            self.position += len;
            slice
        }
    };
}
#[cfg(feature = "bytes")]
macro_rules! impl_bytes_buf {
    () => {
        #[inline]
        fn remaining(&self) -> usize {
            self.left()
        }

        #[inline]
        fn chunk(&self) -> &[u8] {
            self.buf()
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
    position: usize,
}

#[allow(dead_code)]
impl<'a> WriteBuf<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            position: 0,
        }
    }

    impl_write_buf!();
}

#[cfg(feature = "bytes")]
#[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
impl<'a> bytes::Buf for WriteBuf<'a> {
    impl_bytes_buf!();
}


/// The type parameter `B` should be `[u8, $n]`.
#[derive(Debug)]
pub struct OwnedWriteBuf<B: AsRef<[u8]>> {
    buf: B,
    position: usize
}

impl<B: AsRef<[u8]>> OwnedWriteBuf<B> {
    pub fn new(buf: B) -> Self {
        Self {
            buf,
            position: 0,
        }
    }

    impl_write_buf!();
}

#[cfg(feature = "bytes")]
#[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
impl<B: AsRef<[u8]>> bytes::Buf for OwnedWriteBuf<B> {
    impl_bytes_buf!();
}

impl<'a, B: AsRef<[u8]>> From<&'a mut OwnedWriteBuf<B>> for WriteBuf<'a> { // TODO: guard
    #[inline]
    fn from(value: &'a mut OwnedWriteBuf<B>) -> Self {
        let mut buf = Self::new(value.buf.as_ref());
        buf.set_position(value.position);
        buf
    }
}


#[cfg(test)]
fn __owned_write_buf_u8_array() {
    let _ = OwnedWriteBuf::<[u8; 16]>::new([0; 16]);
}
#[cfg(all(test, feature = "alloc"))]
fn __owned_write_buf_u8_vec() {
    let _ = OwnedWriteBuf::<alloc::vec::Vec<u8>>::new(alloc::vec::Vec::new());
}
