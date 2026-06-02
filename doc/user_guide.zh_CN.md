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
- `Charset`、`UnicodeBom`、`ByteOrder`、`Codec`、`Transcoder`、强类型
  编解码错误，以及带策略的 wrapper，便于构建更高层适配器。自定义
  buffered adapter 需要直接从 `qubit-codec` 引入 core engine 和 hook。

本库不是通用文本处理库。它刻意停留在字素簇切分、规范化、排序、区域相关大小写
映射、显示宽度、自动 charset 检测和 `std::io` 读写适配器之下。这些能力应使用
`unicode-segmentation`、`unicode-normalization`、`unicode-width`、ICU4X
或专门的文本 I/O crate。

## 安装

```toml
[dependencies]
qubit-codec-text = "0.1"
```

`qubit-codec` 是核心运行时依赖。`qubit-codec-text` 只重导出普通 text-codec
调用需要的一小组 core trait 和状态类型；自定义 adapter 应直接从 `qubit-codec`
引入 engine、hook 和通用 adapter。

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
    Transcoder,
    Utf8Codec,
};
```

## 架构

本库拆成几个很小的层次。

| 层次 | 主要类型 | 作用 |
| --- | --- | --- |
| 命名空间辅助工具 | `Ascii`、`Unicode`、`Utf8`、`Utf16`、`Utf32` | 常量、分类、长度和 BOM 辅助函数。 |
| Charset 元数据 | `Charset`、`UnicodeBom`、`ByteOrder` | 稳定 charset 身份、别名、固定字节序和 BOM 元数据。 |
| 低层 codec | `Codec<Value = char>`、内置 codec 结构体 | 从调用方缓冲区解码或编码一个完整 Unicode 标量值。 |
| 文本 codec 元数据 | `CharsetCodec`、`CharsetEncodeProbe` | 为低层 codec 实现附加 charset 元数据和精确编码长度探测。 |
| 策略包装器 | `CharsetDecoder`、`CharsetEncoder` | 在批量转换时应用 malformed / unmappable 策略；分别实现 `BufferedDecoder` / `BufferedEncoder`。`CharsetDecoder` 复用 core 的 `BufferedDecodeEngine` 循环，`CharsetEncoder` 复用 core 的 `BufferedEncodeEngine` 循环。 |
| Charset 转换 | `CharsetConverter` | 先把源单元解码成 `char`，再编码成目标单元；实现 `BufferedConverter`。 |
| 进度 API | `Transcoder`、`TranscodeProgress`、`TranscodeStatus` | 报告部分进度、输入不足和输出回压。 |
| 错误类型 | `CharsetDecodeError`、`CharsetEncodeError`、`CharsetConvertError` | 保留 charset、错误种类、绝对下标和可选原始值。 |

所有 codec 操作都面向缓冲区。调用方传入完整输入 slice、完整输出 slice 和绝对
起始下标。返回的 `read` / `written` 是相对这些起始下标的计数；错误下标和
`TranscodeStatus` 中的下标是传入缓冲区内的绝对下标。

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

内置 text codec 结构体实现了领域无关的 `qubit_codec::Codec` trait，其中
`Value = char`。这个 trait 是最低层单值契约：`decode_unchecked` 从调用方输入单元中解码
一个 Unicode 标量值，`encode_unchecked` 把一个 Unicode 标量值写入调用方输出单元。

`CharsetCodec` 与这个 unsafe trait 处于同一低层，只增加 `charset()` 元数据和
存储 `Unit` 类型。`CharsetEncodeProbe` 增加 `encode_len()`，encoder 用它提前
校验可映射性，并在调用 unsafe `encode_unchecked` 前计算精确输出单元数。

通过 `decode_unchecked` 解码时，调用方必须在调用前从当前输入下标开始提供
至少 `codec.min_units_per_value()` 个可读单元。调用方通常应尽量提供到
`codec.max_units_per_value().get()`，除非 EOF 已经无法继续读取。内置 codec 会解码完整的
较短表示，例如单字节 UTF-8 ASCII；对不完整或畸形前缀返回
`CharsetDecodeErrorKind::IncompleteSequence` / `MalformedSequence`。
`CharsetDecoder` 根据这些错误为开放的 buffered input 返回 `NeedInput`。

| Codec | 存储单元 | Charset |
| --- | --- | --- |
| `AsciiCodec` | `u8` | `Charset::ASCII` |
| `Latin1Codec` | `u8` | `Charset::ISO_8859_1` |
| `Utf8Codec` | `u8` | `Charset::UTF_8` |
| `Utf16U16Codec` | `u16` | `Charset::UTF_16` |
| `Utf16ByteCodec` | `u8` | `Charset::UTF_16LE` 或 `Charset::UTF_16BE` |
| `Utf32U32Codec` | `u32` | `Charset::UTF_32` |
| `Utf32ByteCodec` | `u8` | `Charset::UTF_32LE` 或 `Charset::UTF_32BE` |

从闭合或已经充分缓冲的输入 slice 解码单个标量值：

```rust
use qubit_codec_text::{
    Codec,
    Utf8Codec,
};

