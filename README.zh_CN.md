# Qubit Text Codec

[![Rust CI](https://github.com/qubit-ltd/rs-text-codec/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-text-codec/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-text-codec/coverage-badge.json)](https://qubit-ltd.github.io/rs-text-codec/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-text-codec.svg?color=blue)](https://crates.io/crates/qubit-text-codec)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的缓冲区级 charset 编解码原语，以及 Unicode / ASCII 支撑工具。

## 概述

Qubit Text Codec 是一个低层编解码核心，服务于那些需要在 Rust 普通
`str`、`String` 和 `char` API 之下做显式控制的代码。它提供：

- ASCII、Unicode、UTF-8、UTF-16、UTF-32 命名空间辅助工具。
- Charset 身份元数据、字节序辅助工具和 Unicode BOM 检测。
- ASCII、ISO-8859-1、UTF-8、UTF-16、UTF-32 的缓冲区级 codec。
- 带策略的 `CharsetDecoder`、`CharsetEncoder` 和 `CharsetConverter`。
- 带精确缓冲区下标的强类型 decode / encode / convert 错误。
- 从 `qubit-io` 重导出的 `Coder`、`CoderProgress`、`CoderStatus` 和
  `ByteOrder`。

本库刻意停留在 `std::io` 读写适配器、自动 charset 检测、规范化、切分、
排序、显示宽度和区域相关文本行为之下。

## 文档

- [用户指南](doc/user_guide.zh_CN.md)
- [API 文档](https://docs.rs/qubit-text-codec)
- [英文 README](README.md)

## 安装

```toml
[dependencies]
qubit-text-codec = "0.1"
```

`qubit-io` 是运行时依赖。只有在业务代码直接使用 `qubit_io::...` API 时，
才需要额外添加 `qubit-io = "0.5"`。

## 快速示例

```rust
use qubit_text_codec::{
    CharsetCodec,
    CharsetEncoder,
    Coder,
    CoderStatus,
    DecodeStatus,
    UnicodeBom,
    Utf8,
    Utf8Codec,
};

assert_eq!(Some(UnicodeBom::Utf8), UnicodeBom::detect(&[0xef, 0xbb, 0xbf]));
assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xe4));

let decoded = Utf8Codec
    .decode_one("中".as_bytes(), 0)
    .expect("valid UTF-8 input");
assert_eq!(
    DecodeStatus::Complete {
        value: '中',
        consumed: 3,
    },
    decoded,
);

let mut encoder = CharsetEncoder::new(Utf8Codec);
let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
let progress = encoder
    .convert(&['😀'], 0, &mut output, 0)
    .expect("UTF-8 output buffer is large enough");

assert_eq!(CoderStatus::Complete, progress.status());
assert_eq!("😀".as_bytes(), &output[..progress.written()]);
```

## 开发

```bash
# 运行测试
cargo test

# 按 CI 口径对齐格式和 clippy
./align-ci.sh

# 运行完整本地 CI
RS_CI_SKIP_TOOLCHAIN_UPDATE=1 ./ci-check.sh
```

## 许可证

Copyright (c) 2026. Haixing Hu.

根据 Apache License 2.0 授权。完整许可证文本见 [LICENSE](LICENSE)。

## 相关项目

- [qubit-io](https://github.com/qubit-ltd/rs-io)：面向 Rust 的流和字节 I/O
  工具库。
- Qubit 旗下的更多 Rust 库发布在 GitHub 组织
  [qubit-ltd](https://github.com/qubit-ltd)。

仓库地址：[https://github.com/qubit-ltd/rs-text-codec](https://github.com/qubit-ltd/rs-text-codec)
