cargo build --no-default-features --features "sync"
cargo build --no-default-features --features "sync_raw"
cargo build --no-default-features --features "sync_raw_size"
cargo build --no-default-features --features "sync_bools"
cargo build --no-default-features --features "sync_varint"
cargo build --no-default-features --features "sync_varint_size"
cargo build --no-default-features --features "sync_varint_long"
cargo build --no-default-features --features "sync_varint_long_size"
cargo build --no-default-features --features "sync_signed_varint"
cargo build --no-default-features --features "sync_signed_varint_size"
cargo build --no-default-features --features "sync_signed_varint_long"
cargo build --no-default-features --features "sync_signed_varint_long_size"
cargo build --no-default-features --features "sync_float_varint"
cargo build --no-default-features --features "sync_float_varint_long"
cargo build --no-default-features --features "sync_u8_vec"
cargo build --no-default-features --features "sync_string"
cargo build --no-default-features --features "sync_default"
cargo build --no-default-features --features "sync_full"
cargo build --no-default-features --features "std,sync_full"
cargo build --no-default-features --features "sync_full,bytes"

cargo build --no-default-features --features "async"
cargo build --no-default-features --features "async_raw"
cargo build --no-default-features --features "async_raw_size"
cargo build --no-default-features --features "async_bools"
cargo build --no-default-features --features "async_varint"
cargo build --no-default-features --features "async_varint_size"
cargo build --no-default-features --features "async_varint_long"
cargo build --no-default-features --features "async_varint_long_size"
cargo build --no-default-features --features "async_signed_varint"
cargo build --no-default-features --features "async_signed_varint_size"
cargo build --no-default-features --features "async_signed_varint_long"
cargo build --no-default-features --features "async_signed_varint_long_size"
cargo build --no-default-features --features "async_float_varint"
cargo build --no-default-features --features "async_float_varint_long"
cargo build --no-default-features --features "async_u8_vec"
cargo build --no-default-features --features "async_string"
cargo build --no-default-features --features "async_default"
cargo build --no-default-features --features "async_full"
cargo build --no-default-features --features "tokio,async_full"
cargo build --no-default-features --features "async_full,bytes"

cargo build --no-default-features --features "default"
cargo build --no-default-features --features "full"
cargo build --no-default-features --features "full,std,tokio,bytes"

cargo test --no-default-features --features "full,std,tokio,bytes"
