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
- Essential `qubit-codec` primitives re-exported for callers:
  `Codec`, `Transcoder`, `TranscodeProgress`, `TranscodeStatus`, `CapacityError`,
  and `ByteOrder`.

This crate intentionally stays below `std::io` reader/writer adapters,
automatic charset detection, normalization, segmentation, collation, display
width, and locale-aware text behavior.

## Design Goals

- **Buffer-Level Control**: expose charset codecs that operate on caller-managed
  buffers.
- **Unicode Fundamentals**: provide ASCII, Unicode, UTF-8, UTF-16, and UTF-32
  primitives without higher-level locale behavior.
- **Policy-Aware Conversion**: make malformed and unmappable handling explicit
  through decoder and encoder configuration.
- **Precise Diagnostics**: report typed errors with source indices and context.
- **I/O Independence**: keep stream adapters in `qubit-io-text`.
- **Small Core Dependency**: depend on `qubit-codec` for shared transcoder and byte
  order primitives.

## Features

### Charset Metadata

- **`Charset`**: identifies supported charsets and their byte-order behavior.
- **`UnicodeBom`**: detects Unicode byte order marks.
- **ASCII and Unicode namespaces**: expose constants and validation helpers.

### Buffer-Level Codecs

- **`AsciiCodec`**: ASCII byte codec.
- **`Latin1Codec`**: ISO-8859-1 byte codec.
- **`Utf8Codec`**: UTF-8 byte codec.
- **`Utf16ByteCodec` / `Utf32ByteCodec`**: byte-oriented UTF-16 and UTF-32
  codecs with explicit byte order.
- **`Utf16U16Codec` / `Utf32U32Codec`**: unit-oriented UTF-16 and UTF-32 codecs.

### Stateful Converters

- **`CharsetDecoder`**: decodes input units into `char` output.
- **`CharsetEncoder`**: encodes `char` input into target units.
- **`CharsetConverter`**: converts between decoder and encoder pairs.
- **`MalformedAction` / `UnmappableAction`**: configure strict or replacement
  behavior.
- **EOF finalization**: `finish()` only drains internally retained output after
  callers have handled any incomplete source tail reported by `NeedInput`.

### Focused Public API

- **`prelude` module**: imports common charset, codec, error, and core transcoder
  types.
- **No stream I/O**: use `qubit-io-text` for reader and writer adapters.

## Documentation

