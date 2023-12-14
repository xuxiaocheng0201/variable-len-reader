use std::io::{Error, ErrorKind, Read, Result, Write};
use crate::variable_len::{read_variable_u64, write_variable_u64};

pub fn read_u8_vec(source: &mut impl Read) -> Result<Vec<u8>> {
    let length = read_variable_u64(source)? as usize;
    let mut bytes = vec![0; length];
    source.read_exact(&mut bytes)?;
    Ok(bytes)
}
pub fn write_u8_vec(target: &mut impl Write, message: &[u8]) -> Result<usize> {
    let size = write_variable_u64(target, message.len() as u64)?;
    target.write_all(message)?;
    Ok(size + message.len())
}

pub fn read_string(source: &mut impl Read) -> Result<String> {
    match String::from_utf8(read_u8_vec(source)?) {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string())),
    }
}
pub fn write_string(target: &mut impl Write, message: &String) -> Result<usize> {
    write_u8_vec(target, message.as_bytes())
}
