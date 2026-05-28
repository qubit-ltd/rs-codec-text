use qubit_codec_text::{
    BufferedEncoder,
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeProbe,
    CharsetEncodeResult,
    CharsetEncoder,
    Codec,
    TranscodeStatus,
    Transcoder,
    UnmappableAction,
};
use std::cell::Cell;

macro_rules! impl_test_codec {
    ($ty:ty, $max_units:expr) => {
        unsafe impl Codec<char, u8> for $ty {
            type DecodeError = CharsetDecodeError;
            type EncodeError = CharsetEncodeError;

            fn min_units_per_value(&self) -> usize {
                1
            }

            fn max_units_per_value(&self) -> usize {
                $max_units
            }

            unsafe fn decode_unchecked(&self, _input: &[u8], index: usize) -> CharsetDecodeResult<(char, usize)> {
                let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
                Err(CharsetDecodeError::new(self.charset(), kind, index))
            }

            unsafe fn encode_unchecked(
                &self,
                value: &char,
                output: &mut [u8],
                index: usize,
            ) -> CharsetEncodeResult<usize> {
                let required = <Self as CharsetEncodeProbe>::encode_len(self, *value, index)?;
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
    type Unit = u8;

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

impl_test_codec!(AsciiBytesCodec, 1);

#[test]
fn test_charset_encoder_is_buffered_encoder() {
    fn assert_buffered_encoder<T: BufferedEncoder<char, u8>>() {}

    assert_buffered_encoder::<CharsetEncoder<AsciiBytesCodec>>();
}

#[derive(Clone, Copy, Debug, Default)]
struct InvalidBangCodec;

impl CharsetCodec for InvalidBangCodec {
    type Unit = u8;

    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for InvalidBangCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if ch == '!' {
            let kind = CharsetEncodeErrorKind::InvalidCodePoint { value: ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

impl_test_codec!(InvalidBangCodec, 1);

#[derive(Clone, Copy, Debug, Default)]
struct ReplacementFallbackCodec;

impl CharsetCodec for ReplacementFallbackCodec {
    type Unit = u8;

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
        if ch == '?' {
            return Ok(1);
        }
        if !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

impl_test_codec!(ReplacementFallbackCodec, 1);

#[derive(Clone, Copy, Debug, Default)]
struct ReplacementAllUnencodableCodec;

impl CharsetCodec for ReplacementAllUnencodableCodec {
    type Unit = u8;

    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for ReplacementAllUnencodableCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if ch == '\u{fffd}' || ch == '?' || !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

impl_test_codec!(ReplacementAllUnencodableCodec, 1);

#[derive(Debug, Default)]
struct CountingAsciiEncoderCodec {
    encode_calls: Cell<usize>,
}

impl CountingAsciiEncoderCodec {
    fn encode_calls(&self) -> usize {
        self.encode_calls.get()
    }
}

impl CharsetCodec for CountingAsciiEncoderCodec {
    type Unit = u8;

    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for CountingAsciiEncoderCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        let current = self.encode_calls.get();
        self.encode_calls.set(current + 1);
        if !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

unsafe impl Codec<char, u8> for CountingAsciiEncoderCodec {
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    fn min_units_per_value(&self) -> usize {
        1
    }

    fn max_units_per_value(&self) -> usize {
        1
    }

    unsafe fn decode_unchecked(&self, _input: &[u8], index: usize) -> CharsetDecodeResult<(char, usize)> {
        let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
        Err(CharsetDecodeError::new(self.charset(), kind, index))
    }

    unsafe fn encode_unchecked(&self, value: &char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        debug_assert!(index < output.len());
        output[index] = *value as u8;
        Ok(1)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ZeroLengthReplacementCodec;

impl CharsetCodec for ZeroLengthReplacementCodec {
    type Unit = u8;

    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for ZeroLengthReplacementCodec {
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if ch == CharsetEncoder::<Self>::DEFAULT_REPLACEMENT {
            return Ok(0);
        }
        if !ch.is_ascii() {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

impl_test_codec!(ZeroLengthReplacementCodec, 1);

#[test]
fn test_charset_encoder_exposes_configuration_and_bounds() {
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);

    assert_eq!(Charset::ASCII, encoder.codec().charset());
    assert_eq!(Charset::ASCII, encoder.codec_mut().charset());
    assert_eq!(UnmappableAction::Replace, encoder.unmappable_action());
    assert_eq!('?', encoder.replacement());
    assert_eq!(Some(3), encoder.max_output_len(3));

    encoder
        .set_replacement('*')
        .expect("user replacement should be encodable");
    encoder.set_unmappable_action(UnmappableAction::Ignore);

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

    encoder.set_unmappable_action(UnmappableAction::Ignore);
    let mut ignored_output = [0_u8; 2];
    let progress = encoder
        .transcode(&input, 0, &mut ignored_output, 0)
        .expect("ignore unmappable input");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(b"AB", &ignored_output);

    encoder.set_unmappable_action(UnmappableAction::Report);
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

    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
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
    assert_eq!(CharsetEncodeErrorKind::InvalidInputIndex { input_len: 2 }, error.kind());
    assert_eq!(input.len() + 1, error.index());

    let beyond_output = output.len() + 1;
    let progress = encoder
        .transcode(&input, 0, &mut output, beyond_output)
        .expect("output index beyond output slice needs more output");
    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());

    let progress = encoder
        .transcode(&input, 0, &mut output, 0)
        .expect("normal encoding stops when output fills");
    assert!(matches!(progress.status(), TranscodeStatus::NeedOutput { .. }));
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
}

#[test]
fn test_charset_encoder_reports_unmappable_replacement() {
    let input = ['中'];
    let mut output = [0_u8; 1];
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);
    let error = encoder
        .set_replacement('é')
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

    assert!(matches!(error.kind(), CharsetEncodeErrorKind::InvalidCodePoint { .. },));
    assert_eq!(Some('!' as u32), error.value());
}

#[test]
fn test_charset_encoder_with_replacement_accepts_valid_character() {
    let encoder = CharsetEncoder::new(AsciiBytesCodec)
        .with_replacement('!')
        .expect("replacement character should be accepted");

    assert_eq!('!', encoder.replacement());
}

#[test]
fn test_charset_encoder_new_falls_back_to_fallback_replacement_when_default_is_not_encodable() {
    let mut encoder = CharsetEncoder::new(ReplacementFallbackCodec);

    let mut output = [0_u8; 1];
    let progress = encoder
        .transcode(['中'].as_slice(), 0, &mut output, 0)
        .expect("fallback replacement should be used");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(
        CharsetEncoder::<ReplacementFallbackCodec>::DEFAULT_FALLBACK_REPLACEMENT,
        '?'
    );
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(b"?", &output);
}

#[test]
#[should_panic]
fn test_charset_encoder_new_panics_if_no_default_or_fallback_replacement_is_encodable() {
    let _encoder = CharsetEncoder::new(ReplacementAllUnencodableCodec);
}

#[test]
fn test_charset_encoder_with_replacement_rejects_unencodable_character_immediately() {
    let error = CharsetEncoder::new(AsciiBytesCodec)
        .with_replacement('中')
        .expect_err("unmappable replacement should be rejected");

    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::UnmappableCharacter { .. },
    ));
}

#[test]
fn test_charset_encoder_replacement_encoding_is_cached() {
    let mut encoder = CharsetEncoder::new(CountingAsciiEncoderCodec::default());

    encoder
        .set_replacement('*')
        .expect("user replacement should be encodable");
    assert_eq!(3, encoder.codec().encode_calls());

    let input = ['A', '中'];
    let mut output = [0_u8; 2];
    let progress = encoder
        .transcode(&input, 0, &mut output, 0)
        .expect("replace unmappable character");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(2, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(b"A*", &output);
    assert_eq!(5, encoder.codec().encode_calls());
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
        CharsetEncoder::<ZeroLengthReplacementCodec>::DEFAULT_REPLACEMENT,
        encoder.replacement()
    );
}

#[test]
fn test_charset_encoder_compares_configuration_and_formats_debug() {
    let left = CharsetEncoder::new(AsciiBytesCodec)
        .with_replacement('!')
        .expect("replacement should be encodable");
    let right = CharsetEncoder::new(AsciiBytesCodec)
        .with_replacement('!')
        .expect("replacement should be encodable");
    let mut different = CharsetEncoder::new(AsciiBytesCodec)
        .with_replacement('!')
        .expect("replacement should be encodable");

    different.set_unmappable_action(UnmappableAction::Ignore);

    assert_eq!(left, right);
    assert_ne!(left, different);

    let debug = format!("{left:?}");
    assert!(debug.contains("CharsetEncoder"));
    assert!(debug.contains("replacement_units_len"));
}
