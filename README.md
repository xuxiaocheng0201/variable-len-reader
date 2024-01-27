# Variable Len Reader

[![Crate](https://img.shields.io/crates/v/variable-len-reader.svg)](https://crates.io/crates/variable-len-reader)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/blob/master/LICENSE)

**Read this in other languages: [English](README.md), [简体中文](README_zh.md).**

# Description

A Rust crate to read variable length data based on varint format.


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
variable-len-reader = "^2.3"
```


# Example

Directly use in tcp stream:

```rust
use std::net::{TcpListener, TcpStream};
use variable_len_reader::{VariableReader, VariableWriter};

fn main() {
    let addr = "localhost:25564";
    let server = TcpListener::bind(addr).unwrap();
    let mut client = TcpStream::connect(addr).unwrap();
    let mut server = server.incoming().next().unwrap().unwrap();

    // Write
    client.write_string(&"Hello world!").unwrap();

    // Read
    let message = server.read_string().unwrap();
    assert_eq!("Hello world!", message);
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
use tokio::net::{TcpListener, TcpStream};
use variable_len_reader::{AsyncVariableReader, AsyncVariableWriter};

#[tokio::main]
async fn main() {
    let addr = "localhost:25564";
    let server = TcpListener::bind(addr).await.unwrap();
    let mut client = TcpStream::connect(addr).await.unwrap();
    let (mut server, _) = server.accept().await.unwrap();

    // Write
    client.write_string(&"Hello tokio!").await.unwrap();

    // Read
    let message = server.read_string().await.unwrap();
    assert_eq!("Hello tokio!", message);
}
```
