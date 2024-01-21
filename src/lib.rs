#![doc = include_str!("../README.md")]

use std::io::Result;
use crate::bufs::{ReadBuf, WriteBuf};
// #[cfg(feature = "signed")]
// use crate::zigzag::Zigzag;

extern crate pin_project_lite;
extern crate tokio;

pub mod bufs;
// #[cfg(feature = "raw")]
// mod raw;
// #[cfg(feature = "bools")]
// mod bools;
// #[cfg(feature = "varint")]
// mod varint;
// #[cfg(feature = "signed")]
// mod signed;
#[cfg(any(feature = "signed", feature = "async_signed"))]
pub mod zigzag;
#[cfg(feature = "async")]
pub mod asynchronous;

mod reader;
pub use reader::*;

mod writer;
pub use writer::*;

#[cfg(test)]
mod tests;

pub trait VariableReadable {
    fn read_single(&mut self) -> Result<u8>;

    fn read_more(&mut self, buf: &mut ReadBuf<'_>) -> Result<()> {
        while buf.left() > 0 {
            buf.put(self.read_single()?);
        }
        Ok(())
    }
}

pub trait VariableWritable {
    fn write_single(&mut self, byte: u8) -> Result<usize>;

    fn write_more(&mut self, buf: &mut WriteBuf<'_>) -> Result<usize> {
        while buf.left() > 0 {
            self.write_single(buf.get())?;
            buf.skip(1);
        }
        Ok(buf.buf().len())
    }
}

#[cfg(test)]
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
