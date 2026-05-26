# Qubit Text Codec

[![Rust CI](https://github.com/qubit-ltd/rs-text-codec/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-text-codec/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-text-codec/coverage-badge.json)](https://qubit-ltd.github.io/rs-text-codec/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-text-codec.svg?color=blue)](https://crates.io/crates/qubit-text-codec)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的缓冲区级 UTF 编解码原语，以及 Unicode / ASCII 支撑工具。

## 概述

Qubit Text Codec 是一个低层编解码核心，服务于那些需要在 Rust 普通 `str`、`String` 和 `char` API 之下做显式控制的代码。当前内置编解码器聚焦 Unicode 传输格式：ASCII、ISO-8859-1（Latin-1）、UTF-8、UTF-16 和 UTF-32；在需要区分码元与字节表示的地方，同时提供码元版和面向字节的版本。

本库也提供编解码适配器需要共用的小型基础能力：charset 身份元数据、从 `qubit-io` 重导出的通用 `Coder` 接口和进度类型、低层 `CharsetCodec`（其关联类型 `Unit` 表示存储单元）、带策略的 `CharsetEncoder` / `CharsetDecoder` / `CharsetConverter`、解码状态、字节序和 BOM 辅助工具，以及具体的编码/解码错误类型。ASCII 和 Unicode 命名空间辅助工具保留在这里，是因为 UTF 编解码器和文本解析器经常需要在缓冲区边界附近直接做这些检查。

适合使用本库的场景包括：

- 需要 ASCII 分类、大小写转换、数字转换和 ASCII 折叠；
- 需要 Unicode 码点与标量值检查、代理项检查、平面计算、非字符/控制字符分类；
- 需要 UTF-8、UTF-16、UTF-32 命名空间辅助工具来做字节/码元分类和长度计算；
- 需要面向缓冲区的 `CharsetCodec`，用于 ASCII、ISO-8859-1、UTF-8、UTF-16、UTF-32；
- 需要带 malformed / unmappable 处理策略的 charset encoder、decoder 和 converter；
- 需要处理 UTF-16 / UTF-32 字节流的字节序和 BOM；
- 需要一组小型接口和错误类型体系，供未来非 Unicode 编码适配器复用，但不把本库扩成文本 I/O 框架。

普通文本处理应优先使用 Rust 标准库文本 API。当文本解析器、二进制格式或文本 I/O 适配器需要严格的缓冲区级 UTF 编解码行为和精确错误位置时，再使用本库。

API 参考文档可在 [docs.rs](https://docs.rs/qubit-text-codec) 查看。

## 安装

```toml
[dependencies]
qubit-text-codec = "0.1"
```

`qubit-io` 是本库的运行时依赖；公开 API 用到的核心缓冲区级接口和
progress 类型已经由 `qubit-text-codec` 重导出。只有在业务代码直接使用
`qubit_io::...` API 时，才需要额外显式添加 `qubit-io = "0.5"`。

## 快速示例

```rust
use qubit_text_codec::{
    ByteOrder,
    CharsetCodec,
    CharsetDecoder,
    CharsetEncoder,
    Coder,
    CoderStatus,
    DecodeStatus,
    Unicode,
    UnicodeBom,
    Utf8,
    Utf8Codec,
    Utf16,
    Utf16ByteCodec,
};

assert!(Unicode::is_scalar_value('中' as u32));
assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xE4));
assert_eq!(2, Utf16::unit_len('😀'));
assert_eq!(Some(UnicodeBom::Utf8), UnicodeBom::detect(&[0xEF, 0xBB, 0xBF]));

let codec = Utf8Codec;
let decoded = codec.decode_one("中".as_bytes(), 0)?;
assert_eq!(
    DecodeStatus::Complete {
        value: '中',
        consumed: 3,
    },
    decoded,
);

let mut encoder = CharsetEncoder::new(Utf8Codec);
let mut utf8 = [0; Utf8::MAX_BYTES_PER_CHAR];
let progress = encoder.convert(&['😀'], 0, &mut utf8, 0)?;
assert_eq!(CoderStatus::Complete, progress.status());
assert_eq!("😀".as_bytes(), &utf8[..progress.written()]);

let mut decoder = CharsetDecoder::new(Utf8Codec);
let mut chars = ['\0'; 1];
let progress = decoder.convert("A".as_bytes(), 0, &mut chars, 0)?;
assert_eq!(CoderStatus::Complete, progress.status());
assert_eq!('A', chars[0]);

let mut utf16 = CharsetEncoder::new(Utf16ByteCodec::new(ByteOrder::LittleEndian));
let mut bytes = [0; Utf16::MAX_BYTES_PER_CHAR];
let progress = utf16.convert(&['😀'], 0, &mut bytes, 0)?;
assert_eq!(&[0x3D, 0xD8, 0x00, 0xDE], &bytes[..progress.written()]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

## 主要能力

### 标准依据

UTF-8 解码遵循 [Unicode Standard 表 3-7](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G7404) 中“格式良好的字节序列”规则，以及等价的 [RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629) 语法。过长编码、代理项码点的 UTF-8 编码、非法续字节，以及超过 `U+10FFFF` 的序列都会被视为非法序列。

### 命名空间枚举

`qubit-text-codec` 暴露无状态命名空间枚举，用于常量、分类、转换和长度计算。编码和解码行为放在专门的编解码器类型中。

| 命名空间 | 用途 |
| --- | --- |
| `Ascii` | ASCII 常量、分类、大小写转换、数字转换、忽略大小写比较和 ASCII 折叠 |
| `Unicode` | Unicode 码点范围检查、标量值检查、代理项检查、平面计算、非字符检查、控制字符检查，以及 `u32` 到 `char` 的转换 |
| `Utf8` | UTF-8 字节分类和字节长度计算 |
| `Utf16` | UTF-16 代理项分类、代理项对组合/分解、码元长度计算和 UTF-16 BOM 检测 |
| `Utf32` | UTF-32 标量单元校验、单元长度计算和 UTF-32 BOM 检测 |

### 编解码层次

编码和解码被拆成三层，全部基于调用方提供的缓冲区工作。

| 层次 | 类型 | 用途 |
| --- | --- |
| 通用转换 | `Coder<Input, Output>` | 把一种 code unit 序列转换成另一种，并返回 `CoderProgress` |
| 低层 charset 算法 | `CharsetCodec` | 用其关联存储单元对单个 Unicode `char` 编码或解码 |
| 带策略的解码器 | `CharsetDecoder<C>` | 把源单元转换为 `char`，并应用 `MalformedAction` |
| 带策略的编码器 | `CharsetEncoder<C>` | 把 `char` 转换为目标单元，并应用 `UnmappableAction` |
| charset 转换器 | `CharsetConverter<D, E>` | 组合一个 decoder 和一个 encoder，在两个 charset 之间转换 |

`Unit` 表示缓冲区的存储单元，不总是 Unicode 码元。UTF-8 使用 `u8`，UTF-16 的码元版使用 `u16`，按字节序列化的 UTF-16 使用 `u8`，UTF-32 的码元版使用 `u32`，按字节序列化的 UTF-32 使用 `u8`。

`Coder` 是通用流式 API。它返回 `NeedInput` / `NeedOutput` 时，会给出绝对下标、还需数量和已用数量，适合“可分片 + 回压”场景。

- 需要增量处理时直接用 `Coder::convert`；
- 只想把 malformed / unmappable 映射策略固定成可复用行为时，用 `CharsetDecoder` / `CharsetEncoder`；
- 想把一个 charset 转成另一个 charset（通常通过中间 Unicode 码点流）时，用 `CharsetConverter`。

所有转换接口都接收完整输入/输出 slice 以及绝对起始下标。返回的 progress 计数相对于这些起始下标，而错误对象里的 index 直接指向传入缓冲区中的绝对位置。

`Charset` 是轻量的 charset 身份描述对象，包含稳定 `id`、展示用
`name` 和可接受的 `aliases`。内置描述对象包括 `Charset::ASCII`、`Charset::ISO_8859_1`、`Charset::UTF_8`、`Charset::UTF_16`、`Charset::UTF_16LE`、
`Charset::UTF_16BE`、`Charset::UTF_32`、`Charset::UTF_32LE` 和
`Charset::UTF_32BE`。泛化的 `UTF_16` 和 `UTF_32` 表示 Unicode 码元形式
或 BOM-aware 字节流标签；`LE` / `BE` 变体表示固定字节序的字节流 charset。
外部编解码库可以定义自己的静态描述对象，例如
`Charset::new("gbk", "GBK", &["cp936"])`。相等性和哈希只基于 `id`，
`matches_label` 会用 ASCII 忽略大小写比较来匹配 id、展示名和别名。

### 内置编解码器

| 编解码器族 | 存储单元 | 低层 codec |
| --- | --- | --- |
| ASCII 字节 | `u8` | `AsciiCodec` |
| ISO-8859-1 字节 | `u8` | `Latin1Codec` |
| UTF-8 字节 | `u8` | `Utf8Codec` |
| UTF-16 码元 | `u16` | `Utf16U16Codec` |
| UTF-16 字节 | `u8` | `Utf16ByteCodec` |
| UTF-32 码元 | `u32` | `Utf32U32Codec` |
| UTF-32 字节 | `u8` | `Utf32ByteCodec` |

字节编解码器持有一个 `ByteOrder` 值。如果字节流可能包含 BOM，可使用 `UnicodeBom::detect`、`Utf16::detect_bom` 或 `Utf32::detect_bom`。字节编解码器不会自动检测、跳过或写出 BOM。流式调用方应先缓冲最多 4 个字节，或读到 EOF，再判断 BOM，因为 UTF-32 little-endian（`FF FE 00 00`）和 UTF-16 little-endian 前缀（`FF FE`）存在重叠。

### 解码状态与错误类型

`CharsetCodec::decode_one` 和策略层 decoder 会区分输入不足和输入非法：

| 类型 | 用途 |
| --- | --- |
| `DecodeStatus::Complete { value, consumed }` | 已解码出完整标量值和消耗的单元数 |
| `DecodeStatus::NeedMore { required, available }` | 当前前缀目前合法，但还需要更多单元 |
| `CharsetDecodeError` | 包含 charset、解码错误种类、输入单元下标和可选原始值 |
| `CharsetEncodeError` | 包含 charset、编码错误种类、输出/输入下标和可选原始值 |

`DecodeStatus::NeedMore` 不是错误。流式文本读取器应在可能时继续读取更多输入，并在输入结束时把它转成不完整序列错误或合适的 `std::io::Error`。当关闭输入应转换为 `CharsetDecodeErrorKind::IncompleteSequence` 时，可使用 `DecodeStatus::incomplete_error(charset, index)`。

`CharsetDecodeErrorKind::InvalidInputIndex` 表示调用方传入的输入下标超过了当前缓冲区范围，并可通过 `input_len()` 取回缓冲区长度。它不同于 `MalformedSequence`；后者只表示缓冲区中实际存在、但对当前 charset 非法的字节或码元。

与原始值相关的错误，例如非法 UTF-32 单元或无法映射的字符，可通过 `value()` 取回该值。

### ASCII 辅助工具

`Ascii` 让仅 ASCII 行为保持显式且可预测：

| 方法组 | 示例 |
| --- | --- |
| 范围检查 | `is_ascii_byte`、`is_ascii_char`、`is_ascii_code_point` |
| 分类 | `is_whitespace_byte`、`is_letter_char`、`is_digit_code_point`、`is_hex_digit_char`、`is_printable_byte`、`is_control_code_point` |
| 转换 | `byte_to_uppercase`、`char_to_lowercase`、`char_to_digit`、`code_point_to_hex_digit` |
| 比较和折叠 | `equals_ignore_case_char`、`equals_ignore_case_code_point`、`fold`、`fold_to_string` |

## 预导入模块

`qubit_text_codec::prelude` 重导出核心命名空间枚举、coder 和 charset codec 接口、策略包装器、内置 codec 类型、charset 描述对象、字节序/BOM 辅助工具、解码状态、处理动作和文本编码/解码错误类型。

```rust
use qubit_text_codec::prelude::*;
```

## 库边界

`qubit-text-codec` 不是通用文本处理库。它有意保持在字素簇切分、规范化、排序、按区域设置处理大小写映射、转写、自动编码识别、显示宽度计算以及 `std::io` 读写适配器之下。

当前内置编解码器覆盖 ASCII、ISO-8859-1、UTF-8、UTF-16 和 UTF-32。它不替代 `encoding_rs` 来处理 GBK、Big5、Shift_JIS 或 Windows 代码页等历史编码 / Web 兼容编码。未来的库可以基于这里的接口和错误模型补充这些编码，也可以把表驱动规则和兼容性细节委托给专门库。

这些更高层 Unicode 语义应使用 `unicode-segmentation`、`unicode-normalization`、`unicode-width` 或 ICU4X 等专门库。

## 依赖

本库使用 `qubit-io` 提供并重导出缓冲区级 API 所需的 `Coder` 接口、
progress/status 值、字节序辅助工具和二进制字节编解码能力；同时使用
`thiserror` 实现错误类型的 `Display` 和 `Error`。

## 测试与代码覆盖率

本项目为 ASCII 分类与折叠、Unicode 码点辅助工具、BOM 和字节序处理、charset 描述对象、ASCII/Latin-1/UTF-8/UTF-16/UTF-32 命名空间辅助工具、缓冲区级编解码器和文本编码动作/解码动作错误类型保持测试覆盖。

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

欢迎贡献。请随时提交拉取请求。

### 开发指南

- 遵循 Rust API 指南。
- 除非需要底层面向缓冲区的编解码控制，否则优先使用 Rust 标准文本 API。
- 命名空间枚举只聚焦在常量、分类和长度计算辅助工具。
- charset 相关算法应放在具体的 `CharsetCodec` 类型中；malformed / unmappable 策略应放在 `CharsetEncoder`、`CharsetDecoder` 或 `CharsetConverter` 中。
- 规范化、切分、排序、显示宽度和按区域设置处理的行为请使用专门的 Unicode 库或 ICU4X。
- 保持全面的测试覆盖。
- 公共 API 在有助于说明行为时应提供文档和示例。
- 提交 PR 前确保 `./ci-check.sh` 通过。

## 作者

**Haixing Hu**

## 相关项目

- [qubit-io](https://github.com/qubit-ltd/rs-io)：面向 Rust 的流和字节 I/O 工具库。
- Qubit 旗下的更多 Rust 库发布在 GitHub 组织 [qubit-ltd](https://github.com/qubit-ltd)。

---

仓库地址：[https://github.com/qubit-ltd/rs-text-codec](https://github.com/qubit-ltd/rs-text-codec)
