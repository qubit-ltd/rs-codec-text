# Qubit Text Codec User Guide

[中文](user_guide.zh_CN.md) · [README](../README.md) · [API reference](https://docs.rs/qubit-codec-text)

This guide applies to `qubit-codec-text` 0.9. It is for developers building
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
qubit-codec-text = "0.9"
qubit-codec = "0.17"
```

Enable the optional `serde` feature only to serialize `Charset` values.

## Core Workflow

```rust
use qubit_codec::Transcoder;
use qubit_codec_text::{CharsetConverter, Utf16U16Codec, Utf8Codec};

let input = "A中".as_bytes();
let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
let capacity = converter.max_total_output_len(input.len()).expect("capacity bound");
let mut output = vec![0_u16; capacity];
let written = converter
    .transcode_complete_into(input, &mut output)
    .expect("complete UTF-8 input and sufficient output");

assert_eq!(2, written);
assert_eq!([0x0041, 0x4e2d], output[..written]);
```

`max_total_output_len` sums the codec's maximum reset, transcode, and finish
output bounds. The complete helper checks this conservative bound before reset;
it does not trial-write the message. This input writes two code units while the
required complete-call bound is eight. The capacity calculation can overflow,
so handle its `Result`. Use streaming calls and handle `NeedOutput` when output
must be limited to a small fixed window.

### Streaming input through EOF

During an open stream, `read()` and `written()` are increments from the input
and output indices passed to that call. `NeedInput` leaves an incomplete tail
unconsumed; retain it and append the next input chunk. On `NeedOutput`, drain
the produced output and advance by the reported progress before retrying. A
converter may also hold a pending character internally, so do not replay input
that was already reported as read.

After the caller knows that input has ended, pass the retained tail to
`transcode_eof`. Continue draining or enlarging output and calling
`transcode_eof` while it reports `NeedOutput`. Only after all EOF input has
been handled should the caller call `finish` for lifecycle output. `finish`
does not receive input and cannot process a saved incomplete tail.

The following complete UTF-8 to UTF-16 example uses three chunks and a
two-unit output window. It asserts that both `NeedInput` and `NeedOutput`
actually occur. These built-in codecs emit no reset or finish output, so empty
lifecycle buffers are sufficient here; custom codecs may need output space.
The `transcode_complete_into` helper runs reset and finish itself, so do not
reset before calling it.

```rust
use qubit_codec::TranscodeStatus;
use qubit_codec_text::CharsetConverter;
use qubit_codec_text::Utf16U16Codec;
use qubit_codec_text::Utf8Codec;

fn drain(
    converter: &mut CharsetConverter<Utf8Codec, Utf16U16Codec>,
    pending: &mut Vec<u8>,
    eof: bool,
    result: &mut Vec<u16>,
    observed: &mut (bool, bool),
) {
    loop {
        let mut output = [0_u16; 2];
        let progress = if eof {
            converter.transcode_eof(pending, 0, &mut output, 0)
        } else {
            converter.transcode(pending, 0, &mut output, 0)
        }.expect("default replacement policy");
        result.extend_from_slice(&output[..progress.written()]);
        pending.drain(..progress.read());
        match progress.status() {
            TranscodeStatus::Complete => break,
            TranscodeStatus::NeedInput { .. } => {
                assert!(!eof);
                observed.0 = true;
                break;
            }
            TranscodeStatus::NeedOutput { .. } => {
                observed.1 = true;
                assert!(progress.read() > 0 || progress.written() > 0);
            }
        }
    }
}

fn main() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    // These built-in codecs have no reset or finish output.
    assert_eq!(0, converter.reset(&mut [], 0).expect("reset"));
    let chunks: [&[u8]; 3] = [
        &[0x41, 0xe4],
        &[0xb8, 0xad, 0xf0, 0x9f, 0x98, 0x80],
        &[0xe2, 0x82],
    ];
    let mut pending = Vec::new();
    let mut result = Vec::new();
    let mut observed = (false, false);
    for chunk in chunks {
        pending.extend_from_slice(chunk);
        drain(&mut converter, &mut pending, false, &mut result, &mut observed);
    }
    drain(&mut converter, &mut pending, true, &mut result, &mut observed);
    assert!(pending.is_empty());
    assert_eq!(0, converter.finish(&mut [], 0).expect("finish"));
    assert!(observed.0, "the example must exercise NeedInput");
    assert!(observed.1, "the example must exercise NeedOutput");
    assert_eq!(vec![0x0041, 0x4e2d, 0xd83d, 0xde00, 0xfffd], result);
}
```

The first chunk leaves `E4` unread; the second completes `中`. One output slot
then remains, but `😀` needs a surrogate pair, so the call must report
`NeedOutput` and drain again. The third chunk leaves an incomplete tail, which
`transcode_eof` handles under the default `Replace` policy as U+FFFD. Each call
appends only written output and removes only the reported read prefix. If a
general call reports zero-progress `NeedOutput`, enlarge output to its
`required` value before retrying.

### Stream state and recovery

| Event or result | Next action |
| --- | --- |
| New decoder, encoder, or converter | Successfully call `reset` before streaming. |
| Open-stream `Complete` | The available input was consumed; more chunks may follow. |
| `NeedInput` | Keep the unread suffix and append the next chunk. |
| `NeedOutput` | Collect output, advance input by `read()`, then continue without replaying consumed input. |
| Confirmed EOF | Pass the retained suffix to `transcode_eof`; continue through `NeedOutput` until EOF input and pending output are handled. |
| Successful `finish` | The logical stream is closed; successfully reset before another stream. |
| Reset/finish index or capacity precheck fails | Correct the index or buffer and retry. |
| Reset/finish begins and then fails | State may have changed; successfully reset before reuse. The application owns replay or recovery of business data. |

Lifecycle failures have dedicated kinds: `TranscodeBeforeReset`,
`FinishBeforeReset`, `TranscodeAfterFinish`, `FinishAfterFinish`, and
`LifecyclePoisoned`. They describe call ordering or uncertain lifecycle state,
not malformed text. Other failures, such as malformed input, unmappable
characters, and capacity errors, follow their decode or encode policies. The
`transcode_complete_into` helpers run the full lifecycle themselves and need
no preceding reset. These wrapper kinds correspond to core
[`TranscodeFailure::TranscodeBeforeReset`](https://docs.rs/qubit-codec/0.17.0/qubit_codec/enum.TranscodeFailure.html#variant.TranscodeBeforeReset),
[`FinishBeforeReset`](https://docs.rs/qubit-codec/0.17.0/qubit_codec/enum.TranscodeFailure.html#variant.FinishBeforeReset),
[`TranscodeAfterFinish`](https://docs.rs/qubit-codec/0.17.0/qubit_codec/enum.TranscodeFailure.html#variant.TranscodeAfterFinish),
[`FinishAfterFinish`](https://docs.rs/qubit-codec/0.17.0/qubit_codec/enum.TranscodeFailure.html#variant.FinishAfterFinish), and
[`LifecyclePoisoned`](https://docs.rs/qubit-codec/0.17.0/qubit_codec/enum.TranscodeFailure.html#variant.LifecyclePoisoned).

This example shows a two-byte UTF-8 prefix becoming U+FFFD at EOF under the
default `Replace` policy:

```rust
use qubit_codec::TranscodeStatus;
use qubit_codec_text::{CharsetDecoder, Utf8Codec};

