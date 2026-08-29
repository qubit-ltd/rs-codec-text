// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use std::cell::Cell;
use std::rc::Rc;

use qubit_codec::CapacityError;
use qubit_codec::Codec;
use qubit_codec::DecodeFailure;
use qubit_codec::TranscodeConvertError;
use qubit_codec::TranscodeConvertErrorOf;
use qubit_codec::TranscodeConverter;
use qubit_codec::TranscodeFailure;
use qubit_codec::TranscodeProgress;
use qubit_codec::TranscodeStatus;
use qubit_codec::Transcoder;
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
use qubit_codec_text::MalformedAction;
use qubit_codec_text::UnmappableAction;
use qubit_codec_text::Utf8Codec;
use qubit_codec_text::Utf16U16Codec;

fn reset_for_test<T: Transcoder>(transcoder: &mut T) {
    let mut output: [T::Output; 0] = [];
    assert!(transcoder.reset(&mut output, 0).is_ok());
}

fn map_convert_error(error: TranscodeConvertErrorOf<Utf8Codec, Utf16U16Codec>) -> CharsetConvertError {
    match error {
        TranscodeConvertError::Failure(failure) => map_convert_failure(failure),
        TranscodeConvertError::DecodeDomain(error) => CharsetConvertError::Decode(error.into_source()),
        TranscodeConvertError::EncodeDomain(error) => CharsetConvertError::Encode(error.into_source()),
        TranscodeConvertError::Unencodable { input_index, value } => {
            CharsetConvertError::Encode(CharsetEncodeError::map_unencodable(Charset::UTF_8, input_index, value))
        }
    }
}

fn map_convert_failure(failure: TranscodeFailure) -> CharsetConvertError {
    match failure {
        TranscodeFailure::InvalidInputIndex { .. }
        | TranscodeFailure::IncompleteInput { .. }
        | TranscodeFailure::TrailingInput { .. } => {
            CharsetConvertError::Decode(CharsetDecodeError::map_transcode_failure(Charset::UTF_8, failure))
        }
        TranscodeFailure::InvalidOutputIndex { .. }
        | TranscodeFailure::InvalidOutputRange { .. }
        | TranscodeFailure::InsufficientOutput { .. }
        | TranscodeFailure::OutputLengthOverflow => {
            CharsetConvertError::Encode(CharsetEncodeError::map_transcode_failure(Charset::UTF_8, failure))
        }
        _ => CharsetConvertError::Decode(CharsetDecodeError::map_transcode_failure(Charset::UTF_8, failure)),
    }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
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
    let error = converter
        .transcode_complete_into(b"AB", &mut short_output)
        .map_err(map_convert_error)
        .expect_err("complete conversion maps insufficient output");

    match error {
        CharsetConvertError::Encode(error) => {
            assert!(matches!(error.kind(), CharsetEncodeErrorKind::BufferTooSmall { .. },));
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
    let error = report
        .transcode_complete_into(&[0xe4], &mut report_output)
        .map_err(map_convert_error)
        .expect_err("report policy should reject incomplete UTF-8 at EOF");
    assert!(matches!(
        error,
        CharsetConvertError::Decode(error)
            if error.kind()
                == CharsetDecodeErrorKind::IncompleteSequence {
                    required: 3,
                    available: 1,
                }
    ));
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

    let error = converter
        .transcode(input, input.len() + 1, &mut output, 0)
        .map_err(map_convert_error)
        .expect_err("input index outside input slice should fail");

    match error {
        CharsetConvertError::Decode(error) => {
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

    let error = converter
        .finish(&mut empty_output, 0)
        .map_err(map_convert_error)
        .expect_err("pending character still needs output at finish");
    match error {
        CharsetConvertError::Encode(error) => {
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

    let error = converter
        .finish(&mut output, 1)
        .map_err(map_convert_error)
        .expect_err("target encoder reports out-of-range output index");
    match error {
        CharsetConvertError::Encode(error) => {
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

    let error = converter
        .transcode(&[0x80, b'A', b'B', b'C'], 0, &mut output, 0)
        .map_err(map_convert_error)
        .expect_err("malformed source input is reported");
    assert!(matches!(error, CharsetConvertError::Decode(_)));

    let mut converter = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        AsciiBytesCodec,
        CharsetDecodePolicy::default(),
        CharsetEncodePolicy::report(),
    )
    .expect("report target policy should be constructible");
    reset_for_test(&mut converter);
    let mut ascii_output = [0_u8; 1];

    let error = converter
        .transcode("é".as_bytes(), 0, &mut ascii_output, 0)
        .map_err(map_convert_error)
        .expect_err("unmappable target character is reported");
    assert!(matches!(error, CharsetConvertError::Encode(_)));
}

#[test]
fn test_charset_converter_maps_source_decode_reset_error() {
    let mut converter = CharsetConverter::from_codecs(FailingLifecycleCodec::<0>, AsciiBytesCodec);
    let mut output = [];

    let error = converter
        .reset(&mut output, 0)
        .map_err(map_convert_error)
        .expect_err("source decode reset error should be mapped");

    match error {
        CharsetConvertError::Decode(error) => {
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

    let error = converter
        .finish(&mut output, 0)
        .map_err(map_convert_error)
        .expect_err("source decode flush error should be mapped");

    match error {
        CharsetConvertError::Decode(error) => {
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

    let error = converter
        .reset(&mut output, 0)
        .map_err(map_convert_error)
        .expect_err("target encode reset error should be mapped");

    match error {
        CharsetConvertError::Encode(error) => {
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

    let error = converter
        .finish(&mut output, 0)
        .map_err(map_convert_error)
        .expect_err("target encode flush error should be mapped");

    match error {
        CharsetConvertError::Encode(error) => {
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
