# Qubit Text Codec 用户指南

本文说明 `qubit-codec-text` 提供的能力、核心类型之间的关系，以及如何在面向
缓冲区的文本编解码代码中使用本库。

简短概览见[中文 README](../README.zh_CN.md)。英文说明见
[English user guide](user_guide.md)。

## 用途

`qubit-codec-text` 是一个低层文本编解码核心。它面向解析器、二进制格式、
文本 I/O 适配器等场景，这些场景通常需要直接控制字节或码元缓冲区、保留精确
错误位置，并明确区分 malformed / unmappable 的处理策略。

适合使用本库的场景包括：

- ASCII 常量、分类、转换、比较和折叠。
- Unicode 标量值 / 码点检查、代理项检查、平面计算、非字符检查和控制字符检查。
- UTF-8、UTF-16、UTF-32 命名空间辅助函数，用于长度计算和 BOM 检测。
- ASCII、ISO-8859-1、UTF-8、UTF-16、UTF-32 的缓冲区级 codec。
- 带策略的 decoder、encoder 和 converter，支持 replace、ignore、report。
- `Charset`、`UnicodeBom`、`ByteOrder`、`Coder` 和强类型编解码错误，
  便于构建更高层适配器。

本库不是通用文本处理库。它刻意停留在字素簇切分、规范化、排序、区域相关大小写
映射、显示宽度、自动 charset 检测和 `std::io` 读写适配器之下。这些能力应使用
`unicode-segmentation`、`unicode-normalization`、`unicode-width`、ICU4X
或专门的文本 I/O crate。

## 安装

```toml
[dependencies]
qubit-codec-text = "0.1"
```

`qubit-codec` 是核心运行时依赖；公开 API 使用的核心缓冲区级 trait 已经由
`qubit-codec-text` 重导出。

需要紧凑导入时：

```rust
use qubit_codec_text::prelude::*;
```

需要显式导入时：

```rust
use qubit_codec_text::{
    CharsetCodec,
    CharsetDecoder,
    CharsetEncoder,
    Coder,
    Utf8Codec,
};
```

## 架构

本库拆成几个很小的层次。

| 层次 | 主要类型 | 作用 |
| --- | --- | --- |
| 命名空间辅助工具 | `Ascii`、`Unicode`、`Utf8`、`Utf16`、`Utf32` | 常量、分类、长度和 BOM 辅助函数。 |
| Charset 元数据 | `Charset`、`UnicodeBom`、`ByteOrder` | 稳定 charset 身份、别名、固定字节序和 BOM 元数据。 |
| 低层 codec | `Codec<char, Unit>`、内置 codec 结构体 | 从调用方缓冲区解码或编码一个完整 Unicode 标量值。 |
| 文本 codec wrapper | `CharsetCodec`、`DecodeStatus` | 增加 charset 元数据、边界检查和单个标量值的不完整输入报告。 |
| 策略包装器 | `CharsetDecoder`、`CharsetEncoder` | 在批量转换时应用 malformed / unmappable 策略。 |
| Charset 转换 | `CharsetConverter` | 先把源单元解码成 `char`，再编码成目标单元。 |
| 进度 API | `Coder`、`CoderProgress`、`CoderStatus` | 报告部分进度、输入不足和输出回压。 |
| 错误类型 | `CharsetDecodeError`、`CharsetEncodeError`、`CharsetConvertError` | 保留 charset、错误种类、绝对下标和可选原始值。 |

所有 codec 操作都面向缓冲区。调用方传入完整输入 slice、完整输出 slice 和绝对
起始下标。返回的 `read` / `written` 是相对这些起始下标的计数；错误下标和
`CoderStatus` 中的下标是传入缓冲区内的绝对下标。

## 命名空间辅助工具

命名空间枚举是无状态的，只组织常量和辅助函数，不持有缓冲区。

```rust
use qubit_codec_text::{
    Ascii,
    Unicode,
    Utf8,
    Utf16,
    Utf32,
};

assert!(Ascii::is_letter_byte(b'A'));
assert_eq!(b'a', Ascii::byte_to_lowercase(b'A'));
assert_eq!(Some(10), Ascii::char_to_digit('A', 16));

assert!(Unicode::is_scalar_value('中' as u32));
assert_eq!(Some(0), Unicode::plane('A' as u32));
assert_eq!(Some('A'), Unicode::to_char(0x41));

assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xe4));
assert_eq!(2, Utf16::unit_len('😀'));
assert!(Utf32::is_valid_unit('中' as u32));
```

