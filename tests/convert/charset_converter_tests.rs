// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use std::cell::Cell;
use std::rc::Rc;

use proptest::arbitrary::any;
use proptest::proptest;
use proptest::test_runner::Config as ProptestConfig;
use qubit_codec::ByteOrder;
use qubit_codec::CapacityError;
use qubit_codec::Codec;
use qubit_codec::DecodeFailure;
use qubit_codec::TranscodeConvertErrorOf;
use qubit_codec::TranscodeConverter;
use qubit_codec::TranscodeFailure;
use qubit_codec::TranscodeProgress;
use qubit_codec::TranscodeStatus;
use qubit_codec::Transcoder;
use qubit_codec_text::AsciiCodec;
use qubit_codec_text::Charset;
use qubit_codec_text::CharsetCodec;
use qubit_codec_text::CharsetConvertError;
use qubit_codec_text::CharsetConverter;
use qubit_codec_text::CharsetDecodeError;
use qubit_codec_text::CharsetDecodeErrorKind;
use qubit_codec_text::CharsetDecodePolicy;
use qubit_codec_text::CharsetEncodeError;
use qubit_codec_text::CharsetEncodeErrorKind;
use qubit_codec_text::CharsetEncodePolicy;
use qubit_codec_text::CharsetEncodeResult;
use qubit_codec_text::CharsetEncoder;
use qubit_codec_text::Latin1Codec;
use qubit_codec_text::MalformedAction;
use qubit_codec_text::UnmappableAction;
use qubit_codec_text::Utf8Codec;
use qubit_codec_text::Utf16ByteCodec;
use qubit_codec_text::Utf16U16Codec;
use qubit_codec_text::Utf32ByteCodec;

fn reset_for_test<T: Transcoder>(transcoder: &mut T) {
    let mut output: [T::Output; 0] = [];
    assert!(transcoder.reset(&mut output, 0).is_ok());
}

#[test]
fn test_converter_encode_error_index_uses_source_units() {
    let mut converter = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        Latin1Codec,
        CharsetDecodePolicy::report(),
        CharsetEncodePolicy::report(),
    )
    .expect("valid report policies");
    let input = "é中".as_bytes();
    let capacity = converter.max_total_output_len(input.len()).expect("output bound");
    let mut output = vec![0_u8; capacity];
    let raw = converter
        .transcode_complete_into(input, &mut output)
        .expect_err("Latin-1 rejects 中");
    let CharsetConvertError::Encode(error) = converter.map_transcode_error(raw) else {
        panic!("expected target encoding error");
    };
    assert_eq!(Charset::ISO_8859_1, error.charset());
    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter { value: '中' as u32 },
        error.kind(),
    );
    assert_eq!(2, error.index(), "converter input index counts UTF-8 bytes");
    assert_eq!(0xe9, output[0], "successful prefix remains observable");

    let mut encoder =
        CharsetEncoder::with_policy(Latin1Codec, CharsetEncodePolicy::report()).expect("valid report policy");
    let chars = ['é', '中'];
    let capacity = encoder.max_total_output_len(chars.len()).expect("output bound");
    let mut encoded = vec![0_u8; capacity];
    let raw = encoder
        .transcode_complete_into(&chars, &mut encoded)
        .expect_err("Latin-1 rejects 中");
    let error = encoder.map_transcode_error(raw);
    assert_eq!(1, error.index(), "encoder input index counts char entries");
    assert_eq!(Charset::ISO_8859_1, error.charset());
    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter { value: '中' as u32 },
        error.kind(),
    );
}

#[test]
fn test_converter_encode_error_index_preserves_nonzero_source_offset() {
    let mut converter = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        Latin1Codec,
        CharsetDecodePolicy::report(),
        CharsetEncodePolicy::report(),
    )
    .expect("valid report policies");
    reset_for_test(&mut converter);
    let mut output = [0_u8; 8];
    let raw = converter
        .transcode("!é中".as_bytes(), 1, &mut output, 0)
        .expect_err("Latin-1 rejects 中 after a two-byte prefix");
    let CharsetConvertError::Encode(error) = converter.map_transcode_error(raw) else {
        panic!("expected target encoding error");
    };
    assert_eq!(Charset::ISO_8859_1, error.charset());
    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter { value: '中' as u32 },
        error.kind(),
    );
    assert_eq!(3, error.index(), "index is absolute within the passed source slice");
    assert_eq!(0xe9, output[0]);
}

fn replacement_interaction_converter(policy: CharsetEncodePolicy) -> CharsetConverter<Utf8Codec, AsciiCodec> {
    CharsetConverter::from_codecs_with_policies(Utf8Codec, AsciiCodec, CharsetDecodePolicy::replace('\u{fffd}'), policy)
        .expect("ASCII policy is valid")
}

fn assert_replacement_report(
    converter: &CharsetConverter<Utf8Codec, AsciiCodec>,
    raw: TranscodeConvertErrorOf<Utf8Codec, AsciiCodec>,
) {
    match converter.map_transcode_error(raw) {
        CharsetConvertError::Encode(error) => {
            assert_eq!(Charset::ASCII, error.charset());
            assert_eq!(
                CharsetEncodeErrorKind::UnmappableCharacter { value: 0xfffd },
                error.kind(),
            );
        }
        other => panic!("expected target encode error, got {other:?}"),
    }
}

#[test]
fn test_replacement_policy_interaction_complete() {
    let cases: &[(&[u8], &[u8], &[u8])] = &[(b"A\xffB", b"AB", b"A?B"), (b"A\xe4\xb8", b"A", b"A?")];
    for &(input, ignored, replaced) in cases {
        for policy in [CharsetEncodePolicy::ignore(), CharsetEncodePolicy::replace('?')] {
            let expected = if policy == CharsetEncodePolicy::ignore() {
                ignored
            } else {
                replaced
            };
            let mut converter = replacement_interaction_converter(policy);
            let capacity = converter.max_total_output_len(input.len()).expect("output bound");
            let mut output = vec![0; capacity];
            let written = converter
                .transcode_complete_into(input, &mut output)
                .expect("conversion");
            assert_eq!(expected, &output[..written]);
        }
        let mut converter = replacement_interaction_converter(CharsetEncodePolicy::report());
        let capacity = converter.max_total_output_len(input.len()).expect("output bound");
        let mut output = vec![0; capacity];
        let raw = converter
            .transcode_complete_into(input, &mut output)
            .expect_err("report rejects replacement");
        assert_eq!(b"A", &output[..1]);
        assert_replacement_report(&converter, raw);
    }
}

