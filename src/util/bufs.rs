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

    pub fn advance(&mut self, cnt: usize) {
        let new = self.filled.checked_add(cnt).expect("filled overflow");
        assert!(
            new <= self.buf.len(),
            "filled must not become larger than buf.len()"
        );
        self.filled = new;
    }

    pub fn buf(&self) -> &[u8] {
        self.buf
    }

    pub fn buf_mut(&mut self) -> &mut [u8] {
        self.buf
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
}

impl<'a, 'b> From<&'b mut ReadBuf<'a>> for tokio::io::ReadBuf<'b> {
    fn from(value: &'b mut ReadBuf<'a>) -> Self {
        let mut buf = Self::new(value.buf);
        buf.advance(value.filled);
        buf
    }
}

#[cfg(feature = "bytes")]
unsafe impl<'a> bytes::BufMut for ReadBuf<'a> {
    fn remaining_mut(&self) -> usize {
        self.left()
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.advance(cnt)
    }

    fn chunk_mut(&mut self) -> &mut bytes::buf::UninitSlice {
        bytes::buf::UninitSlice::new(self.buf_mut())
    }
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

    pub fn skip(&mut self, cnt: usize) {
        let new = self.read.checked_add(cnt).expect("read overflow");
        assert!(
            new <= self.buf.len(),
            "read must not become larger than buf.len()"
        );
        self.read = new;
    }

    pub fn buf(&self) -> &[u8] {
        self.buf
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
}

#[cfg(feature = "bytes")]
impl<'a> bytes::Buf for WriteBuf<'a> {
    fn remaining(&self) -> usize {
        self.left()
    }

    fn chunk(&self) -> &[u8] {
        self.buf
    }

    fn advance(&mut self, cnt: usize) {
        self.skip(cnt)
    }
}
