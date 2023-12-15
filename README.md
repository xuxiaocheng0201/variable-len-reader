# Variable Len Reader

![Crate](https://img.shields.io/crates/v/variable-len-reader.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/variable-len-reader)
![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/variable-len-reader)
![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/variable-len-reader)
![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/variable-len-reader)

**Read this in other languages: [English](README.md), [简体中文](README_zh.md).**

# Description

A Rust crate to read variable length data. (VarInt)

>Read and write compressed data. Of each such byte, only 7 bits will be used to describe the actual value
since its most significant bit indicates whether the next byte is part of the same int.
Micro-optimization for int values that are expected to have values below 128.


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
variable-len-reader = "*"
```


# Example

Directly use in tcp stream:

```rust
use std::net::{TcpListener, TcpStream};
use variable_len_reader::variable_len::{read_variable_u32, write_variable_u32};

fn main() {
    let addr = "localhost:25564";
    let server = TcpListener::bind(addr).unwrap();
    let mut client = TcpStream::connect(addr).unwrap();
    let mut server = server.incoming().next().unwrap().unwrap();

    // Write
    write_variable_u32(&mut client, 1234).unwrap();

    // Read
    let message = read_variable_u32(&mut server).unwrap();
    assert_eq!(1234, message);
}
```

Use with [bytes](https://crates.io/crates/bytes) crate:

```rust
use bytes::{Buf, BufMut, BytesMut};
use variable_len_reader::variable_len::{read_variable_u32, write_variable_u32};

fn main() {
    let mut writer = BytesMut::new().writer();

    // Write
    write_variable_u32(&mut writer, 4321).unwrap();

    let bytes = writer.into_inner();
    let mut reader = bytes.reader();

    // Read
    let message = read_variable_u32(&mut reader).unwrap();
    assert_eq!(4321, message);
}
```
