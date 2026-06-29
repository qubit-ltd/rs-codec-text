# Qubit Text Codec User Guide

This guide explains what `qubit-codec-text` provides, how the pieces fit
together, and how to use the crate in buffer-oriented text codec code.

For a shorter project overview, see the [README](../README.md). For Chinese,
see the [Chinese user guide](user_guide.zh_CN.md).

## Purpose

`qubit-codec-text` is a low-level text codec core. It is intended for parsers,
binary formats, and text I/O adapters that need explicit control over byte or
code-unit buffers, exact error positions, and strict malformed/unmappable
policy.

Use this crate when you need:

- ASCII constants, classification, conversion, comparison, and folding helpers.
- Unicode scalar/code-point checks, surrogate checks, plane calculation,
  noncharacter checks, and control checks.
- UTF-8, UTF-16, and UTF-32 helper namespaces for byte/code-unit length and BOM
  detection.
- Buffer-level codecs for ASCII, ISO-8859-1, UTF-8, UTF-16, and UTF-32.
- Policy-aware decoders, encoders, and converters with replace, ignore, and
  report modes.
- `Charset`, `UnicodeBom`, typed encode/decode errors, and policy-aware
  wrappers for building higher-level adapters. Import `ByteOrder`, `Codec`,
  `Transcoder`, progress/status types, core engines, and hooks directly from
  `qubit-codec`.

This crate is not a general text processing library. It intentionally stays
below grapheme segmentation, normalization, collation, locale-aware case
mapping, display width, automatic charset detection, and `std::io` reader/writer
adapters. Use crates such as `unicode-segmentation`, `unicode-normalization`,
`unicode-width`, ICU4X, or a dedicated text I/O crate for those layers.

## Installation

```toml
[dependencies]
qubit-codec-text = "0.2"
qubit-codec = "0.10"
```

`qubit-codec` is the core runtime dependency. `qubit-codec-text` does not
re-export `qubit-codec` types; add or import `qubit-codec` directly when naming
core traits, progress/status types, byte order, engines, hooks, or adapters.

Enable the optional `serde` feature to serialize `Charset` as its stable `id`:

```toml
[dependencies]
qubit-codec-text = { version = "0.2", features = ["serde"] }
```

For compact imports:

```rust
use qubit_codec_text::*;
```

For explicit imports:

```rust
use qubit_codec::Transcoder;
use qubit_codec_text::{
    CharsetCodec,
    CharsetDecoder,
    CharsetEncoder,
    Utf8Codec,
};
```

## Architecture

The crate is split into a few small layers.

| Layer | Main types | Role |
| --- | --- | --- |
| Namespace helpers | `Ascii`, `Unicode`, `Utf8`, `Utf16`, `Utf32` | Constants, classification, sizing, and BOM helper functions. |
| Charset metadata | `Charset`, `UnicodeBom`, `qubit_codec::ByteOrder` | Stable charset identity, aliases, fixed byte order, and BOM metadata. |
| Low-level codecs | `qubit_codec::Codec<Value = char>`, built-in codec structs | Decode or encode one complete Unicode scalar value from/to caller-owned buffers. |
| Text codec metadata | `CharsetCodec` | Attach charset metadata to low-level text codec implementations. |
| Policy wrappers | `CharsetDecoder`, `CharsetEncoder` | Apply malformed/unmappable policy while converting many units; implement `TranscodeDecoder` / `TranscodeEncoder`. `CharsetDecoder` reuses the core `TranscodeDecodeEngine` loop, and `CharsetEncoder` reuses the core `TranscodeEncodeEngine` loop. |
| Charset conversion | `CharsetConverter` | Decode source units to `char`, then encode them to target units; implements `TranscodeConverter`. |
| Progress API | `qubit_codec::Transcoder`, `qubit_codec::TranscodeProgress`, `qubit_codec::TranscodeStatus` | Report partial progress, input starvation, and output backpressure. |
| Errors | `CharsetDecodeError`, `CharsetEncodeError`, `CharsetConvertError` | Preserve charset, kind, absolute index, and optional raw value. |