let mut decoder = CharsetDecoder::new(Utf8Codec);
let mut output = ['\0'; 1];
assert_eq!(0, decoder.reset(&mut output, 0).expect("reset"));
let pending = [0xe4, 0xb8];
let open = decoder.transcode(&pending, 0, &mut output, 0).expect("open stream");
assert!(matches!(open.status(), TranscodeStatus::NeedInput { .. }));
assert_eq!((0, 0), (open.read(), open.written()));
let eof = decoder.transcode_eof(&pending, 0, &mut output, 0).expect("EOF replacement");
assert_eq!(TranscodeStatus::Complete, eof.status());
assert_eq!((2, 1), (eof.read(), eof.written()));
assert_eq!('\u{fffd}', output[0]);
assert_eq!(0, decoder.finish(&mut output, 1).expect("finish"));
```

With `Ignore`, EOF consumes the tail without output; with `Report`, it returns
an error. A `NeedOutput` result is normal backpressure (`Ok`), not an error.
An open-stream `Complete` means the current input was consumed, not that the
upstream has reached EOF.

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

### Selecting a replacement for ASCII

An automatically constructed ASCII encoder chooses `?` as its replacement.
`CharsetEncodePolicy::default()` always selects U+FFFD, which ASCII cannot
encode, so passing it explicitly is rejected when constructing the encoder.
Use `default_for(&codec)` when the replacement should match the target, or
`replace('?')` when a protocol requires a fixed replacement. Use `report()`
when unmappable characters must fail:

```rust
use qubit_codec_text::{
    AsciiCodec, CharsetEncodeErrorKind, CharsetEncodePolicy, CharsetEncoder,
};

