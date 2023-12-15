# 可变长数据读写器

![Crate](https://img.shields.io/crates/v/variable-len-reader.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/variable-len-reader)
![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/variable-len-reader)
![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/variable-len-reader)
![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/variable-len-reader)

**其他语言版本：[English](README.md)，[简体中文](README_zh.md)。**

# 描述

一个类似于 VarInt 的可变长数据读写器。

>读取和写入压缩过数据。在每个这样的字节中，只有7位将用于描述实际值，
它的最高有效位指示下一个字节是否是同一int的一部分。

# 用法

将以下内容添加到你的`Cargo.toml`：

```toml
[dependencies]
variable-len-reader = "*"
```


# 示例

直接在tcp流中使用：

```rust
use std::net::{TcpListener, TcpStream};
use variable_len_reader::variable_len::{read_variable_u32, write_variable_u32};

fn main() {
    let addr = "localhost:25564";
    let server = TcpListener::bind(addr).unwrap();
    let mut client = TcpStream::connect(addr).unwrap();
    let mut server = server.incoming().next().unwrap().unwrap();

    // 写
    write_variable_u32(&mut client, 4321).unwrap();

    // 读
    let message = read_variable_u32(&mut server).unwrap();
    assert_eq!(4321, message);
}
```

和 [bytes](https://crates.io/crates/bytes) 库一起使用：

```rust
use bytes::{Buf, BufMut, BytesMut};
use variable_len_reader::variable_len::{read_variable_u32, write_variable_u32};

fn main() {
    let mut writer = BytesMut::new().writer();

    // 写
    write_variable_u32(&mut writer, 1234).unwrap();

    let bytes = writer.into_inner();
    let mut reader = bytes.reader();

    // 读
    let message = read_variable_u32(&mut reader).unwrap();
    assert_eq!(1234, message);
}
```
