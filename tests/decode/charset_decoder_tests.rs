use qubit_codec::{
    CapacityError,
    Codec,
    TranscodeDecoder,
    TranscodeError,
    TranscodeProgress,
    TranscodeStatus,
    Transcoder,
};
use qubit_codec_text::{
    BomDetectStatus,
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
    MalformedAction,
    UnicodeBom,
    Utf8Codec,
    Utf16ByteCodec,
    Utf32U32Codec,
};

#[derive(Clone, Copy, Debug, Default)]
struct InvalidInputErrorCodec;

impl CharsetCodec for InvalidInputErrorCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl Codec for InvalidInputErrorCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        input_index: usize,
    ) -> Result<
        (char, core::num::NonZeroUsize),
        qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
    > {
        let kind = CharsetDecodeErrorKind::malformed_unknown();
        Err(CharsetDecodeError::new(Charset::ASCII, kind, input_index)
            .into_codec_failure())
    }

    unsafe fn encode(
        &mut self,
        _value: &char,
        _output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        let kind = CharsetEncodeErrorKind::UnmappableCharacter {
            value: output_index as u32,
        };
        Err(CharsetEncodeError::new(Charset::ASCII, kind, output_index))
    }
}

#[test]
fn test_charset_decoder_is_transcode_decoder() {
    fn assert_transcode_decoder<T: TranscodeDecoder<u8, char>>() {}

    assert_transcode_decoder::<CharsetDecoder<Utf8Codec>>();
}

#[derive(Clone, Copy, Debug, Default)]
struct PendingInvalidInputErrorCodec;

impl CharsetCodec for PendingInvalidInputErrorCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl Codec for PendingInvalidInputErrorCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize = qubit_io::nz!(2);

    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> Result<
        (char, core::num::NonZeroUsize),
        qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
    > {
        if input.len().saturating_sub(input_index) == 1 {
            let kind = CharsetDecodeErrorKind::IncompleteSequence {
                required: 2,
                available: 1,
            };
            return Err(CharsetDecodeError::new(
                Charset::ASCII,
                kind,
                input_index,
            )
            .into_codec_failure());
        }
        let kind = CharsetDecodeErrorKind::malformed_unknown();
        Err(CharsetDecodeError::new(Charset::ASCII, kind, input_index)
            .into_codec_failure())
    }

    unsafe fn encode(
        &mut self,
        _value: &char,
        _output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        let kind = CharsetEncodeErrorKind::UnmappableCharacter {
            value: output_index as u32,
        };
        Err(CharsetEncodeError::new(Charset::ASCII, kind, output_index))
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct DecodeFlushErrorCodec;

impl CharsetCodec for DecodeFlushErrorCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct IncompletePrefixCodec;

impl CharsetCodec for IncompletePrefixCodec {
    fn charset(&self) -> Charset {
        Charset::UTF_8
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct IncompleteAsInvalidCodec;

impl CharsetCodec for IncompleteAsInvalidCodec {
    fn charset(&self) -> Charset {
        Charset::UTF_8
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct MalformedWithoutConsumedCodec;

impl CharsetCodec for MalformedWithoutConsumedCodec {
    fn charset(&self) -> Charset {
        Charset::UTF_8
    }
}

impl Codec for MalformedWithoutConsumedCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        input_index: usize,
    ) -> Result<
        (char, core::num::NonZeroUsize),
        qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
    > {
        let kind = CharsetDecodeErrorKind::malformed(0xff);
        Err(CharsetDecodeError::new(Charset::UTF_8, kind, input_index)
            .into_codec_failure())
    }

    unsafe fn encode(
        &mut self,
        _value: &char,
        _output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        let kind = CharsetEncodeErrorKind::UnmappableCharacter {
            value: output_index as u32,
        };
        Err(CharsetEncodeError::new(Charset::UTF_8, kind, output_index))
    }
}

impl Codec for IncompletePrefixCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize = qubit_io::nz!(2);

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        input_index: usize,
    ) -> Result<
        (char, core::num::NonZeroUsize),
        qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
    > {
        let kind = CharsetDecodeErrorKind::IncompleteSequence {
            required: 2,
            available: 1,
        };
        Err(CharsetDecodeError::new(Charset::UTF_8, kind, input_index)
            .into_codec_failure())
    }

    unsafe fn encode(
        &mut self,
        _value: &char,
        _output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        let kind = CharsetEncodeErrorKind::UnmappableCharacter {
            value: output_index as u32,
        };
        Err(CharsetEncodeError::new(Charset::UTF_8, kind, output_index))
    }
}

impl Codec for IncompleteAsInvalidCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        input_index: usize,
    ) -> Result<
        (char, core::num::NonZeroUsize),
        qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
    > {
        let kind = CharsetDecodeErrorKind::IncompleteSequence {
            required: 2,
            available: 1,
        };
        let error = CharsetDecodeError::new(Charset::UTF_8, kind, input_index);
        Err(qubit_codec::DecodeFailure::invalid_without_consumed(error))
    }

    unsafe fn encode(
        &mut self,
        _value: &char,
        _output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        let kind = CharsetEncodeErrorKind::UnmappableCharacter {
            value: output_index as u32,
        };
        Err(CharsetEncodeError::new(Charset::UTF_8, kind, output_index))
    }
}

