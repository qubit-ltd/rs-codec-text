use qubit_unicode::{
    TextDecodingErrorKind,
    TextEncodingErrorKind,
};

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