#[test]
fn test_replacement_policy_interaction_chunked() {
    for (input, ignored, replaced) in [
        (b"A\xffB".as_slice(), b"AB".as_slice(), b"A?B".as_slice()),
        (b"A\xe4\xb8", b"A".as_slice(), b"A?".as_slice()),
    ] {
        for policy in [
            CharsetEncodePolicy::ignore(),
            CharsetEncodePolicy::replace('?'),
            CharsetEncodePolicy::report(),
        ] {
            for windows in [&[1][..], &[0, 1][..]] {
                let mut converter = replacement_interaction_converter(policy);
                reset_for_test(&mut converter);
                let mut pending = Vec::new();
                let mut output = Vec::new();
                let mut added = 0;
                let mut eof = false;
                let mut min_window = 0;
                let mut window_cursor = 0;
                let mut steps = 0;
                let mut reported = false;
                loop {
                    steps += 1;
                    assert!(steps <= 128 + 64 * input.len(), "stream did not converge");
                    if pending.is_empty() && added < input.len() {
                        pending.push(input[added]);
                        added += 1;
                    } else if added == input.len() {
                        eof = true;
                    }
                    let size = min_window.max(windows[window_cursor % windows.len()]);
                    window_cursor += 1;
                    min_window = 0;
                    let mut window = vec![0; size];
                    let result = if eof {
                        converter.transcode_eof(&pending, 0, &mut window, 0)
                    } else {
                        converter.transcode(&pending, 0, &mut window, 0)
                    };
                    let progress = match result {
                        Ok(progress) => progress,
                        Err(raw) => {
                            assert_eq!(CharsetEncodePolicy::report(), policy);
                            assert_eq!(b"A", output.as_slice());
                            assert_replacement_report(&converter, raw);
                            reported = true;
                            break;
                        }
                    };
                    output.extend_from_slice(&window[..progress.written()]);
                    pending.drain(..progress.read());
                    match progress.status() {
                        TranscodeStatus::Complete if eof => break,
                        TranscodeStatus::NeedOutput { required } => min_window = required.get(),
                        TranscodeStatus::NeedInput { .. } => {
                            if added < input.len() {
                                pending.push(input[added]);
                                added += 1;
                            }
                        }
                        TranscodeStatus::Complete => {}
                    }
                }
                if policy == CharsetEncodePolicy::report() {
                    assert!(reported);
                } else {
                    assert_eq!(
                        if policy == CharsetEncodePolicy::ignore() {
                            ignored
                        } else {
                            replaced
                        },
                        output
                    );
                    assert_eq!(0, converter.finish(&mut [], 0).expect("finish"));
                }
            }
        }
    }
}

#[test]
fn test_replacement_policy_interaction_pending_retry() {
    let mut converter = replacement_interaction_converter(CharsetEncodePolicy::replace('?'));
    reset_for_test(&mut converter);
    let mut initial = [0; 1];
    let first = converter.transcode(b"A", 0, &mut initial, 0).expect("write prefix");
    assert_eq!(b"A", &initial[..first.written()]);
    assert_eq!(TranscodeStatus::Complete, first.status());
    let mut empty = [];
    let pending = converter
        .transcode(b"\xff", 0, &mut empty, 0)
        .expect("replacement waits for output");
    assert!(matches!(pending.status(), TranscodeStatus::NeedOutput { .. }));
    let mut retry = [0; 1];
    let remaining = &b"\xff"[pending.read()..];
    let resumed = converter
        .transcode(remaining, 0, &mut retry, 0)
        .expect("retry replacement");
    assert_eq!(b"?", &retry[..resumed.written()]);
    assert_eq!(remaining.len(), resumed.read());
    let mut tail = [0; 1];
    let last = converter.transcode(b"B", 0, &mut tail, 0).expect("convert tail");
    assert_eq!(b"B", &tail[..last.written()]);
    assert_eq!(1, last.read());
    assert_eq!(0, converter.finish(&mut [], 0).expect("finish"));

    let mut eof_converter = replacement_interaction_converter(CharsetEncodePolicy::replace('?'));
    reset_for_test(&mut eof_converter);
    let mut prefix = [0; 1];
    let written = eof_converter
        .transcode(b"A", 0, &mut prefix, 0)
        .expect("write EOF prefix");
    assert_eq!(b"A", &prefix[..written.written()]);
    let mut no_output = [];
    let blocked = eof_converter
        .transcode_eof(b"\xe4\xb8", 0, &mut no_output, 0)
        .expect("EOF replacement waits");
    assert!(matches!(blocked.status(), TranscodeStatus::NeedOutput { .. }));
    let mut replacement = [0; 1];
    let eof_input = b"\xe4\xb8";
    let remaining = &eof_input[blocked.read()..];
    let retried = eof_converter
        .transcode_eof(remaining, 0, &mut replacement, 0)
        .expect("retry EOF replacement");
    assert_eq!(b"?", &replacement[..retried.written()]);
    assert_eq!(remaining.len(), retried.read());
    let mut eof_tail = [0; 1];
    let completed = eof_converter
        .transcode_eof(&[], 0, &mut eof_tail, 0)
        .expect("complete EOF");
    assert_eq!(0, completed.written());
    assert_eq!(TranscodeStatus::Complete, completed.status());
    assert_eq!(0, eof_converter.finish(&mut [], 0).expect("finish EOF converter"));
}

fn assert_chunked_matches_complete<C, F>(
    make_codec: F,
    context: &str,
    input: &[u8],
    chunks: &[usize],
    windows: &[usize],
) where
    C: CharsetCodec<Unit = u8>,
    F: Fn() -> C,
{
    let mut baseline_converter = CharsetConverter::from_codecs(make_codec(), Utf16U16Codec);
    let capacity = baseline_converter
        .max_total_output_len(input.len())
        .expect("complete output capacity");
    let mut baseline = vec![0_u16; capacity];
    let baseline_len = baseline_converter
        .transcode_complete_into(input, &mut baseline)
        .expect("complete conversion");
    baseline.truncate(baseline_len);

    let mut converter = CharsetConverter::from_codecs(make_codec(), Utf16U16Codec);
    assert_eq!(0, converter.reset(&mut [], 0).expect("reset"));
    let mut pending = Vec::new();
    let mut result = Vec::new();
    let mut added = 0;
    let mut consumed = 0;
    let mut feed = true;
    let mut eof = false;
    let mut minimum_window = 0;
    let mut chunk_cursor = 0;
    let mut window_cursor = 0;
    let mut steps = 0;
    let max_steps = 128_usize.saturating_add(64_usize.saturating_mul(input.len().saturating_add(chunks.len())));
    loop {
        steps += 1;
        assert!(steps <= max_steps, "stream did not converge");
        if feed {
            if added == input.len() {
                eof = true;
            } else {
                let requested = *chunks.get(chunk_cursor).expect("a chunk covers remaining input");
                assert!(requested > 0, "input chunks must be positive");
                let count = requested.min(input.len() - added);
                pending.extend_from_slice(&input[added..added + count]);
                added += count;
                chunk_cursor += 1;
            }
            feed = false;
        }
        let requested_window = if windows.is_empty() {
            1
        } else {
            windows[window_cursor % windows.len()]
        };
        window_cursor += 1;
        let window = requested_window.max(minimum_window);
        minimum_window = 0;
        let mut output = vec![0_u16; window];
        let progress = if eof {
            converter.transcode_eof(&pending, 0, &mut output, 0)
        } else {
            converter.transcode(&pending, 0, &mut output, 0)
        }
        .expect("Replace accepts arbitrary byte input");
        assert!(progress.read() <= pending.len());
        assert!(progress.written() <= output.len());
        consumed += progress.read();
        result.extend_from_slice(&output[..progress.written()]);
        pending.drain(..progress.read());
        match progress.status() {
            TranscodeStatus::Complete => {
                assert!(pending.is_empty());
                if eof {
                    break;
                }
                feed = true;
            }
            TranscodeStatus::NeedInput { .. } => {
                assert!(!eof, "EOF must resolve incomplete input");
                feed = true;
            }
            TranscodeStatus::NeedOutput { required } => {
                if progress.read() == 0 && progress.written() == 0 {
                    minimum_window = required.get();
                }
            }
        }
    }
    assert_eq!(input.len(), added);
    assert_eq!(input.len(), consumed);
    assert!(pending.is_empty());
    assert_eq!(0, converter.finish(&mut [], 0).expect("finish"));
    assert_eq!(
        baseline,
        result,
        "chunked output differs from complete output: case={context}, codec={}, input={input:02x?}, chunks={chunks:?}, windows={windows:?}",
        core::any::type_name::<C>(),
    );
}

