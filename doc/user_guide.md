# Qubit Text Codec User Guide

[中文](user_guide.zh_CN.md) · [README](../README.md) · [API reference](https://docs.rs/qubit-codec-text)

This guide applies to `qubit-codec-text` 0.3. It is for developers building
buffer-oriented parsers, protocol handlers, and I/O adapters that must preserve
conversion progress and choose how malformed or unmappable data is handled.

## Conceptual Model

The crate separates charset metadata, one-scalar codecs, and buffered policy
wrappers:

```text
input units -> CharsetDecoder -> char values -> CharsetEncoder -> output units
                      \                         /
                       \-> CharsetConverter ---/
```

`Charset` identifies supported character sets and labels. `CharsetCodec`
implementations decode or encode one `char`; `CharsetDecoder`,
`CharsetEncoder`, and `CharsetConverter` own the policy and report progress.
The shared progress/status types are imported from `qubit-codec`.

## Scenario: Convert Incoming UTF-8 to UTF-16

An adapter receives UTF-8 bytes and must put UTF-16 code units into an output
buffer. For one complete message, use the checked convenience method below.
For a stream, call `transcode` for each buffer, retain an unconsumed incomplete
tail, handle output backpressure, and call `finish` only after EOF.

## Installation and Minimal Configuration

```toml
[dependencies]
qubit-codec-text = "0.3"
qubit-codec = "0.11"
```

Enable the optional `serde` feature only to serialize `Charset` values.

## Core Workflow

```rust
use qubit_codec_text::{CharsetConverter, Utf16U16Codec, Utf8Codec};

let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
let mut output = [0_u16; 2];
let written = converter
    .transcode_complete_into("A中".as_bytes(), &mut output)
    .expect("complete UTF-8 input and sufficient output");

assert_eq!(2, written);
assert_eq!([0x0041, 0x4e2d], output);
```

Use `CharsetDecoder::new(Utf8Codec)` when output is `char` values, or
`CharsetEncoder::new(Utf8Codec)` when input is `char` values. The
`transcode_complete_into` methods apply the configured policy when decoder
input ends inside an incomplete sequence: `Replace` emits the replacement
character, `Ignore` discards the incomplete tail, and `Report` returns an
error. They reject an output buffer that is too small.

## Advanced Usage

### Policies

| Condition | Default | Alternatives |
| --- | --- | --- |
| Source units are malformed | `MalformedAction::Replace` with U+FFFD | `Ignore` skips the malformed range; `Report` returns an error. |
| Character cannot be represented by target charset | `UnmappableAction::Replace` | `Ignore` skips it; `Report` returns an error. |

Choose an explicit policy with `CharsetDecodePolicy` or `CharsetEncodePolicy`
when replacement is not acceptable. `CharsetEncoder::with_policy` and
`CharsetConverter::from_codecs_with_policies` validate a replacement that must
be encodable by the target codec.

### BOM and Byte Order

`UnicodeBom::detect` recognizes UTF-8, UTF-16, and UTF-32 BOMs from closed
input. A streaming prefix can be ambiguous: `FF FE` might become a UTF-32LE
BOM. Use `UnicodeBom::detect_progress(bytes, eof)` or
`CharsetDecoder::<C>::detect_and_strip_bom_progress` until enough bytes arrive
or EOF is known. Byte-oriented UTF-16 and UTF-32 codecs require an explicit
`ByteOrder` and never automatically consume or produce a BOM.

### Charset Labels

`Charset::from_label` matches built-ins and registered descriptors after loose
ASCII normalization (trimming, case folding, and ignoring `-`/`_`).
`from_whatwg_label` uses different, WHATWG-style preprocessing but is not a
complete WHATWG Encoding Standard label table. Register a custom descriptor
with `Charset::register` or `Charset::register_new`; `new_static` does not
register it.

## Errors and Diagnostics

Low-level codecs report `CharsetDecodeError` or `CharsetEncodeError`, carrying
the charset, an error kind, and an index. Their kinds distinguish incomplete
sequences, malformed input, invalid scalar values, capacity, and unmappable
characters where applicable. `CharsetConverter::map_transcode_error` maps a
low-level converter failure to `CharsetConvertError`, identifying whether the
source decoding or target encoding side failed.

In streaming code, `TranscodeProgress` describes how many units were read and
written and reports a status such as `NeedInput` or output backpressure. Do not
treat `NeedInput` as a malformed error: preserve the tail and retry it with
later input. At EOF, a remaining incomplete sequence is handled by the
configured policy: replacement, discard, or an error.

## Troubleshooting

| Symptom | Check |
| --- | --- |
| A UTF-8 tail asks for more input | Keep the unconsumed tail; do not call `finish` before EOF. |
| UTF-16/32 bytes decode incorrectly | Confirm the selected `ByteOrder` and explicitly detect or strip any BOM. |
| A character becomes a replacement | Inspect the malformed/unmappable policy and whether the target charset represents that character. |
| Output is incomplete | Inspect progress, allocate or drain more output, then continue from the reported offsets. |

## Limitations and Best Practices

This is not a full text-processing or `std::io` library. It does not provide
grapheme segmentation, normalization, collation, display width, locale-aware
casing, automatic charset detection, stream ownership, or line-ending policy.
Use fixed, caller-owned buffers only when their capacity and status are checked;
call the unsafe single-value `Codec` methods only after satisfying their bounds
and value-domain contracts.

## Further Reading

- [README](../README.md)
- [中文用户指南](user_guide.zh_CN.md)
- [API reference](https://docs.rs/qubit-codec-text)