All codec operations are buffer-oriented. Callers pass a complete input slice,
a complete output slice, and absolute start indices. Returned `read` and
`written` counts are relative to those start indices. Error indices and
`TranscodeStatus` indices are absolute within the supplied buffers.

## Namespace Helpers

The namespace enums are stateless. They group constants and helper functions
without owning buffers.

```rust
use qubit_codec_text::{
    Ascii,
    Unicode,
    Utf8,
    Utf16,
    Utf32,
};

assert!(Ascii::is_letter_byte(b'A'));
assert_eq!(b'a', Ascii::byte_to_lowercase(b'A'));
assert_eq!(Some(10), Ascii::char_to_hex_digit('A'));

assert!(Unicode::is_scalar_value('中' as u32));
assert_eq!(Some(0), Unicode::plane('A' as u32));
assert_eq!(Some('A'), Unicode::to_char(0x41));

assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xe4));
assert_eq!(2, Utf16::unit_len('😀'));
assert!(Utf32::is_valid_unit('中' as u32));
```

`Ascii` also provides complete printable/digit/letter arrays and ASCII folding
helpers for stable ASCII-only comparisons. These helpers do not implement full
Unicode case folding.

## Charset Metadata

`Charset` is a small identity descriptor. Equality and hashing use the stable
`id`; the display name and aliases are used for presentation and label matching.

```rust
use qubit_codec_text::Charset;

assert_eq!("utf-8", Charset::UTF_8.id());
assert_eq!("UTF-8", Charset::UTF_8.name());
assert!(Charset::UTF_8.matches_label("utf8"));
assert!(Charset::UTF_8.matches_label("utf_8"));
assert_eq!(Some(Charset::UTF_8), Charset::from_label("utf8"));
assert_eq!(Some(Charset::UTF_8), Charset::from_whatwg_label("UTF-8"));

const GBK: Charset = Charset::new_static("gbk", "GBK", &["cp936"]);
assert!(GBK.matches_label("CP936"));

let gb18030 = Charset::register_new(
    "gb18030",
    "GB18030",
    &["gb-18030"],
)
.expect("register GB18030 charset");
assert_eq!(Some(gb18030), Charset::from_label("gb_18030"));
```

`Charset::new_static` is an unchecked constructor for constant initialization.
It does not validate labels and does not register the descriptor globally. Use
`Charset::try_new` when static descriptor labels should be validated without
mutating the registry. Use `Charset::register` or `Charset::register_new` when
a custom descriptor should be discoverable through lookup APIs.
`Charset::registered()` returns a snapshot of runtime-registered descriptors;
built-ins remain available through `Charset::BUILTINS`. Registering the exact
same descriptor more than once is idempotent. Labels whose loose normalized
form is empty are rejected with `CharsetRegistrationErrorKind::InvalidLabel`.

`Charset::from_label` searches built-in and registered descriptors with loose
normalization: it trims ASCII whitespace, folds ASCII case, and ignores `-` and
`_` separators. `Charset::from_whatwg_label` uses WHATWG-style preprocessing:
it trims ASCII whitespace and folds ASCII case while preserving punctuation and
separators before matching identifiers, names, and aliases.
It is not the full WHATWG Encoding Standard label table and does not remap
charset semantics such as treating `iso-8859-1` as Windows-1252.

Built-in descriptors:

| Charset | Meaning |
| --- | --- |
| `Charset::ASCII` | US-ASCII bytes. |
| `Charset::ISO_8859_1` | ISO-8859-1 / Latin-1 bytes. |
| `Charset::UTF_8` | UTF-8 bytes. |
| `Charset::UTF_16` | Generic UTF-16 code-unit form or BOM-aware label. |
| `Charset::UTF_16LE`, `Charset::UTF_16BE` | Fixed-endian UTF-16 byte streams. |
| `Charset::UTF_32` | Generic UTF-32 code-unit form or BOM-aware label. |
| `Charset::UTF_32LE`, `Charset::UTF_32BE` | Fixed-endian UTF-32 byte streams. |

Use `Charset::from_utf16_byte_order`, `Charset::from_utf32_byte_order`, and
`Charset::byte_order` when you need to map between byte-order decisions and
charset labels.

