use qubit_text_codec::{
    Charset,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
};

#[test]
fn test_charset_decode_error_exposes_context() {
    let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
    let error = CharsetDecodeError::new(Charset::UTF_8, kind, 7);

    assert_eq!(Charset::UTF_8, error.charset());
    assert_eq!(CharsetDecodeErrorKind::MalformedSequence { value: None }, error.kind());
    assert_eq!(7, error.index());
    assert_eq!(None, error.value());
    assert_eq!(10, error.offset_by(3).index());
    assert_eq!(
        "UTF-8 decoding error at index 7: The encoded text sequence is malformed.",
        error.to_string(),
    );

    let kind = CharsetDecodeErrorKind::IncompleteSequence {
        required: 7,
        available: 0,
    };
    let incomplete = CharsetDecodeError::new(Charset::UTF_16, kind, 3);
    assert_eq!(Charset::UTF_16, incomplete.charset());
    assert!(matches!(
        incomplete.kind(),
        CharsetDecodeErrorKind::IncompleteSequence { .. },
    ));
    assert_eq!(3, incomplete.index());
    assert_eq!(
        7,
        incomplete
            .required()
            .expect("required payload is set for incomplete sequence"),
    );
    assert_eq!(
        0,
        incomplete
            .available()
            .expect("available payload is set for incomplete sequence"),
    );
    assert_eq!(
        "UTF-16 decoding error at index 3: The encoded text sequence is incomplete (required 7 units, available 0 units).",
        incomplete.to_string(),
    );

    let kind = CharsetDecodeErrorKind::InvalidCodePoint { value: 0x110000 };
    let invalid = CharsetDecodeError::new(Charset::UTF_32, kind, 5);
    assert_eq!(Charset::UTF_32, invalid.charset());
    assert!(matches!(
        invalid.kind(),
        CharsetDecodeErrorKind::InvalidCodePoint { value: 0x110000 },
    ));
    assert_eq!(5, invalid.index());
    assert_eq!(Some(0x110000), invalid.value());
    assert_eq!(
        "UTF-32 decoding error at index 5 for value 0x110000: The decoded code point 0x110000 is not a valid Unicode scalar value.",
        invalid.to_string(),
    );

    let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: 2 };
    let invalid_index = CharsetDecodeError::new(Charset::UTF_8, kind, 3);
    assert_eq!(Some(2), invalid_index.input_len());
    assert_eq!(
        "UTF-8 decoding error at index 3: The input unit index is outside the input buffer.",
        invalid_index.to_string(),
    );
}
