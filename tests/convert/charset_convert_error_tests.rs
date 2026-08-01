use qubit_codec::{TranscodeConvertError, TranscodeConvertErrorOf, TranscodeFailure};
use qubit_codec_text::{
    AsciiCodec, Charset, CharsetConvertError, CharsetDecodeError, CharsetDecodeErrorKind,
    CharsetEncodeError, CharsetEncodeErrorKind, Utf8Codec,
};

#[test]
fn test_charset_convert_error_wraps_decode_and_encode_errors() {
    let kind = CharsetDecodeErrorKind::malformed_unknown();
    let decode = CharsetConvertError::from(CharsetDecodeError::new(Charset::UTF_8, kind, 2));
    assert!(
        decode
            .to_string()
            .contains("Failed to decode source charset")
    );

    let kind = CharsetEncodeErrorKind::BufferTooSmall {
        required: 4,
        available: 0,
    };
    let encode = CharsetConvertError::from(CharsetEncodeError::new(Charset::UTF_8, kind, 4));
    assert!(
        encode
            .to_string()
            .contains("Failed to encode target charset")
    );
}

#[test]
fn test_charset_convert_error_maps_framework_output_failure_to_target_charset() {
    let framework_error: TranscodeConvertErrorOf<Utf8Codec, AsciiCodec> =
        TranscodeConvertError::Failure(TranscodeFailure::insufficient_output(2, 4, 1));

    let error = CharsetConvertError::from_transcode_error::<Utf8Codec, AsciiCodec>(
        Charset::UTF_8,
        Charset::ASCII,
        framework_error,
    );

    assert_eq!(
        CharsetConvertError::Encode(CharsetEncodeError::new(
            Charset::ASCII,
            CharsetEncodeErrorKind::BufferTooSmall {
                required: 4,
                available: 1,
            },
            2,
        )),
        error,
    );
}

#[test]
fn test_charset_convert_error_maps_every_converter_error_domain() {
    type FrameworkError = TranscodeConvertErrorOf<Utf8Codec, AsciiCodec>;

    let source_failures = [
        TranscodeFailure::invalid_input_index(4, 3),
        TranscodeFailure::incomplete_input(2, 4, 1),
        TranscodeFailure::trailing_input(1, 2),
        TranscodeFailure::unsupported_decode_lifecycle_output(1, 0),
        TranscodeFailure::TranscodeAfterFinish,
        TranscodeFailure::FinishAfterFinish,
    ];
    for failure in source_failures {
        let error = CharsetConvertError::from_transcode_error::<Utf8Codec, AsciiCodec>(
            Charset::UTF_8,
            Charset::ASCII,
            FrameworkError::Failure(failure),
        );
        assert!(matches!(error, CharsetConvertError::Decode(_)));
    }

    let target_failures = [
        TranscodeFailure::invalid_output_index(4, 3),
        TranscodeFailure::invalid_output_range(2, 3, 4),
        TranscodeFailure::insufficient_output(1, 3, 2),
        TranscodeFailure::output_length_overflow(),
    ];
    for failure in target_failures {
        let error = CharsetConvertError::from_transcode_error::<Utf8Codec, AsciiCodec>(
            Charset::UTF_8,
            Charset::ASCII,
            FrameworkError::Failure(failure),
        );
        assert!(matches!(error, CharsetConvertError::Encode(_)));
    }

    let decode = CharsetDecodeError::new(
        Charset::UTF_8,
        CharsetDecodeErrorKind::malformed_unknown(),
        2,
    );
    let error = CharsetConvertError::from_transcode_error::<Utf8Codec, AsciiCodec>(
        Charset::UTF_8,
        Charset::ASCII,
        FrameworkError::decode_domain_main(decode, 2),
    );
    assert_eq!(CharsetConvertError::Decode(decode), error);

    let encode = CharsetEncodeError::new(
        Charset::ASCII,
        CharsetEncodeErrorKind::UnmappableCharacter {
            value: '中' as u32
        },
        3,
    );
    let error = CharsetConvertError::from_transcode_error::<Utf8Codec, AsciiCodec>(
        Charset::UTF_8,
        Charset::ASCII,
        FrameworkError::encode_domain_main(encode, 3),
    );
    assert_eq!(CharsetConvertError::Encode(encode), error);

    let error = CharsetConvertError::from_transcode_error::<Utf8Codec, AsciiCodec>(
        Charset::UTF_8,
        Charset::ASCII,
        FrameworkError::unencodable(5, '中'),
    );
    assert_eq!(
        CharsetConvertError::Encode(CharsetEncodeError::new(
            Charset::ASCII,
            CharsetEncodeErrorKind::UnmappableCharacter {
                value: '中' as u32
            },
            5,
        )),
        error,
    );
}