## BOM and Byte Order

`UnicodeBom` detects supported Unicode byte order marks from the beginning of a
byte buffer.

```rust
use qubit_codec::ByteOrder;
use qubit_codec_text::{
    Charset,
    UnicodeBom,
};

let bom = UnicodeBom::detect(&[0xff, 0xfe, 0x00, 0x00]);
assert_eq!(Some(UnicodeBom::Utf32LittleEndian), bom);

let bom = bom.expect("BOM should be present");
assert_eq!(Charset::UTF_32LE, bom.charset());
assert_eq!(Some(ByteOrder::LittleEndian), bom.byte_order());
assert_eq!(&[0xff, 0xfe, 0x00, 0x00], bom.bytes());
```

UTF-32 BOMs are checked before UTF-16 BOMs because `FF FE 00 00` starts with
the UTF-16LE prefix `FF FE`. Streaming callers should buffer up to four bytes,
or read until EOF, before deciding that no longer BOM can be present.
Use `UnicodeBom::detect_progress(bytes, eof)` when the caller needs an explicit
`Pending` signal for stream prefixes such as `FF FE`.

The byte-oriented UTF-16 and UTF-32 codecs carry a `ByteOrder`, but they do not
detect, skip, or emit BOM bytes automatically. The caller owns BOM handling.
For closed byte input, `CharsetDecoder::<C>::detect_and_strip_bom(input)`
returns the detected BOM and the input slice after the BOM prefix. Streaming
callers should use `detect_and_strip_bom_progress(input, eof)` so ambiguous
prefixes such as `FF FE` remain unstripped while detection is still pending.

## Low-Level Codecs

The built-in text codec structs implement the domain-neutral
`qubit_codec::Codec` trait with `Value = char`. That trait is the lowest-level
single-value contract: `decode` decodes one Unicode scalar value from
caller-owned input units, and `encode` writes one Unicode scalar value
to caller-owned output units.

`CharsetCodec` stays at this same low-level layer. It adds only `charset()`
metadata; storage units, value-domain checks, exact encode sizing, and unsafe
single-value encode/decode are inherited from `Codec`. Charset encoders call
`Codec::can_encode_value()` before `Codec::encode_len()` and unsafe
`Codec::encode()`. For the same codec state and character, `encode_len()` must
equal the number of units that `encode()` writes under that precondition.

For decoding through `decode`, callers must provide at least
`Codec::MIN_UNITS_PER_VALUE` readable units from the current input index before
calling `decode`. Callers should normally provide up to
`Codec::MAX_UNITS_PER_VALUE.get()` unless EOF makes that impossible. Built-in codecs
decode complete shorter representations, such as one-byte UTF-8 ASCII, and
return `CharsetDecodeErrorKind::IncompleteSequence` / `MalformedSequence` for
incomplete or malformed prefixes. `CharsetDecoder` uses those errors to report
`NeedInput` for open buffered input.

| Codec | Storage unit | Charset |
| --- | --- | --- |
| `AsciiCodec` | `u8` | `Charset::ASCII` |
| `Latin1Codec` | `u8` | `Charset::ISO_8859_1` |
| `Utf8Codec` | `u8` | `Charset::UTF_8` |
| `Utf16U16Codec` | `u16` | `Charset::UTF_16` |
| `Utf16ByteCodec` | `u8` | `Charset::UTF_16LE` or `Charset::UTF_16BE` |
| `Utf32U32Codec` | `u32` | `Charset::UTF_32` |
| `Utf32ByteCodec` | `u8` | `Charset::UTF_32LE` or `Charset::UTF_32BE` |

Decode one scalar value from a closed or sufficiently buffered input slice:

```rust
use qubit_codec::Codec;
use qubit_codec_text::Utf8Codec;

let decoded = unsafe {
    Utf8Codec
        .decode("中".as_bytes(), 0)
}
    .expect("valid UTF-8 input");
assert_eq!(('中', 3), decoded);
```

Decode a closed EOF tail:

```rust
use qubit_codec::Codec;
use qubit_codec_text::{
    CharsetDecodeErrorKind,
    Utf8Codec,
};

let error = unsafe {
    Utf8Codec
        .decode(&[0xe4], 0)
}
    .expect_err("closed input ended inside a UTF-8 scalar value");

assert_eq!(
    CharsetDecodeErrorKind::IncompleteSequence {
        required: 3,
        available: 1,
    },
    error.kind(),
);
```

Encode one scalar value:

```rust
use qubit_codec::Codec;
use qubit_codec_text::{
    Utf8Codec,
    Utf8,
};

let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
assert!(Utf8Codec.can_encode_value(&'é'));
let required = Utf8Codec.encode_len(&'é').get();
let written = unsafe {
    Utf8Codec
        .encode(&'é', &mut output, 0)
}
    .expect("buffer is large enough");

assert_eq!(2, required);
assert_eq!("é".as_bytes(), &output[..written.get()]);
```

Low-level codecs are strict about malformed units and invalid decoded scalar
values. Buffer bounds and encode value-domain checks are part of the unsafe
caller contract: checked wrappers validate indices, capacity, and
`can_encode_value()` before entering hot `Codec` methods. Policy decisions are
handled by the wrappers described below.

## EOF and Incomplete Input

The low-level codec layer has only closed input: a short buffer is interpreted
as EOF, not as "maybe more data later". The streaming distinction belongs to
`CharsetDecoder`.

`CharsetDecoder::transcode` asks the codec whether the currently available units
already contain one complete scalar. Complete shorter representations are decoded
immediately. If the current chunk is only a valid incomplete prefix,
`transcode()` returns `TranscodeStatus::NeedInput` without consuming that tail.
The caller owns tail preservation, refill, and EOF policy. After the caller has
handled any incomplete tail, `finish()` only drains internally retained output.

Internally, `CharsetDecoder` stores malformed-input policy in decode hooks and
delegates to `TranscodeDecodeEngine<C, H>`. The engine owns repeated
`decode` calls, output-capacity progress, and status reporting, while
input-buffer refill stays with the caller.

## Policy Decoding

`CharsetDecoder<C>` converts source units to `char` values and applies
`MalformedAction`.

| Action | Behavior |
| --- | --- |
| `MalformedAction::Replace` | Emit the decoder replacement character. This is the default. |
| `MalformedAction::Ignore` | Skip the malformed range and emit nothing. |
| `MalformedAction::Report` | Return `CharsetDecodeError`. |

```rust
use qubit_codec::{
    TranscodeStatus,
    Transcoder,
};
use qubit_codec_text::{
    CharsetDecoder,
    Utf8Codec,
};

let mut decoder = CharsetDecoder::new(Utf8Codec);
let mut output = ['\0'; 2];

let progress = decoder
    .transcode("Aé".as_bytes(), 0, &mut output, 0)
    .expect("valid UTF-8 input");
assert_eq!(TranscodeStatus::Complete, progress.status());
assert_eq!(3, progress.read());
assert_eq!(2, progress.written());
assert_eq!(['A', 'é'], output);

```

For strict validation:

```rust
use qubit_codec::Transcoder;
use qubit_codec_text::{
    CharsetDecoder,
    CharsetDecodePolicy,
    Utf8Codec,
};

let mut decoder = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());

let mut output = ['\0'; 1];
let error = decoder
    .transcode(&[0x80], 0, &mut output, 0)
    .expect_err("invalid UTF-8");

assert_eq!(0, error.index());
```

If a codec reports malformed input whose consumed width is larger than the
currently visible input, `CharsetDecoder::transcode` returns `NeedInput` with
that required width instead of truncating and consuming the prefix as malformed.
For closed one-shot input, size a `char` buffer with `max_total_output_len` and
use `Transcoder::transcode_complete_into`. The `qubit-io-text` crate provides
`CharsetStringDecoder` when the desired output is a `String`.

```rust
use qubit_codec::Transcoder;
use qubit_codec_text::{
    CharsetDecoder,
    Utf8Codec,
};

let mut decoder = CharsetDecoder::new(Utf8Codec);
let mut output = vec!['\0'; decoder
    .max_total_output_len("A中".len())
    .expect("UTF-8 output bound")];
let written = decoder
    .transcode_complete_into("A中".as_bytes(), &mut output)
    .expect("complete UTF-8 input");
let text: String = output[..written].iter().collect();
assert_eq!("A中", text);
```