impl Codec for DecodeFlushErrorCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        _input_index: usize,
    ) -> Result<
        (char, core::num::NonZeroUsize),
        qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
    > {
        Ok(('A', core::num::NonZeroUsize::MIN))
    }

    unsafe fn decode_flush(
        &mut self,
        _output: &mut [char],
        output_index: usize,
    ) -> CharsetDecodeResult<usize> {
        let kind = CharsetDecodeErrorKind::malformed_unknown();
        Err(CharsetDecodeError::new(Charset::ASCII, kind, output_index))
    }

    unsafe fn encode(
        &mut self,
        _value: &char,
        _output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        let kind = CharsetEncodeErrorKind::UnmappableCharacter {
            value: output_index as u32,
        };
        Err(CharsetEncodeError::new(Charset::ASCII, kind, output_index))
    }
}

#[test]
fn test_charset_decoder_exposes_configuration_and_bounds() {
    let decoder = CharsetDecoder::new(Utf8Codec);

    assert_eq!(Charset::UTF_8, decoder.charset());
    assert_eq!(&Utf8Codec, decoder.codec());
    assert_eq!(MalformedAction::Replace, decoder.malformed_action());
    assert_eq!('\u{fffd}', decoder.replacement());
    assert_eq!(Ok(3), decoder.max_transcode_output_len(3));
    assert_eq!(Ok(0), decoder.max_finish_output_len());
    assert_eq!(Ok(0), decoder.max_reset_output_len());

    let mut decoder = CharsetDecoder::new(Utf8Codec);
    assert_eq!(Charset::UTF_8, decoder.codec_mut().charset());
    assert_eq!(Utf8Codec, decoder.into_codec());

    let decoder_with_replacement = CharsetDecoder::with_policy(
        Utf8Codec,
        CharsetDecodePolicy::replace('!'),
    );
    assert_eq!('!', decoder_with_replacement.replacement());

    let decoder = CharsetDecoder::with_policy(
        Utf8Codec,
        CharsetDecodePolicy::ignore_with_replacement('?'),
    );

    assert_eq!('?', decoder.replacement());
    assert_eq!(MalformedAction::Ignore, decoder.malformed_action());
}