`Ascii` 还提供完整的 printable / digit / letter 数组，以及稳定的 ASCII-only
折叠辅助函数。这些辅助函数不等价于完整 Unicode 大小写折叠。

## Charset 元数据

`Charset` 是轻量身份描述对象。相等性和哈希只使用稳定 `id`；展示名和别名用于
展示与标签匹配。

```rust
use qubit_codec_text::Charset;

assert_eq!("utf-8", Charset::UTF_8.id());
assert_eq!("UTF-8", Charset::UTF_8.name());
assert!(Charset::UTF_8.matches_label("utf8"));

const GBK: Charset = Charset::new("gbk", "GBK", &["cp936"]);
assert!(GBK.matches_label("CP936"));
```

内置描述对象：

| Charset | 含义 |
| --- | --- |
| `Charset::ASCII` | US-ASCII 字节。 |
| `Charset::ISO_8859_1` | ISO-8859-1 / Latin-1 字节。 |
| `Charset::UTF_8` | UTF-8 字节。 |
| `Charset::UTF_16` | 泛化 UTF-16 码元形式或 BOM-aware 标签。 |
| `Charset::UTF_16LE`、`Charset::UTF_16BE` | 固定字节序 UTF-16 字节流。 |
| `Charset::UTF_32` | 泛化 UTF-32 码元形式或 BOM-aware 标签。 |
| `Charset::UTF_32LE`、`Charset::UTF_32BE` | 固定字节序 UTF-32 字节流。 |

需要在字节序判断和 charset 标签之间转换时，可使用
`Charset::from_utf16_byte_order`、`Charset::from_utf32_byte_order` 和
`Charset::byte_order`。

## BOM 与字节序

`UnicodeBom` 从字节缓冲区开头检测受支持的 Unicode BOM。

```rust
use qubit_codec_text::{
    ByteOrder,
    Charset,
    UnicodeBom,
};

let bom = UnicodeBom::detect(&[0xff, 0xfe, 0x00, 0x00]);
assert_eq!(Some(UnicodeBom::Utf32LittleEndian), bom);

let bom = bom.expect("BOM should be present");
assert_eq!(Charset::UTF_32LE, bom.charset());
assert_eq!(Some(ByteOrder::LittleEndian), bom.byte_order());
assert_eq!(&[0xff, 0xfe, 0x00, 0x00], bom.bytes());
```

UTF-32 BOM 会先于 UTF-16 BOM 检查，因为 `FF FE 00 00` 以 UTF-16LE 前缀
`FF FE` 开头。流式调用方应先缓冲最多 4 个字节，或读到 EOF，再判断是否存在更长
BOM。

面向字节的 UTF-16 / UTF-32 codec 持有 `ByteOrder`，但不会自动检测、跳过或写出
BOM。BOM 处理由调用方负责。

## 低层 Codec

内置 text codec 结构体实现了领域无关的 `qubit_codec::Codec<char, Unit>`
trait。这个 trait 是最低层完整值契约：调用 `decode_unchecked` 和
`encode_unchecked` 前，调用方必须已经知道一个值所需的输入或输出单元足够。

`CharsetCodec` 位于这个 unsafe trait 之上，是文本专用的安全 wrapper。它增加
`charset()` 元数据、`max_units_per_char()`、带检查的 `decode_one()` 和
`encode_one()`。`decode_one()` 是能够为开放流返回 `DecodeStatus::NeedMore`
的层；底层 `Codec` trait 不承载 partial-input 状态。

| Codec | 存储单元 | Charset |
| --- | --- | --- |
| `AsciiCodec` | `u8` | `Charset::ASCII` |
| `Latin1Codec` | `u8` | `Charset::ISO_8859_1` |
| `Utf8Codec` | `u8` | `Charset::UTF_8` |
| `Utf16U16Codec` | `u16` | `Charset::UTF_16` |
| `Utf16ByteCodec` | `u8` | `Charset::UTF_16LE` 或 `Charset::UTF_16BE` |
| `Utf32U32Codec` | `u32` | `Charset::UTF_32` |
| `Utf32ByteCodec` | `u8` | `Charset::UTF_32LE` 或 `Charset::UTF_32BE` |

