use qubit_unicode::{
    TextCodingError,
    TextDecodingError,
    TextDecodingErrorKind,
    TextEncoding,
    TextEncodingError,
    TextEncodingErrorKind,
};

#[test]
fn test_text_decoding_error_exposes_context() {
    let error = TextDecodingError::new(
        TextEncoding::Utf8,
        TextDecodingErrorKind::MalformedSequence,
        7,
    );

    assert_eq!(TextEncoding::Utf8, error.encoding());
    assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
    assert_eq!(7, error.index());
    assert_eq!(10, error.offset_by(3).index());
    assert_eq!(
        "UTF-8 decoding error at index 7: The encoded text sequence is malformed.",
        error.to_string(),
    );
}

#[test]
fn test_text_encoding_error_exposes_context() {
    let error = TextEncodingError::new(
        TextEncoding::Utf16,
        TextEncodingErrorKind::BufferTooSmall,
        2,
    );

    assert_eq!(TextEncoding::Utf16, error.encoding());
    assert_eq!(TextEncodingErrorKind::BufferTooSmall, error.kind());
    assert_eq!(2, error.index());
    assert_eq!(7, error.offset_by(5).index());
    assert_eq!(
        "UTF-16 encoding error at index 2: The output buffer is too small.",
        error.to_string(),
    );
}

#[test]
fn test_text_coding_error_wraps_encoding_and_decoding_errors() {
    let decoding = TextDecodingError::new(
        TextEncoding::Utf32,
        TextDecodingErrorKind::InvalidCodePoint,
        0,
    );
    let encoding = TextEncodingError::new(
        TextEncoding::Named("GBK"),
        TextEncodingErrorKind::UnmappableCharacter,
        1,
    );

    assert!(matches!(
        TextCodingError::from(decoding),
        TextCodingError::Decoding(_)
    ));
    assert!(matches!(
        TextCodingError::from(encoding),
        TextCodingError::Encoding(_)
    ));
}