#[test]
fn test_charset_decoder_transcoder_trait_methods_forward() {
    type Decoder = CharsetDecoder<Utf8Codec>;
    type DecoderResult<T> = Result<T, TranscodeError<CharsetDecodeError>>;
    type TranscodeFn = fn(
        &mut Decoder,
        &[u8],
        usize,
        &mut [char],
        usize,
    ) -> DecoderResult<TranscodeProgress>;
    type OutputFn =
        fn(&mut Decoder, &mut [char], usize) -> DecoderResult<usize>;

    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = ['\0'; 1];
    let max_transcode_output_len: fn(
        &Decoder,
        usize,
    ) -> Result<usize, CapacityError> = std::hint::black_box(
        <Decoder as Transcoder<u8, char>>::max_transcode_output_len,
    );
    let max_finish_output_len: fn(&Decoder) -> Result<usize, CapacityError> =
        std::hint::black_box(
            <Decoder as Transcoder<u8, char>>::max_finish_output_len,
        );
    let max_reset_output_len: fn(&Decoder) -> Result<usize, CapacityError> =
        std::hint::black_box(
            <Decoder as Transcoder<u8, char>>::max_reset_output_len,
        );
    let reset: OutputFn =
        std::hint::black_box(<Decoder as Transcoder<u8, char>>::reset);
    let transcode: TranscodeFn =
        std::hint::black_box(<Decoder as Transcoder<u8, char>>::transcode);
    let finish: OutputFn =
        std::hint::black_box(<Decoder as Transcoder<u8, char>>::finish);

    assert_eq!(Ok(1), max_transcode_output_len(&decoder, 1));
    assert_eq!(Ok(0), max_finish_output_len(&decoder));
    assert_eq!(Ok(0), max_reset_output_len(&decoder));
    assert_eq!(Ok(0), reset(&mut decoder, &mut [], 0));
    let progress = transcode(&mut decoder, b"A", 0, &mut output, 0)
        .expect("decoder should transcode through the trait");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(['A'], output);
    assert_eq!(Ok(0), finish(&mut decoder, &mut [], 0));
}

#[test]
fn test_charset_decode_policy_direct_function_items_cover_constructors() {
    let replace: fn(char) -> CharsetDecodePolicy =
        std::hint::black_box(CharsetDecodePolicy::replace);
    let ignore: fn() -> CharsetDecodePolicy =
        std::hint::black_box(CharsetDecodePolicy::ignore);
    let ignore_with_replacement: fn(char) -> CharsetDecodePolicy =
        std::hint::black_box(CharsetDecodePolicy::ignore_with_replacement);
    let report: fn() -> CharsetDecodePolicy =
        std::hint::black_box(CharsetDecodePolicy::report);
    let default: fn() -> CharsetDecodePolicy =
        std::hint::black_box(CharsetDecodePolicy::default);

    let replace_policy = replace('!');
    assert_eq!(MalformedAction::Replace, replace_policy.malformed_action());
    assert_eq!('!', replace_policy.replacement());

    let ignore_policy = ignore();
    assert_eq!(MalformedAction::Ignore, ignore_policy.malformed_action());
    assert_eq!(
        CharsetDecodePolicy::DEFAULT_REPLACEMENT,
        ignore_policy.replacement(),
    );

    let ignore_policy = ignore_with_replacement('?');
    assert_eq!(MalformedAction::Ignore, ignore_policy.malformed_action());
    assert_eq!('?', ignore_policy.replacement());

    let report_policy = report();
    assert_eq!(MalformedAction::Report, report_policy.malformed_action());
    assert_eq!(
        CharsetDecodePolicy::DEFAULT_REPLACEMENT,
        report_policy.replacement(),
    );
    assert_eq!(replace(CharsetDecodePolicy::DEFAULT_REPLACEMENT), default());
}

#[test]
fn test_charset_decoder_detect_and_strip_bom_for_byte_codecs() {
    let input = [0xef, 0xbb, 0xbf, b'A'];
    let (bom, stripped) =
        CharsetDecoder::<Utf8Codec>::detect_and_strip_bom(&input);

    assert_eq!(Some(UnicodeBom::Utf8), bom);
    assert_eq!(b"A", stripped);

    let input = [0xfe, 0xff, 0x00, b'A'];
    let (bom, stripped) =
        CharsetDecoder::<Utf16ByteCodec>::detect_and_strip_bom(&input);

    assert_eq!(Some(UnicodeBom::Utf16BigEndian), bom);
    assert_eq!(&[0x00, b'A'], stripped);

    let input = *b"AB";
    let (bom, stripped) =
        CharsetDecoder::<Utf8Codec>::detect_and_strip_bom(&input);

    assert_eq!(None, bom);
    assert_eq!(&input, stripped);
}

