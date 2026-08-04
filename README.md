# Qubit Text Codec

[![Rust CI](https://github.com/qubit-ltd/rs-codec-text/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-codec-text/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-codec-text/coverage-badge.json)](https://qubit-ltd.github.io/rs-codec-text/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-codec-text.svg?color=blue)](https://crates.io/crates/qubit-codec-text)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

`qubit-codec-text` provides buffer-oriented Unicode and charset codecs for Rust.
It helps parser, protocol, and I/O-adapter authors decode, encode, and convert
ASCII, Latin-1, UTF-8, UTF-16, and UTF-32 while keeping progress and policy explicit.

## Installation

```toml
[dependencies]
qubit-codec-text = "0.4"
qubit-codec = "0.11"
```

Enable serialization of `Charset` only when needed:

```toml
qubit-codec-text = { version = "0.4", features = ["serde"] }
```

## Quick Start

Convert UTF-8 bytes to UTF-16 code units with a caller-owned output buffer:

```rust
use qubit_codec_text::{CharsetConverter, Utf16U16Codec, Utf8Codec};

let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
let mut output = [0_u16; 2];
let written = converter
    .transcode_complete_into("AB".as_bytes(), &mut output)
    .expect("UTF-8 text converts to UTF-16");

assert_eq!(2, written);
assert_eq!([65, 66], output);
```

## Why This Project Exists

Text conversion often sits below application strings: a caller may receive partial
buffers, need a fixed output unit type, or have a protocol-specific replacement policy.
This crate supplies the codec and policy layer without owning `std::io` streams,
buffering, Unicode normalization, or locale rules.

## What It Provides

| Capability | Public API | Boundary |
| --- | --- | --- |
| Charset metadata and labels | `Charset`, `UnicodeBom` | Built-ins cover ASCII, Latin-1, UTF-8, UTF-16, and UTF-32 families; lookup is not a full WHATWG encoding table. |
| Low-level scalar codecs | `AsciiCodec`, `Latin1Codec`, `Utf8Codec`, UTF-16/32 codecs | Unsafe single-value methods operate on caller-owned units. |
| Buffered conversion | `CharsetDecoder`, `CharsetEncoder`, `CharsetConverter` | Reports progress, output backpressure, and the need for input through `qubit-codec` types. |
| Policy | `MalformedAction`, `UnmappableAction` | Choose `Replace`, `Ignore`, or `Report`. |
| Character helpers | `Ascii`, `Unicode`, `Utf8`, `Utf16`, `Utf32` | No grapheme segmentation, normalization, collation, or locale-aware casing. |

Byte-oriented UTF-16 and UTF-32 codecs use an explicit `ByteOrder`; they do not
automatically emit, skip, or select a BOM. Stream ownership, buffering, and
`std::io::Error` conversion belong to higher-level adapters.

## Learn More

- [User guide](doc/user_guide.md)
- [API reference](https://docs.rs/qubit-codec-text)
- [中文 README](README.zh_CN.md)
- [中文用户指南](doc/user_guide.zh_CN.md)

## Testing

```bash
# Run tests with the default feature set
cargo test

# Run tests with all declared features
cargo test --all-features

# Project CI checks
./ci-check.sh

# Check code coverage
./coverage.sh
```

## License

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Please follow the Rust API guidelines, keep public
API documentation and tests current, and run `./align-ci.sh` to format code and
`./ci-check.sh` to satisfy CI requirements before submitting a pull request.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

Repository: [https://github.com/qubit-ltd/rs-codec-text](https://github.com/qubit-ltd/rs-codec-text)