fn run_chunked_backpressure(input: &[u8], controls: &[u8]) -> Vec<u16> {
    let mut added = 0;
    let mut chunks = Vec::new();
    while added < input.len() {
        let control = if controls.is_empty() {
            0
        } else {
            controls[chunks.len() % controls.len()] as usize
        };
        let count = (control % 7 + 1).min(input.len() - added);
        chunks.push(count);
        added += count;
    }
    let windows: Vec<usize> = if controls.is_empty() {
        vec![1]
    } else {
        controls.iter().map(|value| *value as usize % 3).collect()
    };
    run_chunked_backpressure_result(input, &chunks, &windows)
}

fn run_chunked_backpressure_result(input: &[u8], chunks: &[usize], windows: &[usize]) -> Vec<u16> {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let capacity = converter
        .max_total_output_len(input.len())
        .expect("complete output capacity");
    let mut baseline = vec![0_u16; capacity];
    let length = converter
        .transcode_complete_into(input, &mut baseline)
        .expect("complete conversion");
    baseline.truncate(length);
    assert_chunked_matches_complete(|| Utf8Codec, "Utf8Codec", input, chunks, windows);
    baseline
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct AsciiBytesCodec;

impl CharsetCodec for AsciiBytesCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl Codec for AsciiBytesCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = 1;

    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;

    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;

    fn can_encode_value(&self, value: &char) -> bool {
        value.is_ascii()
    }

    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> Result<(char, core::num::NonZeroUsize), DecodeFailure<CharsetDecodeError>> {
        if input_index >= input.len() {
            let kind = CharsetDecodeErrorKind::IncompleteSequence {
                required: 1,
                available: input.len().saturating_sub(input_index),
            };
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, input_index).into_codec_failure());
        }
        let value = input[input_index];
        if value > 0x7f {
            let kind = CharsetDecodeErrorKind::malformed(value as u32);
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, input_index).into_codec_failure());
        }
        Ok((value as char, core::num::NonZeroUsize::MIN))
    }

    unsafe fn encode(&mut self, value: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
        debug_assert!(self.can_encode_value(value));
        debug_assert!(output_index < output.len());
        unsafe {
            // SAFETY: The caller guarantees that `output_index` is writable.
            *output.as_mut_ptr().add(output_index) = *value as u8;
        }
        Ok(1)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct NonCloneUnit(u8);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct NonCloneUnitCodec;

impl CharsetCodec for NonCloneUnitCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl Codec for NonCloneUnitCodec {
    type Value = char;
    type Unit = NonCloneUnit;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = 1;
    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;
    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;

    fn can_encode_value(&self, value: &char) -> bool {
        value.is_ascii()
    }

    unsafe fn decode(
        &mut self,
        input: &[NonCloneUnit],
        input_index: usize,
    ) -> Result<(char, core::num::NonZeroUsize), DecodeFailure<CharsetDecodeError>> {
        let value = input[input_index].0;
        if value.is_ascii() {
            Ok((value as char, core::num::NonZeroUsize::MIN))
        } else {
            Err(CharsetDecodeError::new(
                Charset::ASCII,
                CharsetDecodeErrorKind::malformed(value as u32),
                input_index,
            )
            .into_codec_failure())
        }
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        output: &mut [NonCloneUnit],
        output_index: usize,
    ) -> CharsetEncodeResult<usize> {
        output[output_index] = NonCloneUnit(*value as u8);
        Ok(1)
    }
}

#[derive(Clone, Debug)]
struct CountingEncodeProbeCodec {
    calls: Rc<Cell<usize>>,
}

impl CountingEncodeProbeCodec {
    fn new(calls: Rc<Cell<usize>>) -> Self {
        Self { calls }
    }
}

impl CharsetCodec for CountingEncodeProbeCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl Codec for CountingEncodeProbeCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = <AsciiBytesCodec as Codec>::MIN_UNITS_PER_VALUE;

    const MAX_ENCODE_UNITS_PER_VALUE: usize = <AsciiBytesCodec as Codec>::MAX_ENCODE_UNITS_PER_VALUE;

    const MAX_DECODE_UNITS_PER_VALUE: usize = <AsciiBytesCodec as Codec>::MAX_DECODE_UNITS_PER_VALUE;

    fn can_encode_value(&self, value: &char) -> bool {
        self.calls.set(self.calls.get() + 1);
        AsciiBytesCodec.can_encode_value(value)
    }

    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> Result<(char, core::num::NonZeroUsize), DecodeFailure<CharsetDecodeError>> {
        unsafe { AsciiBytesCodec.decode(input, input_index) }
    }

    unsafe fn encode(&mut self, value: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
        unsafe { AsciiBytesCodec.encode(value, output, output_index) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NonDefaultUnit(u8);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NonDefaultUnitCodec;

impl CharsetCodec for NonDefaultUnitCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl Codec for NonDefaultUnitCodec {
    type Value = char;
    type Unit = NonDefaultUnit;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = 1;

    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;

    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;

    fn can_encode_value(&self, value: &char) -> bool {
        value.is_ascii()
    }

    unsafe fn decode(
        &mut self,
        _input: &[NonDefaultUnit],
        input_index: usize,
    ) -> Result<(char, core::num::NonZeroUsize), DecodeFailure<CharsetDecodeError>> {
        let kind = CharsetDecodeErrorKind::malformed_unknown();
        Err(CharsetDecodeError::new(Charset::ASCII, kind, input_index).into_codec_failure())
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        output: &mut [NonDefaultUnit],
        output_index: usize,
    ) -> CharsetEncodeResult<usize> {
        debug_assert!(self.can_encode_value(value));
        debug_assert!(output_index < output.len());
        unsafe {
            // SAFETY: The caller guarantees that `output_index` is writable.
            *output.as_mut_ptr().add(output_index) = NonDefaultUnit(*value as u8);
        }
        Ok(1)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ReplacementFallbackCodec;

impl CharsetCodec for ReplacementFallbackCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct RejectingEncodeCodec;

impl CharsetCodec for RejectingEncodeCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct FailingLifecycleCodec<const MODE: u8>;

impl<const MODE: u8> CharsetCodec for FailingLifecycleCodec<MODE> {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl<const MODE: u8> Codec for FailingLifecycleCodec<MODE> {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = <AsciiBytesCodec as Codec>::MIN_UNITS_PER_VALUE;

    const MAX_ENCODE_UNITS_PER_VALUE: usize = <AsciiBytesCodec as Codec>::MAX_ENCODE_UNITS_PER_VALUE;

    const MAX_DECODE_UNITS_PER_VALUE: usize = <AsciiBytesCodec as Codec>::MAX_DECODE_UNITS_PER_VALUE;

    fn can_encode_value(&self, value: &char) -> bool {
        AsciiBytesCodec.can_encode_value(value)
    }

    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> Result<(char, core::num::NonZeroUsize), DecodeFailure<CharsetDecodeError>> {
        unsafe { AsciiBytesCodec.decode(input, input_index) }
    }

    unsafe fn encode(&mut self, value: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
        unsafe { AsciiBytesCodec.encode(value, output, output_index) }
    }

    unsafe fn decode_reset(&mut self, _output: &mut [char], output_index: usize) -> Result<usize, CharsetDecodeError> {
        if MODE == 0 {
            return Err(decode_lifecycle_error(output_index));
        }
        Ok(0)
    }

    unsafe fn decode_finish(&mut self, _output: &mut [char], output_index: usize) -> Result<usize, CharsetDecodeError> {
        if MODE == 1 {
            return Err(decode_lifecycle_error(output_index));
        }
        Ok(0)
    }

    unsafe fn encode_reset(&mut self, _output: &mut [u8], output_index: usize) -> Result<usize, CharsetEncodeError> {
        if MODE == 2 {
            return Err(encode_lifecycle_error(output_index));
        }
        Ok(0)
    }

    unsafe fn encode_finish(&mut self, _output: &mut [u8], output_index: usize) -> Result<usize, CharsetEncodeError> {
        if MODE == 3 {
            return Err(encode_lifecycle_error(output_index));
        }
        Ok(0)
    }
}

fn decode_lifecycle_error(index: usize) -> CharsetDecodeError {
    CharsetDecodeError::new(Charset::ASCII, CharsetDecodeErrorKind::malformed_unknown(), index)
}

fn encode_lifecycle_error(index: usize) -> CharsetEncodeError {
    CharsetEncodeError::new(
        Charset::ASCII,
        CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 0 },
        index,
    )
}

impl Codec for RejectingEncodeCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = 1;

    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;

    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;

    fn can_encode_value(&self, _value: &char) -> bool {
        false
    }

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        input_index: usize,
    ) -> Result<(char, core::num::NonZeroUsize), DecodeFailure<CharsetDecodeError>> {
        let kind = CharsetDecodeErrorKind::malformed_unknown();
        Err(CharsetDecodeError::new(Charset::ASCII, kind, input_index).into_codec_failure())
    }

    unsafe fn encode(&mut self, value: &char, _output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
        let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: *value as u32 };
        Err(CharsetEncodeError::new(Charset::ASCII, kind, output_index))
    }
}

impl Codec for ReplacementFallbackCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = 1;

    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;

    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;

    fn can_encode_value(&self, value: &char) -> bool {
        *value == '?' || value.is_ascii()
    }

    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> Result<(char, core::num::NonZeroUsize), DecodeFailure<CharsetDecodeError>> {
        unsafe { AsciiBytesCodec.decode(input, input_index) }
    }

    unsafe fn encode(&mut self, value: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
        debug_assert!(self.can_encode_value(value));
        debug_assert!(output_index < output.len());
        unsafe {
            // SAFETY: The caller guarantees that `output_index` is writable.
            *output.as_mut_ptr().add(output_index) = *value as u8;
        }
        Ok(1)
    }
}

#[test]
fn test_charset_converter_is_transcode_converter() {
    fn assert_transcode_converter<T: TranscodeConverter<Input = u8, Output = u16>>() {}

    assert_transcode_converter::<CharsetConverter<Utf8Codec, Utf16U16Codec>>();
}

#[test]
fn test_charset_converter_supports_non_clone_target_units() {
    fn assert_transcode_converter<T>()
    where
        T: TranscodeConverter<Input = u8, Output = NonCloneUnit>,
    {
    }

    assert_transcode_converter::<CharsetConverter<AsciiBytesCodec, NonCloneUnitCodec>>();

    let mut converter = CharsetConverter::from_codecs(AsciiBytesCodec, NonCloneUnitCodec);
    reset_for_test(&mut converter);
    let mut output = [NonCloneUnit(0)];
    let progress = converter
        .transcode(b"A", 0, &mut output, 0)
        .expect("non-Clone target units should be supported");

    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(b'A', output[0].0);
}

#[test]
fn test_charset_converter_transcoder_trait_methods_forward() {
    type Converter = CharsetConverter<Utf8Codec, Utf16U16Codec>;
    type ConverterResult<T> = Result<T, TranscodeConvertErrorOf<Utf8Codec, Utf16U16Codec>>;
    type TranscodeFn = fn(&mut Converter, &[u8], usize, &mut [u16], usize) -> ConverterResult<TranscodeProgress>;
    type OutputFn = fn(&mut Converter, &mut [u16], usize) -> ConverterResult<usize>;

    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let mut output = [0_u16; 1];
    let max_transcode_output_len: fn(&Converter, usize) -> Result<usize, CapacityError> =
        std::hint::black_box(<Converter as Transcoder>::max_transcode_output_len);
    let max_finish_output_len: fn(&Converter) -> Result<usize, CapacityError> =
        std::hint::black_box(<Converter as Transcoder>::max_finish_output_len);
    let max_reset_output_len: fn(&Converter) -> Result<usize, CapacityError> =
        std::hint::black_box(<Converter as Transcoder>::max_reset_output_len);
    let reset: OutputFn = std::hint::black_box(<Converter as Transcoder>::reset);
    let transcode: TranscodeFn = std::hint::black_box(<Converter as Transcoder>::transcode);
    let finish: OutputFn = std::hint::black_box(<Converter as Transcoder>::finish);

    assert_eq!(Ok(4), max_transcode_output_len(&converter, 1));
    assert_eq!(Ok(2), max_finish_output_len(&converter));
    assert_eq!(Ok(0), max_reset_output_len(&converter));
    assert_eq!(Ok(0), reset(&mut converter, &mut [], 0));
    let progress =
        transcode(&mut converter, b"A", 0, &mut output, 0).expect("converter should transcode through the trait");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(['A' as u16], output);
    assert_eq!(Ok(0), finish(&mut converter, &mut [], 0));
}

#[test]
fn test_charset_converter_complete_into_maps_framework_errors() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let output_len = converter
        .max_total_output_len(2)
        .expect("complete conversion bound should fit usize");
    let mut output = vec![0_u16; output_len];

    let written = converter
        .transcode_complete_into(b"AB", &mut output)
        .expect("complete conversion should write all characters");

    assert_eq!(2, written);
    assert_eq!(['A' as u16, 'B' as u16], output[..written]);

    let mut short_output = [0_u16; 2];
    let raw_error = converter
        .transcode_complete_into(b"AB", &mut short_output)
        .expect_err("complete conversion maps insufficient output");
    let error = converter.map_transcode_error(raw_error);

    match error {
        CharsetConvertError::Encode(error) => {
            assert_eq!(Charset::UTF_16, error.charset());
            assert_eq!(
                CharsetEncodeErrorKind::BufferTooSmall {
                    required: 8,
                    available: 2
                },
                error.kind(),
            );
            assert_eq!(0, error.index());
        }
        other => panic!("expected encode capacity error, got {other:?}"),
    }
}

