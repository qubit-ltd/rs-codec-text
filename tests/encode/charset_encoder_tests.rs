use qubit_codec::TranscodeEncoder;
use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodePolicy,
    CharsetEncodeProbe,
    CharsetEncodeResult,
    CharsetEncoder,
    Codec,
    TranscodeStatus,
    Transcoder,
    UnmappableAction,
};
use std::{
    cell::Cell,
    rc::Rc,
};

macro_rules! impl_test_codec {
    ($ty:ty, $max_units:expr) => {
        unsafe impl Codec for $ty {
            type Value = char;
            type Unit = u8;
            type DecodeError = CharsetDecodeError;
            type EncodeError = CharsetEncodeError;
            type DecodeState = ();
            type EncodeState = ();

            fn min_units_per_value(&self) -> core::num::NonZeroUsize {
                core::num::NonZeroUsize::MIN
            }

            fn max_units_per_value(&self) -> core::num::NonZeroUsize {
                core::num::NonZeroUsize::new($max_units)
                    .expect("test maximum width is non-zero")
            }

            unsafe fn decode(
                &mut self,
                _input: &[u8],
                index: usize,
            ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
                let kind =
                    CharsetDecodeErrorKind::MalformedSequence { value: None };
                Err(CharsetDecodeError::new(self.charset(), kind, index))
            }

            unsafe fn encode(
                &mut self,
                value: &char,
                output: &mut [u8],
                index: usize,
            ) -> CharsetEncodeResult<usize> {
                let required = <Self as CharsetEncodeProbe>::encode_len(
                    self, *value, index,
                )?;
                debug_assert!(index + required <= output.len());
                if required > 0 {
                    output[index] = *value as u8;
                }
                Ok(required)
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

impl CharsetEncodeProbe for AsciiBytesCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter {
                value: ch as u32,
            };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

impl_test_codec!(AsciiBytesCodec, 1);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
struct NonDefaultUnit(u8);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NonDefaultUnitCodec;

impl CharsetCodec for NonDefaultUnitCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for NonDefaultUnitCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter {
                value: ch as u32,
            };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

unsafe impl Codec for NonDefaultUnitCodec {
    type Value = char;
    type Unit = NonDefaultUnit;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;
    type DecodeState = ();
    type EncodeState = ();

    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
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
    ) -> CharsetEncodeResult<usize> {
        let required = self.encode_len(*value, index)?;
        debug_assert!(index + required <= output.len());
        output[index] = NonDefaultUnit(*value as u8);
        Ok(required)
    }
}

#[test]
fn test_charset_encoder_is_transcode_encoder() {
    fn assert_transcode_encoder<T: TranscodeEncoder<char, u8>>() {}

    assert_transcode_encoder::<CharsetEncoder<AsciiBytesCodec>>();
}

#[test]
fn test_charset_encoder_exposes_error_context() {
    let encoder = CharsetEncoder::new(AsciiBytesCodec);

    assert_eq!(Charset::ASCII, encoder.error_context());
}

#[derive(Clone, Copy, Debug, Default)]
struct InvalidBangCodec;

impl CharsetCodec for InvalidBangCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for InvalidBangCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if ch == '!' {
            let kind =
                CharsetEncodeErrorKind::InvalidCodePoint { value: ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

impl_test_codec!(InvalidBangCodec, 1);

#[derive(Clone, Copy, Debug, Default)]
struct FailingReplacementWriteCodec;

impl CharsetCodec for FailingReplacementWriteCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for FailingReplacementWriteCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if ch == '!' {
            return Ok(1);
        }
        if !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter {
                value: ch as u32,
            };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

unsafe impl Codec for FailingReplacementWriteCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;
    type DecodeState = ();
    type EncodeState = ();

    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
        Err(CharsetDecodeError::new(self.charset(), kind, index))
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        _output: &mut [u8],
        index: usize,
    ) -> CharsetEncodeResult<usize> {
        let kind = CharsetEncodeErrorKind::InvalidCodePoint {
            value: *value as u32,
        };
        Err(CharsetEncodeError::new(self.charset(), kind, index))
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct ReplacementFallbackCodec;

impl CharsetCodec for ReplacementFallbackCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for ReplacementFallbackCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if ch == '\u{fffd}' {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter {
                value: ch as u32,
            };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        if ch == '?' {
            return Ok(1);
        }
        if !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter {
                value: ch as u32,
            };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

impl_test_codec!(ReplacementFallbackCodec, 1);

#[derive(Clone, Copy, Debug, Default)]
struct ReplacementAllUnencodableCodec;

impl CharsetCodec for ReplacementAllUnencodableCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for ReplacementAllUnencodableCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if ch == '\u{fffd}' || ch == '?' || !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter {
                value: ch as u32,
            };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

impl_test_codec!(ReplacementAllUnencodableCodec, 1);

#[derive(Clone, Debug, Default)]
struct CountingAsciiEncoderCodec {
    encode_calls: Rc<Cell<usize>>,
}

impl CharsetCodec for CountingAsciiEncoderCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for CountingAsciiEncoderCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        let current = self.encode_calls.get();
        self.encode_calls.set(current + 1);
        if !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter {
                value: ch as u32,
            };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

unsafe impl Codec for CountingAsciiEncoderCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;
    type DecodeState = ();
    type EncodeState = ();

    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    unsafe fn decode(
        &mut self,
        _input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
        Err(CharsetDecodeError::new(self.charset(), kind, index))
    }

    unsafe fn encode(
        &mut self,
        value: &char,
        output: &mut [u8],
        index: usize,
    ) -> CharsetEncodeResult<usize> {
        debug_assert!(index < output.len());
        output[index] = *value as u8;
        Ok(1)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ZeroLengthReplacementCodec;

impl CharsetCodec for ZeroLengthReplacementCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for ZeroLengthReplacementCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if ch == CharsetEncodePolicy::DEFAULT_REPLACEMENT {
            return Ok(0);
        }
        if !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter {
                value: ch as u32,
            };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

impl_test_codec!(ZeroLengthReplacementCodec, 1);

#[test]
fn test_charset_encoder_exposes_configuration_and_bounds() {
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);

    assert_eq!(UnmappableAction::Replace, encoder.unmappable_action());
    assert_eq!('?', encoder.replacement());
    assert_eq!(Ok(3), encoder.max_output_len(3));
    assert_eq!(Ok(0), encoder.max_finish_output_len());
    encoder.reset(&mut [], 0).expect("reset");

    let encoder = CharsetEncoder::with_policy(
        AsciiBytesCodec,
        CharsetEncodePolicy::ignore_with_replacement('*'),
    )
    .expect("ignored replacement policy should be constructible");

    assert_eq!('*', encoder.replacement());
    assert_eq!(UnmappableAction::Ignore, encoder.unmappable_action());
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

    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::UnmappableCharacter { .. },
    ));
    assert_eq!(1, error.index());
    assert_eq!(Some('é' as u32), error.value());
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
    assert_eq!(
        CharsetEncodeErrorKind::InvalidInputIndex { input_len: 2 },
        error.kind()
    );
    assert_eq!(input.len() + 1, error.index());

    let beyond_output = output.len() + 1;
    let error = encoder
        .transcode(&input, 0, &mut output, beyond_output)
        .expect_err("output index outside output slice should be rejected");
    assert_eq!(
        CharsetEncodeErrorKind::InvalidOutputIndex {
            output_len: output.len(),
        },
        error.kind()
    );
    assert_eq!(beyond_output, error.index());

    let error = encoder.finish(&mut output, beyond_output).expect_err(
        "finish output index beyond output slice should be rejected",
    );
    assert_eq!(
        CharsetEncodeErrorKind::InvalidOutputIndex {
            output_len: output.len(),
        },
        error.kind(),
    );
    assert_eq!(beyond_output, error.index());

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

    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::InvalidCodePoint { .. },
    ));
    assert_eq!(Some('!' as u32), error.value());
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

    assert_eq!(
        CharsetEncodeErrorKind::InvalidCodePoint { value: '!' as u32 },
        error.kind()
    );
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
    assert_eq!(3, encode_calls.get());
}

#[test]
fn test_charset_encoder_supports_zero_length_replacement_units() {
    let input = ['中'];
    let mut output = [0_u8; 0];
    let mut encoder = CharsetEncoder::new(ZeroLengthReplacementCodec);

    let progress = encoder
        .transcode(&input, 0, &mut output, 0)
        .expect("zero-length replacement should be written as no units");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(0, progress.written());
    assert_eq!(
        CharsetEncodePolicy::DEFAULT_REPLACEMENT,
        encoder.replacement()
    );
}

#[test]
fn test_charset_encoder_compares_configuration_and_formats_debug() {
    let left = CharsetEncoder::with_policy(
        AsciiBytesCodec,
        CharsetEncodePolicy::replace('!'),
    )
    .expect("replacement should be encodable");
    let right = CharsetEncoder::with_policy(
        AsciiBytesCodec,
        CharsetEncodePolicy::replace('!'),
    )
    .expect("replacement should be encodable");
    let different = CharsetEncoder::with_policy(
        AsciiBytesCodec,
        CharsetEncodePolicy::ignore_with_replacement('!'),
    )
    .expect("replacement should be encodable");

    assert_eq!(left, right);
    assert_ne!(left, different);

    let debug = format!("{left:?}");
    assert!(debug.contains("CharsetEncoder"));
    assert!(debug.contains("replacement_units_len"));
}
