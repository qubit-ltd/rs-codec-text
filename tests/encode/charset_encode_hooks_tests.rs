// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::num::NonZeroUsize;

use qubit_codec::Codec;
use qubit_codec::DecodeFailure;
use qubit_codec::TranscodeConvertError;
use qubit_codec::TranscodeEncodeError;
use qubit_codec::TranscodeStatus;
use qubit_codec_text::AsciiCodec;
use qubit_codec_text::Charset;
use qubit_codec_text::CharsetCodec;
use qubit_codec_text::CharsetConvertError;
use qubit_codec_text::CharsetConverter;
use qubit_codec_text::CharsetDecodeError;
use qubit_codec_text::CharsetDecodePolicy;
use qubit_codec_text::CharsetEncodeError;
use qubit_codec_text::CharsetEncodeErrorKind;
use qubit_codec_text::CharsetEncodePolicy;
use qubit_codec_text::CharsetEncoder;
use qubit_codec_text::Utf8Codec;

#[derive(Clone, Copy, Debug)]
enum ReplacementTransition {
    Reset,
    Encode,
    Never,
}

#[derive(Debug)]
struct StatefulReplacementCodec {
    transition: ReplacementTransition,
    shifted: bool,
}

impl StatefulReplacementCodec {
    fn new(transition: ReplacementTransition) -> Self {
        Self {
            transition,
            shifted: false,
        }
    }
}

impl CharsetCodec for StatefulReplacementCodec {
    fn charset(&self) -> Charset {
        Charset::new_static("stateful-replacement-probe", "Stateful replacement probe", &[])
    }
}

impl Codec for StatefulReplacementCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = 1;
    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;
    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;

    fn can_encode_value(&self, value: &char) -> bool {
        value.is_ascii() || *value == 'é' || (*value == '\u{fffd}' && !self.shifted)
    }

    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> Result<(char, NonZeroUsize), DecodeFailure<CharsetDecodeError>> {
        let ch = match input[input_index] {
            0x80 => '\u{fffd}',
            0x81 => 'é',
            value => char::from(value),
        };
        Ok((ch, NonZeroUsize::MIN))
    }

    fn encode_len(&self, _value: &char) -> usize {
        1
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        output: &mut [u8],
        output_index: usize,
    ) -> Result<usize, CharsetEncodeError> {
        assert!(self.can_encode_value(value));
        output[output_index] = match *value {
            '\u{fffd}' => 0x80,
            'é' => 0x81,
            ch => ch as u8,
        };
        if matches!(self.transition, ReplacementTransition::Encode) && *value == 'é' {
            self.shifted = true;
        }
        Ok(1)
    }

    unsafe fn encode_reset(&mut self, _output: &mut [u8], _output_index: usize) -> Result<usize, CharsetEncodeError> {
        self.shifted = matches!(self.transition, ReplacementTransition::Reset);
        Ok(0)
    }
}

#[test]
fn test_charset_encode_hooks_apply_the_configured_unmappable_action() {
    let input = ['A', '中', 'B'];

    let mut replace = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::replace('?'))
        .expect("replacement character should be encodable");
    let mut replace_output = [0_u8; 3];
    let written = replace
        .transcode_complete_into(&input, &mut replace_output)
        .expect("replacement policy should encode unmappable input");
    assert_eq!(b"A?B", &replace_output[..written]);

    let mut ignore = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::ignore())
        .expect("ignore policy should be constructible");
    let mut ignore_output = [0_u8; 3];
    let written = ignore
        .transcode_complete_into(&input, &mut ignore_output)
        .expect("ignore policy should skip unmappable input");
    assert_eq!(b"AB", &ignore_output[..written]);

    let mut report = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::report())
        .expect("report policy should be constructible");
    let mut report_output = [0_u8; 3];
    let error = report
        .transcode_complete_into(&input, &mut report_output)
        .expect_err("report policy should reject unmappable input");
    let TranscodeEncodeError::Domain(error) = error else {
        panic!("report policy should return a charset-domain error");
    };
    assert!(matches!(
        error.into_source().kind(),
        CharsetEncodeErrorKind::UnmappableCharacter { .. }
    ));
    assert_eq!(1, error.into_source().index());
}

