use qubit_codec_text::CharsetDecodeErrorKind;

#[test]
fn test_charset_decode_error_kind_displays_messages() {
    assert_eq!(
        "The encoded text sequence is malformed.",
        CharsetDecodeErrorKind::malformed_unknown().to_string(),
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

    assert_eq!(None, CharsetDecodeErrorKind::malformed_unknown().required());
    assert_eq!(
        None,
        CharsetDecodeErrorKind::malformed_unknown().available()
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

    assert_eq!(None, CharsetDecodeErrorKind::malformed_unknown().value());
    assert_eq!(Some(0x41), CharsetDecodeErrorKind::malformed(0x41).value());
    assert_eq!(Some(0xd800), invalid.value());
    assert_eq!(
        None,
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 5,
            available: 3
        }
        .value()
    );
}

#[test]
fn test_charset_decode_error_kind_malformed_constructors() {
    assert_eq!(
        CharsetDecodeErrorKind::malformed(0x80),
        CharsetDecodeErrorKind::malformed(0x80),
    );
    assert_eq!(
        CharsetDecodeErrorKind::malformed_unknown(),
        CharsetDecodeErrorKind::malformed_unknown(),
    );
}

#[test]
fn test_charset_decode_error_kind_exposes_decode_policy_helpers() {
    let malformed = CharsetDecodeErrorKind::malformed(0x80);
    let incomplete = CharsetDecodeErrorKind::IncompleteSequence {
        required: 3,
        available: 1,
    };
    let invalid_code_point =
        CharsetDecodeErrorKind::InvalidCodePoint { value: 0x110000 };

    assert!(!malformed.is_incomplete());
    assert!(incomplete.is_incomplete());
    assert!(!invalid_code_point.is_incomplete());

    assert_eq!(None, malformed.incomplete());
    assert_eq!(Some((3, 1)), incomplete.incomplete());
    assert_eq!(None, invalid_code_point.incomplete());

    assert!(malformed.is_malformed_input());
    assert!(!incomplete.is_malformed_input());
    assert!(invalid_code_point.is_malformed_input());
}