#[test]
fn test_charset_decoder_detect_and_strip_bom_progress_preserves_pending_prefix()
{
    let input = [0xff, 0xfe];
    let (status, stripped) =
        CharsetDecoder::<Utf8Codec>::detect_and_strip_bom_progress(
            &input, false,
        );

    assert_eq!(BomDetectStatus::Pending, status);
    assert_eq!(&input, stripped);

    let (status, stripped) =
        CharsetDecoder::<Utf8Codec>::detect_and_strip_bom_progress(
            &input, true,
        );

    assert_eq!(
        BomDetectStatus::Match(UnicodeBom::Utf16LittleEndian),
        status
    );
    assert_eq!(&[] as &[u8], stripped);
}

#[test]
fn test_charset_decoder_detect_and_strip_bom_progress_strips_only_matches() {
    let input = [0xef, 0xbb, 0xbf, b'A'];
    let (status, stripped) =
        CharsetDecoder::<Utf8Codec>::detect_and_strip_bom_progress(
            &input, false,
        );

    assert_eq!(BomDetectStatus::Match(UnicodeBom::Utf8), status);
    assert_eq!(b"A", stripped);

    let input = *b"AB";
    let (status, stripped) =
        CharsetDecoder::<Utf8Codec>::detect_and_strip_bom_progress(
            &input, false,
        );

    assert_eq!(BomDetectStatus::None, status);
    assert_eq!(&input, stripped);
}

#[test]
fn test_charset_decoder_reports_need_output_after_partial_progress() {
    let mut decoder = CharsetDecoder::new(Utf32U32Codec);
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&['A' as u32, 'B' as u32], 0, &mut output, 0)
        .expect("second scalar needs more output");

    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedOutput { .. }
    ));
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
    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());

    let progress = decoder
        .transcode(&[0xe4, 0xb8], 0, &mut output, 0)
        .expect("caller-preserved prefix still needs input");
    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());

    let progress = decoder
        .transcode(&[0xe4, 0xb8, 0xad, b'!'], 0, &mut output, 0)
        .expect("third byte completes the scalar and ASCII tail");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(4, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(['中', '!'], output);

    let written = decoder
        .finish(&mut output, 0)
        .expect("finish has no pending tail");
    assert_eq!(0, written);
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

    let mut decoder =
        CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::ignore());
    let progress = decoder
        .transcode(&[0x80], 0, &mut output, 0)
        .expect("malformed byte is ignored immediately");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(0, progress.written());

    let mut decoder =
        CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());
    let error = decoder
        .transcode(&[0x80], 0, &mut output, 0)
        .expect_err("reported malformed byte should fail");
    match error {
        TranscodeError::Domain(error) => {
            assert_eq!(CharsetDecodeErrorKind::malformed(0x80), error.kind());
            assert_eq!(0, error.index());
        }
        other => panic!("expected decode domain error, got {other:?}"),
    }
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

    let written = decoder
        .finish(&mut output, 0)
        .expect("EOF has no buffered ASCII tail");
    assert_eq!(0, written);
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

    let written = decoder
        .finish(&mut output, 0)
        .expect("finish has no buffered tail");
    assert_eq!(0, written);
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

    let mut decoder =
        CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::ignore());
    let mut ignored_output = ['\0'; 4];
    let progress = decoder
        .transcode(&input, 0, &mut ignored_output, 0)
        .expect("ignore malformed input");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(4, progress.written());
    assert_eq!(['A', 'B', 'C', 'D'], ignored_output);

    let mut decoder =
        CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());
    let error = decoder
        .transcode(&input[1..], 0, &mut output, 0)
        .expect_err("report malformed input");

    match error {
        TranscodeError::Domain(error) => {
            assert_eq!(CharsetDecodeErrorKind::malformed(0x80), error.kind());
            assert_eq!(0, error.index());
        }
        other => panic!("expected decode domain error, got {other:?}"),
    }
}

