# Qubit Text Codec

[![Rust CI](https://github.com/qubit-ltd/rs-codec-text/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-codec-text/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-codec-text/coverage-badge.json)](https://qubit-ltd.github.io/rs-codec-text/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-codec-text.svg?color=blue)](https://crates.io/crates/qubit-codec-text)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

`qubit-codec-text` 提供面向缓冲区的 Unicode 与 charset codec，适合 Rust 的解析器、
协议和 I/O adapter 作者在 ASCII、Latin-1、UTF-8、UTF-16、UTF-32 之间进行解码、
编码与转换，并显式管理进度及畸形或不可映射数据的策略。

## 安装

```toml
[dependencies]
qubit-codec-text = "0.4"
qubit-codec = "0.11"
```

只有需要序列化 `Charset` 时才启用 `serde`：

```toml
qubit-codec-text = { version = "0.4", features = ["serde"] }
```

## 快速开始

使用调用方持有的输出缓冲区，将 UTF-8 字节转换为 UTF-16 码元：

```rust
use qubit_codec_text::{CharsetConverter, Utf16U16Codec, Utf8Codec};

let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
let mut output = [0_u16; 2];
let written = converter
    .transcode_complete_into("AB".as_bytes(), &mut output)
    .expect("UTF-8 文本可转换为 UTF-16");

assert_eq!(2, written);
assert_eq!([65, 66], output);
```

## 为什么需要这个项目

文本转换常位于应用字符串层以下：调用方可能拿到部分缓冲区、需要固定输出码元类型，
或需要协议专用的替换策略。本库提供 codec 和策略层，但不接管 `std::io` stream、
缓冲、Unicode 规范化或区域规则。

## 核心能力

| 能力 | 公开 API | 边界 |
| --- | --- | --- |
| Charset 元数据与标签 | `Charset`、`UnicodeBom` | 内置 ASCII、Latin-1、UTF-8、UTF-16、UTF-32 家族；查找不是完整 WHATWG 编码表。 |
| 底层标量 codec | `AsciiCodec`、`Latin1Codec`、`Utf8Codec`、UTF-16/32 codec | unsafe 单值方法直接操作调用方持有的码元。 |
| 缓冲转换 | `CharsetDecoder`、`CharsetEncoder`、`CharsetConverter` | 通过 `qubit-codec` 类型报告进度、输出背压和更多输入需求。 |
| 策略 | `MalformedAction`、`UnmappableAction` | 可选 `Replace`、`Ignore` 或 `Report`。 |
| 字符辅助工具 | `Ascii`、`Unicode`、`Utf8`、`Utf16`、`Utf32` | 不提供字素簇切分、规范化、排序或区域感知大小写。 |

面向字节的 UTF-16 与 UTF-32 codec 使用显式 `ByteOrder`，不会自动写出、跳过或选择
BOM。stream 所有权、缓冲和 `std::io::Error` 转换属于更高层 adapter。

## 延伸阅读

- [用户指南](doc/user_guide.zh_CN.md)
- [API 文档](https://docs.rs/qubit-codec-text)
- [英文 README](README.md)
- [English user guide](doc/user_guide.md)

## 测试

```bash
# 使用默认 feature 集运行测试
cargo test

# 使用项目声明的全部 feature 运行测试
cargo test --all-features

# 运行项目 CI 检查
./ci-check.sh

# 检查代码覆盖率
./coverage.sh
```

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

本项目基于 Apache License 2.0 授权。完整许可证文本请参阅
[LICENSE](LICENSE)。

## 贡献

欢迎贡献。请遵循 Rust API 指南，及时更新公共 API 文档与测试，并在提交
Pull Request 前运行 `./align-ci.sh`格式化代码，运行`./ci-check.sh`对齐CI要求。

## 作者

**Haixing Hu** - *Qubit Co. Ltd.*

仓库地址：[https://github.com/qubit-ltd/rs-codec-text](https://github.com/qubit-ltd/rs-codec-text)