#[test]
fn test_charset_converter_applies_decode_policy_to_eof_tail() {
    let mut replace = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        Utf16U16Codec,
        CharsetDecodePolicy::replace('!'),
        CharsetEncodePolicy::default(),
    )
    .expect("replacement policy should be encodable");
    let mut replace_output = [0_u16; 6];
    let replace_written = replace
        .transcode_complete_into(&[0xe4], &mut replace_output)
        .expect("replacement policy should repair incomplete UTF-8 at EOF");
    assert_eq!(1, replace_written);
    assert_eq!(['!' as u16], replace_output[0..replace_written]);

    let mut ignore = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        Utf16U16Codec,
        CharsetDecodePolicy::ignore(),
        CharsetEncodePolicy::default(),
    )
    .expect("ignore policy should be encodable");
    let mut ignore_output = [0_u16; 6];
    let ignore_written = ignore
        .transcode_complete_into(&[0xe4], &mut ignore_output)
        .expect("ignore policy should drop incomplete UTF-8 at EOF");
    assert_eq!(0, ignore_written);

    let mut report = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        Utf16U16Codec,
        CharsetDecodePolicy::report(),
        CharsetEncodePolicy::default(),
    )
    .expect("report policy should be encodable");
    let mut report_output = [0_u16; 6];
    let raw_error = report
        .transcode_complete_into(&[0xe4], &mut report_output)
        .expect_err("report policy should reject incomplete UTF-8 at EOF");
    let error = report.map_transcode_error(raw_error);
    match error {
        CharsetConvertError::Decode(error) => {
            assert_eq!(Charset::UTF_8, error.charset());
            assert_eq!(0, error.index());
            assert_eq!(
                CharsetDecodeErrorKind::IncompleteSequence {
                    required: 3,
                    available: 1
                },
                error.kind()
            );
        }
        other => panic!("expected decode error, got {other:?}"),
    }
}

