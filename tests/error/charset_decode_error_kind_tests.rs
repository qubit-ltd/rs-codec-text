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
        "The output character index is outside the output buffer.",
        CharsetDecodeErrorKind::InvalidOutputIndex { output_len: 1 }.to_string(),
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

    assert_eq!(
        None,
        CharsetDecodeErrorKind::MalformedSequence { value: None }.value()
    );
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
    assert_eq!(
        Some(1),
        CharsetDecodeErrorKind::InvalidOutputIndex { output_len: 1 }.output_len()
    );
    assert_eq!(None, invalid.output_len());
}

#[test]
fn test_charset_decode_error_kind_exposes_decode_policy_helpers() {
    let malformed = CharsetDecodeErrorKind::MalformedSequence { value: Some(0x80) };
    let incomplete = CharsetDecodeErrorKind::IncompleteSequence {
        required: 3,
        available: 1,
    };
    let invalid_index = CharsetDecodeErrorKind::InvalidInputIndex { input_len: 2 };
    let invalid_output_index = CharsetDecodeErrorKind::InvalidOutputIndex { output_len: 1 };
    let invalid_code_point = CharsetDecodeErrorKind::InvalidCodePoint { value: 0x110000 };

    assert!(!malformed.is_incomplete());
    assert!(incomplete.is_incomplete());
    assert!(!invalid_index.is_incomplete());
    assert!(!invalid_output_index.is_incomplete());
    assert!(!invalid_code_point.is_incomplete());

    assert_eq!(None, malformed.incomplete());
    assert_eq!(Some((3, 1)), incomplete.incomplete());
    assert_eq!(None, invalid_index.incomplete());
    assert_eq!(None, invalid_output_index.incomplete());
    assert_eq!(None, invalid_code_point.incomplete());

    assert!(malformed.is_malformed_input());
    assert!(!incomplete.is_malformed_input());
    assert!(!invalid_index.is_malformed_input());
    assert!(!invalid_output_index.is_malformed_input());
    assert!(invalid_code_point.is_malformed_input());
}
