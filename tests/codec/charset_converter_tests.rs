use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetConvertError,
    CharsetConverter,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetDecoder,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
    CharsetEncoder,
    Coder,
    CoderStatus,
    DecodeStatus,
    MalformedAction,
    UnmappableAction,
    Utf8Codec,
    Utf16U16Codec,
};

#[derive(Clone, Copy, Debug, Default)]
struct AsciiBytesCodec;

impl CharsetCodec for AsciiBytesCodec {
    type Unit = u8;
    fn charset(&self) -> Charset {
        Charset::ASCII
    }

    fn max_units_per_char(&self) -> usize {
        1
    }

    fn decode_one(&self, input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        if index >= input.len() {
            return Ok(DecodeStatus::NeedMore {
                required: index + 1,
                available: input.len().saturating_sub(index),
            });
        }
        let value = input[index];
        if value > 0x7f {
            let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, index));
        }
        Ok(DecodeStatus::Complete {
            value: value as char,
            consumed: 1,
        })
    }

    fn encode_one(&self, ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        if !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        if index >= output.len() {
            let kind = CharsetEncodeErrorKind::BufferTooSmall {
                required: index + 1,
                available: 0,
            };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        output[index] = ch as u8;
        Ok(1)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct ZeroWidthCodec;

impl CharsetCodec for ZeroWidthCodec {
    type Unit = u8;
    fn charset(&self) -> Charset {
        Charset::ASCII
    }

    fn max_units_per_char(&self) -> usize {
        0
    }

    fn decode_one(&self, _input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
        Err(CharsetDecodeError::new(Charset::ASCII, kind, index))
    }

    fn encode_one(&self, _ch: char, _output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + 1,
            available: 0,
        };
        Err(CharsetEncodeError::new(Charset::ASCII, kind, index))
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct NeedOutputNoReadCodec;

impl CharsetCodec for NeedOutputNoReadCodec {
    type Unit = u8;

    fn charset(&self) -> Charset {
        Charset::ASCII
    }

    fn max_units_per_char(&self) -> usize {
        1
    }

    fn decode_one(&self, input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        Ok(DecodeStatus::NeedMore {
            required: input.len() + 1,
            available: input.len().saturating_sub(index),
        })
    }

    fn encode_one(&self, _ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        if index >= output.len() {
            let kind = CharsetEncodeErrorKind::BufferTooSmall {
                required: index + 1,
                available: 0,
            };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }

        output[index] = 0x41;
        Ok(1)
    }
}

#[test]
fn test_charset_converter_exposes_configuration_and_bounds() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);

    assert_eq!(Charset::UTF_8, converter.decoder().codec().charset());
    assert_eq!(Charset::UTF_16, converter.encoder().codec().charset());
    assert_eq!(Some(6), converter.max_output_len(3));

    converter.decoder_mut().set_malformed_action(MalformedAction::Ignore);
    converter.encoder_mut().set_unmappable_action(UnmappableAction::Ignore);

    assert_eq!(MalformedAction::Ignore, converter.decoder().malformed_action());
    assert_eq!(UnmappableAction::Ignore, converter.encoder().unmappable_action());

    converter.reset();
}

#[test]
fn test_charset_converter_from_codecs_uses_default_policies() {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);

    let mut output = [0_u16; 1];
    let progress = converter
        .convert("A".as_bytes(), 0, &mut output, 0)
        .expect("converter constructed from code pair");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!('A' as u16, output[0]);
}

#[test]
fn test_charset_converter_convert_empty_without_input_is_complete() {
    let mut converter = CharsetConverter::new(CharsetDecoder::new(Utf8Codec), CharsetEncoder::new(Utf16U16Codec));
    let progress = converter
        .convert(b"", 0, &mut [0_u16; 0], 0)
        .expect("empty input returns complete");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());
}

#[test]
fn test_charset_converter_combines_decoder_and_encoder_with_offsets() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let input = "A中".as_bytes();
    let mut output = [0_u16; 1];

    let progress = converter
        .convert(input, 1, &mut output, 0)
        .expect("UTF-8 to UTF-16 conversion");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!('中' as u16, output[0]);
}

#[test]
fn test_charset_converter_reports_invalid_input_index() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let input = b"A";
    let mut output = [0_u16; 1];

    let error = converter
        .convert(input, input.len() + 1, &mut output, 0)
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
fn test_charset_converter_finish_without_pending_returns_complete() {
    let mut converter = CharsetConverter::new(CharsetDecoder::new(Utf8Codec), CharsetEncoder::new(Utf16U16Codec));
    let mut output = [0_u16; 1];

    let progress = converter.finish(&mut output, 0).expect("finish without pending");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());
}