- [User Guide](doc/user_guide.md)
- [API Reference](https://docs.rs/qubit-codec-text)
- [Chinese README](README.zh_CN.md)

## Installation

```toml
[dependencies]
qubit-codec-text = "0.1"
```

`qubit-codec` is the core runtime dependency. This crate re-exports only the
core traits and status types that are part of normal text-codec calls; import
generic engines, hooks, and adapters directly from `qubit-codec`.

## Quick Start

```rust
use qubit_codec_text::{
    CharsetEncoder,
    Codec,
    TranscodeStatus,
    Transcoder,
    UnicodeBom,
    Utf8,
    Utf8Codec,
};

assert_eq!(Some(UnicodeBom::Utf8), UnicodeBom::detect(&[0xef, 0xbb, 0xbf]));
assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xe4));

let (decoded, consumed) = unsafe {
    Utf8Codec
        .decode_unchecked("中".as_bytes(), 0)
}
    .expect("valid UTF-8 input");
assert_eq!(('中', 3), (decoded, consumed.get()));

let mut encoder = CharsetEncoder::new(Utf8Codec);
let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
let progress = encoder
    .transcode(&['😀'], 0, &mut output, 0)
    .expect("UTF-8 output buffer is large enough");

assert_eq!(TranscodeStatus::Complete, progress.status());
assert_eq!("😀".as_bytes(), &output[..progress.written()]);
```

## API Reference

### Charset and Unicode Types

| Type | Purpose |
|------|---------|
| `Charset` | Supported charset identity and byte-order metadata |
| `UnicodeBom` | Unicode BOM detection |
| `Ascii`, `Unicode`, `Utf8`, `Utf16`, `Utf32` | Namespace helpers for character set rules |

### Codec Types

| Type | Purpose |
|------|---------|
| `AsciiCodec` | ASCII byte encoding and decoding |
| `Latin1Codec` | ISO-8859-1 byte encoding and decoding |
| `Utf8Codec` | UTF-8 byte encoding and decoding |
| `Utf16ByteCodec` / `Utf32ByteCodec` | Explicit-byte-order Unicode byte codecs |
| `Utf16U16Codec` / `Utf32U32Codec` | Unit-oriented Unicode codecs |
| `Codec<char, Unit>` | Lowest-level complete-value codec trait re-exported from `qubit-codec` |
| `CharsetCodec` | Charset metadata attached to low-level text codec implementations |
| `CharsetEncodeProbe` | Exact per-character output sizing and mappability probe |

### Converter Types

| Type | Purpose |
|------|---------|
| `CharsetDecoder<C>` | Stateful buffer decoder implementing `BufferedDecoder<C::Unit, char>` and reusing `BufferedDecodeEngine` for decode iteration and progress reporting |
| `CharsetEncoder<C>` | Stateful buffer encoder implementing `BufferedEncoder<char, C::Unit>` and reusing `BufferedEncodeEngine` for its buffered loop |
| `CharsetEncodePlan` | Plan payload used by `CharsetEncoder`'s internal encode hooks |
| `CharsetConverter<D, E>` | Decode and encode between two charset codecs, implementing `BufferedConverter<D::Unit, E::Unit>` |
| `MalformedAction` | Policy for malformed input |
| `UnmappableAction` | Policy for unencodable output characters |

### Error Types

| Type | Purpose |
|------|---------|
| `CharsetDecodeError` / `CharsetDecodeErrorKind` | Decode failure with precise index |
| `CharsetEncodeError` / `CharsetEncodeErrorKind` | Encode failure with precise index |
| `CharsetConvertError` | Converter-level decode or encode failure |

## Performance Considerations

Codec implementations work against caller-provided input and output buffers.
`CharsetDecoder` calls `Codec::decode_unchecked` once at least
`codec.min_units_per_value()` units are readable, and charset codecs report
incomplete prefixes through `CharsetDecodeError`. `NeedInput` means the current
units are a valid incomplete prefix left in the caller-owned input buffer; after
EOF, the caller handles that tail before calling `finish()` to drain internal
output. Internally, `CharsetDecoder` stores its policy in decode hooks and
reuses `BufferedDecodeEngine` for repeated `decode_unchecked` calls, output
capacity progress, and status reporting.
`CharsetEncoder` stores its unmappable policy in encode hooks and reuses
`BufferedEncodeEngine` for input iteration and output capacity checks while
still applying text-specific replacement, ignore, and report policy. It reports
`NeedOutput` through the shared `Transcoder` progress model so callers can
control allocation and buffer reuse.

## Testing & Code Coverage

This project keeps charset behavior covered by integration tests under `tests/`.

### Running Tests

```bash
# Run tests
cargo test

# Run with coverage report
./coverage.sh

# Generate text format report
./coverage.sh text

# Align formatting and clippy with CI
./align-ci.sh

# Run CI checks (format, clippy, test, coverage, audit)
RS_CI_SKIP_TOOLCHAIN_UPDATE=1 ./ci-check.sh
```

## Dependencies

Runtime dependencies are intentionally small:

- `qubit-codec` provides shared byte-order and transcoder primitives.
- `thiserror` provides the public error type implementations.

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

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

- Keep this crate focused on buffer-level text codecs.
- Keep documentation aligned with user guides and public API names.
- Add tests for strict, replacement, malformed, and unmappable behavior.
- Ensure all checks pass before submitting a PR.

## Author

**Haixing Hu**

## Related Projects

- [qubit-codec](https://github.com/qubit-ltd/rs-codec): shared core codec traits
  and byte-order markers.
- [qubit-io-text](https://github.com/qubit-ltd/rs-io-text): text stream adapters
  for Rust.
- More Rust libraries from Qubit are published under the
  [qubit-ltd](https://github.com/qubit-ltd) organization on GitHub.

Repository: [https://github.com/qubit-ltd/rs-codec-text](https://github.com/qubit-ltd/rs-codec-text)