解码单个标量值：

```rust
use qubit_codec_text::{
    CharsetCodec,
    DecodeStatus,
    Utf8Codec,
};

let status = Utf8Codec
    .decode_one("中".as_bytes(), 0)
    .expect("valid UTF-8 input");
assert_eq!(
    DecodeStatus::Complete {
        value: '中',
        consumed: 3,
    },
    status,
);

let status = Utf8Codec
    .decode_one(&[0xe4], 0)
    .expect("valid incomplete UTF-8 prefix");
assert_eq!(
    DecodeStatus::NeedMore {
        required: 3,
        available: 1,
    },
    status,
);

```

编码单个标量值：

```rust
use qubit_codec_text::{
    CharsetCodec,
    Utf8Codec,
    Utf8,
};

let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
let written = Utf8Codec
    .encode_one('é', &mut output, 0)
    .expect("buffer is large enough");

assert_eq!("é".as_bytes(), &output[..written]);
```

低层 codec 是严格的。`CharsetCodec` 会把 malformed input、非法输入下标、
非法标量值、无法映射的字符、输出缓冲区不足都报告为强类型错误。策略决策交给后面的包装器。

## DecodeStatus 与不完整输入

`DecodeStatus` 只由安全的 `CharsetCodec::decode_one` wrapper 返回。它不是底层
`Codec<char, Unit>` trait 的一部分。

| 状态 | 含义 |
| --- | --- |
| `Complete { value, consumed }` | 已解码出一个 Unicode 标量值。`consumed` 必须大于 0。 |
| `NeedMore { required, available }` | 当前前缀目前合法，但还需要更多输入单元。 |

流仍打开时，`NeedMore` 不是错误。到达 EOF 后，调用方可以把它转换为不完整序列错误：

```rust
use qubit_codec_text::{
    Charset,
    DecodeStatus,
};

let status = DecodeStatus::NeedMore {
    required: 3,
    available: 1,
};
let error = status.incomplete_error(Charset::UTF_8, 0);

assert_eq!(Some(3), error.required());
assert_eq!(Some(1), error.available());
```

## 带策略的解码

`CharsetDecoder<C>` 把源单元转换成 `char`，并应用 `MalformedAction`。

| 策略 | 行为 |
| --- | --- |
| `MalformedAction::Replace` | 输出 decoder 的替换字符。这是默认策略。 |
| `MalformedAction::Ignore` | 跳过 malformed 范围，不输出字符。 |
| `MalformedAction::Report` | 返回 `CharsetDecodeError`。 |

```rust
use qubit_codec_text::{
    CharsetDecoder,
    Coder,
    CoderStatus,
    Utf8Codec,
};

let mut decoder = CharsetDecoder::new(Utf8Codec);
let mut output = ['\0'; 2];

let progress = decoder
    .convert("Aé".as_bytes(), 0, &mut output, 0)
    .expect("valid UTF-8 input");

assert_eq!(CoderStatus::Complete, progress.status());
assert_eq!(3, progress.read());
assert_eq!(2, progress.written());
assert_eq!(['A', 'é'], output);

```

严格校验时：

```rust
use qubit_codec_text::{
    CharsetDecoder,
    Coder,
    MalformedAction,
    Utf8Codec,
};

let mut decoder = CharsetDecoder::new(Utf8Codec);
decoder.set_malformed_action(MalformedAction::Report);

let mut output = ['\0'; 1];
let error = decoder.convert(&[0x80], 0, &mut output, 0).expect_err("invalid UTF-8");

assert_eq!(0, error.index());
```

如果自定义 `CharsetCodec` 违反 `DecodeStatus` 不变量，`CharsetDecoder::convert`
可能 panic。内置 codec 满足这些不变量。

## 带策略的编码

`CharsetEncoder<C>` 把 `char` 转换成目标单元，并应用 `UnmappableAction`。

| 策略 | 行为 |
| --- | --- |
| `UnmappableAction::Replace` | 编码配置好的替换字符。这是默认策略。 |
| `UnmappableAction::Ignore` | 跳过输入字符，不输出单元。 |
| `UnmappableAction::Report` | 返回 `CharsetEncodeError`。 |

