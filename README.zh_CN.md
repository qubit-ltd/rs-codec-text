# Qubit Unicode

[![Rust CI](https://github.com/qubit-ltd/rs-unicode/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-unicode/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/qubit-unicode.svg?color=blue)](https://crates.io/crates/qubit-unicode)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的底层 Unicode、UTF-8、UTF-16 和 ASCII 工具。

## 概述

Qubit Unicode 提供一组小型 namespace enum，用于普通 Rust `str` API
之下的 code-unit 与 code-point 操作：

- `Ascii`：ASCII 分类、大小写转换、数字转换和兼容 Java 版的 ASCII folding；
- `Unicode`：码点范围检查、代理对工具、平面计算和 Java 风格 `\uXXXX` 转义；
- `Utf8`：严格 UTF-8 代码单元分类、游标移动、解码以及写入调用方提供的字节缓冲区；
- `Utf16`：UTF-16 代码单元分类、代理对游标移动、解码、编码以及
  Java/JavaScript 风格 `\uXXXX` 转义。

普通文本处理应优先使用 Rust 标准库的 `str`、`String` 和 `char` API。
当 parser 或 codec 需要精确控制 byte 或 UTF-16 code unit 时，再使用本 crate。

## 安装

```toml
[dependencies]
qubit-unicode = "0.1"
```

## 示例

```rust
use qubit_unicode::{ParsingPosition, Utf8};

let bytes = "A中".as_bytes();
let mut pos = ParsingPosition::new(1);

let ch = Utf8::get_next(&mut pos, bytes, bytes.len())?;
assert_eq!(Some('中'), ch);
assert_eq!(4, pos.index());

# Ok::<(), qubit_unicode::UnicodeError>(())
```

## Crate 边界

本 crate 有意保持在完整 Unicode 文本处理能力之下，不实现 grapheme cluster
分割、normalization、collation、locale-aware 大小写转换或显示宽度计算。
这些更高层语义应使用 `unicode-segmentation`、`unicode-normalization`、
`unicode-width` 或 ICU4X 等专门 crate。

## 开发

```bash
./align-ci.sh
./ci-check.sh
```

## 许可证

本项目基于 Apache License, Version 2.0 授权。详见 [LICENSE](LICENSE)。
