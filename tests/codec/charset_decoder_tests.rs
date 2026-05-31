use qubit_codec::BufferedDecoder;
use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodePolicy,
    CharsetDecodeResult,
    CharsetDecoder,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
    Codec,
    MalformedAction,
    TranscodeStatus,
    Transcoder,
    Utf8Codec,
    Utf32U32Codec,
};

#[derive(Clone, Copy, Debug, Default)]
struct InvalidInputErrorCodec;

impl CharsetCodec for InvalidInputErrorCodec {
    type Unit = u8;

    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

unsafe impl Codec<char, u8> for InvalidInputErrorCodec {
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    unsafe fn decode_unchecked(
        &self,
        _input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: 0 };
        Err(CharsetDecodeError::new(Charset::ASCII, kind, index))
    }

    unsafe fn encode_unchecked(&self, _value: &char, _output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: index as u32 };
        Err(CharsetEncodeError::new(Charset::ASCII, kind, index))
    }
}

#[test]
fn test_charset_decoder_is_buffered_decoder() {
    fn assert_buffered_decoder<T: BufferedDecoder<u8, char>>() {}

    assert_buffered_decoder::<CharsetDecoder<Utf8Codec>>();
}

#[derive(Clone, Copy, Debug, Default)]
struct PendingInvalidInputErrorCodec;

impl CharsetCodec for PendingInvalidInputErrorCodec {
    type Unit = u8;

    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

unsafe impl Codec<char, u8> for PendingInvalidInputErrorCodec {
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        unsafe { core::num::NonZeroUsize::new_unchecked(2) }
    }

    unsafe fn decode_unchecked(
        &self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        if input.len().saturating_sub(index) == 1 {
            let kind = CharsetDecodeErrorKind::IncompleteSequence {
                required: 2,
                available: 1,
            };
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, index));
        }
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: 0 };
        Err(CharsetDecodeError::new(Charset::ASCII, kind, index))
    }

    unsafe fn encode_unchecked(&self, _value: &char, _output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: index as u32 };
        Err(CharsetEncodeError::new(Charset::ASCII, kind, index))
    }
}

#[test]
fn test_charset_decoder_exposes_configuration_and_bounds() {
    let decoder = CharsetDecoder::new(Utf8Codec);

    assert_eq!(MalformedAction::Replace, decoder.malformed_action());
    assert_eq!('\u{fffd}', decoder.replacement());
    assert_eq!(Ok(3), decoder.max_output_len(3));
    assert_eq!(Ok(0), decoder.max_finish_output_len());

    let decoder_with_replacement = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::replace('!'));
    assert_eq!('!', decoder_with_replacement.replacement());

    let decoder = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::ignore_with_replacement('?'));

    assert_eq!('?', decoder.replacement());
    assert_eq!(MalformedAction::Ignore, decoder.malformed_action());
}

#[test]
fn test_charset_decoder_reports_need_output_after_partial_progress() {
    let mut decoder = CharsetDecoder::new(Utf32U32Codec);
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&['A' as u32, 'B' as u32], 0, &mut output, 0)
        .expect("second scalar needs more output");

    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!('A', output[0]);
}

#[test]
fn test_charset_decoder_leaves_incomplete_input_to_caller_across_chunks() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = ['\0'; 2];

    let progress = decoder
        .transcode(&[0xe4], 0, &mut output, 0)
        .expect("first byte needs more input");
    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());

    let progress = decoder
        .transcode(&[0xe4, 0xb8], 0, &mut output, 0)
        .expect("caller-preserved prefix still needs input");
    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());

    let progress = decoder
        .transcode(&[0xe4, 0xb8, 0xad, b'!'], 0, &mut output, 0)
        .expect("third byte completes the scalar and ASCII tail");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(4, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(['中', '!'], output);

    let finish = decoder.finish(&mut output, 0).expect("finish has no pending tail");
    assert_eq!(TranscodeStatus::Complete, finish.status());
    assert_eq!(0, finish.written());
}

#[test]
fn test_charset_decoder_applies_policy_to_available_malformed_input() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = ['\0'; 4];

    let progress = decoder
        .transcode(&[0x80], 0, &mut output, 0)
        .expect("malformed byte is replaced immediately");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(CharsetDecodePolicy::DEFAULT_REPLACEMENT, output[0]);

    let mut decoder = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::ignore());
    let progress = decoder
        .transcode(&[0x80], 0, &mut output, 0)
        .expect("malformed byte is ignored immediately");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(0, progress.written());

    let mut decoder = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());
    let error = decoder
        .transcode(&[0x80], 0, &mut output, 0)
        .expect_err("reported malformed byte should fail");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0x80) },
        error.kind()
    );
    assert_eq!(0, error.index());
}