```rust
use qubit_codec_text::{
    CharsetEncoder,
    Coder,
    CoderStatus,
    Utf8Codec,
    Utf8,
};

let mut encoder = CharsetEncoder::new(Utf8Codec);
let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

let progress = encoder
    .convert(&['😀'], 0, &mut output, 0)
    .expect("UTF-8 output buffer is large enough");

assert_eq!(CoderStatus::Complete, progress.status());
assert_eq!(1, progress.read());
assert_eq!(4, progress.written());
assert_eq!("😀".as_bytes(), &output[..progress.written()]);

```

ASCII 输出的严格 unmappable 处理：

```rust
use qubit_codec_text::{
    AsciiCodec,
    CharsetEncoder,
    Coder,
    UnmappableAction,
};

let mut encoder = CharsetEncoder::new(AsciiCodec);
encoder.set_unmappable_action(UnmappableAction::Report);

let mut output = [0_u8; 1];
let error = encoder.convert(&['é'], 0, &mut output, 0).expect_err("not ASCII");

assert_eq!(0, error.index());
assert_eq!(Some('é' as u32), error.value());
```

`CharsetEncoder::new` 会缓存替换字符。它先尝试 `U+FFFD`，再回退到 `?`。
只有当传入的 codec 连这两个替换字符都无法编码时才会 panic。内置 codec 不会触发
这个分支；对自定义 codec 来说，这个 panic 表示 codec 不变量被破坏，而不是可恢复
文本输入错误。

需要自定义替换字符时，可使用 `with_replacement` 或 `set_replacement` 提前验证：

```rust
use qubit_codec_text::{
    AsciiCodec,
    CharsetEncoder,
};

let encoder = CharsetEncoder::new(AsciiCodec)
    .with_replacement('?')
    .expect("ASCII replacement is encodable");
assert_eq!('?', encoder.replacement());
```

## Charset 转换

`CharsetConverter<D, E>` 组合一个源 decoder 和一个目标 encoder，中间表示是
`char`。

```rust
use qubit_codec_text::{
    CharsetConverter,
    Coder,
    CoderStatus,
    Utf8Codec,
    Utf16U16Codec,
};

let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
let mut output = [0_u16; 2];

let progress = converter
    .convert("A中".as_bytes(), 0, &mut output, 0)
    .expect("valid UTF-8 input and enough UTF-16 output");

assert_eq!(CoderStatus::Complete, progress.status());
assert_eq!(4, progress.read());
assert_eq!(2, progress.written());
assert_eq!(['A' as u16, '中' as u16], output);

```

如果目标输出缓冲区已满，converter 最多保留一个已解码但尚未写出的 pending 字符。
之后可以用更大的输出缓冲区再次调用 `convert`，或在源输入结束后调用 `finish` 刷出
pending 输出。

`CharsetConvertError` 会区分源端解码失败和目标端编码失败：

```rust
use qubit_codec_text::{
    CharsetConvertError,
    CharsetConverter,
    Coder,
    Utf8Codec,
    Utf16U16Codec,
};

let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
let mut output = [0_u16; 1];

let error = converter
    .convert(b"A", 2, &mut output, 0)
    .expect_err("source index is outside input");

assert!(matches!(error, CharsetConvertError::Decode(_)));
```

## 进度与缓冲

`Coder<Input, Output>` 从 `qubit-codec` 重导出。它有三个核心方法：

| 方法 | 含义 |
| --- | --- |
| `max_output_len(input_len)` | 返回已知的输出上界。 |
| `convert(input, input_index, output, output_index)` | 从调用方缓冲区尽可能转换数据。 |
| `finish(output, output_index)` | 所有源输入结束后刷新 buffered output。 |

`CoderProgress` 包含：

- `status()`：`Complete`、`NeedInput` 或 `NeedOutput`。
- `read()`：相对 `input_index` 消耗的输入单元数。
- `written()`：相对 `output_index` 写出的输出单元数。
- `required()`、`available()`、`index()`：输入不足或输出回压时的辅助访问器。

`CoderStatus` 使用绝对下标：

