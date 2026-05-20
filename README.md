# Qubit Unicode

[![Rust CI](https://github.com/qubit-ltd/rs-unicode/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-unicode/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/qubit-unicode.svg?color=blue)](https://crates.io/crates/qubit-unicode)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文-blue.svg)](README.zh_CN.md)

Low-level Unicode, UTF-8, UTF-16, and ASCII utilities for Rust.

## Overview

Qubit Unicode provides small namespace enums for code-unit and code-point
operations that are useful below normal Rust `str` APIs:

- `Ascii` for ASCII classification, case conversion, digit conversion, and
  Java-compatible ASCII folding;
- `Unicode` for code point range checks, surrogate-pair helpers, planes, and
  Java-style `\uXXXX` escaping;
- `Utf8` for strict UTF-8 code-unit classification, cursor movement, decoding,
  and encoding into caller-provided byte buffers;
- `Utf16` for UTF-16 code-unit classification, surrogate-pair cursor movement,
  decoding, encoding, and Java/JavaScript-style `\uXXXX` escaping.

Prefer Rust's standard `str`, `String`, and `char` APIs for ordinary text
handling. Use this crate when a parser or codec needs precise byte or UTF-16
code-unit control.

## Installation

```toml
[dependencies]
qubit-unicode = "0.1"
```

## Example

```rust
use qubit_unicode::{ParsingPosition, Utf8};

let bytes = "A中".as_bytes();
let mut pos = ParsingPosition::new(1);

let ch = Utf8::get_next(&mut pos, bytes, bytes.len())?;
assert_eq!(Some('中'), ch);
assert_eq!(4, pos.index());

# Ok::<(), qubit_unicode::UnicodeError>(())
```

## Crate Boundary

This crate intentionally stays below full Unicode text processing. It does not
implement grapheme-cluster segmentation, normalization, collation, locale-aware
case mapping, or display-width calculation. Use specialized crates such as
`unicode-segmentation`, `unicode-normalization`, `unicode-width`, or ICU4X for
those higher-level semantics.

## Development

```bash
./align-ci.sh
./ci-check.sh
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
