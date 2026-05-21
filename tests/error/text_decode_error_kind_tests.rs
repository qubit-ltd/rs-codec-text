use qubit_text_codec::TextDecodeErrorKind;

#[test]
fn test_text_decode_error_kind_displays_messages() {
    assert_eq!(
        "The encoded text sequence is malformed.",
        TextDecodeErrorKind::MalformedSequence.to_string(),
    );
    assert_eq!(
        "The encoded text sequence is incomplete.",
        TextDecodeErrorKind::IncompleteSequence.to_string(),
    );
    assert_eq!(
        "The decoded code point is not a valid Unicode scalar value.",
        TextDecodeErrorKind::InvalidCodePoint.to_string(),
    );
}
