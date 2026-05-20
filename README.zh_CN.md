# Qubit Unicode

[![Rust CI](https://github.com/qubit-ltd/rs-unicode/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-unicode/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-unicode/coverage-badge.json)](https://qubit-ltd.github.io/rs-unicode/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-unicode.svg?color=blue)](https://crates.io/crates/qubit-unicode)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的底层 Unicode、UTF-8、UTF-16 和 ASCII 工具。

## 概述

Qubit Unicode 提供一组小型 namespace enum，用于普通 Rust `str` API 之下的 code-unit 与 code-point 操作。它面向 parser、codec、兼容层和二进制格式，用于显式控制 byte、UTF-16 code unit、Unicode scalar value 或 ASCII-only 行为。

适合使用本 crate 的场景包括：

- 需要 ASCII 分类、大小写转换、数字转换和 ASCII folding；
- 需要 Unicode 码点范围检查、代理对工具、平面计算和 Java 风格 `\uXXXX` 转义；
- 需要严格 UTF-8 byte 分类、游标移动、解码以及写入调用方提供的字节缓冲区；
- 需要 UTF-16 code unit 分类、代理对游标移动、解码、编码以及 Java/JavaScript 风格 `\uXXXX` 转义；
- 需要对 malformed 或 incomplete Unicode 序列提供明确的位置和错误类型。

普通文本处理应优先使用 Rust 标准库的 `str`、`String` 和 `char` API。
当 parser 或 codec 需要精确控制 byte 或 UTF-16 code unit 时，再使用本 crate。

详细用法、示例和 API 选择建议请参见[中文用户手册](doc/user_guide.zh_CN.md)。API 参考文档可在 [docs.rs](https://docs.rs/qubit-unicode) 查看。

## 安装

```toml
[dependencies]
qubit-unicode = "0.1"
```

## 快速示例

```rust
use qubit_unicode::{
    Ascii,
    ParsingPosition,
    Unicode,
    Utf8,
    Utf16,
};

assert!(Ascii::equals_ignore_case_char('Q', 'q'));
assert_eq!(Some(10), Ascii::to_hex_digit_char('A'));
assert_eq!("\\u1F600", Unicode::escape(0x1f600).unwrap());

let bytes = "A中".as_bytes();
let mut pos = ParsingPosition::new(1);

let ch = Utf8::get_next(&mut pos, bytes, bytes.len())?;
assert_eq!(Some('中'), ch);
assert_eq!(4, pos.index());

let mut units = [0; Utf16::MAX_CODE_UNIT_COUNT];
let capacity = units.len();
let written = Utf16::put(0x1f600, 0, &mut units, capacity)?;
assert_eq!(2, written);
assert_eq!([0xd83d, 0xde00], units);
assert_eq!("\\uD83D\\uDE00", Utf16::escape(0x1f600).unwrap());

# Ok::<(), qubit_unicode::UnicodeError>(())
```

## 主要能力

### Namespace Enum

`qubit-unicode` 暴露的是无状态 namespace enum，而不是需要分配的 helper object：

| Namespace | 用途 |
| --- | --- |
| `Ascii` | ASCII 常量、分类、大小写转换、数字转换、忽略大小写比较和 ASCII folding |
| `Unicode` | Unicode scalar 范围检查、BMP / supplementary 检查、代理对组合与分解、平面计算和 Java 风格转义 |
| `Utf8` | UTF-8 byte 分类、code unit 数量计算、游标移动、严格解码、反向解码和 scalar 编码 |
| `Utf16` | UTF-16 code unit 分类、代理对处理、游标移动、解码、反向解码、scalar 编码和 UTF-16 转义 |

### ASCII Helper

`Ascii` 让 ASCII-only 行为保持显式且可预测：

| 方法组 | 示例 |
| --- | --- |
| 范围检查 | `is_ascii_byte`、`is_ascii_char`、`is_ascii_code_point` |
| 分类 | `is_whitespace_byte`、`is_letter_char`、`is_digit_code_point`、`is_hex_digit_char`、`is_printable_byte`、`is_control_code_point` |
| 转换 | `to_upper_case_byte`、`to_lower_case_char`、`to_digit_char`、`to_hex_digit_code_point` |
| 比较和 folding | `equals_ignore_case_char`、`equals_ignore_case_code_point`、`fold`、`fold_to_string` |

### UTF-8 与 UTF-16 游标 API

`Utf8` 和 `Utf16` 操作调用方提供的 slice 与 `ParsingPosition`。当 parser 需要在较大的 buffer 内局部解码、且不希望接管 buffer 所有权时，这些 API 会更合适：

| 方法组 | UTF-8 示例 | UTF-16 示例 |
| --- | --- | --- |
| Code unit 分类 | `is_single`、`is_leading`、`is_trailing` | `is_single`、`is_leading`、`is_trailing`、`is_surrogate` |
| 长度计算 | `trailing_count`、`code_unit_count` | `trailing_count`、`code_unit_count` |
| 游标移动 | `set_to_start`、`set_to_terminal`、`forward`、`backward` | `set_to_start`、`set_to_terminal`、`forward`、`backward` |
| 解码和编码 | `get_next`、`get_previous`、`put` | `get_next`、`get_previous`、`put`、`escape` |

### 位置与错误类型

游标 API 返回 `UnicodeResult<T>`，并报告问题被检测到的位置：

| 类型 | 用途 |
| --- | --- |
| `ParsingPosition` | 可变游标，包含可选的错误索引和错误类型 |
| `UnicodeError` | 包含 `UnicodeErrorKind` 和 byte/code-unit 索引的错误值 |
| `UnicodeErrorKind` | `BufferOverflow`、`Malformed` 或 `Incomplete` |
| `UnicodeResult<T>` | `Result<T, UnicodeError>` 的便捷别名 |

## Prelude

`qubit_unicode::prelude` 重导出小型公共 API：namespace enum、游标与错误类型，以及 `UnicodeResult`。

```rust
use qubit_unicode::prelude::*;
```

## Crate 边界

`qubit-unicode` 有意保持在完整 Unicode 文本处理能力之下，不实现 grapheme cluster 分割、normalization、collation、locale-aware 大小写转换、transliteration、encoding detection 或显示宽度计算。

这些更高层语义应使用 `unicode-segmentation`、`unicode-normalization`、`unicode-width` 或 ICU4X 等专门 crate。

## 依赖

本 crate 使用 `thiserror` 实现错误类型的 `Display` 和 `Error`。

## 测试与代码覆盖率

本项目为 ASCII 分类与 folding、Unicode 码点 helper、UTF-8 游标与编码行为、UTF-16 代理对处理、位置跟踪和错误报告保持测试覆盖。

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
- 除非需要底层 byte 或 UTF-16 code unit 控制，否则优先使用 Rust 标准文本 API。
- 将本 crate 聚焦在 Unicode scalar value、UTF-8 byte、UTF-16 code unit 和 ASCII-only helper。
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

仓库地址：[https://github.com/qubit-ltd/rs-unicode](https://github.com/qubit-ltd/rs-unicode)
