#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::io::Result;

#[cfg(feature = "async")]
extern crate pin_project_lite;
#[cfg(feature = "async")]
extern crate tokio;

pub mod util;
#[cfg(feature = "async")]
mod asynchronous;
#[cfg(feature = "async")]
pub use asynchronous::*;

mod reader;
pub use reader::*;

mod writer;
pub use writer::*;

#[cfg(test)]
mod tests;

pub trait VariableReadable {
    fn read_single(&mut self) -> Result<u8>;

    fn read_more(&mut self, buf: &mut [u8]) -> Result<()> {
        for i in 0..buf.len() {
            buf[i] = self.read_single()?;
        }
        Ok(())
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    fn read_more_buf<B: bytes::BufMut>(&mut self, len: usize, buf: &mut B) -> Result<()> {
        let mut t = vec![0; len];
        self.read_more(&mut t)?;
        buf.put_slice(&t);
        Ok(())
    }
}

pub trait VariableWritable {
    fn write_single(&mut self, byte: u8) -> Result<usize>;

    fn write_more(&mut self, buf: &[u8]) -> Result<usize> {
        for i in 0..buf.len() {
            self.write_single(buf[i])?;
        }
        Ok(buf.len())
    }

    #[cfg(feature = "bytes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bytes")))]
    fn write_more_buf<B: bytes::Buf>(&mut self, buf: &mut B) -> Result<usize> {
        let mut len = 0;
        while buf.has_remaining() {
            let written = self.write_more(buf.chunk())?;
            buf.advance(written);
            len += written;
        }
        Ok(len)
    }
}

#[cfg(test)] // TODO
pub(crate) mod channel {
    use std::io::Result;
    use std::sync::mpsc::{Receiver, Sender};
    use crate::{VariableReadable, VariableWritable};

    pub(crate) struct SenderWriter<T>(pub Sender<T>);
    pub(crate) struct ReceiverReader<T>(pub Receiver<T>);

    impl VariableWritable for SenderWriter<u8> {
        fn write_single(&mut self, byte: u8) -> Result<usize> {
            self.0.send(byte)
                .map(|_| 1)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }

    impl VariableReadable for ReceiverReader<u8> {
        fn read_single(&mut self) -> Result<u8> {
            self.0.recv()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::UnexpectedEof, e))
        }
    }
}