let decoded = unsafe {
    Utf8Codec
        .decode_unchecked("中".as_bytes(), 0)
}
    .expect("valid UTF-8 input");
assert_eq!(('中', 3), decoded);
```

解码闭合输入中的不完整尾部：

```rust
use qubit_codec_text::{
    CharsetDecodeErrorKind,
    Codec,
    Utf8Codec,
};

let error = unsafe {
    Utf8Codec
        .decode_unchecked(&[0xe4], 0)
}
    .expect_err("closed input ended inside a UTF-8 scalar value");

assert_eq!(
    CharsetDecodeErrorKind::IncompleteSequence {
        required: 3,
        available: 1,
    },
    error.kind(),
);
```

编码单个标量值：

```rust
use qubit_codec_text::{
    CharsetEncodeProbe,
    Codec,
    Utf8Codec,
    Utf8,
};

let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
let required = Utf8Codec
    .encode_len('é', 0)
    .expect("UTF-8 can encode every scalar value");
let written = unsafe {
    Utf8Codec
        .encode_unchecked(&'é', &mut output, 0)
}
    .expect("buffer is large enough");

assert_eq!(2, required);
assert_eq!("é".as_bytes(), &output[..written]);
```

低层 codec 是严格的。它们会把 malformed input、非法输入下标、非法标量值、无法映射
的字符、输出缓冲区不足都报告为强类型错误。策略决策交给后面的包装器。

## EOF 与不完整输入

低层 codec 层只有闭合输入：短缓冲区表示 EOF，而不是“未来可能还有更多数据”。
流式状态的区分属于 `CharsetDecoder`。

`CharsetDecoder::transcode` 会询问 codec 当前可用单元是否已经构成一个完整 scalar。
完整的较短表示会立即解码。如果当前 chunk 只是合法但不完整的前缀，它返回
`TranscodeStatus::NeedInput`，且不消耗这个尾部。尾部保存、后续填充和 EOF 策略都由
调用方负责。调用方处理完不完整尾部后，再调用 `finish()` 刷新内部暂存输出。

内部实现上，`CharsetDecoder` 把 malformed-input 策略保存在 decode hooks 中，并转发给
`BufferedDecodeEngine<C, H>`。engine 负责重复调用 `decode_unchecked`、
输出容量 progress 和状态报告；输入缓冲区填充由调用方负责。

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
    Transcoder,
    TranscodeStatus,
    Utf8Codec,
};

let mut decoder = CharsetDecoder::new(Utf8Codec);
let mut output = ['\0'; 2];

let progress = decoder
    .transcode("Aé".as_bytes(), 0, &mut output, 0)
    .expect("valid UTF-8 input");
assert_eq!(TranscodeStatus::Complete, progress.status());
assert_eq!(3, progress.read());
assert_eq!(2, progress.written());
assert_eq!(['A', 'é'], output);

```

严格校验时：

```rust
use qubit_codec_text::{
    CharsetDecoder,
    CharsetDecodePolicy,
    Transcoder,
    Utf8Codec,
};

let mut decoder = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());

let mut output = ['\0'; 1];
let error = decoder
    .transcode(&[0x80], 0, &mut output, 0)
    .expect_err("invalid UTF-8");

assert_eq!(0, error.index());
```

