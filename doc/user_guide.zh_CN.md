# Qubit Text Codec 用户指南

[English](user_guide.md) · [README](../README.zh_CN.md) · [API 文档](https://docs.rs/qubit-codec-text)

本文适用于 `qubit-codec-text` 0.3，面向构建缓冲区级解析器、协议处理器和 I/O
adapter 的开发者：既要保留转换进度，也要明确选择畸形或不可映射数据的处理方式。

## 概念模型

本库将 charset 元数据、单标量 codec 和带策略的缓冲包装器分开：

```text
输入码元 -> CharsetDecoder -> char 值 -> CharsetEncoder -> 输出码元
                    \                         /
                     \-> CharsetConverter ---/
```

`Charset` 标识支持的字符集及其标签。`CharsetCodec` 实现负责解码或编码单个
`char`；`CharsetDecoder`、`CharsetEncoder` 和 `CharsetConverter` 负责策略与进度。
共享进度/状态类型从 `qubit-codec` 导入。

## 贯穿场景：把输入 UTF-8 转为 UTF-16

一个 adapter 接收 UTF-8 字节并写入 UTF-16 码元输出缓冲区。完整消息可使用下面的
检查型便捷方法；流式场景中，应对每个缓冲区调用 `transcode`，保留未消费的不完整尾部，
处理输出背压，并且只在 EOF 后调用 `finish`。

## 安装与最小配置

```toml
[dependencies]
qubit-codec-text = "0.3"
qubit-codec = "0.10"
```

只有在需要序列化 `Charset` 时才启用可选 `serde` feature。

## 核心流程

```rust
use qubit_codec_text::{CharsetConverter, Utf16U16Codec, Utf8Codec};

let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
let mut output = [0_u16; 2];
let written = converter
    .transcode_complete_into("A中".as_bytes(), &mut output)
    .expect("完整 UTF-8 输入且输出空间足够");

assert_eq!(2, written);
assert_eq!([0x0041, 0x4e2d], output);
```

输出为 `char` 时使用 `CharsetDecoder::new(Utf8Codec)`；输入为 `char` 时使用
`CharsetEncoder::new(Utf8Codec)`。`transcode_complete_into` 会拒绝以不完整序列
结尾的完整输入，也会拒绝过小的输出缓冲区。

## 进阶用法

### 策略

| 情形 | 默认行为 | 可选行为 |
| --- | --- | --- |
| 源码元畸形 | `MalformedAction::Replace`，写入 U+FFFD | `Ignore` 跳过畸形范围；`Report` 返回错误。 |
| 字符无法由目标 charset 表示 | `UnmappableAction::Replace` | `Ignore` 跳过；`Report` 返回错误。 |

替换不可接受时，应使用 `CharsetDecodePolicy` 或 `CharsetEncodePolicy` 显式设置。
`CharsetEncoder::with_policy` 与 `CharsetConverter::from_codecs_with_policies`
会校验必须可由目标 codec 编码的替换字符。

### BOM 与字节序

`UnicodeBom::detect` 在闭合输入中识别 UTF-8、UTF-16 和 UTF-32 BOM。流前缀可能
有歧义：`FF FE` 也可能继续成为 UTF-32LE BOM。应使用
`UnicodeBom::detect_progress(bytes, eof)` 或
`CharsetDecoder::<C>::detect_and_strip_bom_progress`，直到得到足够字节或确认 EOF。
面向字节的 UTF-16/32 codec 要求显式 `ByteOrder`，且绝不会自动消费或生成 BOM。

### Charset 标签

`Charset::from_label` 以宽松 ASCII 归一化匹配内置和已注册描述对象：会裁剪、折叠
大小写，并忽略 `-`、`_`。`from_whatwg_label` 采用不同的 WHATWG 风格预处理，但
不是完整 WHATWG Encoding Standard 标签表。自定义描述对象使用 `Charset::register`
或 `Charset::register_new` 注册；`new_static` 不会注册。

## 错误与诊断

底层 codec 报告 `CharsetDecodeError` 或 `CharsetEncodeError`，其中包含 charset、错误
种类和下标。错误种类会在适用时区分不完整序列、畸形输入、无效标量、容量和不可映射
字符。`CharsetConverter::map_transcode_error` 可将底层 converter failure
映射为 `CharsetConvertError`，说明失败发生在源端解码还是目标端编码。

流式代码通过 `TranscodeProgress` 得到已读/已写码元数和 `NeedInput`、输出背压等状态。
`NeedInput` 不是畸形错误：保留尾部并在后续输入到达后重试。EOF 后仍残留的不完整序列
才是完整输入错误。

## 排障

| 现象 | 检查方式 |
| --- | --- |
| UTF-8 尾部要求更多输入 | 保留未消费尾部；EOF 之前不要调用 `finish`。 |
| UTF-16/32 字节解码错误 | 确认 `ByteOrder`，并显式检测或剥离 BOM。 |
| 字符变成替换字符 | 检查 malformed/unmappable 策略，以及目标 charset 是否能表示该字符。 |
| 输出不完整 | 查看 progress；增加或排空输出空间后，从报告的偏移继续。 |

## 限制与最佳实践

本库不是完整文本处理库，也不是 `std::io` 库。它不提供字素簇切分、规范化、排序、
显示宽度、区域感知大小写、自动 charset 检测、stream 所有权或行尾策略。使用调用方
持有的固定缓冲区时必须检查容量和状态；只有满足边界与值域契约后，才能调用 unsafe
单值 `Codec` 方法。

## 延伸阅读

- [README](../README.zh_CN.md)
- [English user guide](user_guide.md)
- [API 文档](https://docs.rs/qubit-codec-text)
