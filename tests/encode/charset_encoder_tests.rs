use qubit_codec::{
    CapacityError,
    Codec,
    TranscodeEncoder,
    TranscodeError,
    TranscodeProgress,
    TranscodeStatus,
    Transcoder,
};
use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodePolicy,
    CharsetEncodeResult,
    CharsetEncoder,
    UnmappableAction,
    Utf8Codec,
};
use std::{
    cell::Cell,
    rc::Rc,
};

macro_rules! impl_test_codec {
    ($ty:ty, $max_units:expr, $can_encode:expr) => {
        impl Codec for $ty {
            type Value = char;
            type Unit = u8;
            type DecodeError = CharsetDecodeError;
            type EncodeError = CharsetEncodeError;

            const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize =
                core::num::NonZeroUsize::MIN;

            const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize =
                qubit_io::nz!($max_units);

            fn can_encode_value(&self, value: &char) -> bool {
                let can_encode: fn(char) -> bool = $can_encode;
                can_encode(*value)
            }

            unsafe fn decode(
                &mut self,
                _input: &[u8],
                input_index: usize,
            ) -> Result<
                (char, core::num::NonZeroUsize),
                qubit_codec::DecodeFailure<
                    qubit_codec_text::CharsetDecodeError,
                >,
            > {
                let kind = CharsetDecodeErrorKind::malformed_unknown();
                Err(CharsetDecodeError::new(self.charset(), kind, input_index)
                    .into_codec_failure())
            }

            unsafe fn encode(
                &mut self,
                value: &char,
                output: &mut [u8],
                output_index: usize,
            ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
                debug_assert!(self.can_encode_value(value));
                let required = self.encode_len(value).get();
                debug_assert!(
                    output_index
                        .checked_add(required)
                        .is_some_and(|end| end <= output.len())
                );
                unsafe {
                    // SAFETY: The caller guarantees that `required` units are
                    // writable from `output_index`.
                    *output.as_mut_ptr().add(output_index) = *value as u8;
                }
                Ok(core::num::NonZeroUsize::new(required)
                    .expect("test codec encode writes at least one unit"))
            }
        }
    };
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct AsciiBytesCodec;

impl CharsetCodec for AsciiBytesCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl_test_codec!(AsciiBytesCodec, 1, |ch: char| ch.is_ascii());

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

    const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    fn can_encode_value(&self, value: &char) -> bool {
        value.is_ascii()
    }

    unsafe fn decode(
        &mut self,
        _input: &[NonDefaultUnit],
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
        value: &char,
        output: &mut [NonDefaultUnit],
        output_index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        debug_assert!(self.can_encode_value(value));
        debug_assert!(output_index < output.len());
        unsafe {
            // SAFETY: The caller guarantees that `output_index` is writable.
            *output.as_mut_ptr().add(output_index) =
                NonDefaultUnit(*value as u8);
        }
        Ok(core::num::NonZeroUsize::MIN)
    }
}

#[test]
fn test_charset_encoder_is_transcode_encoder() {
    fn assert_transcode_encoder<T: TranscodeEncoder<char, u8>>() {}

    assert_transcode_encoder::<CharsetEncoder<AsciiBytesCodec>>();
}

#[derive(Clone, Copy, Debug, Default)]
struct InvalidBangCodec;