## Policy Encoding

`CharsetEncoder<C>` converts `char` values to target units and applies
`UnmappableAction`.

| Action | Behavior |
| --- | --- |
| `UnmappableAction::Replace` | Encode the configured replacement character. This is the default. |
| `UnmappableAction::Ignore` | Skip the input character and emit nothing. |
| `UnmappableAction::Report` | Return `CharsetEncodeError`. |

```rust
use qubit_codec::{
    TranscodeStatus,
    Transcoder,
};
use qubit_codec_text::{
    CharsetEncoder,
    Utf8Codec,
    Utf8,
};

let mut encoder = CharsetEncoder::new(Utf8Codec);
let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

let progress = encoder
    .transcode(&['😀'], 0, &mut output, 0)
    .expect("UTF-8 output buffer is large enough");

assert_eq!(TranscodeStatus::Complete, progress.status());
assert_eq!(1, progress.read());
assert_eq!(4, progress.written());
assert_eq!("😀".as_bytes(), &output[..progress.written()]);

```

For ASCII output with strict unmappable handling:

```rust
use qubit_codec::Transcoder;
use qubit_codec_text::{
    AsciiCodec,
    CharsetEncoder,
    CharsetEncodePolicy,
};

let mut encoder = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::report())
    .expect("report policy is constructible");

let mut output = [0_u8; 1];
let error = encoder.transcode(&['é'], 0, &mut output, 0).expect_err("not ASCII");

assert_eq!(0, error.index());
assert_eq!(Some('é' as u32), error.value());
```

`CharsetEncoder::new` validates the replacement character with
`can_encode_value()` and records its `encode_len()` width. It first tries
`U+FFFD`, then falls back to `?`. It panics only if the supplied codec cannot
encode either replacement. Built-in codecs do not trigger this; for a custom
codec, that panic indicates a broken codec invariant rather than recoverable
text input. Replacement output is still written through `encode()`, so
non-policy codec errors from that write are surfaced during transcoding.

Internally, `CharsetEncoder` stores unmappable-input policy in encode hooks and
delegates to `TranscodeEncodeEngine<C, H>`. The engine owns input iteration,
output capacity checks, and `TranscodeProgress` construction; the hooks supply
the charset-specific plan for original, replacement, or ignored characters.

Use `with_policy` to validate a custom replacement character up front:

```rust
use qubit_codec_text::{
    AsciiCodec,
    CharsetEncodePolicy,
    CharsetEncoder,
};

let encoder = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::replace('?'))
    .expect("ASCII replacement is encodable");
assert_eq!('?', encoder.replacement());
```

For closed string input, collect the scalar values and encode them with
`Transcoder::transcode_complete_into`. The `qubit-io-text` crate provides
`CharsetStringEncoder` for direct `&str` convenience methods.

```rust
use qubit_codec::Transcoder;
use qubit_codec_text::{
    AsciiCodec,
    CharsetEncoder,
};

let mut encoder = CharsetEncoder::new(AsciiCodec);
let input = "A中".chars().collect::<Vec<_>>();
let mut output = vec![0_u8; encoder
    .max_total_output_len(input.len())
    .expect("ASCII output bound")];
let written = encoder
    .transcode_complete_into(&input, &mut output)
    .expect("ASCII policy output");
output.truncate(written);
assert_eq!(b"A?", output.as_slice());
```

## Charset Conversion

`CharsetConverter<D, E>` combines one source decoder and one target encoder. It
uses `char` values as the intermediate representation.

