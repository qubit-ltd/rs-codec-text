use qubit_text_codec::{
    TextEncoder,
    TextEncoding,
    TextEncodingErrorKind,
    Utf8,
    Utf8Encoder,
};

#[test]
fn test_utf8_encoder_exposes_encoding_and_unit_width() {
    let encoder = Utf8Encoder;

    assert_eq!(TextEncoding::UTF_8, encoder.encoding());
    assert_eq!(Utf8::MAX_UNITS_PER_CHAR, encoder.max_units_per_char());
}

#[test]
fn test_utf8_encoder_encodes_chars_and_reports_errors() {
    let encoder = Utf8Encoder;
    let mut buffer = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

    let written = encoder.encode_char('中', &mut buffer).expect("encode CJK");
    assert_eq!(3, written);
    assert_eq!("中".as_bytes(), &buffer[..written]);

    let written = encoder
        .encode_code_point(0x1f600, &mut buffer)
        .expect("encode emoji");
    assert_eq!(4, written);
    assert_eq!("😀".as_bytes(), &buffer[..written]);

    let mut small = [0_u8; 2];
    let error = encoder
        .encode_char('中', &mut small)
        .expect_err("small buffer must fail");
    assert_eq!(TextEncodingErrorKind::BufferTooSmall, error.kind());
    assert_eq!(2, error.index());

    let error = encoder
        .encode_code_point(0xd800, &mut buffer)
        .expect_err("surrogate is not a scalar value");
    assert_eq!(TextEncodingErrorKind::InvalidCodePoint, error.kind());
}