#[test]
fn test_charset_decoder_reports_invalid_indices_capacity_and_need_input() {
    let input = b"AB";
    let mut output = ['\0'; 1];
    let mut decoder = CharsetDecoder::new(Utf8Codec);

    let error = decoder
        .transcode(input, input.len() + 1, &mut output, 0)
        .expect_err("input index outside input slice");
    assert!(matches!(
        error,
        TranscodeError::InvalidInputIndex {
            index,
            len,
        } if index == input.len() + 1 && len == input.len()
    ));

    let beyond_output = output.len() + 1;
    let error = decoder
        .transcode(input, 0, &mut output, beyond_output)
        .expect_err("output index outside output slice should be rejected");
    assert!(matches!(
        error,
        TranscodeError::InvalidOutputIndex {
            index,
            len,
        } if index == beyond_output && len == output.len()
    ));

    let progress = decoder
        .transcode(&[0xe4], 0, &mut output, 0)
        .expect("short prefix needs input before EOF");
    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));
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

    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));
    assert_eq!(0, progress.read());
    assert_eq!(Ok(0), decoder.max_finish_output_len());

    let written = decoder
        .finish(&mut output, 0)
        .expect("finish does not process caller-owned incomplete input");
    assert_eq!(0, written);
    assert_eq!('\0', output[0]);
    assert_eq!(Ok(0), decoder.max_finish_output_len());
}

#[test]
fn test_charset_decoder_finish_without_pending_input_is_complete() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = ['\0'; 1];

    let written = decoder
        .finish(&mut output, 0)
        .expect("finish without pending input is complete");
    assert_eq!(0, written);
    assert_eq!(Ok(0), decoder.max_finish_output_len());
}

#[test]
fn test_charset_decoder_finish_ignores_output_capacity_for_caller_owned_tail() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = [];

    let progress = decoder
        .transcode(&[0xe4], 0, &mut ['\0'; 1], 0)
        .expect("partial UTF-8 prefix needs input");

    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));

    let written = decoder
        .finish(&mut output, 0)
        .expect("finish has no decoder-owned replacement output");
    assert_eq!(0, written);
    assert_eq!(Ok(0), decoder.max_finish_output_len());
}

#[test]
fn test_charset_decoder_finish_ignores_incomplete_input() {
    let mut decoder =
        CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::ignore());
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0xe4], 0, &mut output, 0)
        .expect("partial UTF-8 prefix needs input");

    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));
    assert_eq!(Ok(0), decoder.max_finish_output_len());

    let written = decoder
        .finish(&mut output, 0)
        .expect("ignore policy drops incomplete EOF input");
    assert_eq!(0, written);
    assert_eq!(Ok(0), decoder.max_finish_output_len());
}

#[test]
fn test_charset_decoder_finish_does_not_report_incomplete_input() {
    let mut decoder =
        CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0xe4, 0xb8], 0, &mut output, 0)
        .expect("partial UTF-8 prefix needs input");

    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));
    assert_eq!(Ok(0), decoder.max_finish_output_len());

    let written = decoder
        .finish(&mut output, 0)
        .expect("finish does not process caller-owned incomplete input");
    assert_eq!(0, written);
}

#[test]
fn test_charset_decoder_reset_clears_incomplete_input() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0xe4], 0, &mut output, 0)
        .expect("partial UTF-8 prefix needs input");

    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));
    assert_eq!(Ok(0), decoder.max_finish_output_len());

    decoder.reset(&mut [], 0).expect("reset");
    let written = decoder
        .finish(&mut output, 0)
        .expect("reset removes pending incomplete input");
    assert_eq!(0, written);
    assert_eq!(Ok(0), decoder.max_finish_output_len());
}

#[test]
fn test_charset_decoder_reset_reports_invalid_output_index() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    let mut output = ['\0'; 1];
    let invalid_index = output.len() + 1;

    let error = decoder
        .reset(&mut output, invalid_index)
        .expect_err("reset should reject output index outside output slice");

    assert!(matches!(
        error,
        TranscodeError::InvalidOutputIndex {
            index,
            len,
        } if index == invalid_index && len == output.len()
    ));
}

