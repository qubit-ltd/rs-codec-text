use qubit_text_codec::TextEncodeErrorKind;

#[test]
fn test_text_encode_error_kind_displays_messages() {
    assert_eq!(
        "The code point is not a valid Unicode scalar value.",
        TextEncodeErrorKind::InvalidCodePoint.to_string(),
    );
    assert_eq!(
        "The character cannot be represented by the target encoding.",
        TextEncodeErrorKind::UnmappableCharacter.to_string(),
    );
    assert_eq!(
        "The output buffer is too small.",
        TextEncodeErrorKind::BufferTooSmall.to_string(),
    );
}
