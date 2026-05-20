# Qubit Unicode 用户指南

Qubit Unicode 面向 parser、codec 和兼容层，用于显式控制 Unicode scalar
value、UTF-8 byte、UTF-16 code unit 和 ASCII-only 行为。

## API 选择

普通场景先使用 Rust 标准库：

- 用 `std::str::from_utf8` 校验完整 UTF-8 字节切片；
- 用 `String::from_utf16` 和 `char::decode_utf16` 做常规 UTF-16 解码；
- 用 `char::encode_utf8` 和 `char::encode_utf16` 编码单个 scalar value。

当你需要游标式局部解码、精确区分 malformed / incomplete、写入调用方提供的
缓冲区，或兼容 Java 的 ASCII folding / Unicode escape 行为时，再使用本 crate。

## 错误处理

游标和编码 API 返回 `UnicodeResult<T>`。错误包含 `UnicodeErrorKind` 和检测到
问题的索引。错误分类包括：

- `BufferOverflow`；
- `Malformed`；
- `Incomplete`。

## ASCII Folding

`Ascii::fold` 移植自 Java `Ascii.fold` 映射表，最多写出
`Ascii::MAX_FOLDING` 个输出字符。未知的非 ASCII 字符保持原样。
