use crate::synchronous::VariableWritable;

/// AP means all-platform. This is used for usize/isize converting to u128/i128.
/// CP means current-platform. It writes usize/isize directly.
#[allow(dead_code)]
macro_rules! write_size_ap {
    (f $feature: meta, $primitive: ty, $func: ident, $internal: ty, $write_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self, num: $primitive) -> ::core::result::Result<$primitive, Self::Error> {
            self.$write_internal(num as $internal)
        }
    };
}

include!("write_bools.rs");

include!("write_raw.rs");
include!("write_raw_size.rs");

include!("write_varint.rs");
include!("write_varint_size.rs");
include!("write_varint_long.rs");
include!("write_varint_long_size.rs");

include!("write_signed_varint.rs");
include!("write_signed_varint_size.rs");
include!("write_signed_varint_long.rs");
include!("write_signed_varint_long_size.rs");

include!("write_float_varint.rs");
include!("write_float_varint_long.rs");

pub trait VariableWriter: VariableWritable {
    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<usize, Self::Error> {
        self.write_single(if b { 1 } else { 0 })
    }

    define_write_bools!();

    define_write_raw!();
    define_write_raw_size!();

    define_write_varint!();
    define_write_varint_size!();
    define_write_varint_long!();
    define_write_varint_long_size!();

    define_write_signed_varint!();
    define_write_signed_varint_size!();
    define_write_signed_varint_long!();
    define_write_signed_varint_long_size!();

    define_write_float_varint!();
    define_write_float_varint_long!();

    #[cfg(feature = "sync_vec_u8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sync_vec_u8")))]
    #[inline]
    fn write_u8_vec(&mut self, message: &[u8]) -> Result<usize, Self::Error> {
        let mut len = self.write_usize_varint_ap(message.len())?;
        len += self.write_more(message)?;
        Ok(len)
    }

    #[cfg(feature = "sync_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sync_string")))]
    #[inline]
    fn write_string(&mut self, message: &str) -> Result<usize, Self::Error> {
        self.write_u8_vec(message.as_bytes())
    }
}

impl<W: VariableWritable> VariableWriter for W {
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<W: std::io::Write> VariableWritable for W {
    type Error = std::io::Error;

    #[inline]
    fn write_single(&mut self, byte: u8) -> Result<usize, Self::Error> {
        W::write_all(self, &[byte])?;
        Ok(1)
    }

    #[inline]
    fn write_more(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        W::write_all(self, buf)?;
        Ok(buf.len())
    }
}