#[test]
fn test_charset_converter_exposes_configuration_and_bounds() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);

    assert_eq!(Charset::UTF_8, converter.source_charset());
    assert_eq!(Charset::UTF_16, converter.target_charset());
    assert_eq!(&Utf8Codec, converter.source_codec());
    assert_eq!(&Utf16U16Codec, converter.target_codec());
    assert_eq!(Charset::UTF_8, converter.source_codec_mut().charset());
    assert_eq!(Charset::UTF_16, converter.target_codec_mut().charset());
    assert_eq!(CharsetDecodePolicy::default(), converter.decode_policy());
    assert_eq!(CharsetEncodePolicy::default(), converter.encode_policy());
    assert_eq!(MalformedAction::Replace, converter.malformed_action());
    assert_eq!(CharsetDecodePolicy::DEFAULT_REPLACEMENT, converter.decode_replacement());
    assert_eq!(UnmappableAction::Replace, converter.unmappable_action());
    assert_eq!(CharsetEncodePolicy::DEFAULT_REPLACEMENT, converter.replacement());
    assert_eq!(Ok(8), converter.max_transcode_output_len(3));
    assert_eq!(Ok(2), converter.max_finish_output_len());
    assert_eq!(Ok(0), converter.max_reset_output_len());

    converter.reset(&mut [], 0).expect("reset");

    let converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    assert_eq!((Utf8Codec, Utf16U16Codec), converter.into_codecs());
}

#[test]
fn test_charset_converter_with_policies_prevalidates_replacement_once() {
    let calls = Rc::new(Cell::new(0));
    let target = CountingEncodeProbeCodec::new(Rc::clone(&calls));

    let converter = CharsetConverter::from_codecs_with_policies(
        AsciiBytesCodec,
        target,
        CharsetDecodePolicy::default(),
        CharsetEncodePolicy::replace('!'),
    )
    .expect("replacement is encodable");

    assert_eq!(UnmappableAction::Replace, converter.unmappable_action());
    assert_eq!(1, calls.get());
}

#[test]
fn test_charset_converter_with_policies_rejects_unencodable_replacement() {
    let error = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        AsciiBytesCodec,
        CharsetDecodePolicy::default(),
        CharsetEncodePolicy::replace('中'),
    )
    .expect_err("unencodable target replacement should be rejected");

    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter { value: '中' as u32 },
        error.kind()
    );
    assert_eq!(0, error.index());
}

#[test]
fn test_charset_converter_with_explicit_policies_exposes_effective_configuration() {
    let decode_policy = CharsetDecodePolicy::replace('!');
    let encode_policy = CharsetEncodePolicy::replace('?');
    let converter =
        CharsetConverter::from_codecs_with_policies(Utf8Codec, AsciiBytesCodec, decode_policy, encode_policy)
            .expect("explicit replacement policy should be encodable");

    assert_eq!(decode_policy, converter.decode_policy());
    assert_eq!(encode_policy, converter.encode_policy());
    assert_eq!(MalformedAction::Replace, converter.malformed_action());
    assert_eq!('!', converter.decode_replacement());
    assert_eq!(UnmappableAction::Replace, converter.unmappable_action());
    assert_eq!('?', converter.replacement());
}

#[test]
#[should_panic(expected = "cannot initialize CharsetConverter target")]
fn test_charset_converter_from_codecs_panics_when_no_default_replacement_is_encodable() {
    let _ = CharsetConverter::from_codecs(Utf8Codec, RejectingEncodeCodec);
}

#[test]
fn test_charset_converter_from_codecs_converts_available_ascii_without_finish() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let mut output = [0_u16; 4];

    let progress = converter
        .transcode(b"ABCD", 0, &mut output, 0)
        .expect("ASCII source decodes without waiting for EOF");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(4, progress.read());
    assert_eq!(4, progress.written());
    assert_eq!(['A' as u16, 'B' as u16, 'C' as u16, 'D' as u16], output);
    assert_eq!(Ok(2), converter.max_finish_output_len());

    let written = converter.finish(&mut output, 0).expect("finish has no buffered tail");
    assert_eq!(0, written);
}

#[test]
fn test_charset_converter_drains_decoder_need_output_batches() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let mut output = [0_u16; 9];

    let progress = converter
        .transcode(b"ABCDEFGHI", 0, &mut output, 0)
        .expect("converter should keep decoding after decoder output fills");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(9, progress.read());
    assert_eq!(9, progress.written());
    assert_eq!(
        [
            'A' as u16, 'B' as u16, 'C' as u16, 'D' as u16, 'E' as u16, 'F' as u16, 'G' as u16, 'H' as u16, 'I' as u16,
        ],
        output,
    );
}

#[test]
fn test_charset_converter_reports_invalid_input_index() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let input = b"A";
    let mut output = [0_u16; 1];

    let raw_error = converter
        .transcode(input, input.len() + 1, &mut output, 0)
        .expect_err("input index outside input slice should fail");
    let error = converter.map_transcode_error(raw_error);

    match error {
        CharsetConvertError::Decode(error) => {
            assert_eq!(Charset::UTF_8, error.charset());
            assert_eq!(input.len() + 1, error.index());
            assert_eq!(
                CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() },
                error.kind(),
            );
        }
        other => panic!("expected invalid input index, got {other:?}"),
    }
}

