# 可变长数据读写器

[![Crate](https://img.shields.io/crates/v/variable-len-reader.svg)](https://crates.io/crates/variable-len-reader)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/variable-len-reader)](https://github.com/xuxiaocheng0201/variable-len-reader/blob/master/LICENSE)

**其他语言版本：[English](README.md)，[简体中文](README_zh.md)。**

# 描述

一个基于 VarInt 的可变长数据读写器。


# 特点

* 读和写。
* 同步和异步双实现。
* 支持 [bytes](https://crates.io/crates/bytes) 和 [tokio](https://crates.io/crates/tokio) 库。
* 长 chunk 版本的 varint 实现。（但是不建议使用，因为这太愚蠢了。）
* 支持正负数读写。（通过 zigzag 编码）
* 支持直接读写 usize/isize 或将其转换到 u128/i128 再读写。（带有 `ap` 后缀）
* 支持额外的类型比如 `f32`，`f64`，`vec<u8>` 和 `string`。
* 内置实现 `std::io::Read`，`std::io::Write` 和 `tokio::io::AsyncRead`，`tokio::io::AsyncWrite`。
* 支持链式的 `bytes::Buf`。
* 支持 no-std。


# 用法

将以下内容添加到你的`Cargo.toml`：

```toml
[dependencies]
variable-len-reader = "^3.2"
```


# 示例

直接在tcp流中使用：

```rust
use std::net::{TcpListener, TcpStream};
use anyhow::Result;
use variable_len_reader::{VariableReader, VariableWriter};

fn main() -> Result<()> {
    let server = TcpListener::bind("localhost:0")?;
    let mut client = TcpStream::connect(server.local_addr()?)?;
    let mut server = server.incoming().next().unwrap()?;

    // 写
    client.write_string(&"Hello world!")?;

    // 读
    let message = server.read_string()?;
    assert_eq!("Hello world!", message);
    
    Ok(())
}
```

和 [bytes](https://crates.io/crates/bytes) 库一起使用：

```rust
use bytes::{Buf, BufMut, BytesMut};
use variable_len_reader::{VariableReader, VariableWriter};

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

基于 [tokio](https://crates.io/crates/tokio) 的异步模式:
(需要启用 'async_default' 功能)

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

    // 写
    AsyncWriterHelper(&mut client).help_write_string(&"Hello tokio!").await?;

    // 读
    let message = AsyncReaderHelper(&mut server).help_read_string().await?;
    assert_eq!("Hello tokio!", message);
    
    Ok(())
}
```
