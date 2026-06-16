use std::{cell::Cell, rc::Rc};

use qubit_codec::{TranscodeConverter, TranscodeError};
use qubit_codec_text::{
    Charset, CharsetCodec, CharsetConvertError, CharsetConverter, CharsetDecodeError,
    CharsetDecodeErrorKind, CharsetDecodePolicy, CharsetDecodeResult, CharsetEncodeError,
    CharsetEncodeErrorKind, CharsetEncodePolicy, CharsetEncodeResult, Codec, MalformedAction,
    TranscodeStatus, Transcoder, UnmappableAction, Utf8Codec, Utf16U16Codec,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct AsciiBytesCodec;

impl CharsetCodec for AsciiBytesCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

unsafe impl Codec for AsciiBytesCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn can_encode_value(&self, value: &char) -> bool {
        value.is_ascii()
    }

    unsafe fn decode(
        &mut self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        if index >= input.len() {
            let kind = CharsetDecodeErrorKind::IncompleteSequence {
                required: 1,
                available: input.len().saturating_sub(index),
            };
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, index));
        }
        let value = input[index];
        if value > 0x7f {
            let kind = CharsetDecodeErrorKind::MalformedSequence {
                value: Some(value as u32),
            };
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, index));
        }
        Ok((value as char, core::num::NonZeroUsize::MIN))
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        output: &mut [u8],
        index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        debug_assert!(self.can_encode_value(value));
        debug_assert!(index < output.len());
        unsafe {
            // SAFETY: The caller guarantees that `index` is writable.
            *output.as_mut_ptr().add(index) = *value as u8;
        }
        Ok(core::num::NonZeroUsize::MIN)
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

unsafe impl Codec for CountingEncodeProbeCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        AsciiBytesCodec.min_units_per_value()
    }

    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        AsciiBytesCodec.max_units_per_value()
    }

    fn can_encode_value(&self, value: &char) -> bool {
        self.calls.set(self.calls.get() + 1);
        AsciiBytesCodec.can_encode_value(value)
    }

    unsafe fn decode(
        &mut self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        unsafe { AsciiBytesCodec.decode(input, index) }
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        output: &mut [u8],
        index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        unsafe { AsciiBytesCodec.encode(value, output, index) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
struct NonDefaultUnit(u8);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NonDefaultUnitCodec;

impl CharsetCodec for NonDefaultUnitCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

unsafe impl Codec for NonDefaultUnitCodec {
    type Value = char;
    type Unit = NonDefaultUnit;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn can_encode_value(&self, value: &char) -> bool {
        value.is_ascii()
    }

    unsafe fn decode(
        &mut self,
        _input: &[NonDefaultUnit],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
        Err(CharsetDecodeError::new(Charset::ASCII, kind, index))
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        output: &mut [NonDefaultUnit],
        index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        debug_assert!(self.can_encode_value(value));
        debug_assert!(index < output.len());
        unsafe {
            // SAFETY: The caller guarantees that `index` is writable.
            *output.as_mut_ptr().add(index) = NonDefaultUnit(*value as u8);
        }
        Ok(core::num::NonZeroUsize::MIN)
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

unsafe impl Codec for RejectingEncodeCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn can_encode_value(&self, _value: &char) -> bool {
        false
    }

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
        Err(CharsetDecodeError::new(Charset::ASCII, kind, index))
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        _output: &mut [u8],
        index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        let kind = CharsetEncodeErrorKind::UnmappableCharacter {
            value: *value as u32,
        };
        Err(CharsetEncodeError::new(Charset::ASCII, kind, index))
    }
}

unsafe impl Codec for ReplacementFallbackCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn can_encode_value(&self, value: &char) -> bool {
        *value == '?' || value.is_ascii()
    }

    unsafe fn decode(
        &mut self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        unsafe { AsciiBytesCodec.decode(input, index) }
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        output: &mut [u8],
        index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        debug_assert!(self.can_encode_value(value));
        debug_assert!(index < output.len());
        unsafe {
            // SAFETY: The caller guarantees that `index` is writable.
            *output.as_mut_ptr().add(index) = *value as u8;
        }
        Ok(core::num::NonZeroUsize::MIN)
    }
}

#[test]
fn test_charset_converter_is_transcode_converter() {
    fn assert_transcode_converter<T: TranscodeConverter<u8, u16>>() {}

    assert_transcode_converter::<CharsetConverter<Utf8Codec, Utf16U16Codec>>();
}

#[test]
fn test_charset_converter_exposes_configuration_and_bounds() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);

    assert_eq!(CharsetDecodePolicy::default(), converter.decode_policy());
    assert_eq!(CharsetEncodePolicy::default(), converter.encode_policy());
    assert_eq!(MalformedAction::Replace, converter.malformed_action());
    assert_eq!(
        CharsetDecodePolicy::DEFAULT_REPLACEMENT,
        converter.decode_replacement()
    );
    assert_eq!(UnmappableAction::Replace, converter.unmappable_action());
    assert_eq!(
        CharsetEncodePolicy::DEFAULT_REPLACEMENT,
        converter.replacement()
    );
    assert_eq!(Ok(6), converter.max_output_len(3));
    assert_eq!(Ok(0), converter.max_finish_output_len());
    assert_eq!(Ok(0), converter.max_reset_output_len());

    converter.reset(&mut [], 0).expect("reset");
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
        CharsetEncodeErrorKind::UnmappableCharacter {
            value: '中' as u32
        },
        error.kind()
    );
    assert_eq!(0, error.index());
}

