use qubit_text_codec::{
    TextEncoding,
    TextEncodingError,
    TextEncodingErrorKind,
};

#[test]
fn test_text_encoding_error_exposes_context() {
    const GBK: TextEncoding = TextEncoding::new("gbk", "GBK", &["cp936"]);

    let error = TextEncodingError::buffer_too_small(TextEncoding::UTF_16, 2);

    assert_eq!(TextEncoding::UTF_16, error.encoding());
    assert_eq!(TextEncodingErrorKind::BufferTooSmall, error.kind());
    assert_eq!(2, error.index());
    assert_eq!(None, error.value());
    assert_eq!(7, error.offset_by(5).index());
    assert_eq!(
        "UTF-16 encoding error at index 2: The output buffer is too small.",
        error.to_string(),
    );

    let invalid = TextEncodingError::invalid_code_point(TextEncoding::UTF_8, 0, 0x110000);
    assert_eq!(TextEncoding::UTF_8, invalid.encoding());
    assert_eq!(TextEncodingErrorKind::InvalidCodePoint, invalid.kind());
    assert_eq!(0, invalid.index());
    assert_eq!(Some(0x110000), invalid.value());
    assert_eq!(
        "UTF-8 encoding error at index 0 for value 0x110000: The code point is not a valid Unicode scalar value.",
        invalid.to_string(),
    );

    let unmappable = TextEncodingError::unmappable_character(GBK, 4, '中' as u32);
    assert_eq!(GBK, unmappable.encoding());
    assert_eq!(
        TextEncodingErrorKind::UnmappableCharacter,
        unmappable.kind()
    );
    assert_eq!(4, unmappable.index());
    assert_eq!(Some('中' as u32), unmappable.value());
}
