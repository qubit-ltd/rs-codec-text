// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::num::NonZeroUsize;

use qubit_codec::ByteOrder;
use qubit_codec::Codec;
use qubit_codec::DecodeFailure;
use qubit_codec::TranscodeDecodeError;
use qubit_codec::TranscodeStatus;
use qubit_codec_text::Charset;
use qubit_codec_text::CharsetCodec;
use qubit_codec_text::CharsetDecodeError;
use qubit_codec_text::CharsetDecodeErrorKind;
use qubit_codec_text::CharsetDecodePolicy;
use qubit_codec_text::CharsetDecoder;
use qubit_codec_text::CharsetEncodeError;
use qubit_codec_text::CharsetEncodeResult;
use qubit_codec_text::Utf8Codec;
use qubit_codec_text::Utf16ByteCodec;
use qubit_codec_text::Utf32ByteCodec;

#[derive(Clone, Copy, Debug)]
struct WidthProbeCodec {
    inner: NonZeroUsize,
    outer: NonZeroUsize,
}

impl CharsetCodec for WidthProbeCodec {
    fn charset(&self) -> Charset {
        Charset::ISO_8859_1
    }
}

impl Codec for WidthProbeCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = 1;
    const MAX_DECODE_UNITS_PER_VALUE: usize = 2;
    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;

    fn can_encode_value(&self, value: &char) -> bool {
        (*value as u32) <= 0xff
    }

    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> Result<(char, NonZeroUsize), DecodeFailure<CharsetDecodeError>> {
        if input[input_index..].starts_with(&[0xff, 0xfe]) {
            let error = CharsetDecodeError::new(self.charset(), CharsetDecodeErrorKind::malformed(0xff), input_index)
                .with_consumed(self.inner);
            return Err(DecodeFailure::invalid(error, self.outer));
        }
        Ok((input[input_index] as char, NonZeroUsize::MIN))
    }

    unsafe fn encode(&mut self, value: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
        output[output_index] = *value as u8;
        Ok(1)
    }
}

#[test]
fn test_charset_decode_hooks_apply_the_configured_malformed_action() {
    let input = [b'A', 0x80, b'B'];

    let mut replace = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::replace('!'));
    let mut replace_output = ['\0'; 3];
    let written = replace
        .transcode_complete_into(&input, &mut replace_output)
        .expect("replacement policy should decode malformed input");
    assert_eq!(['A', '!', 'B'], replace_output[..written]);

    let mut ignore = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::ignore());
    let mut ignore_output = ['\0'; 3];
    let written = ignore
        .transcode_complete_into(&input, &mut ignore_output)
        .expect("ignore policy should skip malformed input");
    assert_eq!(['A', 'B'], ignore_output[..written]);

    let mut report = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());
    let mut report_output = ['\0'; 3];
    let error = report
        .transcode_complete_into(&input, &mut report_output)
        .expect_err("report policy should reject malformed input");
    let TranscodeDecodeError::Domain(error) = error else {
        panic!("report policy should return a charset-domain error");
    };
    assert!(matches!(
        error.into_source().kind(),
        CharsetDecodeErrorKind::MalformedSequence { .. }
    ));
    assert_eq!(1, error.into_source().index());
}

#[test]
fn test_charset_decode_recovers_using_synchronized_consumed_width() {
    let two = NonZeroUsize::new(2).expect("non-zero width");
    let policies = [
        ("replace", CharsetDecodePolicy::default()),
        ("ignore", CharsetDecodePolicy::ignore()),
        ("report", CharsetDecodePolicy::report()),
    ];
    for (policy_name, policy) in policies {
        let codec = WidthProbeCodec { inner: two, outer: two };
        let mut decoder = CharsetDecoder::with_policy(codec, policy);
        let mut output = ['\0'; 3];
        assert_eq!(0, decoder.reset(&mut [], 0).expect("reset"));
        let result = decoder.transcode(&[0xff, 0xfe, b'A'], 0, &mut output, 0);
        if policy_name == "report" {
            let error = result.expect_err("report policy should reject malformed input");
            let TranscodeDecodeError::Domain(domain) = error else {
                panic!("expected charset-domain error");
            };
            assert_eq!(Some(two), domain.input_consumed());
            let source = domain.into_source();
            assert_eq!(CharsetDecodeErrorKind::malformed(0xff), source.kind());
            assert_eq!(0, source.index());
            assert_eq!(Some(two), source.consumed());
        } else {
            let progress = result.expect("non-strict policy should recover from malformed input");
            assert_eq!(TranscodeStatus::Complete, progress.status());
            assert_eq!(3, progress.read());
            let expected: &[char] = if policy_name == "replace" {
                &['\u{fffd}', 'A']
            } else {
                &['A']
            };
            assert_eq!(expected, &output[..progress.written()]);
        }
    }
    let error =
        CharsetDecodeError::new(Charset::ISO_8859_1, CharsetDecodeErrorKind::malformed(0xff), 0).with_consumed(two);
    let failure = error.into_codec_failure();
    assert_eq!(Some(two), failure.consumed_units());
    assert_eq!(Some(two), failure.invalid_source().unwrap().consumed());
}

#[test]
fn test_report_preserves_consumed_width_for_builtin_byte_codecs() {
    assert_decode_report(
        Utf16ByteCodec::new(ByteOrder::LittleEndian),
        &[0x00, 0xdc],
        2,
        Charset::UTF_16LE,
        CharsetDecodeErrorKind::malformed(0xdc00),
    );
    assert_decode_report(
        Utf16ByteCodec::new(ByteOrder::BigEndian),
        &[0xdc, 0x00],
        2,
        Charset::UTF_16BE,
        CharsetDecodeErrorKind::malformed(0xdc00),
    );
    assert_decode_report(
        Utf32ByteCodec::new(ByteOrder::LittleEndian),
        &[0, 0, 0x11, 0],
        4,
        Charset::UTF_32LE,
        CharsetDecodeErrorKind::InvalidCodePoint { value: 0x11_0000 },
    );
    assert_decode_report(
        Utf32ByteCodec::new(ByteOrder::BigEndian),
        &[0, 0x11, 0, 0],
        4,
        Charset::UTF_32BE,
        CharsetDecodeErrorKind::InvalidCodePoint { value: 0x11_0000 },
    );
}

fn assert_decode_report<C>(codec: C, bytes: &[u8], width: usize, charset: Charset, kind: CharsetDecodeErrorKind)
where
    C: Copy + CharsetCodec + Codec<Value = char, Unit = u8, DecodeError = CharsetDecodeError>,
{
    for start in [0, 1] {
        let mut input = vec![0xaa; start];
        input.extend_from_slice(bytes);
        let mut decoder = CharsetDecoder::with_policy(codec, CharsetDecodePolicy::report());
        let mut output = ['\0'; 1];
        assert_eq!(0, decoder.reset(&mut [], 0).expect("reset"));
        let error = decoder
            .transcode(&input, start, &mut output, 0)
            .expect_err("report policy rejects malformed input");
        let TranscodeDecodeError::Domain(domain) = error else {
            panic!("expected charset-domain error");
        };
        let expected_width = NonZeroUsize::new(width).expect("non-zero width");
        assert_eq!(Some(start), domain.input_index());
        assert_eq!(Some(expected_width), domain.input_consumed());
        let source = domain.into_source();
        assert_eq!(start, source.index());
        assert_eq!(Some(expected_width), source.consumed());
        assert_eq!(charset, source.charset());
        assert_eq!(kind, source.kind());
    }
}