#[test]
fn test_charset_converter_with_explicit_policies_exposes_effective_configuration() {
    let decode_policy = CharsetDecodePolicy::replace('!');
    let encode_policy = CharsetEncodePolicy::replace('?');
    let converter = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        AsciiBytesCodec,
        decode_policy,
        encode_policy,
    )
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
    let mut output = [0_u16; 4];

    let progress = converter
        .transcode(b"ABCD", 0, &mut output, 0)
        .expect("ASCII source decodes without waiting for EOF");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(4, progress.read());
    assert_eq!(4, progress.written());
    assert_eq!(['A' as u16, 'B' as u16, 'C' as u16, 'D' as u16], output);
    assert_eq!(Ok(0), converter.max_finish_output_len());

    let written = converter
        .finish(&mut output, 0)
        .expect("finish has no buffered tail");
    assert_eq!(0, written);
}

#[test]
fn test_charset_converter_drains_decoder_need_output_batches() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let mut output = [0_u16; 9];

    let progress = converter
        .transcode(b"ABCDEFGHI", 0, &mut output, 0)
        .expect("converter should keep decoding after decoder output fills");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(9, progress.read());
    assert_eq!(9, progress.written());
    assert_eq!(
        [
            'A' as u16, 'B' as u16, 'C' as u16, 'D' as u16, 'E' as u16, 'F' as u16, 'G' as u16,
            'H' as u16, 'I' as u16,
        ],
        output,
    );
}

#[test]
fn test_charset_converter_reports_invalid_input_index() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let input = b"A";
    let mut output = [0_u16; 1];

    let error = converter
        .transcode(input, input.len() + 1, &mut output, 0)
        .expect_err("input index outside input slice should fail");

    match error {
        TranscodeError::Domain(_) => {
            panic!("invalid source index must be reported as a framework error")
        }
        TranscodeError::InvalidInputIndex { index, len } => {
            assert_eq!(input.len() + 1, index);
            assert_eq!(input.len(), len);
        }
        other => panic!("expected invalid input index, got {other:?}"),
    }
}

