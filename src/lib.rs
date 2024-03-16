#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "async")]
extern crate pin_project_lite;
#[cfg(feature = "tokio")]
extern crate tokio;

pub mod util;
#[cfg(feature = "sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub mod synchronous;
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod asynchronous;

// #[cfg(test)] // TODO
// pub(crate) mod channel {
//     use std::io::Result;
//     use std::sync::mpsc::{Receiver, Sender};
//     use crate::synchronous::{VariableReadable, VariableWritable};
//
//     pub(crate) struct SenderWriter<T>(pub Sender<T>);
//     pub(crate) struct ReceiverReader<T>(pub Receiver<T>);
//
//     impl VariableWritable for SenderWriter<u8> {
//         fn write_single(&mut self, byte: u8) -> Result<usize> {
//             self.0.send(byte)
//                 .map(|_| 1)
//                 .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
//         }
//     }
//
//     impl VariableReadable for ReceiverReader<u8> {
//         fn read_single(&mut self) -> Result<u8> {
//             self.0.recv()
//                 .map_err(|e| std::io::Error::new(std::io::ErrorKind::UnexpectedEof, e))
//         }
//     }
// }