#[test]
fn test_charset_converter_keeps_pending_character_when_output_is_full() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let mut empty_output = [];

    let progress = converter
        .convert(b"A", 0, &mut empty_output, 0)
        .expect("decoded character stays pending");

    assert!(matches!(progress.status(), CoderStatus::NeedOutput { .. }));
    assert_eq!(1, progress.read());
    assert_eq!(0, progress.written());

    let progress = converter
        .convert(b"", 0, &mut empty_output, 0)
        .expect("pending character still needs output");

    assert!(matches!(progress.status(), CoderStatus::NeedOutput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());

    let mut output = [0_u16; 1];
    let progress = converter
        .convert(b"", 0, &mut output, 0)
        .expect("pending character is written before reading more input");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(0, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!('A' as u16, output[0]);
}

#[test]
fn test_charset_converter_resets_pending_state() {
    let mut converter = CharsetConverter::new(CharsetDecoder::new(Utf8Codec), CharsetEncoder::new(Utf16U16Codec));
    let mut empty_output = [];
    converter
        .convert(b"A", 0, &mut empty_output, 0)
        .expect("converted char becomes pending");

    converter.reset();

    let mut output = [0_u16; 1];
    let progress = converter
        .convert(b"Z", 0, &mut output, 0)
        .expect("reset removes pending state");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!('Z' as u16, output[0]);
}

#[test]
fn test_charset_converter_finish_flushes_pending_character() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);

    let mut empty_output = [];
    let progress = converter
        .convert(b"A", 0, &mut empty_output, 0)
        .expect("converted char becomes pending when output is full");
    assert!(matches!(progress.status(), CoderStatus::NeedOutput { .. }));

    let mut output = [0_u16; 1];
    let finish = converter
        .finish(&mut output, 0)
        .expect("finish flushes pending character");
    assert_eq!(CoderStatus::Complete, finish.status());
    assert_eq!(1, finish.written());
    assert_eq!('A' as u16, output[0]);
}

#[test]
fn test_charset_converter_finish_needs_output_when_target_buffer_is_empty() {
    let mut converter = CharsetConverter::new(CharsetDecoder::new(Utf8Codec), CharsetEncoder::new(Utf16U16Codec));

    let mut empty_output = [];
    let progress = converter
        .convert(b"A", 0, &mut empty_output, 0)
        .expect("first conversion keeps character pending");
    assert!(matches!(progress.status(), CoderStatus::NeedOutput { .. }));

    let finish = converter
        .finish(&mut empty_output, 0)
        .expect("finish needs output capacity");
    assert!(matches!(finish.status(), CoderStatus::NeedOutput { .. }));
    assert_eq!(0, finish.read());
    assert_eq!(0, finish.written());
}

#[test]
fn test_charset_converter_decoder_need_output_without_reading_input() {
    let mut converter = CharsetConverter::new(
        CharsetDecoder::new(NeedOutputNoReadCodec),
        CharsetEncoder::new(Utf16U16Codec),
    );
    let progress = converter
        .convert(b"A", 0, &mut [0_u16; 2], 0)
        .expect("decoder reports need input without consuming input");

    assert!(matches!(progress.status(), CoderStatus::NeedInput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());
}

#[test]
fn test_charset_converter_continues_after_decoder_fills_char_buffer() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let mut output = [0_u16; 2];

    let progress = converter
        .convert(b"AB", 0, &mut output, 0)
        .expect("converter loops after decoder reports char-buffer output full");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(2, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(['A' as u16, 'B' as u16], output);
}

#[test]
fn test_charset_converter_continues_after_decoder_chunk_overflow() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let mut output = [0_u16; 5];

    let progress = converter
        .convert(b"ABCDE", 0, &mut output, 0)
        .expect("decode one buffered chunk then continue");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(5, progress.read());
    assert_eq!(5, progress.written());
    assert_eq!(['A' as u16, 'B' as u16, 'C' as u16, 'D' as u16, 'E' as u16], output);
}

#[test]
fn test_charset_converter_reports_need_input_from_decoder() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let mut output = [0_u16; 1];

    let progress = converter
        .convert(&[0xe4], 0, &mut output, 0)
        .expect("partial source sequence needs more input");

    assert!(matches!(progress.status(), CoderStatus::NeedInput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());
}

#[test]
fn test_charset_converter_propagates_decode_and_encode_errors() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    decoder.set_malformed_action(MalformedAction::Report);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let mut output = [0_u16; 1];

    let error = converter
        .convert(&[0x80], 0, &mut output, 0)
        .expect_err("malformed source input is reported");
    assert!(matches!(error, CharsetConvertError::Decode(_)));

    let decoder = CharsetDecoder::new(Utf8Codec);
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);
    encoder.set_unmappable_action(UnmappableAction::Report);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let mut ascii_output = [0_u8; 1];

    let error = converter
        .convert("é".as_bytes(), 0, &mut ascii_output, 0)
        .expect_err("unmappable target character is reported");
    assert!(matches!(error, CharsetConvertError::Encode(_)));
}

#[test]
fn test_charset_converter_returns_need_output_when_source_decoder_buffer_is_exhausted_immediately() {
    let decoder = CharsetDecoder::new(ZeroWidthCodec);
    let encoder = CharsetEncoder::new(AsciiBytesCodec);
    let mut converter = CharsetConverter::new(decoder, encoder);

    let progress = converter
        .convert(b"A", 0, &mut [0_u8; 1], 0)
        .expect("source decoder should report no internal output space");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
}
