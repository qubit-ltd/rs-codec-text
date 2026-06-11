use qubit_codec::TranscodeError;
use qubit_codec_text::{
    Charset,
    CharsetConvertError,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
};

#[test]
fn test_charset_convert_error_wraps_decode_and_encode_errors() {
    let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
    let decode = CharsetConvertError::from(CharsetDecodeError::new(
        Charset::UTF_8,
        kind,
        2,
    ));
    assert!(
        decode
            .to_string()
            .contains("Failed to decode source charset")
    );

    let kind = CharsetEncodeErrorKind::BufferTooSmall {
        required: 4,
        available: 0,
    };
    let encode = CharsetConvertError::from(CharsetEncodeError::new(
        Charset::UTF_8,
        kind,
        4,
    ));
    assert!(
        encode
            .to_string()
            .contains("Failed to encode target charset")
    );
}

#[test]
fn test_charset_convert_error_transcode_error_constructors_preserve_charsets() {
    let context = (Charset::UTF_8, Charset::ASCII);

    let invalid_input = <CharsetConvertError as TranscodeError<(
        Charset,
        Charset,
    )>>::invalid_input_index(context, 7, 3);
    assert!(matches!(
        invalid_input,
        CharsetConvertError::Decode(error)
            if error.charset() == Charset::UTF_8
                && matches!(
                    error.kind(),
                    CharsetDecodeErrorKind::InvalidInputIndex { input_len: 3 }
                )
    ));

    let invalid_output = <CharsetConvertError as TranscodeError<(
        Charset,
        Charset,
    )>>::invalid_output_index(context, 5, 2);
    assert!(matches!(
        invalid_output,
        CharsetConvertError::Encode(error)
            if error.charset() == Charset::ASCII
                && matches!(
                    error.kind(),
                    CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 2 }
                )
    ));

    let insufficient = <CharsetConvertError as TranscodeError<(
        Charset,
        Charset,
    )>>::insufficient_output(context, 4, 6, 1);
    assert!(matches!(
        insufficient,
        CharsetConvertError::Encode(error)
            if error.charset() == Charset::ASCII
                && matches!(
                    error.kind(),
                    CharsetEncodeErrorKind::BufferTooSmall {
                        required: 6,
                        available: 1,
                    }
                )
    ));
}
