use qubit_text_codec::TextDecodingErrorKind;

#[test]
fn test_text_decoding_error_kind_displays_messages() {
    assert_eq!(
        "The encoded text sequence is malformed.",
        TextDecodingErrorKind::MalformedSequence.to_string(),
    );
    assert_eq!(
        "The encoded text sequence is incomplete.",
        TextDecodingErrorKind::IncompleteSequence.to_string(),
    );
    assert_eq!(
        "The decoded code point is not a valid Unicode scalar value.",
        TextDecodingErrorKind::InvalidCodePoint.to_string(),
    );
}
