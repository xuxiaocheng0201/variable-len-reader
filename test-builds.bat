cargo test --no-default-features --features "default" --lib
cargo test --no-default-features --features "raw" --lib
cargo test --no-default-features --features "raw_size" --lib
cargo test --no-default-features --features "bools" --lib
cargo test --no-default-features --features "varint" --lib
cargo test --no-default-features --features "long_varint" --lib
cargo test --no-default-features --features "signed" --lib
cargo test --no-default-features --features "long_signed" --lib
cargo test --no-default-features --features "vec_u8" --lib
cargo test --no-default-features --features "string" --lib

cargo test --no-default-features --features "async" --lib
cargo test --no-default-features --features "async_default" --lib
cargo test --no-default-features --features "async_raw" --lib
cargo test --no-default-features --features "async_raw_size" --lib
cargo test --no-default-features --features "async_bools" --lib
cargo test --no-default-features --features "async_varint" --lib
cargo test --no-default-features --features "async_long_varint" --lib
cargo test --no-default-features --features "async_signed" --lib
cargo test --no-default-features --features "async_long_signed" --lib
cargo test --no-default-features --features "async_vec_u8" --lib
cargo test --no-default-features --features "async_string" --lib

cargo test --no-default-features --features "full" --lib
cargo test --features "async_default" --doc