#[test]
fn test_stateful_replacement_encoder_returns_domain_error() {
    for transition in [ReplacementTransition::Reset, ReplacementTransition::Encode] {
        for explicit in [false, true] {
            let codec = StatefulReplacementCodec::new(transition);
            let mut encoder = if explicit {
                CharsetEncoder::with_policy(codec, CharsetEncodePolicy::replace('\u{fffd}'))
                    .expect("initial state supports the explicit replacement")
            } else {
                CharsetEncoder::new(codec)
            };
            assert_eq!('\u{fffd}', encoder.replacement());
            encoder.reset(&mut [], 0).expect("reset");
            let mut output = [0x55_u8; 8];
            let raw = encoder
                .transcode(&['!', 'é', '中'], 1, &mut output, 1)
                .expect_err("the selected replacement became unencodable");
            let TranscodeEncodeError::Domain(error) = raw else {
                panic!("expected a charset-domain error");
            };
            assert_eq!(Some(2), error.input_index());
            assert_eq!(2, error.source().index());
            assert_eq!(encoder.charset(), error.source().charset());
            assert_eq!(
                CharsetEncodeErrorKind::UnmappableCharacter { value: 0xfffd },
                error.source().kind()
            );
            assert_eq!([0x55, 0x81, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55], output);
            assert_eq!('\u{fffd}', encoder.replacement());
            assert!(encoder.codec().can_encode_value(&'?'));
        }
    }
}

#[test]
fn test_stateful_replacement_converter_returns_encode_domain_error() {
    for transition in [ReplacementTransition::Reset, ReplacementTransition::Encode] {
        for explicit in [false, true] {
            let codec = StatefulReplacementCodec::new(transition);
            let mut converter = if explicit {
                CharsetConverter::from_codecs_with_policies(
                    Utf8Codec,
                    codec,
                    CharsetDecodePolicy::report(),
                    CharsetEncodePolicy::replace('\u{fffd}'),
                )
                .expect("initial state supports the explicit replacement")
            } else {
                CharsetConverter::from_codecs(Utf8Codec, codec)
            };
            converter.reset(&mut [], 0).expect("reset");
            let mut output = [0x55_u8; 8];
            let raw = converter
                .transcode("!é中".as_bytes(), 1, &mut output, 1)
                .expect_err("the selected replacement became unencodable");
            let TranscodeConvertError::EncodeDomain(error) = raw else {
                panic!("expected a target encode-domain error");
            };
            assert_eq!(Some(3), error.input_index());
            assert_eq!(3, error.source().index());
            assert_eq!(converter.target_charset(), error.source().charset());
            assert_eq!(
                CharsetEncodeErrorKind::UnmappableCharacter { value: 0xfffd },
                error.source().kind()
            );
            assert_eq!([0x55, 0x81, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55], output);
            assert_eq!('\u{fffd}', converter.replacement());
            assert!(converter.target_codec().can_encode_value(&'?'));
        }
    }
}

#[test]
fn test_stateful_replacement_stable_state_encodes_selected_replacement() {
    let mut encoder = CharsetEncoder::new(StatefulReplacementCodec::new(ReplacementTransition::Never));
    encoder.reset(&mut [], 0).expect("reset");
    let mut encoder_output = [0_u8; 4];
    let progress = encoder
        .transcode(&['中'], 0, &mut encoder_output, 0)
        .expect("replacement remains encodable");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(1, progress.written());
    assert_eq!(0x80, encoder_output[0]);
    assert_eq!('\u{fffd}', encoder.replacement());

    let mut converter =
        CharsetConverter::from_codecs(Utf8Codec, StatefulReplacementCodec::new(ReplacementTransition::Never));
    converter.reset(&mut [], 0).expect("reset");
    let mut converter_output = [0_u8; 4];
    let progress = converter
        .transcode("中".as_bytes(), 0, &mut converter_output, 0)
        .expect("replacement remains encodable");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(1, progress.written());
    assert_eq!(0x80, converter_output[0]);
    assert_eq!('\u{fffd}', converter.replacement());
}

