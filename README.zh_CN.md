# Qubit Text Codec

[![Rust CI](https://github.com/qubit-ltd/rs-codec-text/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-codec-text/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-codec-text/coverage-badge.json)](https://qubit-ltd.github.io/rs-codec-text/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-codec-text.svg?color=blue)](https://crates.io/crates/qubit-codec-text)
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
- 面向调用方重导出的必要 `qubit-codec` 原语：`Codec`、`BufferedTranscoder`、
  `TranscodeProgress`、`TranscodeStatus`、`CapacityError` 和 `ByteOrder`。

本库刻意停留在 `std::io` 读写适配器、自动 charset 检测、规范化、切分、
排序、显示宽度和区域相关文本行为之下。

## 设计目标

- **缓冲区级控制**：暴露直接操作调用方管理缓冲区的 charset codec。
- **Unicode 基础能力**：提供 ASCII、Unicode、UTF-8、UTF-16 和 UTF-32 原语，不处理更高层的 locale 行为。
- **策略明确的转换**：通过 decoder 和 encoder 配置显式控制 malformed 与 unmappable 行为。
- **诊断精确**：用强类型错误报告源下标和上下文。
- **不耦合 I/O**：stream adapter 放在 `qubit-io-text`。
- **核心依赖小**：依赖 `qubit-codec` 复用 transcoder 和字节序原语。

## 特性

### Charset 元数据

- **`Charset`**：识别支持的 charset 及其字节序行为。
- **`UnicodeBom`**：检测 Unicode byte order mark。
- **ASCII 与 Unicode 命名空间**：提供常量和校验 helper。

### 缓冲区级 Codec

- **`AsciiCodec`**：ASCII byte codec。
- **`Latin1Codec`**：ISO-8859-1 byte codec。
- **`Utf8Codec`**：UTF-8 byte codec。
- **`Utf16ByteCodec` / `Utf32ByteCodec`**：显式字节序的 Unicode byte codec。
- **`Utf16U16Codec` / `Utf32U32Codec`**：unit-oriented Unicode codec。

### 有状态 Converter

- **`CharsetDecoder`**：把输入单元解码为 `char` 输出。
- **`CharsetEncoder`**：把 `char` 输入编码为目标单元。
- **`CharsetConverter`**：在 decoder 与 encoder 组合之间转换。
- **`MalformedAction` / `UnmappableAction`**：配置 strict 或 replacement 行为。
- **EOF 收尾**：`finish()` 只刷新内部暂存输出；调用方需要先处理 `NeedInput` 报告的不完整源尾部。

### 聚焦的公开 API

- **`prelude` 模块**：导入常用 charset、codec、error 和核心 transcoder 类型。
- **不包含 stream I/O**：reader 和 writer adapter 使用 `qubit-io-text`。

## 文档

- [用户指南](doc/user_guide.zh_CN.md)
- [API 文档](https://docs.rs/qubit-codec-text)
- [英文 README](README.md)

## 安装

```toml
[dependencies]
qubit-codec-text = "0.1"
```

`qubit-codec` 是核心运行时依赖。本库只重导出普通 text-codec 调用需要的
core trait 和状态类型；通用 engine、hook、adapter 请直接从 `qubit-codec`
引入。

## 快速开始

```rust
use qubit_codec_text::{
    CharsetEncoder,
    Codec,
    TranscodeStatus,
    BufferedTranscoder,
    UnicodeBom,
    Utf8,
    Utf8Codec,
};

assert_eq!(Some(UnicodeBom::Utf8), UnicodeBom::detect(&[0xef, 0xbb, 0xbf]));
assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xe4));

let (decoded, consumed) = unsafe {
    Utf8Codec
        .decode_unchecked("中".as_bytes(), 0)
}
    .expect("valid UTF-8 input");
assert_eq!(('中', 3), (decoded, consumed.get()));

let mut encoder = CharsetEncoder::new(Utf8Codec);
let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
let progress = encoder
    .transcode(&['😀'], 0, &mut output, 0)
    .expect("UTF-8 output buffer is large enough");

assert_eq!(TranscodeStatus::Complete, progress.status());
assert_eq!("😀".as_bytes(), &output[..progress.written()]);
```

## API 参考

### Charset 与 Unicode 类型

| 类型 | 用途 |
|------|------|
| `Charset` | 支持的 charset 身份和字节序元数据 |
| `UnicodeBom` | Unicode BOM 检测 |
| `Ascii`、`Unicode`、`Utf8`、`Utf16`、`Utf32` | 字符集规则命名空间 helper |

### Codec 类型

| 类型 | 用途 |
|------|------|
| `AsciiCodec` | ASCII byte 编码和解码 |
| `Latin1Codec` | ISO-8859-1 byte 编码和解码 |
| `Utf8Codec` | UTF-8 byte 编码和解码 |
| `Utf16ByteCodec` / `Utf32ByteCodec` | 显式字节序的 Unicode byte codec |
| `Utf16U16Codec` / `Utf32U32Codec` | Unit-oriented Unicode codec |
| `Codec<Value = char>` | 从 `qubit-codec` 重导出的最低层完整值 codec trait |
| `CharsetCodec` | 附加在低层文本 codec 实现上的 charset 元数据 |
| `CharsetEncodeProbe` | 单字符输出长度和可映射性探测 |

### Converter 类型

| 类型 | 用途 |
|------|------|
| `CharsetDecoder<C>` | 实现 `BufferedDecoder<C::Unit, char>` 的有状态缓冲区 decoder，并复用 `BufferedDecodeEngine` 处理解码迭代和 progress 报告 |
| `CharsetEncoder<C>` | 实现 `BufferedEncoder<char, C::Unit>` 的有状态缓冲区 encoder，并复用 `BufferedEncodeEngine` 的公共循环 |
| `CharsetConverter<D, E>` | 在两个 charset codec 之间 decode + encode，并实现 `BufferedConverter<D::Unit, E::Unit>` |
| `MalformedAction` | Malformed input 处理策略 |
| `UnmappableAction` | 无法编码输出字符的处理策略 |

### 错误类型

| 类型 | 用途 |
|------|------|
| `CharsetDecodeError` / `CharsetDecodeErrorKind` | 带精确下标的 decode failure |
| `CharsetEncodeError` / `CharsetEncodeErrorKind` | 带精确下标的 encode failure |
| `CharsetConvertError` | Converter 层面的 decode 或 encode failure |

## 性能考虑

Codec 实现直接操作调用方提供的输入和输出缓冲区。`CharsetDecoder` 在至少有
`codec.min_units_per_value()` 个可读单元时调用 `Codec::decode_unchecked`，
charset codec 通过 `CharsetDecodeError` 报告不完整前缀。`NeedInput` 表示当前单元是
合法但不完整的前缀，尾部仍留在调用方输入缓冲区中；到达 EOF 后，调用方先处理这个
尾部，再调用 `finish()` 刷新内部暂存输出。内部实现上，`CharsetDecoder` 复用
decode hooks 保存策略，并复用 `BufferedDecodeEngine` 处理重复调用
`decode_unchecked`、输出容量 progress 和状态报告。`CharsetEncoder` 通过 encode hooks 保存
unmappable 策略，并复用 `BufferedEncodeEngine` 处理输入迭代和输出容量检查，
同时保留 text-specific 的 replace、ignore、report 策略。它通过共享的 `BufferedTranscoder` 进度模型报告
`NeedOutput`，调用方可以自行控制分配和缓冲区复用。

## 测试与代码覆盖率

本项目通过 `tests/` 下的集成测试覆盖 charset 行为。

### 运行测试

```bash
# 运行测试
cargo test

# 运行覆盖率报告
./coverage.sh

# 生成文本格式报告
./coverage.sh text

# 按 CI 口径对齐格式和 clippy
./align-ci.sh

# 运行 CI 检查（格式化、clippy、测试、覆盖率、安全审计）
RS_CI_SKIP_TOOLCHAIN_UPDATE=1 ./ci-check.sh
```

## 依赖项

运行时依赖保持很少：

- `qubit-codec` 提供共享字节序和 transcoder 原语。
- `thiserror` 提供公共错误类型实现。

## 许可证

Copyright (c) 2026. Haixing Hu.

根据 Apache 许可证 2.0 版（"许可证"）授权；
除非遵守许可证，否则您不得使用此文件。
您可以在以下位置获取许可证副本：

    http://www.apache.org/licenses/LICENSE-2.0

除非适用法律要求或书面同意，否则根据许可证分发的软件
按"原样"分发，不附带任何明示或暗示的担保或条件。
有关许可证下的特定语言管理权限和限制，请参阅许可证。

完整的许可证文本请参阅 [LICENSE](LICENSE)。

## 贡献

欢迎贡献！请随时提交 Pull Request。

### 开发指南

- 保持本 crate 聚焦缓冲区级 text codec。
- 保持文档与用户指南和公开 API 名称一致。
- 为 strict、replacement、malformed 和 unmappable 行为补测试。
- 提交 PR 前确保所有检查通过。

## 作者

**胡海星**

## 相关项目

- [qubit-codec](https://github.com/qubit-ltd/rs-codec)：共享核心 codec trait 与字节序标记。
- [qubit-io-text](https://github.com/qubit-ltd/rs-io-text)：文本 stream adapter
  工具库。
- Qubit 旗下的更多 Rust 库发布在 GitHub 组织
  [qubit-ltd](https://github.com/qubit-ltd)。

仓库地址：[https://github.com/qubit-ltd/rs-codec-text](https://github.com/qubit-ltd/rs-codec-text)