| 状态 | 含义 |
| --- | --- |
| `Complete` | 当前调用完成，不需要更多输入或输出空间。 |
| `NeedInput { input_index, required, available }` | 在 `input_index` 处需要更多源单元。 |
| `NeedOutput { output_index, required, available }` | 在 `output_index` 处需要更多目标单元空间。 |

输出太小时，策略包装器会返回 `NeedOutput`，这是正常回压，不是错误。输入是合法前缀但
暂时不足时，decoder 返回 `NeedInput`。malformed 输入、非法下标，以及 report 模式下
的 unmappable 字符会返回错误。

## 错误模型

解码错误携带源 charset、错误种类、输入单元下标和可选原始值。

| 解码错误种类 | 含义 |
| --- | --- |
| `MalformedSequence` | 缓冲区中存在单元，但对当前 charset 非法。 |
| `InvalidInputIndex` | 调用方传入了大于输入长度的输入下标。 |
| `IncompleteSequence` | 关闭输入在完整标量值出现前结束。 |
| `InvalidCodePoint` | 解码出的数值不是 Unicode 标量值。 |

编码错误携带目标 charset、错误种类、下标和可选原始值。

| 编码错误种类 | 含义 |
| --- | --- |
| `InvalidCodePoint` | codec 被要求编码非标量码点。 |
| `InvalidInputIndex` | 调用方传入了大于输入长度的字符下标。 |
| `UnmappableCharacter` | 字符无法用目标 charset 表示。 |
| `BufferTooSmall` | 输出缓冲区无法容纳编码结果。 |

常用访问器包括 `charset()`、`kind()`、`index()`、`required()`、
`available()`、`input_len()` 和 `value()`。

## UTF-16 与 UTF-32 字节 Codec

当缓冲区已经是码元数组时，使用 `Utf16U16Codec` 和 `Utf32U32Codec`。当数据是
序列化字节时，使用 `Utf16ByteCodec` 和 `Utf32ByteCodec`。

```rust
use qubit_codec_text::{
    ByteOrder,
    CharsetCodec,
    Utf16ByteCodec,
};

let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
let mut output = [0_u8; 4];

let written = codec
    .encode_one('😀', &mut output, 0)
    .expect("UTF-16 output buffer is large enough");

assert_eq!(&[0x3d, 0xd8, 0x00, 0xde], &output[..written]);
```

字节 codec 会直接读写固定字节序的 byte sequence。公开调用方通常通过
`CharsetCodec`、`CharsetEncoder` 或 `CharsetConverter` 使用它们。

## 扩展新的 Charset

在下游 crate 中新增 charset 时：

1. 定义 codec 类型。
2. 实现 `qubit_codec::Codec<char, Unit>`，负责完整值的 decode / encode。
3. 实现 `CharsetCodec`。
4. 从 `charset()` 返回稳定的 `Charset` 描述对象。
5. 从 `max_units_per_char()` 返回单个标量值最多需要的存储单元数。
6. 在 `decode_one` 和 `encode_one` 中先校验下标与容量，再委托 unsafe
   `Codec` 方法。
7. 使用 `CharsetDecoder`、`CharsetEncoder` 或 `CharsetConverter` 应用策略。

重要的 `decode_one` 不变量：

- `Complete` 必须消耗至少一个单元。
- `Complete.consumed` 不能超过 `input.len() - index`。
- `NeedMore.required` 是绝对输入长度，并且必须大于 `input.len()`。
- `NeedMore.available` 必须等于 `input.len() - index`。
- `index > input.len()` 应返回
  `CharsetDecodeErrorKind::InvalidInputIndex`。

重要的 `encode_one` 约定：

- 输出容量不足时返回 `BufferTooSmall`。
- charset 无法表示某个标量值时返回 `UnmappableCharacter`。
- 如果希望 codec 能和 `CharsetEncoder::new` 一起使用，应保证替换字符 `?`
  可以编码。

## 开发命令

```bash
# 运行测试
cargo test

# 按 CI 口径对齐格式和 clippy
./align-ci.sh

# 运行完整本地 CI
RS_CI_SKIP_TOOLCHAIN_UPDATE=1 ./ci-check.sh

# 生成覆盖率
./coverage.sh text
```

完整 CI 会检查格式、clippy、style、debug/release 构建、测试、doctest、文档、
README 依赖版本、覆盖率和安全审计。
