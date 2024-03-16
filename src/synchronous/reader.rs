use crate::synchronous::VariableReadable;

/// AP means all-platform. This is used for usize/isize converting from u128/i128.
/// CP means current-platform. It reads usize/isize directly.
#[allow(unused_macros)]
macro_rules! read_size_ap {
    (f $feature: meta, $primitive: ty, $func: ident, $read_internal: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[inline]
        fn $func(&mut self) -> ::core::result::Result<$primitive, Self::Error> {
            self.$read_internal().map(|v| v as $primitive)
        }
    };
}

include!("read_bools.rs");

include!("read_raw.rs");
include!("read_raw_size.rs");

include!("read_varint.rs");
include!("read_varint_size.rs");
include!("read_varint_long.rs");
include!("read_varint_long_size.rs");

include!("read_signed_varint.rs");
include!("read_signed_varint_size.rs");
include!("read_signed_varint_long.rs");
include!("read_signed_varint_long_size.rs");

include!("read_float_varint.rs");
include!("read_float_varint_long.rs");

pub trait VariableReader: VariableReadable {
    fn read_bool_error(func_name: &'static str, byte: u8) -> Self::Error;

    #[inline]
    fn read_bool(&mut self) -> Result<bool, Self::Error> {
        match self.read_single()? {
            0 => Ok(false),
            1 => Ok(true),
            b => Err(Self::read_bool_error("read_bool", b)),
        }
    }

    define_read_bools!();

    define_read_raw!();
    define_read_raw_size!();

    define_read_varint!();
    define_read_varint_size!();
    define_read_varint_long!();
    define_read_varint_long_size!();

    define_read_signed_varint!();
    define_read_signed_varint_size!();
    define_read_signed_varint_long!();
    define_read_signed_varint_long_size!();

    define_read_float_varint!();
    define_read_float_varint_long!();

    #[cfg(feature = "sync_vec_u8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sync_vec_u8")))]
    #[inline]
    #[allow(unused_qualifications)] // Only for IDE.
    fn read_u8_vec(&mut self) -> Result<alloc::vec::Vec<u8>, Self::Error> {
        let length = self.read_usize_varint_ap()?;
        let mut bytes = alloc::vec![0; length];
        self.read_more(&mut bytes)?;
        Ok(bytes)
    }

    #[cfg(feature = "sync_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sync_string")))]
    fn read_string_error(func_name: &'static str, error: alloc::string::FromUtf8Error) -> Self::Error;

    #[cfg(feature = "sync_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sync_string")))]
    #[inline]
    #[allow(unused_qualifications)] // Only for IDE.
    fn read_string(&mut self) -> Result<alloc::string::String, Self::Error> {
        match alloc::string::String::from_utf8(self.read_u8_vec()?) {
            Ok(s) => Ok(s),
            Err(e) => Err(Self::read_string_error("read_string", e)),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<R: std::io::Read> VariableReadable for R {
    type Error = std::io::Error;

    #[inline]
    fn read_single(&mut self) -> Result<u8, Self::Error> {
        let mut buf = [0];
        R::read_exact(self, &mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_more(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        R::read_exact(self, buf)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<R: std::io::Read> VariableReader for R {
    #[inline]
    fn read_bool_error(func_name: &'static str, byte: u8) -> Self::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid bool. value {} at {}.", byte, func_name))
    }

    #[cfg(feature = "sync_bools")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sync_bools")))]
    #[inline]
    fn read_bools_error(func_name: &'static str, byte: u8) -> Self::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid bools. value {} at {}.", byte, func_name))
    }

    #[cfg(feature = "sync_varint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sync_varint")))]
    #[inline]
    fn read_varint_error(func_name: &'static str, current: u128) -> Self::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Too long varint value. {} at {}.", current, func_name))
    }

    #[cfg(feature = "sync_string")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sync_string")))]
    #[inline]
    fn read_string_error(_func_name: &'static str, error: alloc::string::FromUtf8Error) -> Self::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, error)
    }
}
