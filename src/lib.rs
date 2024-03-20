#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "async")]
extern crate pin_project_lite;

pub mod util;
#[cfg(feature = "sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub mod synchronous;
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod asynchronous;

mod impls;

#[cfg(feature = "sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub use synchronous::{reader::VariableReader, writer::VariableWriter, VariableReadable, VariableWritable};
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use asynchronous::{reader::AsyncVariableReader, writer::AsyncVariableWriter, AsyncVariableReadable, AsyncVariableWritable};

pub mod helper {
    #[cfg(feature = "async_u8_vec")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_u8_vec")))]
    pub use crate::asynchronous::helper::{AsyncReaderHelper, AsyncWriterHelper};
}

// TODO for 4.x: sync channel like [tokio::io::duplex] impl