impl CharsetCodec for InvalidBangCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl Codec for InvalidBangCodec {
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
        Err(CharsetDecodeError::new(self.charset(), kind, input_index)
            .into_codec_failure())
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        _output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        if *value == '!' {
            let kind = CharsetEncodeErrorKind::InvalidCodePoint {
                value: *value as u32,
            };
            return Err(CharsetEncodeError::new(
                self.charset(),
                kind,
                output_index,
            ));
        }
        Ok(core::num::NonZeroUsize::MIN)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct FailingReplacementWriteCodec;

impl CharsetCodec for FailingReplacementWriteCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl Codec for FailingReplacementWriteCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    fn can_encode_value(&self, value: &char) -> bool {
        *value == '!' || value.is_ascii()
    }

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        input_index: usize,
    ) -> Result<
        (char, core::num::NonZeroUsize),
        qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
    > {
        let kind = CharsetDecodeErrorKind::malformed_unknown();
        Err(CharsetDecodeError::new(self.charset(), kind, input_index)
            .into_codec_failure())
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        _output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        let kind = CharsetEncodeErrorKind::InvalidCodePoint {
            value: *value as u32,
        };
        Err(CharsetEncodeError::new(self.charset(), kind, output_index))
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct EncodeResetErrorCodec;

impl CharsetCodec for EncodeResetErrorCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl Codec for EncodeResetErrorCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    fn can_encode_value(&self, value: &char) -> bool {
        value.is_ascii()
    }

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        input_index: usize,
    ) -> Result<
        (char, core::num::NonZeroUsize),
        qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
    > {
        let kind = CharsetDecodeErrorKind::malformed_unknown();
        Err(CharsetDecodeError::new(self.charset(), kind, input_index)
            .into_codec_failure())
    }

    unsafe fn encode_reset(
        &mut self,
        _output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<usize> {
        let kind = CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 0 };
        Err(CharsetEncodeError::new(self.charset(), kind, output_index))
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        debug_assert!(self.can_encode_value(value));
        debug_assert!(output_index < output.len());
        unsafe {
            // SAFETY: The caller guarantees that `output_index` is writable.
            *output.as_mut_ptr().add(output_index) = *value as u8;
        }
        Ok(core::num::NonZeroUsize::MIN)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct ReplacementFallbackCodec;

impl CharsetCodec for ReplacementFallbackCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl_test_codec!(ReplacementFallbackCodec, 1, |ch: char| ch.is_ascii());

#[derive(Clone, Copy, Debug, Default)]
struct ReplacementAllUnencodableCodec;

impl CharsetCodec for ReplacementAllUnencodableCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl_test_codec!(ReplacementAllUnencodableCodec, 1, |ch: char| ch.is_ascii()
    && ch != '?');

#[derive(Clone, Debug, Default)]
struct CountingAsciiEncoderCodec {
    encode_calls: Rc<Cell<usize>>,
}

impl CharsetCodec for CountingAsciiEncoderCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl Codec for CountingAsciiEncoderCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize =
        core::num::NonZeroUsize::MIN;

    fn can_encode_value(&self, value: &char) -> bool {
        let current = self.encode_calls.get();
        self.encode_calls.set(current + 1);
        value.is_ascii()
    }

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        input_index: usize,
    ) -> Result<
        (char, core::num::NonZeroUsize),
        qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
    > {
        let kind = CharsetDecodeErrorKind::malformed_unknown();
        Err(CharsetDecodeError::new(self.charset(), kind, input_index)
            .into_codec_failure())
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        debug_assert!(output_index < output.len());
        unsafe {
            // SAFETY: The caller guarantees that `output_index` is writable.
            *output.as_mut_ptr().add(output_index) = *value as u8;
        }
        Ok(core::num::NonZeroUsize::MIN)
    }
}

#[test]
fn test_charset_encoder_exposes_configuration_and_bounds() {
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);

    assert_eq!(Charset::ASCII, encoder.charset());
    assert_eq!(&AsciiBytesCodec, encoder.codec());
    assert_eq!(Charset::ASCII, encoder.codec_mut().charset());
    assert_eq!(UnmappableAction::Replace, encoder.unmappable_action());
    assert_eq!('?', encoder.replacement());
    assert_eq!(Ok(3), encoder.max_transcode_output_len(3));
    assert_eq!(Ok(0), encoder.max_finish_output_len());
    assert_eq!(Ok(0), encoder.max_reset_output_len());
    encoder.reset(&mut [], 0).expect("reset");

    let encoder = CharsetEncoder::new(AsciiBytesCodec);
    assert_eq!(AsciiBytesCodec, encoder.into_codec());

    let encoder = CharsetEncoder::with_policy(
        AsciiBytesCodec,
        CharsetEncodePolicy::ignore_with_replacement('*'),
    )
    .expect("ignored replacement policy should be constructible");

    assert_eq!('*', encoder.replacement());
    assert_eq!(UnmappableAction::Ignore, encoder.unmappable_action());
}

