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
// #[cfg(test)]
// mod channel {
//     use std::pin::Pin;
//     use std::task::{Context, Poll};
//     use tokio::sync::mpsc::{Receiver, Sender};
//     use tokio::sync::mpsc::error::{TryRecvError, TrySendError};
//     use crate::{AsyncVariableReadable, AsyncVariableWritable};
//
//     pub(crate) struct SenderWriter<T>(pub Sender<T>);
//     pub(crate) struct ReceiverReader<T>(pub Receiver<T>);
//
//     impl AsyncVariableWritable for SenderWriter<u8> {
//         fn poll_write_single(self: Pin<&mut Self>, cx: &mut Context<'_>, byte: u8) -> Poll<std::io::Result<usize>> {
//             self.0.try_send(byte).map_or_else(|e| match e {
//                 TrySendError::Full(_) => {
//                     cx.waker().wake_by_ref();
//                     Poll::Pending
//                 }
//                 TrySendError::Closed(_) => {
//                     Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "channel disconnected")))
//                 }
//             }, |()| Poll::Ready(Ok(1)))
//         }
//     }
//
//     impl AsyncVariableReadable for ReceiverReader<u8> {
//         fn poll_read_single(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<u8>> {
//             self.0.try_recv().map_or_else(|e| match e {
//                 TryRecvError::Empty => {
//                     cx.waker().wake_by_ref();
//                     Poll::Pending
//                 }
//                 TryRecvError::Disconnected => {
//                     Poll::Ready(Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "channel disconnected")))
//                 }
//             }, |v| Poll::Ready(Ok(v)))
//         }
//     }
// }
