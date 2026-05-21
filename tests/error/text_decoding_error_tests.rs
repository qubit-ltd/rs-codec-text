use qubit_text_codec::{
    TextDecodingError,
    TextDecodingErrorKind,
    TextEncoding,
};

#[test]
fn test_text_decoding_error_exposes_context() {
    let error = TextDecodingError::malformed_sequence(TextEncoding::UTF_8, 7);

    assert_eq!(TextEncoding::UTF_8, error.encoding());
    assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
    assert_eq!(7, error.index());
    assert_eq!(None, error.value());
    assert_eq!(10, error.offset_by(3).index());
    assert_eq!(
        "UTF-8 decoding error at index 7: The encoded text sequence is malformed.",
        error.to_string(),
    );

    let incomplete = TextDecodingError::incomplete_sequence(TextEncoding::UTF_16, 3);
    assert_eq!(TextEncoding::UTF_16, incomplete.encoding());
    assert_eq!(TextDecodingErrorKind::IncompleteSequence, incomplete.kind());
    assert_eq!(3, incomplete.index());

    let invalid = TextDecodingError::invalid_code_point(TextEncoding::UTF_32, 5, 0x110000);
    assert_eq!(TextEncoding::UTF_32, invalid.encoding());
    assert_eq!(TextDecodingErrorKind::InvalidCodePoint, invalid.kind());
    assert_eq!(5, invalid.index());
    assert_eq!(Some(0x110000), invalid.value());
    assert_eq!(
        "UTF-32 decoding error at index 5 for value 0x110000: The decoded code point is not a valid Unicode scalar value.",
        invalid.to_string(),
    );
}
