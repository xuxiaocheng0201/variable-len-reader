# Variable Len Reader

[![Crate](https://img.shields.io/crates/v/variable-len-reader.svg)](https://crates.io/crates/variable-len-reader)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/blob/master/LICENSE)

**Read this in other languages: [English](README.md), [简体中文](README_zh.md).**

# Description

A Rust crate to read variable length data based on varint format.


# Features

* Reading and writing.
* Both synchronous and asynchronous implementations.
* Support [bytes](https://crates.io/crates/bytes) and [tokio](https://crates.io/crates/tokio) crates.
* Long chunk version for varint implementations. (But not recommended to use.)
* Support signed and unsigned value. (Using zigzag encoding.)
* Support usize/isize directly or convert from/to u128/i128 (which is with the `_ap` suffix.).
* Support extra type of `f32`, `f64`, `Vec<u8>` and `string`.
* Built-in implementation of `std::io::Read`, `std::io::Write` and `tokio::io::AsyncRead`, `tokio::io::AsyncWrite`.
* Chaining `bytes::Buf` support.
* no-std support.


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
variable-len-reader = "^3.2"
```


# Example

Directly use in tcp stream:

```rust
use std::net::{TcpListener, TcpStream};
use anyhow::Result;
use variable_len_reader::{VariableReader, VariableWriter};

fn main() -> Result<()> {
    let server = TcpListener::bind("localhost:0")?;
    let mut client = TcpStream::connect(server.local_addr()?)?;
    let mut server = server.incoming().next().unwrap()?;

    // Write
    client.write_string(&"Hello world!")?;

    // Read
    let message = server.read_string()?;
    assert_eq!("Hello world!", message);
    
    Ok(())
}
```

Use with [bytes](https://crates.io/crates/bytes) crate:

```rust
use bytes::{Buf, BufMut, BytesMut};
use variable_len_reader::{VariableReader, VariableWriter};

fn main() {
    let message = "Hello world!";
    let mut writer = BytesMut::new().writer();

    // Write
    writer.write_string(message).unwrap();

    let bytes = writer.into_inner();
    assert_eq!(message.len() as u8, bytes[0]);
    assert_eq!(message.as_bytes(), &bytes[1..]);
    let mut reader = bytes.reader();

    // Read
    let string = reader.read_string().unwrap();
    assert_eq!(message, string);
}
```

Async mode with [tokio](https://crates.io/crates/tokio) crate:
(Require 'async_default' feature)

```rust
use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use variable_len_reader::{AsyncVariableReader, AsyncVariableWriter};
use variable_len_reader::helper::{AsyncReaderHelper, AsyncWriterHelper};

#[tokio::main]
async fn main() -> Result<()> {
    let server = TcpListener::bind("localhost:0").await?;
    let mut client = TcpStream::connect(server.local_addr()?).await?;
    let (mut server, _) = server.accept().await?;

    // Write
    AsyncWriterHelper(&mut client).help_write_string(&"Hello tokio!").await?;

    // Read
    let message = AsyncReaderHelper(&mut server).help_read_string().await?;
    assert_eq!("Hello tokio!", message);
    
    Ok(())
}
```


# License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
