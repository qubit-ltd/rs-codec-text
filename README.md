# Qubit Unicode

[![Rust CI](https://github.com/qubit-ltd/rs-unicode/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-unicode/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-unicode/coverage-badge.json)](https://qubit-ltd.github.io/rs-unicode/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-unicode.svg?color=blue)](https://crates.io/crates/qubit-unicode)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Chinese Document](https://img.shields.io/badge/Document-Chinese-blue.svg)](README.zh_CN.md)

Low-level Unicode constants, text classification helpers, and buffer-oriented UTF codec primitives for Rust.

## Overview

Qubit Unicode provides low-level building blocks for code that needs explicit control below Rust's ordinary `str`, `String`, and `char` APIs. It separates Unicode and encoding namespace helpers from concrete text encoders and decoders, so parser and I/O crates can reuse strict UTF-8, UTF-16, and UTF-32 logic without depending on `std::io`.

Use this crate when you need:

- ASCII classification, case conversion, digit conversion, and ASCII folding;
- Unicode code point and scalar value checks, surrogate checks, plane calculation, and noncharacter/control classification;
- UTF-8, UTF-16, and UTF-32 namespace helpers for byte or code-unit classification and length calculation;
- buffer-level `TextEncoder<T>` and `TextDecoder<T>` implementations for UTF-8, UTF-16, and UTF-32;
- byte-order and BOM handling for UTF-16 and UTF-32 byte streams;
- reusable text coding error types for Unicode codecs and future non-Unicode encoding adapters.

Prefer Rust's standard text APIs for ordinary text handling. Use this crate when a parser, codec, binary format, or text I/O adapter needs strict buffer-level control.

For detailed usage, examples, and API selection guidance, see the [User Guide](doc/user_guide.md).
API reference documentation is available on [docs.rs](https://docs.rs/qubit-unicode).

## Installation

```toml
[dependencies]
qubit-unicode = "0.1"
```

## Quick Example

```rust
use qubit_unicode::{
    ByteOrder,
    DecodeResult,
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
assert_eq!(DecodeResult::Complete(qubit_unicode::Decoded::new('中', 3)), decoded);

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

## Main Capabilities

### Standards

UTF-8 decoding follows the well-formed byte sequence rules in the
[Unicode Standard, Table 3-7](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G7404)
and the equivalent [RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629)
syntax. Malformed byte sequences include overlong encodings, UTF-8 encodings of surrogate code points, invalid continuation bytes, and sequences above `U+10FFFF`.

### Namespace Enums

`qubit-unicode` exposes stateless namespace enums for constants, classification, and sizing. Encoding and decoding live in dedicated codec types.

| Namespace | Purpose |
| --- | --- |
| `Ascii` | ASCII constants, classification, case conversion, digit conversion, case-insensitive comparison, and ASCII folding |
| `Unicode` | Unicode code point range checks, scalar value checks, surrogate checks, plane calculation, noncharacter checks, control checks, and `u32` to `char` conversion |
| `Utf8` | UTF-8 byte classification and byte length calculation |
| `Utf16` | UTF-16 surrogate classification, surrogate-pair composition/decomposition, code-unit length calculation, and UTF-16 BOM detection |
| `Utf32` | UTF-32 scalar unit validation, unit length calculation, and UTF-32 BOM detection |

### Codec Traits

Encoding and decoding are modeled by small traits over caller-provided buffers.

| Trait | Purpose |
| --- | --- |
| `TextDecoder<T>` | Decodes encoded units from `&[T]` into Unicode `char` values |
| `TextEncoder<T>` | Encodes Unicode `char` values into `&mut [T]` |
| `TextCodec<T>` | Blanket trait for types implementing both encoder and decoder for the same storage unit type |

`T` is the buffer storage unit, not always the Unicode code unit. UTF-8 uses `u8`, UTF-16 code-unit codecs use `u16`, byte-serialized UTF-16 uses `u8`, UTF-32 code-unit codecs use `u32`, and byte-serialized UTF-32 uses `u8`.

### Built-in Codecs

| Codec family | Storage unit | Types |
| --- | --- | --- |
| UTF-8 bytes | `u8` | `Utf8Encoder`, `Utf8Decoder`, `Utf8Codec` |
| UTF-16 code units | `u16` | `Utf16U16Encoder`, `Utf16U16Decoder`, `Utf16U16Codec` |
| UTF-16 bytes | `u8` | `Utf16ByteEncoder`, `Utf16ByteDecoder`, `Utf16ByteCodec` |
| UTF-32 code units | `u32` | `Utf32U32Encoder`, `Utf32U32Decoder`, `Utf32U32Codec` |
| UTF-32 bytes | `u8` | `Utf32ByteEncoder`, `Utf32ByteDecoder`, `Utf32ByteCodec` |

Byte codecs carry a `ByteOrder` value. Use `UnicodeBom::detect`, `Utf16::detect_bom`, or `Utf32::detect_bom` when a byte stream may include a BOM.

### Decode Result and Errors

`TextDecoder::decode_prefix` distinguishes incomplete input from malformed input:

| Type | Purpose |
| --- | --- |
| `DecodeResult::Complete(Decoded<char>)` | A complete scalar value and consumed unit count |
| `DecodeResult::NeedMore(NeedMore)` | The prefix is valid so far but more units are required |
| `TextDecodingError` | Encoding, decoding error kind, and input unit index |
| `TextEncodingError` | Encoding, encoding error kind, and output/input index |
| `TextCodingError` | Wrapper for APIs that intentionally combine encoding and decoding failures |

`NeedMore` is not an error. A streaming text reader should read more input when possible, and convert `NeedMore` at EOF into an incomplete-sequence error or an appropriate `std::io::Error`.

### ASCII Helpers

`Ascii` keeps ASCII-only behavior explicit and predictable:

| Method group | Examples |
| --- | --- |
| Range checks | `is_ascii_byte`, `is_ascii_char`, `is_ascii_code_point` |
| Classification | `is_whitespace_byte`, `is_letter_char`, `is_digit_code_point`, `is_hex_digit_char`, `is_printable_byte`, `is_control_code_point` |
| Conversion | `byte_to_uppercase`, `char_to_lowercase`, `char_to_digit`, `code_point_to_hex_digit` |
| Comparison and folding | `equals_ignore_case_char`, `equals_ignore_case_code_point`, `fold`, `fold_to_string` |

## Prelude

`qubit_unicode::prelude` re-exports the core namespace enums, codec traits, built-in codec types, byte-order/BOM helpers, decode-result types, and text coding errors.

```rust
use qubit_unicode::prelude::*;
```

## Crate Boundary

`qubit-unicode` intentionally stays below full Unicode text processing. It does not implement grapheme-cluster segmentation, normalization, collation, locale-aware case mapping, transliteration, automatic encoding detection, or display-width calculation.

It also does not replace `encoding_rs` for legacy or web-compatible encodings such as GBK, Big5, Shift_JIS, or Windows code pages. Future adapters can reuse the text coding traits and error model while delegating non-Unicode encodings to specialized libraries.

Use specialized crates such as `unicode-segmentation`, `unicode-normalization`, `unicode-width`, or ICU4X for higher-level Unicode semantics.

## Dependencies

This crate uses `thiserror` for error `Display` and `Error` implementations.

## Testing & Code Coverage

This project maintains test coverage for ASCII classification and folding, Unicode code point helpers, BOM and byte-order handling, UTF-8/UTF-16/UTF-32 namespace helpers, buffer-level codecs, and text coding errors.

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
- Keep encoding and decoding behavior in concrete codec types implementing `TextEncoder<T>` and `TextDecoder<T>`.
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

Repository: [https://github.com/qubit-ltd/rs-unicode](https://github.com/qubit-ltd/rs-unicode)