#[test]
fn test_charset_encoder_transcoder_trait_methods_forward() {
    type Encoder = CharsetEncoder<AsciiBytesCodec>;
    type EncoderResult<T> = Result<T, TranscodeError<CharsetEncodeError>>;
    type TranscodeFn = fn(
        &mut Encoder,
        &[char],
        usize,
        &mut [u8],
        usize,
    ) -> EncoderResult<TranscodeProgress>;
    type OutputFn = fn(&mut Encoder, &mut [u8], usize) -> EncoderResult<usize>;

    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);
    let input = ['A'];
    let mut output = [0_u8; 1];
    let max_transcode_output_len: fn(
        &Encoder,
        usize,
    ) -> Result<usize, CapacityError> = std::hint::black_box(
        <Encoder as Transcoder<char, u8>>::max_transcode_output_len,
    );
    let max_finish_output_len: fn(&Encoder) -> Result<usize, CapacityError> =
        std::hint::black_box(
            <Encoder as Transcoder<char, u8>>::max_finish_output_len,
        );
    let max_reset_output_len: fn(&Encoder) -> Result<usize, CapacityError> =
        std::hint::black_box(
            <Encoder as Transcoder<char, u8>>::max_reset_output_len,
        );
    let reset: OutputFn =
        std::hint::black_box(<Encoder as Transcoder<char, u8>>::reset);
    let transcode: TranscodeFn =
        std::hint::black_box(<Encoder as Transcoder<char, u8>>::transcode);
    let finish: OutputFn =
        std::hint::black_box(<Encoder as Transcoder<char, u8>>::finish);

    assert_eq!(Ok(1), max_transcode_output_len(&encoder, 1));
    assert_eq!(Ok(0), max_finish_output_len(&encoder));
    assert_eq!(Ok(0), max_reset_output_len(&encoder));
    assert_eq!(Ok(0), reset(&mut encoder, &mut [], 0));
    let progress = transcode(&mut encoder, &input, 0, &mut output, 0)
        .expect("encoder should transcode through the trait");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!([b'A'], output);
    assert_eq!(Ok(0), finish(&mut encoder, &mut [], 0));
}

#[test]
fn test_charset_encode_policy_constructors_and_default_for() {
    let replace: fn(char) -> CharsetEncodePolicy =
        std::hint::black_box(CharsetEncodePolicy::replace);
    let ignore: fn() -> CharsetEncodePolicy =
        std::hint::black_box(CharsetEncodePolicy::ignore);
    let ignore_with_replacement: fn(char) -> CharsetEncodePolicy =
        std::hint::black_box(CharsetEncodePolicy::ignore_with_replacement);
    let report: fn() -> CharsetEncodePolicy =
        std::hint::black_box(CharsetEncodePolicy::report);
    let default: fn() -> CharsetEncodePolicy =
        std::hint::black_box(CharsetEncodePolicy::default);

    assert_eq!(UnmappableAction::Replace, replace('!').unmappable_action());
    assert_eq!('!', replace('!').replacement());
    assert_eq!(UnmappableAction::Ignore, ignore().unmappable_action());
    assert_eq!(
        CharsetEncodePolicy::DEFAULT_REPLACEMENT,
        ignore().replacement()
    );
    assert_eq!(
        UnmappableAction::Ignore,
        ignore_with_replacement('*').unmappable_action()
    );
    assert_eq!('*', ignore_with_replacement('*').replacement());
    assert_eq!(UnmappableAction::Report, report().unmappable_action());
    assert_eq!(
        CharsetEncodePolicy::DEFAULT_REPLACEMENT,
        report().replacement()
    );
    assert_eq!(
        default(),
        CharsetEncodePolicy::default_for(&Utf8Codec).unwrap()
    );
    assert_eq!(
        replace(CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT),
        CharsetEncodePolicy::default_for(&ReplacementFallbackCodec).unwrap()
    );

    let error =
        CharsetEncodePolicy::default_for(&ReplacementAllUnencodableCodec)
            .expect_err("codec cannot encode either default replacement");
    assert_eq!(
        Some(CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT as u32),
        error.value()
    );
}