如果自定义 `CharsetCodec` 违反低层 `Codec::decode_unchecked` 契约，例如报告的
消耗单元数超过已提供输入的单元，`CharsetDecoder::transcode` 可能 panic。内置
codec 满足这个契约。

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
    Transcoder,
    TranscodeStatus,
    Utf8Codec,
    Utf8,
};

let mut encoder = CharsetEncoder::new(Utf8Codec);
let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

let progress = encoder
    .transcode(&['😀'], 0, &mut output, 0)
    .expect("UTF-8 output buffer is large enough");

assert_eq!(TranscodeStatus::Complete, progress.status());
assert_eq!(1, progress.read());
assert_eq!(4, progress.written());
assert_eq!("😀".as_bytes(), &output[..progress.written()]);

```

ASCII 输出的严格 unmappable 处理：

```rust
use qubit_codec_text::{
    AsciiCodec,
    CharsetEncodePolicy,
    CharsetEncoder,
    Transcoder,
};

let mut encoder = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::report())
    .expect("report policy is constructible");

let mut output = [0_u8; 1];
let error = encoder.transcode(&['é'], 0, &mut output, 0).expect_err("not ASCII");

assert_eq!(0, error.index());
assert_eq!(Some('é' as u32), error.value());
```

`CharsetEncoder::new` 会缓存替换字符。它先尝试 `U+FFFD`，再回退到 `?`。
只有当传入的 codec 连这两个替换字符都无法编码时才会 panic。内置 codec 不会触发
这个分支；对自定义 codec 来说，这个 panic 表示 codec 不变量被破坏，而不是可恢复
文本输入错误。

内部实现上，`CharsetEncoder` 把 unmappable-input 策略保存在 encode hooks 中，并转发给
`BufferedEncodeEngine<C, H>`。engine 负责输入迭代、输出容量检查和
`TranscodeProgress` 构造；hooks 提供 original、replacement 或 ignored 字符对应的
charset-specific 计划。

需要自定义替换字符时，可使用 `with_policy` 提前验证：

```rust
use qubit_codec_text::{
    AsciiCodec,
    CharsetEncodePolicy,
    CharsetEncoder,
};

let encoder = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::replace('?'))
    .expect("ASCII replacement is encodable");
assert_eq!('?', encoder.replacement());
```

## Charset 转换

`CharsetConverter<D, E>` 组合一个源 decoder 和一个目标 encoder，中间表示是
`char`。

```rust
use qubit_codec_text::{
    CharsetConverter,
    Transcoder,
    TranscodeStatus,
    Utf8Codec,
    Utf16U16Codec,
};

let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
let mut output = [0_u16; 2];

let progress = converter
    .transcode("A中".as_bytes(), 0, &mut output, 0)
    .expect("valid UTF-8 input and enough UTF-16 output");
assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
assert_eq!(4, progress.read());
assert_eq!(1, progress.written());

let progress = converter
    .finish(&mut output, progress.written())
    .expect("closed tail converts successfully");
assert_eq!(TranscodeStatus::Complete, progress.status());
assert_eq!(['A' as u16, '中' as u16], output);

```

如果目标输出缓冲区已满，converter 最多保留一个已解码但尚未写出的 pending 字符。
之后可以用更大的输出缓冲区再次调用 `transcode`，或在调用方处理完不完整源尾部后调用
`finish` 刷出 pending 输出。

`CharsetConvertError` 会区分源端解码失败和目标端编码失败：

```rust
use qubit_codec_text::{
    CharsetConvertError,
    CharsetConverter,
    Transcoder,
    Utf8Codec,
    Utf16U16Codec,
};

let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
let mut output = [0_u16; 1];

let error = converter
    .transcode(b"A", 2, &mut output, 0)
    .expect_err("source index is outside input");

