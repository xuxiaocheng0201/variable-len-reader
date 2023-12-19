# Variable Len Reader

[![Crate](https://img.shields.io/crates/v/variable-len-reader.svg)](https://crates.io/crates/variable-len-reader)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/blob/master/LICENSE)

**Read this in other languages: [English](README.md), [简体中文](README_zh.md).**

# Description

A Rust crate to read variable length data. (Based on [varint-rs](https://crates.io/crates/varint-rs))

>Read and write compressed data. Of each such byte, only 7 bits will be used to describe the actual value
since its most significant bit indicates whether the next byte is part of the same int.
Micro-optimization for int values that are expected to have values below 128.


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
variable-len-reader = "~0.3"
```


# Example

Directly use in tcp stream:

```rust
use std::net::{TcpListener, TcpStream};
use variable_len_reader::{VariableReadable, VariableWritable};

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
use variable_len_reader::{VariableReadable, VariableWritable};

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