#[test]
fn test_charset_decoder_decodes_short_ascii_without_waiting_for_finish() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = ['\0'; 2];

    let progress = decoder
        .transcode(b"AB", 0, &mut output, 0)
        .expect("ASCII bytes decode without waiting for max UTF-8 width");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(2, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(['A', 'B'], output);
    assert_eq!(Ok(0), decoder.max_finish_output_len());

    let finish = decoder.finish(&mut output, 0).expect("EOF has no buffered ASCII tail");

    assert_eq!(TranscodeStatus::Complete, finish.status());
    assert_eq!(0, finish.read());
    assert_eq!(0, finish.written());
    assert_eq!(Ok(0), decoder.max_finish_output_len());
}

#[test]
fn test_charset_decoder_decodes_all_available_ascii_units() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = ['\0'; 4];

    let progress = decoder
        .transcode(b"ABCD", 0, &mut output, 0)
        .expect("ASCII units decode without a conservative max-width tail");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(4, progress.read());
    assert_eq!(4, progress.written());
    assert_eq!(['A', 'B', 'C', 'D'], output);

    let finish = decoder.finish(&mut output, 0).expect("finish has no buffered tail");
    assert_eq!(0, finish.written());
}

#[test]
fn test_charset_decoder_replaces_reports_and_ignores_malformed_input() {
    let input = [b'A', 0x80, b'B', b'C', b'D'];
    let mut output = ['\0'; 5];
    let mut decoder = CharsetDecoder::new(Utf8Codec);

    let progress = decoder
        .transcode(&input, 0, &mut output, 0)
        .expect("default malformed action replaces");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(5, progress.read());
    assert_eq!(5, progress.written());
    assert_eq!(['A', '\u{fffd}', 'B', 'C', 'D'], output);

    let mut decoder = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::ignore());
    let mut ignored_output = ['\0'; 4];
    let progress = decoder
        .transcode(&input, 0, &mut ignored_output, 0)
        .expect("ignore malformed input");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(4, progress.written());
    assert_eq!(['A', 'B', 'C', 'D'], ignored_output);

    let mut decoder = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());
    let error = decoder
        .transcode(&input[1..], 0, &mut output, 0)
        .expect_err("report malformed input");

    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0x80) },
        error.kind()
    );
    assert_eq!(0, error.index());
}

#[test]
fn test_charset_decoder_reports_invalid_indices_capacity_and_need_input() {
    let input = b"AB";
    let mut output = ['\0'; 1];
    let mut decoder = CharsetDecoder::new(Utf8Codec);

    let error = decoder
        .transcode(input, input.len() + 1, &mut output, 0)
        .expect_err("input index outside input slice");
    assert_eq!(
        CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() },
        error.kind()
    );
    assert_eq!(input.len() + 1, error.index());

    let beyond_output = output.len() + 1;
    let progress = decoder
        .transcode(input, 0, &mut output, beyond_output)
        .expect("output index beyond output slice needs more output");
    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());

    let progress = decoder
        .transcode(&[0xe4], 0, &mut output, 0)
        .expect("short prefix needs input before EOF");
    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());
}

#[test]
fn test_charset_decoder_finish_does_not_replace_incomplete_input() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0xe4, 0xb8], 0, &mut output, 0)
        .expect("partial UTF-8 prefix needs input");

    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(Ok(0), decoder.max_finish_output_len());

    let finish = decoder
        .finish(&mut output, 0)
        .expect("finish does not process caller-owned incomplete input");

    assert_eq!(TranscodeStatus::Complete, finish.status());
    assert_eq!(0, finish.read());
    assert_eq!(0, finish.written());
    assert_eq!('\0', output[0]);
    assert_eq!(Ok(0), decoder.max_finish_output_len());
}

#[test]
fn test_charset_decoder_finish_without_pending_input_is_complete() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = ['\0'; 1];

    let finish = decoder
        .finish(&mut output, 0)
        .expect("finish without pending input is complete");

    assert_eq!(TranscodeStatus::Complete, finish.status());
    assert_eq!(0, finish.read());
    assert_eq!(0, finish.written());
    assert_eq!(Ok(0), decoder.max_finish_output_len());
}

