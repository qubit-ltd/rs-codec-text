use qubit_text_codec::{
    Charset,
    TextDecodeError,
    TextDecodeErrorKind,
};

#[test]
fn test_text_decode_error_exposes_context() {
    let error = TextDecodeError::malformed_sequence(Charset::UTF_8, 7);

    assert_eq!(Charset::UTF_8, error.charset());
    assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(7, error.index());
    assert_eq!(None, error.value());
    assert_eq!(10, error.offset_by(3).index());
    assert_eq!(
        "UTF-8 decoding error at index 7: The encoded text sequence is malformed.",
        error.to_string(),
    );

    let incomplete = TextDecodeError::incomplete_sequence(Charset::UTF_16, 3);
    assert_eq!(Charset::UTF_16, incomplete.charset());
    assert_eq!(TextDecodeErrorKind::IncompleteSequence, incomplete.kind());
    assert_eq!(3, incomplete.index());

    let invalid = TextDecodeError::invalid_code_point(Charset::UTF_32, 5, 0x110000);
    assert_eq!(Charset::UTF_32, invalid.charset());
    assert_eq!(TextDecodeErrorKind::InvalidCodePoint, invalid.kind());
    assert_eq!(5, invalid.index());
    assert_eq!(Some(0x110000), invalid.value());
    assert_eq!(
        "UTF-32 decoding error at index 5 for value 0x110000: The decoded code point is not a valid Unicode scalar value.",
        invalid.to_string(),
    );
}