```rust
use qubit_codec::{
    TranscodeStatus,
    Transcoder,
};
use qubit_codec_text::{
    CharsetConverter,
    Utf8Codec,
    Utf16U16Codec,
};

let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
let mut output = [0_u16; 2];

let progress = converter
    .transcode("A中".as_bytes(), 0, &mut output, 0)
    .expect("valid UTF-8 input and enough UTF-16 output");
assert_eq!(TranscodeStatus::Complete, progress.status());
assert_eq!(4, progress.read());
assert_eq!(2, progress.written());

let written = converter
    .finish(&mut output, progress.written())
    .expect("finish has no buffered tail");
assert_eq!(0, written);
assert_eq!(['A' as u16, '中' as u16], output);

```

Converters keep at most one pending decoded character when target output is
full. Call `transcode` again with more output space, or call `finish` after the
caller has handled any incomplete source tail to drain pending output.

When constructing a converter with explicit policies, the target replacement is
validated once and the resulting encode hooks are reused by the internal
`TranscodeConvertEngine`.

`CharsetConvertError` distinguishes source decode failures from target encode
failures:

```rust
use qubit_codec::Transcoder;
use qubit_codec_text::{
    CharsetConvertError,
    CharsetConverter,
    CharsetDecodePolicy,
    CharsetEncodePolicy,
    Utf8Codec,
    Utf16U16Codec,
};

let mut converter = CharsetConverter::from_codecs_with_policies(
    Utf8Codec,
    Utf16U16Codec,
    CharsetDecodePolicy::report(),
    CharsetEncodePolicy::default(),
)
.expect("UTF-16 replacement is encodable");
let mut output = [0_u16; 1];

let error = converter
    .transcode(&[0x80], 0, &mut output, 0)
    .expect_err("malformed UTF-8 source");

assert!(matches!(
    error,
    CharsetConvertError::Decode(_),
));
```

## Progress and Buffering

`qubit_codec::Transcoder<Input, Output>` models one
logical input stream converted into one logical output stream. Call
`transcode()` for each available segment, then call `finish()` after EOF and
continue while it reports `NeedOutput`. Call `reset()` before reusing the same
instance for another logical stream. It has four central methods:

| Method | Meaning |
| --- | --- |
| `max_transcode_output_len(input_len)` | Returns an upper bound when one is known. |
| `max_finish_output_len()` | Returns an upper bound for output produced by finalizing internal state. |
| `reset()` | Clears retained conversion state while keeping configuration. |
| `transcode(input, input_index, output, output_index)` | Converts as much as possible from caller-owned buffers. |
| `finish(output, output_index)` | Finalizes internal state after the caller has handled incomplete trailing input. |

`TranscodeProgress` contains:

- `status()`: `Complete`, `NeedInput`, or `NeedOutput`.
- `read()`: input units consumed relative to `input_index`.
- `written()`: output units produced relative to `output_index`.

`TranscodeStatus` values use absolute indices:

| Status | Meaning |
| --- | --- |
| `Complete` | The current call completed without needing more input or output. |
| `NeedInput { input_index, required, available }` | More source units are required at `input_index`. |
| `NeedOutput { output_index, required, available }` | More target units are required at `output_index`. |

When output is too small, policy wrappers return `NeedOutput` instead of
throwing an error for normal backpressure. When input is truncated but still a
valid prefix, decoders return `NeedInput` and leave the tail for the caller. If
the caller has reached EOF, it handles that tail before calling `finish()`.
Malformed input, invalid indices, and unmappable characters in report mode are
errors.

## Error Model

Decode errors carry the source charset, error kind, input unit index, and
optional raw value.

| Decode kind | Meaning |
| --- | --- |
| `MalformedSequence` | Units are present but invalid for the charset. |
| `IncompleteSequence` | Closed input ended before a full scalar value was available. |
| `InvalidCodePoint` | Decoded numeric value is not a Unicode scalar value. |
| `InvalidInputIndex` | Caller passed a source-unit index greater than input length. |
| `InvalidOutputIndex` | Caller passed a value index greater than output length. |
| `BufferTooSmall` | Output buffer cannot hold the decoded value or replacement. |
| `OutputLengthOverflow` | Output length arithmetic overflowed during planning. |

The facade maps transcode framework failures such as invalid indices,
insufficient output, and output-length overflow into charset error kinds.

Encode errors carry the target charset, error kind, index, and optional raw
value.

