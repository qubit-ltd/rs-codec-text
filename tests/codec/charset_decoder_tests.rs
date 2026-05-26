use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetDecoder,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
    Coder,
    CoderStatus,
    DecodeStatus,
    MalformedAction,
    Utf8Codec,
    Utf32U32Codec,
};

#[derive(Clone, Copy, Debug, Default)]
struct IncompleteErrorCodec;

impl CharsetCodec for IncompleteErrorCodec {
    type Unit = u8;
    fn charset(&self) -> Charset {
        Charset::ASCII
    }

    fn max_units_per_char(&self) -> usize {
        1
    }

    fn decode_one(&self, _input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        let kind = CharsetDecodeErrorKind::IncompleteSequence {
            required: index + 1,
            available: 0,
        };
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

#[derive(Clone, Copy, Debug)]
struct InvalidDecodeStatusCodec {
    status: DecodeStatus,
}

impl CharsetCodec for InvalidDecodeStatusCodec {
    type Unit = u8;

    fn charset(&self) -> Charset {
        Charset::ASCII
    }

    fn max_units_per_char(&self) -> usize {
        1
    }

    fn decode_one(&self, _input: &[u8], _index: usize) -> CharsetDecodeResult<DecodeStatus> {
        Ok(self.status)
    }

    fn encode_one(&self, _ch: char, _output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + 1,
            available: 0,
        };
        Err(CharsetEncodeError::new(Charset::ASCII, kind, index))
    }
}

#[test]
fn test_charset_decoder_exposes_configuration_and_bounds() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);

    assert_eq!(Charset::UTF_8, decoder.codec().charset());
    assert_eq!(Charset::UTF_8, decoder.codec_mut().charset());
    assert_eq!(MalformedAction::Replace, decoder.malformed_action());
    assert_eq!('\u{fffd}', decoder.replacement());
    assert_eq!(Some(3), decoder.max_output_len(3));

    let decoder_with_replacement = CharsetDecoder::new(Utf8Codec).with_replacement('!');
    assert_eq!('!', decoder_with_replacement.replacement());

    decoder.set_replacement('?');
    decoder.set_malformed_action(MalformedAction::Ignore);

    assert_eq!('?', decoder.replacement());
    assert_eq!(MalformedAction::Ignore, decoder.malformed_action());
}

#[test]
fn test_charset_decoder_replaces_reports_and_ignores_malformed_input() {
    let input = [b'A', 0x80, b'B'];
    let mut output = ['\0'; 3];
    let mut decoder = CharsetDecoder::new(Utf8Codec);

    let progress = decoder
        .convert(&input, 0, &mut output, 0)
        .expect("default malformed action replaces");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(3, progress.written());
    assert_eq!(['A', '\u{fffd}', 'B'], output);

    decoder.set_malformed_action(MalformedAction::Ignore);
    let mut ignored_output = ['\0'; 2];
    let progress = decoder
        .convert(&input, 0, &mut ignored_output, 0)
        .expect("ignore malformed input");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(['A', 'B'], ignored_output);

    decoder.set_malformed_action(MalformedAction::Report);
    let error = decoder
        .convert(&input, 1, &mut output, 0)
        .expect_err("report malformed input");

    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0x80) },
        error.kind()
    );
    assert_eq!(1, error.index());
}

#[test]
fn test_charset_decoder_reports_invalid_indices_capacity_and_need_input() {
    let input = b"AB";
    let mut output = ['\0'; 1];
    let mut decoder = CharsetDecoder::new(Utf8Codec);

    let error = decoder
        .convert(input, input.len() + 1, &mut output, 0)
        .expect_err("input index outside input slice");
    assert_eq!(
        CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() },
        error.kind()
    );
    assert_eq!(input.len() + 1, error.index());

    let beyond_output = output.len() + 1;
    let progress = decoder
        .convert(input, 0, &mut output, beyond_output)
        .expect("output index beyond output slice needs more output");
    assert!(matches!(progress.status(), CoderStatus::NeedOutput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());

    let progress = decoder
        .convert(input, 0, &mut output, 0)
        .expect("decoder stops when output buffer fills");
    assert!(matches!(progress.status(), CoderStatus::NeedOutput { .. }));
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());

    let progress = decoder
        .convert(&[0xe4], 0, &mut output, 0)
        .expect("incomplete UTF-8 prefix needs input");
    assert!(matches!(progress.status(), CoderStatus::NeedInput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());
}

#[test]
fn test_charset_decoder_replaces_malformed_ranges_and_invalid_scalars() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    decoder.set_replacement('?');
    let mut output = ['\0'; 2];

    let progress = decoder
        .convert(&[0xe4, b' ', b'A'], 0, &mut output, 0)
        .expect("invalid continuation is one malformed range");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(['?', 'A'], output);

    let mut utf32_decoder = CharsetDecoder::new(Utf32U32Codec);
    let mut scalar_output = ['\0'; 1];
    let progress = utf32_decoder
        .convert(&[0x110000], 0, &mut scalar_output, 0)
        .expect("invalid scalar is replaced by default");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!('\u{fffd}', scalar_output[0]);
}

#[test]
fn test_charset_decoder_propagates_non_policy_decoding_errors() {
    let input = [0_u8];
    let mut output = ['\0'; 1];
    let mut decoder = CharsetDecoder::new(IncompleteErrorCodec);

    let error = decoder
        .convert(&input, 0, &mut output, 0)
        .expect_err("incomplete-sequence error is not absorbed");

    assert!(matches!(
        error.kind(),
        CharsetDecodeErrorKind::IncompleteSequence { .. },
    ));
    assert_eq!(0, error.index());
}

#[test]
#[should_panic(expected = "Complete with zero consumed units")]
fn test_charset_decoder_asserts_complete_consumes_input() {
    let mut output = ['\0'; 1];
    let mut decoder = CharsetDecoder::new(InvalidDecodeStatusCodec {
        status: DecodeStatus::Complete {
            value: 'A',
            consumed: 0,
        },
    });

    let _ = decoder.convert(b"A", 0, &mut output, 0);
}

#[test]
#[should_panic(expected = "required input length that is not beyond current input")]
fn test_charset_decoder_asserts_need_more_uses_absolute_required_length() {
    let mut output = ['\0'; 1];
    let mut decoder = CharsetDecoder::new(InvalidDecodeStatusCodec {
        status: DecodeStatus::NeedMore {
            required: 1,
            available: 2,
        },
    });

    let _ = decoder.convert(b"AB", 0, &mut output, 0);
}