#[test]
fn test_charset_encoder_replaces_reports_and_ignores_unmappable_input() {
    let input = ['A', 'é', 'B'];
    let mut output = [0_u8; 3];
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);

    let progress = encoder
        .transcode(&input, 0, &mut output, 0)
        .expect("default unmappable action replaces");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(3, progress.written());
    assert_eq!(b"A?B", &output);

    let mut encoder = CharsetEncoder::with_policy(
        AsciiBytesCodec,
        CharsetEncodePolicy::ignore(),
    )
    .expect("ignore policy should be constructible");
    let mut ignored_output = [0_u8; 2];
    let progress = encoder
        .transcode(&input, 0, &mut ignored_output, 0)
        .expect("ignore unmappable input");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(b"AB", &ignored_output);

    let mut encoder = CharsetEncoder::with_policy(
        AsciiBytesCodec,
        CharsetEncodePolicy::report(),
    )
    .expect("report policy should be constructible");
    let error = encoder
        .transcode(&input, 1, &mut output, 0)
        .expect_err("report unmappable input");

    match error {
        TranscodeError::Domain(error) => {
            assert!(matches!(
                error.kind(),
                CharsetEncodeErrorKind::UnmappableCharacter { .. },
            ));
            assert_eq!(1, error.index());
            assert_eq!(Some('é' as u32), error.value());
        }
        other => panic!("expected encode domain error, got {other:?}"),
    }
}

#[test]
fn test_charset_encoder_reports_need_output_when_replacement_does_not_fit() {
    let input = ['A', 'é'];
    let mut output = [0_u8; 1];
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);

    let progress = encoder
        .transcode(&input, 0, &mut output, 0)
        .expect("small output should stop with NeedOutput");

    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedOutput { .. }
    ));
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(b"A", &output);
}

#[test]
fn test_charset_encoder_reports_invalid_indices_and_capacity() {
    let input = ['A', 'B'];
    let mut output = [0_u8; 1];
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);

    let error = encoder
        .transcode(&input, input.len() + 1, &mut output, 0)
        .expect_err("input index is outside input slice");
    assert!(matches!(
        error,
        TranscodeError::InvalidInputIndex {
            index,
            len,
        } if index == input.len() + 1 && len == input.len()
    ));

    let beyond_output = output.len() + 1;
    let error = encoder
        .transcode(&input, 0, &mut output, beyond_output)
        .expect_err("output index outside output slice should be rejected");
    assert!(matches!(
        error,
        TranscodeError::InvalidOutputIndex {
            index,
            len,
        } if index == beyond_output && len == output.len()
    ));

    let error = encoder.finish(&mut output, beyond_output).expect_err(
        "finish output index beyond output slice should be rejected",
    );
    assert!(matches!(
        error,
        TranscodeError::InvalidOutputIndex {
            index,
            len,
        } if index == beyond_output && len == output.len()
    ));

    let progress = encoder
        .transcode(&input, 0, &mut output, 0)
        .expect("normal encoding stops when output fills");
    assert!(matches!(
        progress.status(),
        TranscodeStatus::NeedOutput { .. }
    ));
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
}

#[test]
fn test_charset_encoder_report_policy_does_not_require_default_unit() {
    let mut encoder = CharsetEncoder::with_policy(
        NonDefaultUnitCodec,
        CharsetEncodePolicy::report(),
    )
    .expect("report policy should not pre-encode replacement units");
    let mut output = [NonDefaultUnit(0)];

    let progress = encoder
        .transcode(&['A'], 0, &mut output, 0)
        .expect("ASCII character should encode");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(NonDefaultUnit(b'A'), output[0]);
}

#[test]
fn test_charset_encoder_reports_unmappable_replacement() {
    let input = ['中'];
    let mut output = [0_u8; 1];
    let error = CharsetEncoder::with_policy(
        AsciiBytesCodec,
        CharsetEncodePolicy::replace('é'),
    )
    .expect_err("user replacement should fail when unmappable");

    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::UnmappableCharacter { .. },
    ));

    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);
    let progress = encoder
        .transcode(&input, 0, &mut output, 0)
        .expect("fallback replacement should still be used");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(b"?", &output);
}