| Encode kind | Meaning |
| --- | --- |
| `InvalidCodePoint` | Codec was asked to encode a non-scalar code point. |
| `InvalidInputIndex` | Caller passed a character index greater than input length. |
| `InvalidOutputIndex` | Caller passed a target-unit index greater than output length. |
| `UnmappableCharacter` | Character cannot be represented by the target charset. |
| `BufferTooSmall` | Output buffer cannot hold the encoded value. |
| `OutputLengthOverflow` | Output length arithmetic overflowed during planning. |

Useful decode accessors include `charset()`, `kind()`, `index()`,
`required()`, `available()`, `value()`, and `consumed()`. Encode errors also
expose `input_len()` and `output_len()` for encode-side index failures.

## UTF-16 and UTF-32 Byte Codecs

Use `Utf16U16Codec` and `Utf32U32Codec` when your buffers are already code-unit
arrays. Use `Utf16ByteCodec` and `Utf32ByteCodec` when the data is serialized
as bytes.

```rust
use qubit_codec::{
    ByteOrder,
    Codec,
};
use qubit_codec_text::{
    Utf16ByteCodec,
};

let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
let mut output = [0_u8; 4];

assert!(codec.can_encode_value(&'😀'));
let required = codec.encode_len(&'😀').get();
let written = unsafe {
    codec
        .encode(&'😀', &mut output, 0)
}
    .expect("UTF-16 output buffer is large enough");

assert_eq!(4, required);
assert_eq!(&[0x3d, 0xd8, 0x00, 0xde], &output[..written.get()]);
```

The byte codecs read and write fixed-endian byte sequences directly. Public
callers usually interact with them through `CharsetCodec`, `CharsetEncoder`, or
`CharsetConverter`.

## Extending With Another Charset

To add another charset in a downstream crate:

1. Define a codec type.
2. Implement `qubit_codec::Codec` with `Value = char` for complete-value decode and
   encode.
3. Implement `CharsetCodec` for charset metadata.
4. Return a stable `Charset` descriptor from `charset()`.
5. Return the non-zero maximum storage units needed for one scalar value from
   the `Codec::MAX_UNITS_PER_VALUE` implementation.
6. Return incomplete, malformed, and invalid-scalar failures through
   `CharsetDecodeError` from `Codec::decode()`.
7. Override `Codec::can_encode_value()` when `Value = char` contains values the
   charset cannot represent.
8. Override `Codec::encode_len()` when different encodable values can require
   different output widths.
9. Use `CharsetDecoder`, `CharsetEncoder`, or `CharsetConverter` to apply
   policy.

Important `decode` expectations:

- Success returns a `NonZeroUsize` consumed-unit count.
- Success must not consume beyond `input.len() - index`.
- Callers using `decode` provide at least `Codec::MIN_UNITS_PER_VALUE`
  readable units and should normally provide up to `Codec::MAX_UNITS_PER_VALUE.get()`
  unless EOF prevents that.
- If the currently provided units are a valid but incomplete prefix, return
  `IncompleteSequence`; once the units prove the sequence invalid, return
  `MalformedSequence` or `InvalidCodePoint`.
- `index > input.len()` is a caller contract violation for the unsafe method.

Important `encode` and `encode_len` expectations:

- `can_encode_value(value)` must return `false` when the charset cannot
  represent `value`.
- Callers must only call `encode_len(value)` and unsafe `encode(value, ...)`
  after `can_encode_value(value)` returned `true`.
- Callers must provide enough output capacity for `encode_len(value)` units
  before entering unsafe `encode`.
- `encode_len` must return the exact number of units that `encode`
  will write for the same encodable value and codec state.
- Keep replacement `?` encodable if the codec is meant to work with
  `CharsetEncoder::new`.

## Development Commands

```bash
# Run tests
cargo test

# Align formatting and clippy with CI
./align-ci.sh

# Run the full local CI pipeline
RS_CI_SKIP_TOOLCHAIN_UPDATE=1 ./ci-check.sh

# Generate coverage
./coverage.sh text
```

The full CI pipeline checks formatting, clippy, style, debug/release builds,
tests, doctests, documentation, README dependency versions, coverage, and
security audit.
