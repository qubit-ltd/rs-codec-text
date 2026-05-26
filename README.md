# Qubit Text Codec

[![Rust CI](https://github.com/qubit-ltd/rs-codec-text/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-codec-text/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-codec-text/coverage-badge.json)](https://qubit-ltd.github.io/rs-codec-text/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-codec-text.svg?color=blue)](https://crates.io/crates/qubit-codec-text)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Chinese Document](https://img.shields.io/badge/Document-Chinese-blue.svg)](README.zh_CN.md)

Buffer-oriented charset codec primitives and Unicode/ASCII support utilities
for Rust.

## Overview

Qubit Text Codec is a low-level codec core for code that needs explicit control
below ordinary `str`, `String`, and `char` APIs. It provides:

- ASCII, Unicode, UTF-8, UTF-16, and UTF-32 namespace helpers.
- Charset identity metadata, byte-order helpers, and Unicode BOM detection.
- Buffer-level codecs for ASCII, ISO-8859-1, UTF-8, UTF-16, and UTF-32.
- Policy-aware `CharsetDecoder`, `CharsetEncoder`, and `CharsetConverter`
  wrappers.
- Typed decode/encode/convert errors with precise buffer indices.
- `Coder`, `CoderProgress`, `CoderStatus`, and `ByteOrder` re-exported from
  `qubit-codec`.

This crate intentionally stays below `std::io` reader/writer adapters,
automatic charset detection, normalization, segmentation, collation, display
width, and locale-aware text behavior.

## Documentation

- [User Guide](doc/user_guide.md)
- [API Reference](https://docs.rs/qubit-codec-text)
- [Chinese README](README.zh_CN.md)

## Installation

```toml
[dependencies]
qubit-codec-text = "0.1"
```

`qubit-codec` is the core runtime dependency. The core buffer-level traits used
by this public API are re-exported by `qubit-codec-text`.

## Quick Example

```rust
use qubit_codec_text::{
    CharsetCodec,
    CharsetEncoder,
    Coder,
    CoderStatus,
    DecodeStatus,
    UnicodeBom,
    Utf8,
    Utf8Codec,
};

assert_eq!(Some(UnicodeBom::Utf8), UnicodeBom::detect(&[0xef, 0xbb, 0xbf]));
assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xe4));

let decoded = Utf8Codec
    .decode_one("中".as_bytes(), 0)
    .expect("valid UTF-8 input");
assert_eq!(
    DecodeStatus::Complete {
        value: '中',
        consumed: 3,
    },
    decoded,
);

let mut encoder = CharsetEncoder::new(Utf8Codec);
let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
let progress = encoder
    .convert(&['😀'], 0, &mut output, 0)
    .expect("UTF-8 output buffer is large enough");

assert_eq!(CoderStatus::Complete, progress.status());
assert_eq!("😀".as_bytes(), &output[..progress.written()]);
```

## Development

```bash
# Run tests
cargo test

# Align formatting and clippy with CI
./align-ci.sh

# Run the full local CI pipeline
RS_CI_SKIP_TOOLCHAIN_UPDATE=1 ./ci-check.sh
```

## License

Copyright (c) 2026. Haixing Hu.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Related Projects

- [qubit-io-text](https://github.com/qubit-ltd/rs-io-text): text stream adapters
  for Rust.
- More Rust libraries from Qubit are published under the
  [qubit-ltd](https://github.com/qubit-ltd) organization on GitHub.

Repository: [https://github.com/qubit-ltd/rs-codec-text](https://github.com/qubit-ltd/rs-codec-text)