#[test]
fn test_charset_converter_maps_framework_errors_with_its_charsets() {
    let converter = CharsetConverter::from_codecs(Utf8Codec, AsciiBytesCodec);
    let framework_error = TranscodeFailure::insufficient_output(1, 2, 0).into();

    let error = converter.map_transcode_error(framework_error);

    assert_eq!(
        CharsetConvertError::Encode(CharsetEncodeError::new(
            Charset::ASCII,
            CharsetEncodeErrorKind::BufferTooSmall {
                required: 2,
                available: 0,
            },
            1,
        )),
        error,
    );
}

#[test]
fn test_charset_converter_keeps_pending_character_when_output_is_full() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let mut empty_output = [];

    let progress = converter
        .transcode(b"ABCD", 0, &mut empty_output, 0)
        .expect("decoded character stays pending");

    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
    assert_eq!(1, progress.read());
    assert_eq!(0, progress.written());
    assert_eq!(Ok(2), converter.max_finish_output_len());
    assert_eq!(Ok(8), converter.max_transcode_output_len(3));

    let progress = converter
        .transcode(b"", 0, &mut empty_output, 0)
        .expect("pending character still needs output capacity");
    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());

    let mut output = [0_u16; 4];
    let progress = converter
        .transcode(b"", 0, &mut output, 0)
        .expect("pending character is written before reading more input");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(0, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!('A' as u16, output[0]);

    let progress = converter
        .transcode(&b"ABCD"[1..], 0, &mut output, 1)
        .expect("caller resumes from unread source input");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(3, progress.written());
    assert_eq!(['A' as u16, 'B' as u16, 'C' as u16, 'D' as u16], output);
}

#[test]
fn test_charset_converter_finish_reports_need_output_for_starting_pending_character() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let mut empty_output = [];

    let progress = converter
        .transcode(b"ABCD", 0, &mut empty_output, 0)
        .expect("decoded source character cannot be written");
    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
    assert_eq!(1, progress.read());
    assert_eq!(0, progress.written());

    let raw_error = converter
        .finish(&mut empty_output, 0)
        .expect_err("pending character still needs output at finish");
    let error = converter.map_transcode_error(raw_error);
    match error {
        CharsetConvertError::Encode(error) => {
            assert_eq!(Charset::UTF_16, error.charset());
            assert_eq!(0, error.index());
            assert_eq!(
                CharsetEncodeErrorKind::BufferTooSmall {
                    required: 2,
                    available: 0,
                },
                error.kind(),
            );
        }
        other => panic!("expected insufficient output, got {other:?}"),
    }
}

#[test]
fn test_charset_converter_finish_delegates_to_target_encoder() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let mut output = [];

    let raw_error = converter
        .finish(&mut output, 1)
        .expect_err("target encoder reports out-of-range output index");
    let error = converter.map_transcode_error(raw_error);
    match error {
        CharsetConvertError::Encode(error) => {
            assert_eq!(Charset::UTF_16, error.charset());
            assert_eq!(1, error.index());
            assert_eq!(
                CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 0 },
                error.kind(),
            );
        }
        other => panic!("expected invalid output index, got {other:?}"),
    }
}

#[test]
fn test_charset_converter_finish_writes_starting_pending_character() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let mut empty_output = [];

    let progress = converter
        .transcode(b"ABCD", 0, &mut empty_output, 0)
        .expect("decoded source character cannot be written");
    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));

    let mut output = [0_u16; 4];
    let written = converter
        .finish(&mut output, 0)
        .expect("pending character is written during finish");
    assert_eq!(1, written);
    assert_eq!('A' as u16, output[0]);
}

#[test]
fn test_charset_converter_resets_pending_state() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let mut empty_output = [];
    let _ = converter
        .transcode(b"ABCD", 0, &mut empty_output, 0)
        .expect("converted char becomes pending");

    converter.reset(&mut [], 0).expect("reset");

    let mut output = [0_u16; 4];
    let progress = converter
        .transcode(b"WXYZ", 0, &mut output, 0)
        .expect("reset removes pending state");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(4, progress.read());
    assert_eq!(4, progress.written());
    assert_eq!(['W' as u16, 'X' as u16, 'Y' as u16, 'Z' as u16], output);
}

#[test]
fn test_charset_converter_finish_does_not_finalize_incomplete_source_input() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let mut output = [0_u16; 1];

    let progress = converter
        .transcode(&[0xe4, 0xb8], 0, &mut output, 0)
        .expect("partial source sequence needs more input");

    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());
    assert_eq!(Ok(2), converter.max_finish_output_len());

    let written = converter
        .finish(&mut output, 0)
        .expect("finish does not process caller-owned incomplete source input");
    assert_eq!(0, written);
    assert_eq!(0, output[0]);
    assert_eq!(Ok(2), converter.max_finish_output_len());
}

#[test]
fn test_charset_converter_finish_has_no_output_for_incomplete_source_input() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let mut output = [];

    let progress = converter
        .transcode(&[0xe4], 0, &mut output, 0)
        .expect("partial source sequence needs more input");

    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
    assert_eq!(Ok(2), converter.max_finish_output_len());

    let written = converter
        .finish(&mut output, 0)
        .expect("finish has no decoder-owned replacement output");
    assert_eq!(0, written);
    assert_eq!(Ok(2), converter.max_finish_output_len());
}

#[test]
fn test_charset_converter_finish_does_not_report_incomplete_source_input() {
    let mut converter = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        Utf16U16Codec,
        CharsetDecodePolicy::report(),
        CharsetEncodePolicy::default(),
    )
    .expect("default target policy should be encodable");
    reset_for_test(&mut converter);
    let mut output = [0_u16; 1];

    let progress = converter
        .transcode(&[0xe4], 0, &mut output, 0)
        .expect("partial source sequence needs more input");

    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
    assert_eq!(Ok(2), converter.max_finish_output_len());

    let written = converter
        .finish(&mut output, 0)
        .expect("finish does not process caller-owned incomplete source input");
    assert_eq!(0, written);
}

#[test]
fn test_charset_converter_propagates_decode_and_encode_errors() {
    let mut converter = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        Utf16U16Codec,
        CharsetDecodePolicy::report(),
        CharsetEncodePolicy::default(),
    )
    .expect("default target policy should be encodable");
    reset_for_test(&mut converter);
    let mut output = [0_u16; 1];

    let raw_error = converter
        .transcode(&[0x80, b'A', b'B', b'C'], 0, &mut output, 0)
        .expect_err("malformed source input is reported");
    let error = converter.map_transcode_error(raw_error);
    match error {
        CharsetConvertError::Decode(error) => {
            assert_eq!(Charset::UTF_8, error.charset());
            assert_eq!(0, error.index());
            assert_eq!(CharsetDecodeErrorKind::malformed(0x80), error.kind());
        }
        other => panic!("expected decode error, got {other:?}"),
    }

    let mut converter = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        AsciiBytesCodec,
        CharsetDecodePolicy::default(),
        CharsetEncodePolicy::report(),
    )
    .expect("report target policy should be constructible");
    reset_for_test(&mut converter);
    let mut ascii_output = [0_u8; 1];

    let raw_error = converter
        .transcode("é".as_bytes(), 0, &mut ascii_output, 0)
        .expect_err("unmappable target character is reported");
    let error = converter.map_transcode_error(raw_error);
    match error {
        CharsetConvertError::Encode(error) => {
            assert_eq!(Charset::ASCII, error.charset());
            assert_eq!(0, error.index());
            assert_eq!(
                CharsetEncodeErrorKind::UnmappableCharacter { value: 'é' as u32 },
                error.kind()
            );
        }
        other => panic!("expected encode error, got {other:?}"),
    }
}