#[test]
fn test_stateful_replacement_complete_helper_returns_domain_error() {
    let mut encoder = CharsetEncoder::new(StatefulReplacementCodec::new(ReplacementTransition::Reset));
    let mut encoder_output = [0_u8; 16];
    let raw = encoder
        .transcode_complete_into(&['中'], &mut encoder_output)
        .expect_err("replacement is unencodable after reset");
    let TranscodeEncodeError::Domain(error) = raw else {
        panic!("expected a charset-domain error");
    };
    assert_eq!(0, error.source().index());
    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter { value: 0xfffd },
        error.source().kind()
    );
    assert_eq!([0_u8; 16], encoder_output);
    let mapped = encoder.map_transcode_error(TranscodeEncodeError::Domain(error));
    assert_eq!(0, mapped.index());
    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter { value: 0xfffd },
        mapped.kind()
    );

    let mut converter =
        CharsetConverter::from_codecs(Utf8Codec, StatefulReplacementCodec::new(ReplacementTransition::Reset));
    let mut converter_output = [0_u8; 16];
    let raw = converter
        .transcode_complete_into("中".as_bytes(), &mut converter_output)
        .expect_err("replacement is unencodable after reset");
    let TranscodeConvertError::EncodeDomain(error) = raw else {
        panic!("expected a target encode-domain error");
    };
    assert_eq!(0, error.source().index());
    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter { value: 0xfffd },
        error.source().kind()
    );
    assert_eq!([0_u8; 16], converter_output);
    let mapped = converter.map_transcode_error(TranscodeConvertError::EncodeDomain(error));
    let CharsetConvertError::Encode(mapped) = mapped else {
        panic!("expected a target encode error");
    };
    assert_eq!(0, mapped.index());
    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter { value: 0xfffd },
        mapped.kind()
    );
}

#[test]
fn test_stateful_replacement_backpressure_precedes_replacement_error() {
    let mut encoder = CharsetEncoder::new(StatefulReplacementCodec::new(ReplacementTransition::Reset));
    encoder.reset(&mut [], 0).expect("reset");
    let mut empty = [];
    let progress = encoder
        .transcode(&['中'], 0, &mut empty, 0)
        .expect("backpressure is progress");
    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
    assert_eq!((0, 0), (progress.read(), progress.written()));
    let mut encoder_output = [0x55_u8; 4];
    let error = encoder
        .transcode(&['中'], progress.read(), &mut encoder_output, 0)
        .expect_err("replacement is unencodable after output becomes available");
    let TranscodeEncodeError::Domain(error) = error else {
        panic!("expected a charset-domain error");
    };
    assert_eq!(0, error.source().index());
    assert_eq!([0x55; 4], encoder_output);

    let mut converter =
        CharsetConverter::from_codecs(Utf8Codec, StatefulReplacementCodec::new(ReplacementTransition::Reset));
    converter.reset(&mut [], 0).expect("reset");
    let mut empty = [];
    let converter_input = "中".as_bytes();
    let progress = converter
        .transcode(converter_input, 0, &mut empty, 0)
        .expect("backpressure is progress");
    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
    assert_eq!((3, 0), (progress.read(), progress.written()));
    let mut converter_output = [0x55_u8; 4];
    let error = converter
        .transcode(converter_input, progress.read(), &mut converter_output, 0)
        .expect_err("replacement is unencodable after output becomes available");
    let TranscodeConvertError::EncodeDomain(error) = error else {
        panic!("expected a target encode-domain error, got {error:?}");
    };
    assert_eq!(0, error.source().index());
    assert_eq!([0x55; 4], converter_output);
}
