use qubit_text_codec::TextEncodingErrorKind;

#[test]
fn test_text_encoding_error_kind_displays_messages() {
    assert_eq!(
        "The code point is not a valid Unicode scalar value.",
        TextEncodingErrorKind::InvalidCodePoint.to_string(),
    );
    assert_eq!(
        "The character cannot be represented by the target encoding.",
        TextEncodingErrorKind::UnmappableCharacter.to_string(),
    );
    assert_eq!(
        "The output buffer is too small.",
        TextEncodingErrorKind::BufferTooSmall.to_string(),
    );
}
