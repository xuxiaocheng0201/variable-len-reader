# 可变长数据读写器

[![Crate](https://img.shields.io/crates/v/variable-len-reader.svg)](https://crates.io/crates/variable-len-reader)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/blob/master/LICENSE)

**其他语言版本：[English](README.md)，[简体中文](README_zh.md)。**

# 描述

一个类似于 VarInt 的可变长数据读写器。（基于 [varint-rs](https://crates.io/crates/varint-rs)）

>读取和写入压缩过数据。在每个这样的字节中，只有7位将用于描述实际值，
它的最高有效位指示下一个字节是否是同一int的一部分。


# 用法

将以下内容添加到你的`Cargo.toml`：

```toml
[dependencies]
variable-len-reader = "~0.3"
```


# 示例

直接在tcp流中使用：

```rust
use std::net::{TcpListener, TcpStream};
use variable_len_reader::{VariableReadable, VariableWritable};

fn main() {
    let addr = "localhost:25564";
    let server = TcpListener::bind(addr).unwrap();
    let mut client = TcpStream::connect(addr).unwrap();
    let mut server = server.incoming().next().unwrap().unwrap();

    // 写
    client.write_string(&"Hello world!").unwrap();

    // 读
    let message = server.read_string().unwrap();
    assert_eq!("Hello world!", message);
}
```

和 [bytes](https://crates.io/crates/bytes) 库一起使用：

```rust
use bytes::{Buf, BufMut, BytesMut};
use variable_len_reader::{VariableReadable, VariableWritable};

fn main() {
    let message = "Hello world!";
    let mut writer = BytesMut::new().writer();

    // 写
    writer.write_string(message).unwrap();

    let bytes = writer.into_inner();
    assert_eq!(message.len() as u8, bytes[0]);
    assert_eq!(message.as_bytes(), &bytes[1..]);
    let mut reader = bytes.reader();

    // 读
    let string = reader.read_string().unwrap();
    assert_eq!(message, string);
}
```
