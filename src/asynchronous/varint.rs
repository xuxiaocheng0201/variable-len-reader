macro_rules! varint_read {
    ($primitive: ty, $read_varint: ident, $inside_type: ty, $read_raw: ident) => {
        fn $read_varint(&mut self) -> Pin<Box<dyn Future<Output = Result<$primitive>> + Send + '_>> {
            const SIZE: usize = std::mem::size_of::<$primitive>() << 3;// * 8
            const NUM_BITS: $inside_type = <$inside_type>::MAX >> 1;
            const SIGN_BIT: $inside_type = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$inside_type>::BITS - 1) as usize;
            Box::pin(async move {
                let mut value = 0;
                let mut position = 0;
                loop {
                    let current = self.$read_raw().await?;
                    value |= ((current & NUM_BITS) as $primitive) << position;
                    if current & SIGN_BIT == 0 {
                        break;
                    }
                    position += POS_OFFSET;
                    if position >= SIZE {
                        return Err(Error::new(ErrorKind::InvalidData, format!("Varint {} in stream is too long.", stringify!($read_varint))));
                    }
                }
                Ok(value)
            })
        }
    };
}
pub(crate) use varint_read;

macro_rules! varint_write {
    ($primitive: ty, $write_varint: ident, $inside_type: ty, $write_raw: ident) => {
        fn $write_varint(&mut self, num: $primitive) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> {
            const NUM_BITS: $inside_type = <$inside_type>::MAX >> 1;
            const SIGN_BIT: $inside_type = NUM_BITS + 1;
            const POS_OFFSET: usize = (<$inside_type>::BITS - 1) as usize;
            Box::pin(async move {
                let mut size = 0;
                let mut value = num;
                while value >= SIGN_BIT as $primitive {
                    size += self.$write_raw(((value & (NUM_BITS as $primitive)) as $inside_type) | SIGN_BIT).await?;
                    value >>= POS_OFFSET;
                }
                size += self.$write_raw((value & (NUM_BITS as $primitive)) as $inside_type).await?;
                Ok(size)
            })
        }
    };
}
pub(crate) use varint_write;
