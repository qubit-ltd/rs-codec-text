use qubit_codec_text::CharsetEncodeErrorKind;

#[test]
fn test_charset_encode_error_kind_displays_messages() {
    assert_eq!(
        "The code point is not a valid Unicode scalar value.",
        CharsetEncodeErrorKind::InvalidCodePoint { value: 0x110000 }.to_string(),
    );
    assert_eq!(
        "The character cannot be represented by the target encoding.",
        CharsetEncodeErrorKind::UnmappableCharacter { value: 0x110000 }.to_string(),
    );
    assert_eq!(
        "The input character index is outside the input buffer.",
        CharsetEncodeErrorKind::InvalidInputIndex { input_len: 0 }.to_string(),
    );
    assert_eq!(
        "The output unit index is outside the output buffer.",
        CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 0 }.to_string(),
    );
    assert_eq!(
        "The output buffer is too small (required 4 units, available 1 units).",
        CharsetEncodeErrorKind::BufferTooSmall {
            required: 4,
            available: 1,
        }
        .to_string(),
    );

    let invalid = CharsetEncodeErrorKind::InvalidCodePoint { value: 0x110000 };
    assert_eq!(None, invalid.required());
    assert_eq!(None, invalid.available());
    assert_eq!(Some(0x110000), invalid.value());

    let unmappable = CharsetEncodeErrorKind::UnmappableCharacter { value: 0x110000 };
    assert_eq!(None, unmappable.required());
    assert_eq!(None, unmappable.available());
    assert_eq!(Some(0x110000), unmappable.value());

    assert_eq!(
        None,
        CharsetEncodeErrorKind::InvalidInputIndex { input_len: 0 }.required()
    );
    assert_eq!(
        None,
        CharsetEncodeErrorKind::InvalidInputIndex { input_len: 0 }.available()
    );
    assert_eq!(None, CharsetEncodeErrorKind::InvalidInputIndex { input_len: 0 }.value());
    assert_eq!(
        None,
        CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 0 }.required()
    );
    assert_eq!(
        None,
        CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 0 }.available()
    );
    assert_eq!(
        None,
        CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 0 }.value()
    );
    assert_eq!(
        Some(0),
        CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 0 }.output_len()
    );

    let buffer = CharsetEncodeErrorKind::BufferTooSmall {
        required: 4,
        available: 1,
    };
    assert_eq!(Some(4), buffer.required());
    assert_eq!(Some(1), buffer.available());
    assert_eq!(None, buffer.value());
    assert_eq!(None, buffer.output_len());
}