#[test]
fn test_charset_converter_maps_source_decode_reset_error() {
    let mut converter = CharsetConverter::from_codecs(FailingLifecycleCodec::<0>, AsciiBytesCodec);
    let mut output = [];

    let raw_error = converter
        .reset(&mut output, 0)
        .expect_err("source decode reset error should be mapped");
    let error = converter.map_transcode_error(raw_error);

    match error {
        CharsetConvertError::Decode(error) => {
            assert_eq!(Charset::ASCII, error.charset());
            assert_eq!(CharsetDecodeErrorKind::malformed_unknown(), error.kind());
            assert_eq!(0, error.index());
        }
        other => {
            panic!("expected converter decode domain error, got {other:?}")
        }
    }
}

#[test]
fn test_charset_converter_maps_source_decode_finish_error() {
    let mut converter = CharsetConverter::from_codecs(FailingLifecycleCodec::<1>, AsciiBytesCodec);
    let mut output = [];
    reset_for_test(&mut converter);

    let raw_error = converter
        .finish(&mut output, 0)
        .expect_err("source decode flush error should be mapped");
    let error = converter.map_transcode_error(raw_error);

    match error {
        CharsetConvertError::Decode(error) => {
            assert_eq!(Charset::ASCII, error.charset());
            assert_eq!(CharsetDecodeErrorKind::malformed_unknown(), error.kind());
            assert_eq!(0, error.index());
        }
        other => {
            panic!("expected converter decode domain error, got {other:?}")
        }
    }
}

#[test]
fn test_charset_converter_maps_target_encode_reset_error() {
    let mut converter = CharsetConverter::from_codecs(AsciiBytesCodec, FailingLifecycleCodec::<2>);
    let mut output = [];

    let raw_error = converter
        .reset(&mut output, 0)
        .expect_err("target encode reset error should be mapped");
    let error = converter.map_transcode_error(raw_error);

    match error {
        CharsetConvertError::Encode(error) => {
            assert_eq!(Charset::ASCII, error.charset());
            assert_eq!(
                CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 0 },
                error.kind()
            );
            assert_eq!(0, error.index());
        }
        other => {
            panic!("expected converter encode domain error, got {other:?}")
        }
    }
}

#[test]
fn test_charset_converter_maps_target_encode_finish_error() {
    let mut converter = CharsetConverter::from_codecs(AsciiBytesCodec, FailingLifecycleCodec::<3>);
    let mut output = [];
    reset_for_test(&mut converter);

    let raw_error = converter
        .finish(&mut output, 0)
        .expect_err("target encode flush error should be mapped");
    let error = converter.map_transcode_error(raw_error);

    match error {
        CharsetConvertError::Encode(error) => {
            assert_eq!(Charset::ASCII, error.charset());
            assert_eq!(
                CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 0 },
                error.kind()
            );
            assert_eq!(0, error.index());
        }
        other => {
            panic!("expected converter encode domain error, got {other:?}")
        }
    }
}

#[test]
fn test_charset_converter_falls_back_to_question_mark_when_default_replacement_is_unencodable() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, ReplacementFallbackCodec);
    reset_for_test(&mut converter);
    let mut output = [0_u8; 1];

    assert_eq!(CharsetDecodePolicy::default(), converter.decode_policy());
    assert_eq!(UnmappableAction::Replace, converter.unmappable_action());
    assert_eq!(
        CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT,
        converter.replacement()
    );

    let progress = converter
        .transcode("中".as_bytes(), 0, &mut output, 0)
        .expect("fallback replacement should be encodable");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(b'?', output[0]);
}

#[test]
fn test_charset_converter_report_target_policy_does_not_require_default_unit() {
    let mut converter = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        NonDefaultUnitCodec,
        CharsetDecodePolicy::default(),
        CharsetEncodePolicy::report(),
    )
    .expect("report target policy should not pre-encode replacement units");
    reset_for_test(&mut converter);
    let mut output = [NonDefaultUnit(0)];

    let progress = converter
        .transcode(b"A", 0, &mut output, 0)
        .expect("ASCII character should convert");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(NonDefaultUnit(b'A'), output[0]);
}

#[test]
fn test_charset_converter_converts_available_utf8_to_ascii_without_finish() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, AsciiBytesCodec);
    reset_for_test(&mut converter);
    let mut output = [0_u8; 4];

    let progress = converter
        .transcode(b"ABCD", 0, &mut output, 0)
        .expect("ASCII source converts without waiting for finish");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(4, progress.read());
    assert_eq!(4, progress.written());
    assert_eq!(*b"ABCD", output);
}

#[test]
fn test_chunked_backpressure_matches_stdlib_through_eof() {
    let payloads: &[&[u8]] = &[
        b"",
        b"ABC",
        "A中😀".as_bytes(),
        &[0xff, b'A', 0xe4, 0xb8],
        &[0xe2, 0x82],
        &[0xf0, 0x9f, 0x98, 0x80, 0xc0, 0xaf, b'Z'],
    ];
    let schedules: &[&[u8]] = &[&[], &[0], &[1], &[2], &[0, 1, 2, 6, 0, 4]];
    for input in payloads {
        let expected: Vec<u16> = String::from_utf8_lossy(input).encode_utf16().collect();
        for controls in schedules {
            assert_eq!(
                expected,
                run_chunked_backpressure(input, controls),
                "input={input:02x?}, controls={controls:?}",
            );
        }
    }
}

#[test]
fn test_chunked_backpressure_expands_for_non_bmp_output() {
    let input = "😀".as_bytes();
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let mut short_output = [0_u16; 1];

    let first = converter
        .transcode(input, 0, &mut short_output, 0)
        .expect("one output slot cannot hold a surrogate pair");
    assert!(matches!(first.status(), TranscodeStatus::NeedOutput { required } if required.get() == 2));
    assert_eq!(input.len(), first.read());
    assert_eq!(0, first.written());

    let mut expanded_output = [0_u16; 2];
    let second = converter
        .transcode(&input[first.read()..], 0, &mut expanded_output, 0)
        .expect("retry pending non-BMP output with required capacity");
    assert_eq!(TranscodeStatus::Complete, second.status());
    assert_eq!(0, second.read());
    assert_eq!(2, second.written());
    assert_eq!(input.len(), first.read() + second.read());
    assert_eq!(2, first.written() + second.written());
    assert_eq!("😀".encode_utf16().collect::<Vec<_>>(), expanded_output);
}

