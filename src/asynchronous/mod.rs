use std::io::{Error, ErrorKind, Result};
#[cfg(any(feature = "async_bools", feature = "async_raw", feature = "async_varint", feature = "async_signed", feature = "async_vec_u8", feature = "async_string"))]
use std::{
    future::Future,
    pin::Pin
};
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_signed")]
use crate::zigzag::Zigzag;

#[cfg(feature = "async_bools")]
mod bools;
#[cfg(feature = "async_raw")]
mod raw;
#[cfg(feature = "async_varint")]
mod varint;
#[cfg(feature = "async_signed")]
mod signed;

#[async_trait]
pub trait AsyncVariableReadable: Unpin + Send {
    async fn read(&mut self) -> Result<u8>;

    #[inline]
    async fn read_bool(&mut self) -> Result<bool> {
        match self.read().await? {
            0 => Ok(false),
            1 => Ok(true),
            i => Err(Error::new(ErrorKind::InvalidData, format!("Invalid boolean value: {}", i))),
        }
    }

    #[cfg(feature = "async_bools")]
    bools::define_bools_read!();

    async fn read_more(&mut self, buf: &mut [u8]) -> Result<()> {
        for i in 0..buf.len() {
            buf[i] = self.read().await?;
        }
        Ok(())
    }

    #[cfg(feature = "async_raw")]
    raw::define_raw_read!();

    #[cfg(feature = "async_varint")]
    varint::define_varint_read!();

    #[cfg(feature = "async_signed")]
    signed::define_signed_read!();

    #[cfg(feature = "async_vec_u8")]
    #[inline]
    async fn read_u8_vec(&mut self) -> Result<Vec<u8>> {
        let length = self.read_u128_varint().await? as usize;
        let mut bytes = vec![0; length];
        self.read_more(&mut bytes).await?;
        Ok(bytes)
    }

    #[cfg(feature = "async_string")]
    #[inline]
    async fn read_string(&mut self) -> Result<String> {
        match String::from_utf8(self.read_u8_vec().await?) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string())),
        }
    }
}

#[async_trait]
pub trait AsyncVariableWritable: Unpin + Send {
    async fn write(&mut self, byte: u8) -> Result<usize>;

    #[inline]
    async fn write_bool(&mut self, b: bool) -> Result<usize> {
        self.write(if b { 1 } else { 0 }).await
    }

    #[cfg(feature = "async_bools")]
    bools::define_bools_write!();

    async fn write_more(&mut self, bytes: &[u8]) -> Result<usize> {
        for i in 0..bytes.len() {
            self.write(bytes[i]).await?;
        }
        Ok(bytes.len())
    }

    #[cfg(feature = "async_raw")]
    raw::define_raw_write!();

    #[cfg(feature = "async_varint")]
    varint::define_varint_write!();

    #[cfg(feature = "async_signed")]
    signed::define_signed_write!();

    #[cfg(feature = "async_vec_u8")]
    #[inline]
    async fn write_u8_vec(&mut self, message: &[u8]) -> Result<usize> {
        let mut size = self.write_u128_varint(message.len() as u128).await?;
        size += self.write_more(message).await?;
        Ok(size)
    }

    #[cfg(feature = "async_string")]
    #[inline]
    async fn write_string(&mut self, message: &str) -> Result<usize> {
        self.write_u8_vec(message.as_bytes()).await
    }
}

#[async_trait]
impl<R: AsyncReadExt + Unpin + Send> AsyncVariableReadable for R {
    #[inline]
    async fn read(&mut self) -> Result<u8> {
        let mut buf = [0];
        self.read_exact(&mut buf).await?;
        Ok(buf[0])
    }

    #[inline]
    async fn read_more(&mut self, buf: &mut [u8]) -> Result<()> {
        self.read_exact(buf).await?;
        Ok(())
    }
}

#[async_trait]
impl<W: AsyncWriteExt + Unpin + Send> AsyncVariableWritable for W {
    #[inline]
    async fn write(&mut self, byte: u8) -> Result<usize> {
        self.write_all(&[byte]).await?;
        Ok(1)
    }

    #[inline]
    async fn write_more(&mut self, bytes: &[u8]) -> Result<usize> {
        self.write_all(bytes).await?;
        Ok(bytes.len())
    }
}
