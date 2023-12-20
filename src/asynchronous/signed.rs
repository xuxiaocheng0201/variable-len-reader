macro_rules! signed_read {
    ($primitive: ty, $read_signed: ident, $read_varint: ident) => {
        fn $read_signed(&mut self) -> Pin<Box<dyn Future<Output = Result<$primitive>> + Send + '_>> {
            Box::pin(async move { Ok(self.$read_varint().await?.zigzag()) })
        }
    };
}
pub(crate) use signed_read;

macro_rules! signed_write {
    ($primitive: ty, $write_signed: ident, $write_varint: ident) => {
        fn $write_signed(&mut self, num: $primitive) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> {
            Box::pin(async move { self.$write_varint(num.zigzag()).await })
        }
    };
}
pub(crate) use signed_write;