#[test]
fn test_charset_encoder_propagates_non_policy_encoding_errors() {
    let input = ['!'];
    let mut output = [0_u8; 1];
    let mut encoder = CharsetEncoder::new(InvalidBangCodec);

    let error = encoder
        .transcode(&input, 0, &mut output, 0)
        .expect_err("invalid code point error is not absorbed");

    match error {
        TranscodeError::Domain(error) => {
            assert!(matches!(
                error.kind(),
                CharsetEncodeErrorKind::InvalidCodePoint { .. },
            ));
            assert_eq!(Some('!' as u32), error.value());
        }
        other => panic!("expected encode domain error, got {other:?}"),
    }
}

#[test]
fn test_charset_encoder_with_policy_reports_replacement_write_errors() {
    let mut encoder = CharsetEncoder::with_policy(
        FailingReplacementWriteCodec,
        CharsetEncodePolicy::replace('!'),
    )
    .expect("replacement sizing should be accepted");
    let mut output = [0_u8; 1];

    let error = encoder
        .transcode(&['中'], 0, &mut output, 0)
        .expect_err("replacement writing should surface encode failures");

    match error {
        TranscodeError::Domain(error) => {
            assert_eq!(
                CharsetEncodeErrorKind::InvalidCodePoint { value: '!' as u32 },
                error.kind()
            );
        }
        other => panic!("expected encode domain error, got {other:?}"),
    }
}

#[test]
fn test_charset_encoder_reset_converts_encode_reset_errors() {
    let mut encoder = CharsetEncoder::new(EncodeResetErrorCodec);
    let mut output = [];

    let error = encoder
        .reset(&mut output, 0)
        .expect_err("encode reset errors should be converted");

    match error {
        TranscodeError::Domain(error) => {
            assert_eq!(
                CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 0 },
                error.kind()
            );
            assert_eq!(0, error.index());
        }
        other => panic!("expected encode domain error, got {other:?}"),
    }
}

#[test]
fn test_charset_encoder_with_policy_accepts_valid_replacement() {
    let encoder = CharsetEncoder::with_policy(
        AsciiBytesCodec,
        CharsetEncodePolicy::replace('!'),
    )
    .expect("replacement character should be accepted");

    assert_eq!('!', encoder.replacement());
}

#[test]
fn test_charset_encoder_new_falls_back_to_fallback_replacement_when_default_is_not_encodable()
 {
    let mut encoder = CharsetEncoder::new(ReplacementFallbackCodec);

    let mut output = [0_u8; 1];
    let progress = encoder
        .transcode(['中'].as_slice(), 0, &mut output, 0)
        .expect("fallback replacement should be used");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT, '?');
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(b"?", &output);
}

#[test]
#[should_panic]
fn test_charset_encoder_new_panics_if_no_default_or_fallback_replacement_is_encodable()
 {
    let _encoder = CharsetEncoder::new(ReplacementAllUnencodableCodec);
}

#[test]
fn test_charset_encoder_with_policy_rejects_unencodable_replacement_immediately()
 {
    let error = CharsetEncoder::with_policy(
        AsciiBytesCodec,
        CharsetEncodePolicy::replace('中'),
    )
    .expect_err("unmappable replacement should be rejected");

    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::UnmappableCharacter { .. },
    ));
}

#[test]
fn test_charset_encoder_replacement_width_is_prevalidated() {
    let encode_calls = Rc::new(Cell::new(0));
    let mut encoder = CharsetEncoder::with_policy(
        CountingAsciiEncoderCodec {
            encode_calls: encode_calls.clone(),
        },
        CharsetEncodePolicy::replace('*'),
    )
    .expect("user replacement should be encodable");
    assert_eq!(1, encode_calls.get());

    let input = ['A', '中'];
    let mut output = [0_u8; 2];
    let progress = encoder
        .transcode(&input, 0, &mut output, 0)
        .expect("replace unmappable character");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(2, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(b"A*", &output);
    assert_eq!(4, encode_calls.get());
}

#[test]
fn test_charset_encoder_exposes_configuration_and_formats_debug() {
    let encoder = CharsetEncoder::with_policy(
        AsciiBytesCodec,
        CharsetEncodePolicy::replace('!'),
    )
    .expect("replacement should be encodable");

    assert_eq!(UnmappableAction::Replace, encoder.unmappable_action());
    assert_eq!('!', encoder.replacement());

    let debug = format!("{encoder:?}");
    assert!(debug.contains("CharsetEncoder"));
    assert!(debug.contains("replacement_units_len"));
}