assert!(matches!(error, CharsetConvertError::Decode(_)));
```

## 进度与缓冲

`Transcoder<Input, Output>` 从 `qubit-codec` 重导出。它表示把一个逻辑输入流转换为一个逻辑输出流。
对每段可用输入调用 `transcode()`，到达 EOF 后调用 `finish()`，并在它返回 `NeedOutput`
时继续提供输出空间。复用同一个实例处理下一个逻辑流前，应先调用 `reset()`。它有四个核心方法：

| 方法 | 含义 |
| --- | --- |
| `max_output_len(input_len)` | 返回已知的输出上界。 |
| `max_finish_output_len()` | 返回内部状态收尾阶段可能产生的输出上界。 |
| `reset()` | 保留配置并清空转换状态。 |
| `transcode(input, input_index, output, output_index)` | 从调用方缓冲区尽可能转换数据。 |
| `finish(output, output_index)` | 调用方处理完尾部不完整输入后，刷新内部暂存输出。 |

`TranscodeProgress` 包含：

- `status()`：`Complete`、`NeedInput` 或 `NeedOutput`。
- `read()`：相对 `input_index` 消耗的输入单元数。
- `written()`：相对 `output_index` 写出的输出单元数。
- `additional()`、`available()`、`index()`：输入不足或输出回压时的辅助访问器。

`TranscodeStatus` 使用绝对下标：

| 状态 | 含义 |
| --- | --- |
| `Complete` | 当前调用完成，不需要更多输入或输出空间。 |
| `NeedInput { input_index, additional, available }` | 在 `input_index` 处需要更多源单元。 |
| `NeedOutput { output_index, additional, available }` | 在 `output_index` 处需要更多目标单元空间。 |

输出太小时，策略包装器会返回 `NeedOutput`，这是正常回压，不是错误。输入是合法前缀但
暂时不足时，decoder 返回 `NeedInput`，并把尾部留给调用方。如果调用方已经到达 EOF，
应先处理这个尾部，再调用 `finish()`。malformed 输入、非法下标，以及 report 模式下的
unmappable 字符会返回错误。

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
    CharsetEncodeProbe,
    Codec,
    Utf16ByteCodec,
};

let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
let mut output = [0_u8; 4];

let required = codec
    .encode_len('😀', 0)
    .expect("UTF-16 can encode every scalar value");
let written = unsafe {
    codec
        .encode_unchecked(&'😀', &mut output, 0)
}
    .expect("UTF-16 output buffer is large enough");

assert_eq!(4, required);
assert_eq!(&[0x3d, 0xd8, 0x00, 0xde], &output[..written]);
```

字节 codec 会直接读写固定字节序的 byte sequence。公开调用方通常通过
`CharsetCodec`、`CharsetEncoder` 或 `CharsetConverter` 使用它们。

## 扩展新的 Charset

在下游 crate 中新增 charset 时：

1. 定义 codec 类型。
2. 实现 `qubit_codec::Codec`，其中 `Value = char`，负责完整值的 decode / encode。
3. 实现 `CharsetCodec`，提供 charset 元数据。
4. 从 `charset()` 返回稳定的 `Charset` 描述对象。
5. 在 `Codec::max_units_per_value()` 实现中返回单个标量值最多需要的非零存储单元数。
6. 在 `Codec::decode_unchecked()` 中通过 `CharsetDecodeError` 返回不完整、畸形和
   invalid-scalar failure。
7. 如果该 charset 需要与 `CharsetEncoder` 或 converter 目标端一起使用，实现
   `CharsetEncodeProbe`。
8. 使用 `CharsetDecoder`、`CharsetEncoder` 或 `CharsetConverter` 应用策略。

重要的 `decode_unchecked` 约定：

- 成功时返回 `NonZeroUsize` 类型的已消耗单元数。
- 成功消耗的单元数不能超过 `input.len() - index`。
- 使用 `decode_unchecked` 的调用方至少提供 `min_units_per_value()` 个可读单元，并应
  尽量提供到 `max_units_per_value().get()`，除非 EOF 阻止继续读取。
- 如果当前提供的单元是合法但不完整的前缀，返回 `IncompleteSequence`；一旦这些单元
  足以证明序列非法，返回 `MalformedSequence` 或 `InvalidCodePoint`。
- `index > input.len()` 对 unsafe 方法来说是调用方违反契约。

重要的 `encode_unchecked` 与 `encode_len` 约定：

- 输出容量不足时返回 `BufferTooSmall`。
- charset 无法表示某个标量值时返回 `UnmappableCharacter`。
- `encode_len` 必须返回同一个字符随后由 `encode_unchecked` 写出的精确单元数。
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