#[test]
fn test_chunked_backpressure_expands_for_incomplete_eof_tail() {
    let input = &[0xe2, 0x82];
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    reset_for_test(&mut converter);
    let mut empty_output = [];

    let first = converter
        .transcode_eof(input, 0, &mut empty_output, 0)
        .expect("EOF replacement can remain pending under output backpressure");
    assert!(matches!(first.status(), TranscodeStatus::NeedOutput { required } if required.get() == 1));
    assert_eq!(input.len(), first.read());
    assert_eq!(0, first.written());

    let mut expanded_output = [0_u16; 1];
    let second = converter
        .transcode_eof(&input[first.read()..], 0, &mut expanded_output, 0)
        .expect("retry pending EOF replacement with required capacity");
    assert_eq!(TranscodeStatus::Complete, second.status());
    assert_eq!(0, second.read());
    assert_eq!(1, second.written());
    assert_eq!(input.len(), first.read() + second.read());
    assert_eq!(1, first.written() + second.written());
    assert_eq!(0xfffd, expanded_output[0]);
}

fn utf16_bytes(units: &[u16], order: ByteOrder) -> Vec<u8> {
    units
        .iter()
        .flat_map(|unit| match order {
            ByteOrder::LittleEndian => unit.to_le_bytes(),
            ByteOrder::BigEndian => unit.to_be_bytes(),
            ByteOrder::NativeEndian => unit.to_ne_bytes(),
        })
        .collect()
}

fn utf32_bytes(units: &[u32], order: ByteOrder) -> Vec<u8> {
    units
        .iter()
        .flat_map(|unit| match order {
            ByteOrder::LittleEndian => unit.to_le_bytes(),
            ByteOrder::BigEndian => unit.to_be_bytes(),
            ByteOrder::NativeEndian => unit.to_ne_bytes(),
        })
        .collect()
}

fn check_chunk_splits<C, F>(make_codec: F, context: &str, input: &[u8])
where
    C: CharsetCodec<Unit = u8>,
    F: Fn() -> C + Copy,
{
    for windows in [&[0][..], &[1], &[2], &[0, 1, 2]] {
        if input.is_empty() {
            assert_chunked_matches_complete(make_codec, context, input, &[], windows);
        } else {
            for split in 1..input.len() {
                assert_chunked_matches_complete(make_codec, context, input, &[split, input.len() - split], windows);
            }
            assert_chunked_matches_complete(make_codec, context, input, &vec![1; input.len()], windows);
        }
    }
}

#[test]
fn test_multibyte_chunked_boundaries() {
    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        let utf16_context = format!("Utf16ByteCodec/{order:?}");
        let utf32_context = format!("Utf32ByteCodec/{order:?}");
        let valid = "A中😀";
        let expected: Vec<u16> = valid.encode_utf16().collect();
        let utf16 = utf16_bytes(&expected, order);
        for windows in [&[0][..], &[1], &[2], &[0, 1, 2]] {
            assert_chunked_matches_complete(
                || Utf16ByteCodec::new(order),
                &utf16_context,
                &utf16,
                &vec![1; utf16.len()],
                windows,
            );
        }
        let mut valid_converter = CharsetConverter::from_codecs(Utf16ByteCodec::new(order), Utf16U16Codec);
        let mut valid_output = vec![0_u16; valid_converter.max_total_output_len(utf16.len()).expect("capacity")];
        let valid_len = valid_converter
            .transcode_complete_into(&utf16, &mut valid_output)
            .expect("valid UTF-16");
        assert_eq!(expected, valid_output[..valid_len]);

        let utf16_cases = [
            vec![],
            utf16_bytes(&[0x0041], order),
            utf16_bytes(&[0x4e2d], order),
            utf16_bytes(&[0xd83d, 0xde00], order),
            utf16_bytes(&[0xd800], order),
            utf16_bytes(&[0xdc00], order),
            utf16_bytes(&[0xd800, 0x0041], order),
            vec![0x41],
            [utf16_bytes(&[0xd800], order), vec![0x41]].concat(),
        ];
        for input in &utf16_cases {
            check_chunk_splits(|| Utf16ByteCodec::new(order), &utf16_context, input);
        }

        let utf32_cases = [
            vec![],
            utf32_bytes(&[0x41], order),
            utf32_bytes(&[0x4e2d], order),
            utf32_bytes(&[0x1f600], order),
            utf32_bytes(&[0xd800], order),
            utf32_bytes(&[0x11_0000], order),
            vec![0x41],
            vec![0x41, 0x42],
            vec![0x41, 0x42, 0x43],
        ];
        for input in &utf32_cases {
            check_chunk_splits(|| Utf32ByteCodec::new(order), &utf32_context, input);
        }
        let utf32 = utf32_bytes(&[0x41, 0x4e2d, 0x1f600], order);
        let mut valid_converter = CharsetConverter::from_codecs(Utf32ByteCodec::new(order), Utf16U16Codec);
        let mut valid_output = vec![0_u16; valid_converter.max_total_output_len(utf32.len()).expect("capacity")];
        let valid_len = valid_converter
            .transcode_complete_into(&utf32, &mut valid_output)
            .expect("valid UTF-32");
        assert_eq!(expected, valid_output[..valid_len]);
    }
}

#[test]
fn test_multibyte_chunked_eof_backpressure() {
    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        let utf16_context = format!("Utf16ByteCodec/{order:?}");
        let utf32_context = format!("Utf32ByteCodec/{order:?}");
        let utf16_tail = [utf16_bytes(&[0xd800], order), vec![0x41]].concat();
        assert_chunked_matches_complete(
            || Utf16ByteCodec::new(order),
            &utf16_context,
            &utf16_tail,
            &vec![1; utf16_tail.len()],
            &[0],
        );
        for tail_len in 1..=3 {
            let tail: Vec<u8> = (0..tail_len).map(|value| value as u8 + 0x41).collect();
            assert_chunked_matches_complete(
                || Utf32ByteCodec::new(order),
                &utf32_context,
                &tail,
                &vec![1; tail_len],
                &[0],
            );
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn test_multibyte_chunked_property(
        input in proptest::collection::vec(any::<u8>(), 0..=64),
        controls in proptest::collection::vec(any::<u8>(), 0..=16),
    ) {
        let mut chunks = Vec::new();
        let mut added = 0;
        while added < input.len() {
            let control = if controls.is_empty() {
                0
            } else {
                controls[chunks.len() % controls.len()] as usize
            };
            let count = (control % 7 + 1).min(input.len() - added);
            chunks.push(count);
            added += count;
        }
        let windows = if controls.is_empty() {
            vec![1]
        } else {
            controls.iter().map(|value| *value as usize % 3).collect::<Vec<_>>()
        };
        for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
            let utf16_context = format!("Utf16ByteCodec/{order:?}");
            let utf32_context = format!("Utf32ByteCodec/{order:?}");
            assert_chunked_matches_complete(|| Utf16ByteCodec::new(order), &utf16_context, &input, &chunks, &windows);
            assert_chunked_matches_complete(|| Utf32ByteCodec::new(order), &utf32_context, &input, &chunks, &windows);
        }
    }
}
