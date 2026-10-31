# Qubit Text Codec 用户指南

[English](user_guide.md) · [README](../README.zh_CN.md) · [API 文档](https://docs.rs/qubit-codec-text)

本文适用于 `qubit-codec-text` 0.9，面向构建缓冲区级解析器、协议处理器和 I/O
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

## 实战场景：把输入 UTF-8 转为 UTF-16

一个 adapter 接收 UTF-8 字节并写入 UTF-16 码元输出缓冲区。完整消息可使用下面的
检查型便捷方法；流式场景中，应对每个缓冲区调用 `transcode`，保留未消费的不完整尾部，
处理输出背压，并且只在 EOF 后调用 `finish`。

## 安装与最小配置

```toml
[dependencies]
qubit-codec-text = "0.9"
qubit-codec = "0.17"
```

只有在需要序列化 `Charset` 时才启用可选 `serde` feature。

## 核心流程

```rust
use qubit_codec::Transcoder;
use qubit_codec_text::{CharsetConverter, Utf16U16Codec, Utf8Codec};

let input = "A中".as_bytes();
let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
let capacity = converter.max_total_output_len(input.len()).expect("容量上界");
assert_eq!(12, capacity);
let mut output = vec![0_u16; capacity];
let written = converter
    .transcode_complete_into(input, &mut output)
    .expect("完整 UTF-8 输入且输出空间足够");

assert_eq!(2, written);
assert_eq!([0x0041, 0x4e2d], output[..written]);
```

`max_total_output_len` 会把 codec 对 reset、转码和 finish 的最大输出容量相加。完整转换方法
在 reset 前检查这个保守上界，不会先试写消息。本例实际只写入两个码元，但完整调用要求的
上界为十二个。容量计算可能溢出，因此要处理返回的 `Result`。如果必须限制为固定小窗口，
请改用流式调用并处理 `NeedOutput`。

### 流式处理直到 EOF

开流阶段的 `read()` 和 `written()` 表示相对于本次传入输入、输出下标的增量。
`NeedInput` 表示不完整尾部尚未消费；保留该尾部，并在下一块输入到达后拼接重试。
遇到 `NeedOutput` 时，先排空已生成的输出，再按报告的进度推进输入和输出位置后重试。
converter 也可能在内部保留待写字符，因此不要重复提交已报告为已读的输入。

调用方确认输入结束后，将保留尾部传给 `transcode_eof`。若返回 `NeedOutput`，继续排空或
扩展输出缓冲区并再次调用 `transcode_eof`，直到 EOF 输入处理完毕。随后才能调用
`finish` 写出生命周期尾部。`finish` 不接收输入，不能处理调用方保存的不完整尾部。

下面的 UTF-8 转 UTF-16 完整示例使用三块输入和两个码元的输出窗口，并断言运行时确实
遇到 `NeedInput` 与 `NeedOutput`。示例所用的内置 codec 在 reset 和 finish 时都没有输出，
因此可以传空生命周期缓冲区；自定义 codec 可能需要输出空间。`transcode_complete_into`
会自行执行 reset 和 finish，不要在调用前重复 reset。

```rust
use qubit_codec::TranscodeStatus;
use qubit_codec_text::CharsetConverter;
use qubit_codec_text::Utf16U16Codec;
use qubit_codec_text::Utf8Codec;

fn drain(
    converter: &mut CharsetConverter<Utf8Codec, Utf16U16Codec>,
    pending: &mut Vec<u8>,
    eof: bool,
    result: &mut Vec<u16>,
    observed: &mut (bool, bool),
) {
    loop {
        let mut output = [0_u16; 2];
        let progress = if eof {
            converter.transcode_eof(pending, 0, &mut output, 0)
        } else {
            converter.transcode(pending, 0, &mut output, 0)
        }.expect("default replacement policy");
        result.extend_from_slice(&output[..progress.written()]);
        pending.drain(..progress.read());
        match progress.status() {
            TranscodeStatus::Complete => break,
            TranscodeStatus::NeedInput { .. } => {
                assert!(!eof);
                observed.0 = true;
                break;
            }
            TranscodeStatus::NeedOutput { .. } => {
                observed.1 = true;
                assert!(progress.read() > 0 || progress.written() > 0);
            }
        }
    }
}

fn main() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    // 这两个内置 codec 在 reset 和 finish 时都没有输出。
    assert_eq!(0, converter.reset(&mut [], 0).expect("reset"));
    let chunks: [&[u8]; 3] = [
        &[0x41, 0xe4],
        &[0xb8, 0xad, 0xf0, 0x9f, 0x98, 0x80],
        &[0xe2, 0x82],
    ];
    let mut pending = Vec::new();
    let mut result = Vec::new();
    let mut observed = (false, false);
    for chunk in chunks {
        pending.extend_from_slice(chunk);
        drain(&mut converter, &mut pending, false, &mut result, &mut observed);
    }
    drain(&mut converter, &mut pending, true, &mut result, &mut observed);
    assert!(pending.is_empty());
    assert_eq!(0, converter.finish(&mut [], 0).expect("finish"));
    assert!(observed.0, "the example must exercise NeedInput");
    assert!(observed.1, "the example must exercise NeedOutput");
    assert_eq!(vec![0x0041, 0x4e2d, 0xd83d, 0xde00, 0xfffd], result);
}
```

第一块会留下未消费的 `E4`，第二块补齐“中”。此时输出窗口只剩一个 UTF-16 码元，
而“😀”需要一对代理码元，因此必须经历 `NeedOutput` 并继续排空。第三块留下不完整尾部；
确认 EOF 后交给 `transcode_eof`，默认 `Replace` 策略会输出 U+FFFD。每次调用后，示例只
追加实际写出的输出，并从 `pending` 删除已读前缀。一般情况下若 `NeedOutput` 没有读写进度，
应按其 `required` 值扩充输出缓冲区再重试。

### 流状态与恢复

| 事件或结果 | 后续操作 |
| --- | --- |
| 新建 decoder、encoder 或 converter | 流式调用前先成功执行 `reset`。 |
| 开流阶段返回 `Complete` | 当前可用输入已消费；后续仍可继续接收数据块。 |
| 返回 `NeedInput` | 保留未读尾部，拼接下一块输入后再试。 |
| 返回 `NeedOutput` | 收下已生成的输出，按 `read()` 推进输入，再继续；不要重放已消费的数据。 |
| 确认 EOF | 把保留尾部传给 `transcode_eof`；持续处理 `NeedOutput`，直到 EOF 输入和待写输出都处理完。 |
| `finish` 成功 | 当前逻辑流已关闭；新流开始前先成功执行 `reset`。 |
| reset/finish 的下标或容量预检失败 | 修正下标或缓冲区后重试。 |
| reset/finish 已开始执行后失败 | 状态可能已经变化；复用前必须成功 reset。业务数据如何重放或恢复由应用负责。 |

生命周期错误有专门的 kind：`TranscodeBeforeReset`、`FinishBeforeReset`、
`TranscodeAfterFinish`、`FinishAfterFinish` 和 `LifecyclePoisoned`。它们表示调用顺序错误或
生命周期状态不确定，不代表文本格式错误。畸形输入、目标不可映射字符和容量错误仍按各自
的 decode/encode 规则处理。`transcode_complete_into` 会自行完成整个生命周期，无须事先
调用 `reset`。这些 wrapper kind 对应 core 的
[`TranscodeFailure::TranscodeBeforeReset`](https://docs.rs/qubit-codec/0.17.0/qubit_codec/enum.TranscodeFailure.html#variant.TranscodeBeforeReset)、
[`FinishBeforeReset`](https://docs.rs/qubit-codec/0.17.0/qubit_codec/enum.TranscodeFailure.html#variant.FinishBeforeReset)、
[`TranscodeAfterFinish`](https://docs.rs/qubit-codec/0.17.0/qubit_codec/enum.TranscodeFailure.html#variant.TranscodeAfterFinish)、
[`FinishAfterFinish`](https://docs.rs/qubit-codec/0.17.0/qubit_codec/enum.TranscodeFailure.html#variant.FinishAfterFinish) 和
[`LifecyclePoisoned`](https://docs.rs/qubit-codec/0.17.0/qubit_codec/enum.TranscodeFailure.html#variant.LifecyclePoisoned)。

下面示例展示默认 `Replace` 策略如何在 EOF 时将两个 UTF-8 前缀字节替换为 U+FFFD：

```rust
use qubit_codec::TranscodeStatus;
use qubit_codec_text::{CharsetDecoder, Utf8Codec};

let mut decoder = CharsetDecoder::new(Utf8Codec);
let mut output = ['\0'; 1];
assert_eq!(0, decoder.reset(&mut output, 0).expect("reset"));
let pending = [0xe4, 0xb8];
let open = decoder.transcode(&pending, 0, &mut output, 0).expect("open stream");
assert!(matches!(open.status(), TranscodeStatus::NeedInput { .. }));
assert_eq!((0, 0), (open.read(), open.written()));
let eof = decoder.transcode_eof(&pending, 0, &mut output, 0).expect("EOF replacement");
assert_eq!(TranscodeStatus::Complete, eof.status());
assert_eq!((2, 1), (eof.read(), eof.written()));
assert_eq!('\u{fffd}', output[0]);
assert_eq!(0, decoder.finish(&mut output, 1).expect("finish"));
```

使用 `Ignore` 时，EOF 会消费尾部但不输出；使用 `Report` 时会返回错误。`NeedOutput` 是
通过 `Ok` 返回的正常输出背压，不是错误。开流阶段的 `Complete` 只表示当前输入已处理，
并不表示上游已经到达 EOF。

输出为 `char` 时使用 `CharsetDecoder::new(Utf8Codec)`；输入为 `char` 时使用
`CharsetEncoder::new(Utf8Codec)`。当 decoder 输入以不完整序列结束时，
`transcode_complete_into` 会应用已配置的策略：`Replace` 写入替换字符，
`Ignore` 丢弃不完整尾部，`Report` 返回错误；输出缓冲区过小时则会返回错误。

## 进阶用法

### 策略

| 情形 | 默认行为 | 可选行为 |
| --- | --- | --- |
| 源码元畸形 | `MalformedAction::Replace`，写入 U+FFFD | `Ignore` 跳过畸形范围；`Report` 返回错误。 |
| 字符无法由目标 charset 表示 | `UnmappableAction::Replace` | `Ignore` 跳过；`Report` 返回错误。 |

替换不可接受时，应使用 `CharsetDecodePolicy` 或 `CharsetEncodePolicy` 显式设置。
`CharsetEncoder::with_policy` 与 `CharsetConverter::from_codecs_with_policies`
会校验当前状态下必须可由目标 codec 编码的替换字符。构造时验证成功不保证有状态 codec
在后续状态仍能编码该字符。实际需要替换时，wrapper 会再次检查所选字符；若它已不可编码，
转换返回编码侧领域错误并指出该字符，不会自动改用 `?`。

### 为 ASCII 选择替换策略

自动构造的 ASCII encoder 会选择 `?` 作为替换字符。`CharsetEncodePolicy::default()`
始终选择 U+FFFD，而 ASCII 无法编码它，因此显式传入该策略时，构造 encoder 会被拒绝。
如果希望替换字符适配目标 codec，应使用 `default_for(&codec)`；如果协议要求固定替换字符，
则使用 `replace('?')`。不允许替换不可映射字符时，使用 `report()`：

```rust
use qubit_codec_text::{
    AsciiCodec, CharsetEncodeErrorKind, CharsetEncodePolicy, CharsetEncoder,
};

let automatic = CharsetEncoder::new(AsciiCodec);
assert_eq!('?', automatic.replacement());

let error = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::default())
    .expect_err("ASCII cannot encode U+FFFD");
assert_eq!(
    CharsetEncodeErrorKind::UnmappableCharacter { value: 0xfffd },
    error.kind(),
);

let input = ['A', '中', 'B'];
let policy = CharsetEncodePolicy::default_for(&AsciiCodec).expect("ASCII fallback");
let mut encoder = CharsetEncoder::with_policy(AsciiCodec, policy).expect("valid policy");
let mut output = [0_u8; 3];
let written = encoder.transcode_complete_into(&input, &mut output).expect("replace");
assert_eq!(b"A?B", &output[..written]);

let mut explicit = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::replace('?'))
    .expect("ASCII replacement");
let written = explicit.transcode_complete_into(&input, &mut output).expect("replace");
assert_eq!(b"A?B", &output[..written]);

let mut strict = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::report())
    .expect("report does not encode replacement metadata");
let mut staged = [0_u8; 3];
let raw_error = strict.transcode_complete_into(&input, &mut staged)
    .expect_err("ASCII rejects 中");
let error = strict.map_transcode_error(raw_error);
assert_eq!(
    CharsetEncodeErrorKind::UnmappableCharacter { value: '中' as u32 },
    error.kind(),
);
assert_eq!(b'A', staged[0]);
```

对于 `A中B`，目标相关默认替换和显式协议替换都会得到 `A?B`。`Report` 失败时，暂存缓冲区中
可能已经写入 `A`。应用应丢弃该消息的暂存结果，不发布该前缀，也不根据映射后的错误重放原输入。
显式传入策略不会自动调用 `default_for`。策略可能无效时，应使用能够返回配置错误的构造方法；
例如 `rs-io-text` 中的 `CharsetTextWriter::try_new` 可以返回该错误。
对于有状态 codec，构造时验证成功不保证替换字符在后续状态仍可编码。实际需要替换时若配置的
字符已不可编码，转换返回编码侧错误；不会自动改用 `?`。失败前已写出的输出仍须按现有消息级
暂存与恢复规则处理。

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
为保持兼容性，converter 的生命周期错误归入 `CharsetConvertError::Decode`；这不表示源文本
畸形。目标输出错误仍归入 `Encode`。目标 codec 当前状态下无法编码所选替换字符，也属于目标
编码错误；错误指出该替换字符，converter 的下标按源 codec 码元计数。构造时验证不构成永久
可编码保证，也不会自动选择回退字符。

`CharsetEncoder` 的输入错误下标按输入 `char` 切片的元素计数；`CharsetConverter`
报告不可映射字符时，下标按产生该值的输入中的源编码码元计数。例如，使用 `Report` 将
UTF-8 的“é中”转为 Latin-1 时，错误位于字节下标 2；对 `['é', '中']` 直接编码时，
错误位于 `char` 下标 1。输出位置按目标码元计数。领域错误保留产生方的位置语义，
不能把映射后的下标直接用作安全续传位置。

流式代码通过 `TranscodeProgress` 得到已读/已写码元数和 `NeedInput`、输出背压等状态。
`NeedInput` 不是畸形错误：保留尾部并在后续输入到达后重试。EOF 后仍残留的不完整序列
会按照已配置的策略处理：替换、丢弃或返回错误。

`NeedInput` 和 `NeedOutput` 是携带进度的成功状态。`Err` 不构成事务边界：它不会回滚
已写输出或 codec 状态，也不提供完整的已读/已写进度。除非明确的恢复协议允许，否则不要
重放原调用的输入范围。reset 会开始新的逻辑流，但无法撤销已经交付给应用的输出。解码器
EOF 处理和完整便捷方法也可能在写出输出前缀后失败。

将转码错误映射为 `CharsetDecodeError`、`CharsetEncodeError` 或 `CharsetConvertError` 时，
会保留领域错误自身的数据，例如 charset、kind、自身下标或 consumed 数；映射会丢弃外层
phase、`input_index` 和 `input_consumed`。领域错误自带的下标或消费数可能与这些外层字段
不同。若需要外层上下文，请在映射前检查原始错误；不要根据映射后的错误推断续传位置。
为兼容性，converter 生命周期错误仍映射为 `CharsetConvertError::Decode`，这不表示源文本
畸形。

`CharsetEncoder` 的输入错误下标按输入 `char` 切片的元素计数；`CharsetConverter`
报告不可映射字符时，下标按传入源切片的编码码元计数。例如，使用 `Report` 将
UTF-8 的“é中”转为 Latin-1 时，错误位于字节下标 2；对 `['é', '中']` 直接编码时，
错误位于 char 下标 1。输出位置按目标码元计数。领域错误保留产生方的位置语义，
不能把映射后的下标直接用作安全续传位置。

如果业务要求整条消息要么全部发布、要么全部丢弃，应先转换到该消息专用的临时缓冲区，
仅在完整转换成功后发布。转换失败时丢弃该消息的暂存输出；不要仅凭映射后的诊断把输入
重放到正在使用的 sink。成功 reset 后，再把新消息作为新的逻辑流处理。例如，ASCII encoder
使用 `Report` 处理 `A中` 时，可能先将 `A` 写入调用方的暂存缓冲区，再对 `中` 返回错误；
即使调用失败，该前缀仍然可以观察到。

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
