use core::ops::{Deref, DerefMut};
use crate::{AsyncVariableReader, AsyncVariableWriter};

#[derive(Debug)]
#[repr(transparent)]
pub struct AsyncReaderHelper<'a, R: AsyncVariableReader + Unpin>(pub &'a mut R);

impl<'a, R: AsyncVariableReader + Unpin> Deref for AsyncReaderHelper<'a, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, R: AsyncVariableReader + Unpin> DerefMut for AsyncReaderHelper<'a, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, R: AsyncVariableReader + Unpin> AsyncReaderHelper<'a, R> {
    pub async fn help_read_u8_vec(&'a mut self) -> Result<alloc::vec::Vec<u8>, R::Error> {
        let length = self.read_usize_varint_ap().await?;
        let mut bytes = alloc::vec![0; length];
        self.read_more(&mut bytes).await?;
        Ok(bytes)
    }

    #[cfg(feature = "async_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    pub async fn help_read_string(&'a mut self) -> Result<alloc::string::String, R::Error> {
        match alloc::string::String::from_utf8(self.help_read_u8_vec().await?) {
            Ok(s) => Ok(s),
            Err(e) => Err(R::read_string_error("ReadString", e)),
        }
    }
}


#[derive(Debug)]
#[repr(transparent)]
pub struct AsyncWriterHelper<'a, W: AsyncVariableWriter + Unpin>(pub &'a mut W);

impl<'a, W: AsyncVariableWriter + Unpin> Deref for AsyncWriterHelper<'a, W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, W: AsyncVariableWriter + Unpin> DerefMut for AsyncWriterHelper<'a, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, W: AsyncVariableWriter + Unpin> AsyncWriterHelper<'a, W> {
    pub async fn help_write_u8_vec(&'a mut self, bytes: &[u8]) -> Result<(), W::Error> {
        self.write_usize_varint_ap(bytes.len()).await?;
        self.write_more(bytes).await?;
        Ok(())
    }

    #[cfg(feature = "async_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async_string")))]
    pub async fn help_write_string(&'a mut self, string: &str) -> Result<(), W::Error> {
        self.help_write_u8_vec(string.as_bytes()).await?;
        Ok(())
    }
}