#[test]
fn test_charset_decoder_propagates_non_policy_errors_from_caller_preserved_input()
 {
    let mut decoder = CharsetDecoder::with_policy(
        PendingInvalidInputErrorCodec,
        CharsetDecodePolicy::report(),
    );
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0], 0, &mut output, 0)
        .expect("first unit is caller-owned incomplete input");
    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));

    let error = decoder.transcode(&[0, 1], 0, &mut output, 0).expect_err(
        "non-policy error should propagate from caller-preserved input",
    );
    match error {
        TranscodeError::Domain(error) => {
            assert_eq!(
                CharsetDecodeErrorKind::malformed_unknown(),
                error.kind()
            );
            assert_eq!(0, error.index());
        }
        other => panic!("expected decode domain error, got {other:?}"),
    }
}

#[test]
fn test_charset_decoder_finish_ignores_caller_owned_incomplete_error() {
    let mut decoder = CharsetDecoder::with_policy(
        PendingInvalidInputErrorCodec,
        CharsetDecodePolicy::report(),
    );
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0], 0, &mut output, 0)
        .expect("first unit is caller-owned incomplete input");
    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedInput { .. }
    ));

    let written = decoder
        .finish(&mut output, 0)
        .expect("finish does not process caller-owned incomplete input");
    assert_eq!(0, written);
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
fn test_charset_decoder_waits_when_codec_reports_incomplete_prefix() {
    let mut decoder = CharsetDecoder::new(IncompletePrefixCodec);
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0xff], 0, &mut output, 0)
        .expect("incomplete codec failure should wait for more input");

    match progress.status() {
        TranscodeStatus::NeedInput {
            input_index,
            required,
            available,
        } => {
            assert_eq!(0, input_index);
            assert_eq!(2, required.get());
            assert_eq!(1, available);
        }
        other => panic!("expected NeedInput, got {other:?}"),
    }
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());
}

#[test]
fn test_charset_decoder_uses_default_consumed_for_malformed_without_metadata() {
    let mut decoder = CharsetDecoder::new(MalformedWithoutConsumedCodec);
    let mut output = ['\0'; 1];

    let progress = decoder
        .transcode(&[0xff], 0, &mut output, 0)
        .expect("malformed input without consumed metadata uses one unit");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(CharsetDecodePolicy::DEFAULT_REPLACEMENT, output[0]);
}

#[test]
fn test_charset_decoder_propagates_non_policy_decoding_errors() {
    let input = [0_u8];
    let mut output = ['\0'; 1];
    let mut decoder = CharsetDecoder::with_policy(
        InvalidInputErrorCodec,
        CharsetDecodePolicy::report(),
    );

    let error = decoder
        .transcode(&input, 0, &mut output, 0)
        .expect_err("invalid-index error is not absorbed");

    match error {
        TranscodeError::Domain(error) => {
            assert_eq!(
                CharsetDecodeErrorKind::malformed_unknown(),
                error.kind()
            );
            assert_eq!(0, error.index());
        }
        other => panic!("expected decode domain error, got {other:?}"),
    }
}

#[test]
fn test_charset_decoder_propagates_non_malformed_invalid_decode_errors() {
    let input = [0_u8];
    let mut output = ['\0'; 1];
    let mut decoder = CharsetDecoder::new(IncompleteAsInvalidCodec);

    let error = decoder
        .transcode(&input, 0, &mut output, 0)
        .expect_err("non-malformed invalid decode errors are not policy input");

    match error {
        TranscodeError::Domain(error) => {
            assert_eq!(
                CharsetDecodeErrorKind::IncompleteSequence {
                    required: 2,
                    available: 1,
                },
                error.kind()
            );
            assert_eq!(0, error.index());
        }
        other => panic!("expected decode domain error, got {other:?}"),
    }
}

#[test]
fn test_charset_decoder_finish_converts_decode_flush_errors() {
    let mut decoder = CharsetDecoder::new(DecodeFlushErrorCodec);
    let mut output = [];

    let error = decoder
        .finish(&mut output, 0)
        .expect_err("decode flush errors should be converted");

    match error {
        TranscodeError::Domain(error) => {
            assert_eq!(
                CharsetDecodeErrorKind::malformed_unknown(),
                error.kind()
            );
            assert_eq!(0, error.index());
        }
        other => panic!("expected decode domain error, got {other:?}"),
    }
}