let automatic = CharsetEncoder::new(AsciiCodec);
assert_eq!('?', automatic.replacement());

let error = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::default())
    .expect_err("ASCII cannot encode U+FFFD");
assert_eq!(
    CharsetEncodeErrorKind::UnmappableCharacter { value: 0xfffd },
    error.kind(),
);

let input = ['A', '中', 'B'];
let policy = CharsetEncodePolicy::default_for(&AsciiCodec).expect("ASCII fallback");
let mut encoder = CharsetEncoder::with_policy(AsciiCodec, policy).expect("valid policy");
let mut output = [0_u8; 3];
let written = encoder.transcode_complete_into(&input, &mut output).expect("replace");
assert_eq!(b"A?B", &output[..written]);

let mut explicit = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::replace('?'))
    .expect("ASCII replacement");
let written = explicit.transcode_complete_into(&input, &mut output).expect("replace");
assert_eq!(b"A?B", &output[..written]);

let mut strict = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::report())
    .expect("report does not encode replacement metadata");
let mut staged = [0_u8; 3];
let raw_error = strict.transcode_complete_into(&input, &mut staged)
    .expect_err("ASCII rejects 中");
let error = strict.map_transcode_error(raw_error);
assert_eq!(
    CharsetEncodeErrorKind::UnmappableCharacter { value: '中' as u32 },
    error.kind(),
);
assert_eq!(b'A', staged[0]);
```

For `A中B`, both target-aware default replacement and the explicit protocol
replacement produce `A?B`. With `Report`, the staging buffer may already
contain `A` when conversion fails. Discard the failed message's staged output;
do not publish that prefix or replay the input from the mapped diagnostic.
Passing an explicit policy does not automatically call `default_for`. When a
policy may be invalid, use a constructor that returns configuration errors;
for example, `CharsetTextWriter::try_new` in `rs-io-text` can return that
error.

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
For compatibility, converter lifecycle failures are routed through
`CharsetConvertError::Decode`; this does not mean the source text is malformed.
Target output failures remain `Encode`.

In streaming code, `TranscodeProgress` describes how many units were read and
written and reports a status such as `NeedInput` or output backpressure. Do not
treat `NeedInput` as a malformed error: preserve the tail and retry it with
later input. At EOF, a remaining incomplete sequence is handled by the
configured policy: replacement, discard, or an error.

`NeedInput` and `NeedOutput` are successful progress states. An `Err` is not a
transaction boundary: it does not roll back output already written or codec
state, and it does not provide complete read/written progress. Do not replay the
original input range unless an explicit recovery protocol permits it. Reset
starts a new logical stream, but it cannot undo output already delivered to an
application. Decoder EOF processing and the complete convenience methods can
also fail after writing an output prefix.

When mapping a transcode error to `CharsetDecodeError`, `CharsetEncodeError`, or
`CharsetConvertError`, domain source data such as charset, kind, and its own
index or consumed count is retained. The mapping discards the outer phase and
the outer `input_index` and `input_consumed`; source-owned index or consumption
values may differ from those outer fields. Inspect the original error before
mapping if the outer context is needed, and do not infer a resume position from
the mapped error. Converter lifecycle failures continue to map to
`CharsetConvertError::Decode` for compatibility; that does not indicate
malformed source text.

For an all-or-nothing message boundary, convert into a temporary buffer owned
by that message and publish it only after the complete conversion succeeds. If
conversion fails, discard that message's staged output; do not replay its input
into a live sink based only on the mapped diagnostic. After successful reset,
process a new message as a new logical stream. For example, an ASCII encoder
using `Report` can write `A` before returning an error for `中`; that prefix is
observable in the caller's staging buffer even though the call failed.

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
