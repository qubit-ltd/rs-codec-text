# Qubit Unicode

[![Rust CI](https://github.com/qubit-ltd/rs-unicode/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-unicode/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-unicode/coverage-badge.json)](https://qubit-ltd.github.io/rs-unicode/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-unicode.svg?color=blue)](https://crates.io/crates/qubit-unicode)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Chinese Document](https://img.shields.io/badge/Document-Chinese-blue.svg)](README.zh_CN.md)

Low-level Unicode, UTF-8, UTF-16, and ASCII utilities for Rust.

## Overview

Qubit Unicode provides small namespace enums for code-unit and code-point operations that are useful below normal Rust `str` APIs. It is designed for parsers, codecs, compatibility layers, and binary formats that need explicit control over bytes, UTF-16 code units, Unicode scalar values, or ASCII-only behavior.

Use this crate when you need:

- ASCII classification, case conversion, digit conversion, and ASCII folding;
- Unicode code point range checks, surrogate-pair helpers, plane calculation, and Java-style `\uXXXX` escaping;
- strict UTF-8 byte classification, cursor movement, decoding, and encoding into caller-provided byte buffers;
- UTF-16 code-unit classification, surrogate-pair cursor movement, decoding, encoding, and Java/JavaScript-style `\uXXXX` escaping;
- explicit position and error reporting for malformed or incomplete Unicode sequences.

Prefer Rust's standard `str`, `String`, and `char` APIs for ordinary text handling. Use this crate when a parser or codec needs precise byte or UTF-16 code-unit control.

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

## Main Capabilities

### Namespace Enums

`qubit-unicode` exposes stateless namespace enums instead of heap-allocated helper objects:

| Namespace | Purpose |
| --- | --- |
| `Ascii` | ASCII constants, classification, case conversion, digit conversion, case-insensitive comparison, and ASCII folding |
| `Unicode` | Unicode scalar range checks, BMP and supplementary checks, surrogate-pair composition/decomposition, plane calculation, and Java-style escaping |
| `Utf8` | UTF-8 byte classification, code-unit counts, cursor movement, strict decoding, reverse decoding, and scalar encoding |
| `Utf16` | UTF-16 code-unit classification, surrogate-pair handling, cursor movement, decoding, reverse decoding, scalar encoding, and UTF-16 escaping |

### ASCII Helpers

`Ascii` keeps ASCII-only behavior explicit and predictable:

| Method group | Examples |
| --- | --- |
| Range checks | `is_ascii_byte`, `is_ascii_char`, `is_ascii_code_point` |
| Classification | `is_whitespace_byte`, `is_letter_char`, `is_digit_code_point`, `is_hex_digit_char`, `is_printable_byte`, `is_control_code_point` |
| Conversion | `to_upper_case_byte`, `to_lower_case_char`, `to_digit_char`, `to_hex_digit_code_point` |
| Comparison and folding | `equals_ignore_case_char`, `equals_ignore_case_code_point`, `fold`, `fold_to_string` |

### UTF-8 and UTF-16 Cursor APIs

`Utf8` and `Utf16` operate on caller-provided slices and `ParsingPosition`.
They are useful when a parser needs to decode inside a larger buffer without taking ownership of the buffer:

| Method group | UTF-8 examples | UTF-16 examples |
| --- | --- | --- |
| Code-unit classification | `is_single`, `is_leading`, `is_trailing` | `is_single`, `is_leading`, `is_trailing`, `is_surrogate` |
| Size calculation | `trailing_count`, `code_unit_count` | `trailing_count`, `code_unit_count` |
| Cursor movement | `set_to_start`, `set_to_terminal`, `forward`, `backward` | `set_to_start`, `set_to_terminal`, `forward`, `backward` |
| Decode and encode | `get_next`, `get_previous`, `put` | `get_next`, `get_previous`, `put`, `escape` |

### Position and Error Types

Cursor APIs return `UnicodeResult<T>` and report where a problem was detected:

| Type | Purpose |
| --- | --- |
| `ParsingPosition` | Mutable cursor with optional error index and error kind |
| `UnicodeError` | Error value containing `UnicodeErrorKind` and byte/code-unit index |
| `UnicodeErrorKind` | `BufferOverflow`, `Malformed`, or `Incomplete` |
| `UnicodeResult<T>` | Convenience alias for `Result<T, UnicodeError>` |

## Prelude

`qubit_unicode::prelude` re-exports the small public API surface: namespace enums, cursor and error types, and `UnicodeResult`.

```rust
use qubit_unicode::prelude::*;
```

## Crate Boundary

`qubit-unicode` intentionally stays below full Unicode text processing. It does not implement grapheme-cluster segmentation, normalization, collation, locale-aware case mapping, transliteration, encoding detection, or display-width calculation.

Use specialized crates such as `unicode-segmentation`, `unicode-normalization`, `unicode-width`, or ICU4X for those higher-level semantics.

## Dependencies

This crate uses `thiserror` for error `Display` and `Error` implementations.

## Testing & Code Coverage

This project maintains test coverage for ASCII classification and folding, Unicode code point helpers, UTF-8 cursor and encoding behavior, UTF-16 surrogate handling, position tracking, and error reporting.

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
- Prefer standard Rust text APIs unless low-level byte or UTF-16 code-unit control is required.
- Keep this crate focused on Unicode scalar values, UTF-8 bytes, UTF-16 code units, and ASCII-only helpers.
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
