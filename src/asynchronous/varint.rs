macro_rules! varint_read {
    ($primitive: ty, $read_varint: ident, $inside_type: ty, $read_raw: ident) => {
        #[must_use = "futures do nothing unless you `.await` or poll them"]
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

macro_rules! define_varint_read {
    () => {
        varint::varint_read!(u16, read_u16_varint, u8, read_u8_ne);

        varint::varint_read!(u32, read_u32_varint, u8, read_u8_ne);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u32, read_u32_varint_2_le, u16, read_u16_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u32, read_u32_varint_2_be, u16, read_u16_be);

        varint::varint_read!(u64, read_u64_varint, u8, read_u8_ne);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u64, read_u64_varint_2_le, u16, read_u16_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u64, read_u64_varint_2_be, u16, read_u16_be);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u64, read_u64_varint_4_le, u32, read_u32_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u64, read_u64_varint_4_be, u32, read_u32_be);

        varint::varint_read!(u128, read_u128_varint, u8, read_u8_ne);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u128, read_u128_varint_2_le, u16, read_u16_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u128, read_u128_varint_2_be, u16, read_u16_be);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u128, read_u128_varint_4_le, u32, read_u32_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u128, read_u128_varint_4_be, u32, read_u32_be);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u128, read_u128_varint_8_le, u64, read_u64_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_read!(u128, read_u128_varint_8_be, u64, read_u64_be);
    };
}
pub(crate) use define_varint_read;

macro_rules! varint_write {
    ($primitive: ty, $write_varint: ident, $inside_type: ty, $write_raw: ident) => {
        #[must_use = "futures do nothing unless you `.await` or poll them"]
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

macro_rules! define_varint_write {
    () => {
        varint::varint_write!(u16, write_u16_varint, u8, write_u8_ne);

        varint::varint_write!(u32, write_u32_varint, u8, write_u8_ne);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u32, write_u32_varint_2_le, u16, write_u16_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u32, write_u32_varint_2_be, u16, write_u16_be);

        varint::varint_write!(u64, write_u64_varint, u8, write_u8_ne);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u64, write_u64_varint_2_le, u16, write_u16_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u64, write_u64_varint_2_be, u16, write_u16_be);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u64, write_u64_varint_4_le, u32, write_u32_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u64, write_u64_varint_4_be, u32, write_u32_be);

        varint::varint_write!(u128, write_u128_varint, u8, write_u8_ne);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u128, write_u128_varint_2_le, u16, write_u16_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u128, write_u128_varint_2_be, u16, write_u16_be);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u128, write_u128_varint_4_le, u32, write_u32_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u128, write_u128_varint_4_be, u32, write_u32_be);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u128, write_u128_varint_8_le, u64, write_u64_le);
        #[cfg(feature = "async_long_varint")]
        varint::varint_write!(u128, write_u128_varint_8_be, u64, write_u64_be);
    };
}
pub(crate) use define_varint_write;