#[test]
fn test_charset_decoder_finish_ignores_output_capacity_for_caller_owned_tail() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = [];

    let progress = decoder
        .transcode(&[0xe4], 0, &mut ['\0'; 1], 0)
        .expect("partial UTF-8 prefix needs input");

    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));

    let finish = decoder
        .finish(&mut output, 0)
        .expect("finish has no decoder-owned replacement output");

    assert_eq!(TranscodeStatus::Complete, finish.status());
    assert_eq!(0, finish.read());
    assert_eq!(0, finish.written());
    assert_eq!(Ok(0), decoder.max_finish_output_len());
}

#[test]
fn test_charset_decoder_finish_ignores_incomplete_input() {
    let mut decoder = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::ignore());
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0xe4], 0, &mut output, 0)
        .expect("partial UTF-8 prefix needs input");

    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
    assert_eq!(Ok(0), decoder.max_finish_output_len());

    let finish = decoder
        .finish(&mut output, 0)
        .expect("ignore policy drops incomplete EOF input");

    assert_eq!(TranscodeStatus::Complete, finish.status());
    assert_eq!(0, finish.read());
    assert_eq!(0, finish.written());
    assert_eq!(Ok(0), decoder.max_finish_output_len());
}

#[test]
fn test_charset_decoder_finish_does_not_report_incomplete_input() {
    let mut decoder = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0xe4, 0xb8], 0, &mut output, 0)
        .expect("partial UTF-8 prefix needs input");

    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
    assert_eq!(Ok(0), decoder.max_finish_output_len());

    let finish = decoder
        .finish(&mut output, 0)
        .expect("finish does not process caller-owned incomplete input");

    assert_eq!(TranscodeStatus::Complete, finish.status());
    assert_eq!(0, finish.written());
}

#[test]
fn test_charset_decoder_reset_clears_incomplete_input() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0xe4], 0, &mut output, 0)
        .expect("partial UTF-8 prefix needs input");

    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
    assert_eq!(Ok(0), decoder.max_finish_output_len());

    decoder.reset();
    let finish = decoder
        .finish(&mut output, 0)
        .expect("reset removes pending incomplete input");

    assert_eq!(TranscodeStatus::Complete, finish.status());
    assert_eq!(0, finish.written());
    assert_eq!(Ok(0), decoder.max_finish_output_len());
}

#[test]
fn test_charset_decoder_propagates_non_policy_errors_from_caller_preserved_input() {
    let mut decoder = CharsetDecoder::new(PendingInvalidInputErrorCodec);
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0], 0, &mut output, 0)
        .expect("first unit is caller-owned incomplete input");
    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));

    let error = decoder
        .transcode(&[0, 1], 0, &mut output, 0)
        .expect_err("non-policy error should propagate from caller-preserved input");
    assert!(matches!(error.kind(), CharsetDecodeErrorKind::InvalidInputIndex { .. }));
    assert_eq!(0, error.index());
}

#[test]
fn test_charset_decoder_finish_ignores_caller_owned_incomplete_error() {
    let mut decoder = CharsetDecoder::with_policy(PendingInvalidInputErrorCodec, CharsetDecodePolicy::report());
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0], 0, &mut output, 0)
        .expect("first unit is caller-owned incomplete input");
    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));

    let finish = decoder
        .finish(&mut output, 0)
        .expect("finish does not process caller-owned incomplete input");
    assert_eq!(TranscodeStatus::Complete, finish.status());
    assert_eq!(0, finish.written());
}

#[test]
fn test_charset_decoder_replaces_invalid_scalars() {
    let mut utf32_decoder = CharsetDecoder::new(Utf32U32Codec);
    let mut scalar_output = ['\0'; 1];
    let progress = utf32_decoder
        .transcode(&[0x110000], 0, &mut scalar_output, 0)
        .expect("invalid scalar is replaced by default");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!('\u{fffd}', scalar_output[0]);
}

#[test]
fn test_charset_decoder_propagates_non_policy_decoding_errors() {
    let input = [0_u8];
    let mut output = ['\0'; 1];
    let mut decoder = CharsetDecoder::new(InvalidInputErrorCodec);

    let error = decoder
        .transcode(&input, 0, &mut output, 0)
        .expect_err("invalid-index error is not absorbed");

    assert!(matches!(error.kind(), CharsetDecodeErrorKind::InvalidInputIndex { .. },));
    assert_eq!(0, error.index());
}
