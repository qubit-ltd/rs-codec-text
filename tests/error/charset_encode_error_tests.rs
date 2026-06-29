use qubit_codec_text::{
    Charset,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
};

#[test]
fn test_charset_encode_error_exposes_context() {
    const GBK: Charset = Charset::new_static("gbk", "GBK", &["cp936"]);

    let kind = CharsetEncodeErrorKind::BufferTooSmall {
        required: 4,
        available: 1,
    };
    let error = CharsetEncodeError::new(Charset::UTF_16, kind, 2);

    assert_eq!(Charset::UTF_16, error.charset());
    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::BufferTooSmall { .. },
    ));
    assert_eq!(2, error.index());
    assert_eq!(None, error.value());
    assert_eq!(Some(4), error.required());
    assert_eq!(Some(1), error.available());
    assert_eq!(7, error.offset_by(5).index());
    assert_eq!(
        "UTF-16 encoding error at index 2: The output buffer is too small (required 4 units, available 1 units).",
        error.to_string(),
    );

    let kind = CharsetEncodeErrorKind::InvalidCodePoint { value: 0x110000 };
    let invalid = CharsetEncodeError::new(Charset::UTF_8, kind, 0);
    assert_eq!(Charset::UTF_8, invalid.charset());
    assert!(matches!(
        invalid.kind(),
        CharsetEncodeErrorKind::InvalidCodePoint { value: 0x110000 },
    ));
    assert_eq!(0, invalid.index());
    assert_eq!(Some(0x110000), invalid.value());
    assert_eq!(
        "UTF-8 encoding error at index 0 for value 0x110000: The code point is not a valid Unicode scalar value.",
        invalid.to_string(),
    );

    let kind = CharsetEncodeErrorKind::UnmappableCharacter {
        value: '中' as u32,
    };
    let unmappable = CharsetEncodeError::new(GBK, kind, 4);
    assert_eq!(GBK, unmappable.charset());
    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter {
            value: '中' as u32
        },
        unmappable.kind()
    );
    assert_eq!(4, unmappable.index());
    assert_eq!(Some('中' as u32), unmappable.value());

    let kind = CharsetEncodeErrorKind::InvalidInputIndex { input_len: 0 };
    let invalid_index = CharsetEncodeError::new(Charset::UTF_8, kind, 8);
    assert_eq!(Charset::UTF_8, invalid_index.charset());
    assert_eq!(
        CharsetEncodeErrorKind::InvalidInputIndex { input_len: 0 },
        invalid_index.kind()
    );
    assert_eq!(Some(0), invalid_index.kind().input_len());
    assert_eq!(8, invalid_index.index());
    assert_eq!(None, invalid_index.value());

    let kind = CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 2 };
    let invalid_output = CharsetEncodeError::new(Charset::UTF_8, kind, 4);
    assert_eq!(Some(2), invalid_output.output_len());
}

#[test]
fn test_charset_encode_error_offset_saturates_on_overflow() {
    let kind = CharsetEncodeErrorKind::BufferTooSmall {
        required: 1,
        available: 0,
    };
    let error = CharsetEncodeError::new(Charset::UTF_8, kind, usize::MAX - 1);

    assert_eq!(usize::MAX, error.offset_by(2).index());
}

#[test]
fn test_charset_encode_error_direct_function_items_cover_forwarders() {
    let required: fn(CharsetEncodeError) -> Option<usize> =
        std::hint::black_box(CharsetEncodeError::required);
    let available: fn(CharsetEncodeError) -> Option<usize> =
        std::hint::black_box(CharsetEncodeError::available);
    let output_len: fn(CharsetEncodeError) -> Option<usize> =
        std::hint::black_box(CharsetEncodeError::output_len);
    let value: fn(CharsetEncodeError) -> Option<u32> =
        std::hint::black_box(CharsetEncodeError::value);

    let buffer = CharsetEncodeError::new(
        Charset::UTF_8,
        CharsetEncodeErrorKind::BufferTooSmall {
            required: 4,
            available: 1,
        },
        0,
    );
    assert_eq!(Some(4), required(buffer));
    assert_eq!(Some(1), available(buffer));

    let invalid_output = CharsetEncodeError::new(
        Charset::UTF_8,
        CharsetEncodeErrorKind::InvalidOutputIndex { output_len: 2 },
        0,
    );
    assert_eq!(Some(2), output_len(invalid_output));

    let unmappable = CharsetEncodeError::new(
        Charset::UTF_8,
        CharsetEncodeErrorKind::UnmappableCharacter {
            value: '中' as u32
        },
        0,
    );
    assert_eq!(Some('中' as u32), value(unmappable));
}
