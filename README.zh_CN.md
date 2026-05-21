# Qubit Text Codec

[![Rust CI](https://github.com/qubit-ltd/rs-text-codec/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-text-codec/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-text-codec/coverage-badge.json)](https://qubit-ltd.github.io/rs-text-codec/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-text-codec.svg?color=blue)](https://crates.io/crates/qubit-text-codec)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的 buffer-oriented UTF codec 原语，以及 Unicode / ASCII 支撑工具。

## 概述

Qubit Text Codec 是一个低层 codec core，服务于那些需要在 Rust 普通 `str`、`String` 和 `char` API 之下做显式控制的代码。当前内置 codec 聚焦 Unicode transfer format：UTF-8、UTF-16 和 UTF-32；在需要区分 code unit 与 byte representation 的地方，同时提供 code-unit 和 byte-oriented 版本。

本 crate 也提供 codec adapter 需要共用的小型基础面：encoding identity metadata、encoder/decoder trait、decode status、byte order 和 BOM helper，以及具体的 encoding/decoding error 类型。ASCII 和 Unicode namespace helper 保留在这里，是因为 UTF codec 和 text parser 经常需要在 buffer 边界附近直接做这些检查。

适合使用本 crate 的场景包括：

- 需要 ASCII 分类、大小写转换、数字转换和 ASCII folding；
- 需要 Unicode code point 与 scalar value 检查、surrogate 检查、平面计算、noncharacter/control 分类；
- 需要 UTF-8、UTF-16、UTF-32 namespace helper 来做 byte / code-unit 分类和长度计算；
- 需要面向 buffer 的 `TextEncoder<T>` 和 `TextDecoder<T>`，用于 UTF-8、UTF-16、UTF-32；
- 需要处理 UTF-16 / UTF-32 byte stream 的 byte order 和 BOM；
- 需要一组小型 trait 和 error vocabulary，供未来非 Unicode encoding adapter 复用，但不把本 crate 扩成 text I/O framework。

普通文本处理应优先使用 Rust 标准库文本 API。当 parser、二进制格式或 text I/O adapter 需要严格的 buffer-level UTF codec 行为和精确错误位置时，再使用本 crate。

API 参考文档可在 [docs.rs](https://docs.rs/qubit-text-codec) 查看。

## 安装

```toml
[dependencies]
qubit-text-codec = "0.1"
```

## 快速示例

```rust
use qubit_text_codec::{
    ByteOrder,
    DecodeStatus,
    TextDecoder,
    TextEncoder,
    Unicode,
    UnicodeBom,
    Utf8,
    Utf8Decoder,
    Utf8Encoder,
    Utf16,
    Utf16ByteEncoder,
};

assert!(Unicode::is_scalar_value('中' as u32));
assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xE4));
assert_eq!(2, Utf16::unit_len('😀'));
assert_eq!(Some(UnicodeBom::Utf8), UnicodeBom::detect(&[0xEF, 0xBB, 0xBF]));

let decoder = Utf8Decoder;
let decoded = decoder.decode_prefix("中".as_bytes())?;
assert_eq!(
    DecodeStatus::Complete {
        value: '中',
        consumed: 3,
    },
    decoded,
);

let encoder = Utf8Encoder;
let mut utf8 = [0; Utf8::MAX_BYTES_PER_CHAR];
let written = encoder.encode_char('😀', &mut utf8)?;
assert_eq!("😀".as_bytes(), &utf8[..written]);

let utf16 = Utf16ByteEncoder::new(ByteOrder::LittleEndian);
let mut bytes = [0; Utf16::MAX_BYTES_PER_CHAR];
let written = utf16.encode_char('😀', &mut bytes)?;
assert_eq!(&[0x3D, 0xD8, 0x00, 0xDE], &bytes[..written]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

## 主要能力

### 标准依据

UTF-8 解码遵循 [Unicode Standard, Table 3-7](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G7404) 中的 well-formed byte sequence 规则，以及等价的 [RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629) 语法。overlong encoding、surrogate code point 的 UTF-8 编码、非法 continuation byte，以及超过 `U+10FFFF` 的序列都会被视为 malformed。

### Namespace Enum

`qubit-text-codec` 暴露无状态 namespace enum，用于常量、分类、转换和长度计算。编码和解码行为放在专门的 codec 类型中。

| Namespace | 用途 |
| --- | --- |
| `Ascii` | ASCII 常量、分类、大小写转换、数字转换、忽略大小写比较和 ASCII folding |
| `Unicode` | Unicode code point 范围检查、scalar value 检查、surrogate 检查、平面计算、noncharacter 检查、control 检查，以及 `u32` 到 `char` 的转换 |
| `Utf8` | UTF-8 byte 分类和 byte 长度计算 |
| `Utf16` | UTF-16 surrogate 分类、surrogate pair 组合/分解、code-unit 长度计算和 UTF-16 BOM 检测 |
| `Utf32` | UTF-32 scalar unit 校验、unit 长度计算和 UTF-32 BOM 检测 |

### Codec Trait

编码和解码由一组基于调用方 buffer 的小 trait 表达。

| Trait | 用途 |
| --- | --- |
| `TextDecoder<T>` | 从 `&[T]` 中的 encoded unit 解码 Unicode `char` |
| `TextEncoder<T>` | 把 Unicode `char` 编码到 `&mut [T]` |
| `TextCodec<T>` | 为同一种 storage unit 同时实现 encoder 和 decoder 的 blanket trait |

`T` 表示 buffer 的 storage unit，不总是 Unicode code unit。UTF-8 使用 `u8`，UTF-16 code-unit codec 使用 `u16`，byte-serialized UTF-16 使用 `u8`，UTF-32 code-unit codec 使用 `u32`，byte-serialized UTF-32 使用 `u8`。

`TextEncoding` 是轻量的 encoding 身份描述对象，包含稳定 `id`、展示用
`name` 和可接受的 `aliases`。内置描述对象包括 `TextEncoding::ASCII`、
`TextEncoding::UTF_8`、`TextEncoding::UTF_16` 和 `TextEncoding::UTF_32`。
外部 codec crate 可以定义自己的静态描述对象，例如
`TextEncoding::new("gbk", "GBK", &["cp936"])`。相等性和哈希只基于 `id`，
`matches_label` 会用 ASCII 忽略大小写比较来匹配 id、展示名和别名。

### 内置 Codec

| Codec family | Storage unit | 类型 |
| --- | --- | --- |
| UTF-8 bytes | `u8` | `Utf8Encoder`、`Utf8Decoder`、`Utf8Codec` |
| UTF-16 code units | `u16` | `Utf16U16Encoder`、`Utf16U16Decoder`、`Utf16U16Codec` |
| UTF-16 bytes | `u8` | `Utf16ByteEncoder`、`Utf16ByteDecoder`、`Utf16ByteCodec` |
| UTF-32 code units | `u32` | `Utf32U32Encoder`、`Utf32U32Decoder`、`Utf32U32Codec` |
| UTF-32 bytes | `u8` | `Utf32ByteEncoder`、`Utf32ByteDecoder`、`Utf32ByteCodec` |

Byte codec 持有一个 `ByteOrder` 值。如果 byte stream 可能包含 BOM，可使用 `UnicodeBom::detect`、`Utf16::detect_bom` 或 `Utf32::detect_bom`。

### Decode Status 与错误类型

`TextDecoder::decode_prefix` 会区分输入不足和输入非法：

| 类型 | 用途 |
| --- | --- |
| `DecodeStatus::Complete { value, consumed }` | 已解码出完整 scalar value 和消耗的 unit 数 |
| `DecodeStatus::NeedMore { required, available }` | 当前 prefix 目前合法，但还需要更多 unit |
| `TextDecodingError` | 包含 encoding、decoding error kind 和输入 unit index |
| `TextEncodingError` | 包含 encoding、encoding error kind 和输出/输入 index |

`DecodeStatus::NeedMore` 不是错误。流式 text reader 应在可能时继续读取更多输入，并在 EOF 时把它转成 incomplete-sequence error 或合适的 `std::io::Error`。

### ASCII Helper

`Ascii` 让 ASCII-only 行为保持显式且可预测：

| 方法组 | 示例 |
| --- | --- |
| 范围检查 | `is_ascii_byte`、`is_ascii_char`、`is_ascii_code_point` |
| 分类 | `is_whitespace_byte`、`is_letter_char`、`is_digit_code_point`、`is_hex_digit_char`、`is_printable_byte`、`is_control_code_point` |
| 转换 | `byte_to_uppercase`、`char_to_lowercase`、`char_to_digit`、`code_point_to_hex_digit` |
| 比较和 folding | `equals_ignore_case_char`、`equals_ignore_case_code_point`、`fold`、`fold_to_string` |

## Prelude

`qubit_text_codec::prelude` 重导出核心 namespace enum、codec trait、内置 codec 类型、byte-order/BOM helper、decode-status 类型和 text encoding / decoding error。

```rust
use qubit_text_codec::prelude::*;
```

## Crate 边界

`qubit-text-codec` 不是通用文本处理库。它有意保持在 grapheme cluster segmentation、normalization、collation、locale-aware case mapping、transliteration、自动 encoding detection、display-width calculation 以及 `std::io` reader/writer adapter 之下。

当前内置 codec 覆盖 UTF-8、UTF-16 和 UTF-32。它不替代 `encoding_rs` 来处理 GBK、Big5、Shift_JIS 或 Windows code page 等 legacy / web-compatible encoding。未来 crate 可以基于这里的 trait 和错误模型补充这些 encoding，也可以把表驱动规则和兼容性细节委托给专门库。

这些更高层 Unicode 语义应使用 `unicode-segmentation`、`unicode-normalization`、`unicode-width` 或 ICU4X 等专门 crate。

## 依赖

本 crate 使用 `thiserror` 实现错误类型的 `Display` 和 `Error`。

## 测试与代码覆盖率

本项目为 ASCII 分类与 folding、Unicode code point helper、BOM 和 byte-order 处理、UTF-8/UTF-16/UTF-32 namespace helper、buffer-level UTF codec 和 text encoding / decoding error 保持测试覆盖。

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行覆盖率报告
./coverage.sh

# 生成文本格式报告
./coverage.sh text

# 对齐 CI 代码风格
./align-ci.sh

# 运行 CI 检查（格式化、clippy、测试、覆盖率、audit）
./ci-check.sh
```

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

欢迎贡献。请随时提交 Pull Request。

### 开发指南

- 遵循 Rust API 指南。
- 除非需要底层 buffer-oriented codec 控制，否则优先使用 Rust 标准文本 API。
- Namespace enum 只聚焦在常量、分类和长度计算 helper。
- 编码和解码行为应放在实现 `TextEncoder<T>` 和 `TextDecoder<T>` 的具体 codec 类型中。
- normalization、segmentation、collation、显示宽度和 locale-aware 行为请使用专门 Unicode crate 或 ICU4X。
- 保持全面的测试覆盖。
- 公共 API 在有助于说明行为时应提供文档和示例。
- 提交 PR 前确保 `./ci-check.sh` 通过。

## 作者

**Haixing Hu**

## 相关项目

- [qubit-io](https://github.com/qubit-ltd/rs-io)：面向 Rust 的 stream 和字节 I/O 工具库。
- Qubit 旗下的更多 Rust 库发布在 GitHub 组织 [qubit-ltd](https://github.com/qubit-ltd)。

---

仓库地址：[https://github.com/qubit-ltd/rs-text-codec](https://github.com/qubit-ltd/rs-text-codec)
