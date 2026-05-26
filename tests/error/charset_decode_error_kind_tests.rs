use qubit_codec_text::CharsetDecodeErrorKind;

#[test]
fn test_charset_decode_error_kind_displays_messages() {
    assert_eq!(
        "The encoded text sequence is malformed.",
        CharsetDecodeErrorKind::MalformedSequence { value: None }.to_string(),
    );
    assert_eq!(
        "The input unit index is outside the input buffer.",
        CharsetDecodeErrorKind::InvalidInputIndex { input_len: 2 }.to_string(),
    );
    assert_eq!(
        "The encoded text sequence is incomplete (required 5 units, available 3 units).",
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 5,
            available: 3,
        }
        .to_string(),
    );
    assert_eq!(
        "The decoded code point 0xd800 is not a valid Unicode scalar value.",
        CharsetDecodeErrorKind::InvalidCodePoint { value: 0xd800 }.to_string(),
    );

    assert_eq!(
        None,
        CharsetDecodeErrorKind::MalformedSequence { value: None }.required()
    );
    assert_eq!(
        None,
        CharsetDecodeErrorKind::MalformedSequence { value: None }.available()
    );

    let incomplete = CharsetDecodeErrorKind::IncompleteSequence {
        required: 5,
        available: 3,
    };
    assert_eq!(Some(5), incomplete.required());
    assert_eq!(Some(3), incomplete.available());

    let invalid = CharsetDecodeErrorKind::InvalidCodePoint { value: 0xd800 };
    assert_eq!(None, invalid.required());
    assert_eq!(None, invalid.available());
    assert_eq!(
        None,
        CharsetDecodeErrorKind::InvalidInputIndex { input_len: 2 }.required()
    );
    assert_eq!(
        None,
        CharsetDecodeErrorKind::InvalidInputIndex { input_len: 2 }.available()
    );

    assert_eq!(None, CharsetDecodeErrorKind::MalformedSequence { value: None }.value());
    assert_eq!(
        Some(0x41),
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0x41) }.value()
    );
    assert_eq!(Some(0xd800), invalid.value());
    assert_eq!(
        None,
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 5,
            available: 3
        }
        .value()
    );
    assert_eq!(
        Some(2),
        CharsetDecodeErrorKind::InvalidInputIndex { input_len: 2 }.input_len()
    );
    assert_eq!(None, invalid.input_len());
}
