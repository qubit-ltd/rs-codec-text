# Qubit Text Codec

[![Rust CI](https://github.com/qubit-ltd/rs-text-codec/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-text-codec/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-text-codec/coverage-badge.json)](https://qubit-ltd.github.io/rs-text-codec/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-text-codec.svg?color=blue)](https://crates.io/crates/qubit-text-codec)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Chinese Document](https://img.shields.io/badge/Document-Chinese-blue.svg)](README.zh_CN.md)

Buffer-oriented UTF codec primitives and Unicode/ASCII support utilities for Rust.

## Overview

Qubit Text Codec is a low-level codec core for Rust code that needs explicit control below ordinary `str`, `String`, and `char` APIs. Its current built-in codecs focus on Unicode transfer formats: UTF-8, UTF-16, and UTF-32, with both code-unit and byte-oriented variants where that distinction matters.

The crate also provides the small shared surface that codec adapters need: charset identity metadata, a generic `Coder` trait, low-level `CharsetCodec<T>` implementations, policy-aware `CharsetEncoder` / `CharsetDecoder` / `CharsetConverter` wrappers, decode status values, byte order and BOM helpers, and concrete encoding/decoding error types. ASCII and Unicode namespace helpers are included because UTF codecs and text parsers often need these checks close to the buffer boundary.

Use this crate when you need:

- ASCII classification, case conversion, digit conversion, and ASCII folding;
- Unicode code point and scalar value checks, surrogate checks, plane calculation, and noncharacter/control classification;
- UTF-8, UTF-16, and UTF-32 namespace helpers for byte or code-unit classification and length calculation;
- buffer-level `CharsetCodec<T>` implementations for UTF-8, UTF-16, and UTF-32;
- policy-aware charset encoders, decoders, and converters with malformed/unmappable replacement, ignore, and report actions;
- byte-order and BOM handling for UTF-16 and UTF-32 byte streams;
- a small trait and error vocabulary that future non-Unicode encoding adapters can reuse without making this crate a text I/O framework.

Prefer Rust's standard text APIs for ordinary text handling. Use this crate when a parser, binary format, or text I/O adapter needs strict buffer-level UTF codec behavior and precise error positions.

API reference documentation is available on [docs.rs](https://docs.rs/qubit-text-codec).

## Installation

```toml
[dependencies]
qubit-text-codec = "0.1"
```

## Quick Example

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

## Main Capabilities

### Standards

UTF-8 decoding follows the well-formed byte sequence rules in the
[Unicode Standard, Table 3-7](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G7404)
and the equivalent [RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629)
syntax. Malformed byte sequences include overlong encodings, UTF-8 encodings of surrogate code points, invalid continuation bytes, and sequences above `U+10FFFF`.

### Namespace Enums

`qubit-text-codec` exposes stateless namespace enums for constants, classification, conversion, and sizing. Encoding and decoding behavior lives in dedicated codec types.

| Namespace | Purpose |
| --- | --- |
| `Ascii` | ASCII constants, classification, case conversion, digit conversion, case-insensitive comparison, and ASCII folding |
| `Unicode` | Unicode code point range checks, scalar value checks, surrogate checks, plane calculation, noncharacter checks, control checks, and `u32` to `char` conversion |
| `Utf8` | UTF-8 byte classification and byte length calculation |
| `Utf16` | UTF-16 surrogate classification, surrogate-pair composition/decomposition, code-unit length calculation, and UTF-16 BOM detection |
| `Utf32` | UTF-32 scalar unit validation, unit length calculation, and UTF-32 BOM detection |

### Codec Layers

Encoding and decoding are split into three layers over caller-provided buffers.

| Layer | Type | Purpose |
| --- | --- |
| Generic conversion | `Coder<Input, Output>` | Converts one sequence of code units into another and reports `CoderProgress` |
| Low-level charset algorithm | `CharsetCodec<T>` | Encodes or decodes one Unicode `char` using storage unit `T` |
| Policy decoder | `CharsetDecoder<C, T>` | Converts `T` units into `char` values and applies `MalformedAction` |
| Policy encoder | `CharsetEncoder<C, T>` | Converts `char` values into `T` units and applies `UnmappableAction` |
| Charset conversion | `CharsetConverter<D, E, T1, T2>` | Combines one decoder and one encoder to convert between charsets |

`T` is the buffer storage unit, not always the Unicode code unit. UTF-8 uses `u8`, UTF-16 code-unit codecs use `u16`, byte-serialized UTF-16 uses `u8`, UTF-32 code-unit codecs use `u32`, and byte-serialized UTF-32 uses `u8`.

All conversion APIs receive the full input/output slice plus an absolute start
index. Progress counters are relative to those start indices, while errors
carry absolute indices in the supplied buffers.

`Charset` is a lightweight charset identity descriptor with a stable `id`,
display `name`, and accepted `aliases`. Built-in descriptors are available as
`Charset::ASCII`, `Charset::UTF_8`, `Charset::UTF_16`,
`Charset::UTF_16LE`, `Charset::UTF_16BE`, `Charset::UTF_32`,
`Charset::UTF_32LE`, and `Charset::UTF_32BE`. Generic `UTF_16` and `UTF_32`
represent the Unicode code-unit form or a BOM-aware stream label, while the
`LE` and `BE` variants identify fixed-byte-order byte streams. External codec
crates can define their own static descriptors, for example
`Charset::new("gbk", "GBK", &["cp936"])`. Equality and hashing use only the
`id`, while `matches_label` accepts the id, display name, or aliases with ASCII
case-insensitive comparison.

### Built-in Codecs

| Codec family | Storage unit | Low-level codec |
| --- | --- | --- |
| UTF-8 bytes | `u8` | `Utf8Codec` |
| UTF-16 code units | `u16` | `Utf16U16Codec` |
| UTF-16 bytes | `u8` | `Utf16ByteCodec` |
| UTF-32 code units | `u32` | `Utf32U32Codec` |
| UTF-32 bytes | `u8` | `Utf32ByteCodec` |

Byte codecs carry a `ByteOrder` value. Use `UnicodeBom::detect`, `Utf16::detect_bom`, or `Utf32::detect_bom` when a byte stream may include a BOM. The byte codecs do not detect, skip, or emit BOM bytes automatically. Streaming callers should buffer up to four bytes, or read until EOF, before deciding the BOM because UTF-32 little-endian (`FF FE 00 00`) overlaps the UTF-16 little-endian prefix (`FF FE`).

### Decode Status and Errors

`CharsetCodec::decode_one` and the policy decoders distinguish incomplete input from malformed input:

| Type | Purpose |
| --- | --- |
| `DecodeStatus::Complete { value, consumed }` | A complete scalar value and consumed unit count |
| `DecodeStatus::NeedMore { required, available }` | The prefix is valid so far but more units are required |
| `CharsetDecodeError` | Charset, decoding error kind, input unit index, and optional raw value |
| `CharsetEncodeError` | Charset, encoding error kind, output/input index, and optional raw value |

`DecodeStatus::NeedMore` is not an error. A streaming text reader should read more input when possible, and convert it at EOF into an incomplete-sequence error or an appropriate `std::io::Error`.

Errors tied to a raw value, such as an invalid UTF-32 unit or an unmappable character, expose that value through `value()`.

### ASCII Helpers

`Ascii` keeps ASCII-only behavior explicit and predictable:

| Method group | Examples |
| --- | --- |
| Range checks | `is_ascii_byte`, `is_ascii_char`, `is_ascii_code_point` |
| Classification | `is_whitespace_byte`, `is_letter_char`, `is_digit_code_point`, `is_hex_digit_char`, `is_printable_byte`, `is_control_code_point` |
| Conversion | `byte_to_uppercase`, `char_to_lowercase`, `char_to_digit`, `code_point_to_hex_digit` |
| Comparison and folding | `equals_ignore_case_char`, `equals_ignore_case_code_point`, `fold`, `fold_to_string` |

## Prelude

`qubit_text_codec::prelude` re-exports the core namespace enums, coder and charset codec traits, policy wrappers, built-in codec types, charset descriptors, byte-order/BOM helpers, decode-status types, actions, and charset encode/decode errors.

```rust
use qubit_text_codec::prelude::*;
```

## Crate Boundary

`qubit-text-codec` is not a general text processing library. It intentionally stays below grapheme-cluster segmentation, normalization, collation, locale-aware case mapping, transliteration, automatic encoding detection, display-width calculation, and `std::io` reader/writer adapters.

Its built-in codecs currently cover UTF-8, UTF-16, and UTF-32. It does not replace `encoding_rs` for legacy or web-compatible encodings such as GBK, Big5, Shift_JIS, or Windows code pages. Future crates can add those encodings on top of the shared traits and error model, or delegate their tables and compatibility rules to specialized libraries.

Use specialized crates such as `unicode-segmentation`, `unicode-normalization`, `unicode-width`, or ICU4X for higher-level Unicode semantics.

## Dependencies

This crate uses `thiserror` for error `Display` and `Error` implementations.

## Testing & Code Coverage

This project maintains test coverage for ASCII classification and folding, Unicode code point helpers, BOM and byte-order handling, charset descriptors, UTF-8/UTF-16/UTF-32 namespace helpers, buffer-level UTF codecs, and charset encode/decode errors.

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage report
./coverage.sh

# Generate text format report
./coverage.sh text

# Align code style with CI
./align-ci.sh

# Run CI checks (format, clippy, test, coverage, audit)
./ci-check.sh
```

## License

Copyright (c) 2026. Haixing Hu.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

See [LICENSE](LICENSE) for the full license text.

## Contributing

Contributions are welcome. Please feel free to submit a Pull Request.

### Development Guidelines

- Follow the Rust API guidelines.
- Prefer standard Rust text APIs unless low-level buffer-oriented codec control is required.
- Keep namespace enums focused on constants, classification, and sizing helpers.
- Keep charset-specific algorithms in concrete `CharsetCodec<T>` types and keep malformed/unmappable policy in `CharsetEncoder`, `CharsetDecoder`, or `CharsetConverter`.
- Use specialized Unicode crates or ICU4X for normalization, segmentation, collation, display width, and locale-aware behavior.
- Maintain comprehensive test coverage.
- Document public APIs with examples when they clarify behavior.
- Ensure `./ci-check.sh` passes before submitting a PR.

## Author

**Haixing Hu**

## Related Projects

- [qubit-io](https://github.com/qubit-ltd/rs-io): stream and byte I/O utilities for Rust.
- More Rust libraries from Qubit are published under the [qubit-ltd](https://github.com/qubit-ltd) organization on GitHub.

---

Repository: [https://github.com/qubit-ltd/rs-text-codec](https://github.com/qubit-ltd/rs-text-codec)