#[test]
fn test_charset_converter_keeps_pending_character_when_output_is_full() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let mut empty_output = [];

    let progress = converter
        .transcode(b"ABCD", 0, &mut empty_output, 0)
        .expect("decoded character stays pending");

    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedOutput { .. }
    ));
    assert_eq!(1, progress.read());
    assert_eq!(0, progress.written());
    assert_eq!(Ok(2), converter.max_finish_output_len());
    assert_eq!(Ok(8), converter.max_output_len(3));

    let progress = converter
        .transcode(b"", 0, &mut empty_output, 0)
        .expect("pending character still needs output capacity");
    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedOutput { .. }
    ));
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
    let mut empty_output = [];

    let progress = converter
        .transcode(b"ABCD", 0, &mut empty_output, 0)
        .expect("decoded source character cannot be written");
    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedOutput { .. }
    ));
    assert_eq!(1, progress.read());
    assert_eq!(0, progress.written());

    let error = converter
        .finish(&mut empty_output, 0)
        .expect_err("pending character still needs output at finish");
    match error {
        TranscodeError::InsufficientOutput {
            output_index,
            required,
            available,
        } => {
            assert_eq!(0, output_index);
            assert_eq!(2, required);
            assert_eq!(0, available);
        }
        other => panic!("expected insufficient output, got {other:?}"),
    }
}

#[test]
fn test_charset_converter_finish_delegates_to_target_encoder() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let mut output = [];

    let error = converter
        .finish(&mut output, 1)
        .expect_err("target encoder reports out-of-range output index");
    match error {
        TranscodeError::InvalidOutputIndex { index, len } => {
            assert_eq!(1, index);
            assert_eq!(0, len);
        }
        other => panic!("expected invalid output index, got {other:?}"),
    }
}

#[test]
fn test_charset_converter_finish_writes_starting_pending_character() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let mut empty_output = [];

    let progress = converter
        .transcode(b"ABCD", 0, &mut empty_output, 0)
        .expect("decoded source character cannot be written");
    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedOutput { .. }
    ));

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
    let mut empty_output = [];
    converter
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
    let mut output = [0_u16; 1];

    let progress = converter
        .transcode(&[0xe4, 0xb8], 0, &mut output, 0)
        .expect("partial source sequence needs more input");

    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());
    assert_eq!(Ok(0), converter.max_finish_output_len());

    let written = converter
        .finish(&mut output, 0)
        .expect("finish does not process caller-owned incomplete source input");
    assert_eq!(0, written);
    assert_eq!(0, output[0]);
    assert_eq!(Ok(0), converter.max_finish_output_len());
}

#[test]
fn test_charset_converter_finish_has_no_output_for_incomplete_source_input() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let mut output = [];

    let progress = converter
        .transcode(&[0xe4], 0, &mut output, 0)
        .expect("partial source sequence needs more input");

    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));
    assert_eq!(Ok(0), converter.max_finish_output_len());

    let written = converter
        .finish(&mut output, 0)
        .expect("finish has no decoder-owned replacement output");
    assert_eq!(0, written);
    assert_eq!(Ok(0), converter.max_finish_output_len());
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
    let mut output = [0_u16; 1];

    let progress = converter
        .transcode(&[0xe4], 0, &mut output, 0)
        .expect("partial source sequence needs more input");

    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));
    assert_eq!(Ok(0), converter.max_finish_output_len());

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
    let mut output = [0_u16; 1];

    let error = converter
        .transcode(&[0x80, b'A', b'B', b'C'], 0, &mut output, 0)
        .expect_err("malformed source input is reported");
    assert!(matches!(
        error,
        TranscodeError::Domain(CharsetConvertError::Decode(_))
    ));

    let mut converter = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        AsciiBytesCodec,
        CharsetDecodePolicy::default(),
        CharsetEncodePolicy::report(),
    )
    .expect("report target policy should be constructible");
    let mut ascii_output = [0_u8; 1];

    let error = converter
        .transcode("é".as_bytes(), 0, &mut ascii_output, 0)
        .expect_err("unmappable target character is reported");
    assert!(matches!(
        error,
        TranscodeError::Domain(CharsetConvertError::Encode(_))
    ));
}

#[test]
fn test_charset_converter_falls_back_to_question_mark_when_default_replacement_is_unencodable() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, ReplacementFallbackCodec);
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
    let mut output = [0_u8; 4];

    let progress = converter
        .transcode(b"ABCD", 0, &mut output, 0)
        .expect("ASCII source converts without waiting for finish");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(4, progress.read());
    assert_eq!(4, progress.written());
    assert_eq!(*b"ABCD", output);
}
