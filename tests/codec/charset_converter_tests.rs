use qubit_codec::{
    BufferedConverter,
    FinishError,
};
use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetConvertError,
    CharsetConverter,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodePolicy,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodePolicy,
    CharsetEncodeProbe,
    CharsetEncodeResult,
    Codec,
    TranscodeStatus,
    Transcoder,
    Utf8Codec,
    Utf16U16Codec,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct AsciiBytesCodec;

impl CharsetCodec for AsciiBytesCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for AsciiBytesCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
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

    unsafe fn decode_unchecked(
        &self,
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

    unsafe fn encode_unchecked(&self, value: &char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        let required = self.encode_len(*value, index)?;
        debug_assert!(index + required <= output.len());
        output[index] = *value as u8;
        Ok(required)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ReplacementFallbackCodec;

impl CharsetCodec for ReplacementFallbackCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for ReplacementFallbackCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if ch == '\u{fffd}' {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        if ch == '?' || ch.is_ascii() {
            return Ok(1);
        }
        let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
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

    unsafe fn decode_unchecked(
        &self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        unsafe { AsciiBytesCodec.decode_unchecked(input, index) }
    }

    unsafe fn encode_unchecked(&self, value: &char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        let required = self.encode_len(*value, index)?;
        debug_assert!(index + required <= output.len());
        output[index] = *value as u8;
        Ok(required)
    }
}

#[test]
fn test_charset_converter_is_buffered_converter() {
    fn assert_buffered_converter<T: BufferedConverter<u8, u16>>() {}

    assert_buffered_converter::<CharsetConverter<Utf8Codec, Utf16U16Codec>>();
}

#[test]
fn test_charset_converter_exposes_configuration_and_bounds() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);

    assert_eq!(Ok(6), converter.max_output_len(3));
    assert_eq!(Ok(0), converter.max_finish_output_len());

    converter.reset();
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

    let written = converter.finish(&mut output, 0).expect("finish has no buffered tail");
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
            'A' as u16, 'B' as u16, 'C' as u16, 'D' as u16, 'E' as u16, 'F' as u16, 'G' as u16, 'H' as u16, 'I' as u16,
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
        CharsetConvertError::Decode(error) => {
            assert_eq!(Charset::UTF_8, error.charset());
            assert_eq!(
                CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() },
                error.kind()
            );
            assert_eq!(input.len() + 1, error.index());
        }
        CharsetConvertError::Encode(_) => panic!("invalid source index must be reported as a decode error"),
    }
}

#[test]
fn test_charset_converter_keeps_pending_character_when_output_is_full() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let mut empty_output = [];

    let progress = converter
        .transcode(b"ABCD", 0, &mut empty_output, 0)
        .expect("decoded character stays pending");

    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
    assert_eq!(1, progress.read());
    assert_eq!(0, progress.written());
    assert_eq!(Ok(2), converter.max_finish_output_len());
    assert_eq!(Ok(8), converter.max_output_len(3));

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
    let mut empty_output = [];

    let progress = converter
        .transcode(b"ABCD", 0, &mut empty_output, 0)
        .expect("decoded source character cannot be written");
    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
    assert_eq!(1, progress.read());
    assert_eq!(0, progress.written());

    let error = converter
        .finish(&mut empty_output, 0)
        .expect_err("pending character still needs output at finish");
    assert_eq!(
        FinishError::InsufficientOutput {
            output_index: 0,
            required: 2,
            available: 0,
        },
        error,
    );
}

#[test]
fn test_charset_converter_finish_delegates_to_target_encoder() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let mut output = [];

    let error = converter
        .finish(&mut output, 1)
        .expect_err("target encoder reports out-of-range output index");
    assert_eq!(FinishError::InvalidOutputIndex { index: 1, len: 0 }, error);
}

#[test]
fn test_charset_converter_finish_writes_starting_pending_character() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
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
    let mut empty_output = [];
    converter
        .transcode(b"ABCD", 0, &mut empty_output, 0)
        .expect("converted char becomes pending");

    converter.reset();

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

    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
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

    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
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

    assert!(matches!(progress.status(), TranscodeStatus::NeedInput { .. }));
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
    assert!(matches!(error, CharsetConvertError::Decode(_)));

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
    assert!(matches!(error, CharsetConvertError::Encode(_)));
}

#[test]
fn test_charset_converter_falls_back_to_question_mark_when_default_replacement_is_unencodable() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, ReplacementFallbackCodec);
    let mut output = [0_u8; 1];

    let progress = converter
        .transcode("中".as_bytes(), 0, &mut output, 0)
        .expect("fallback replacement should be encodable");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(b'?', output[0]);
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
