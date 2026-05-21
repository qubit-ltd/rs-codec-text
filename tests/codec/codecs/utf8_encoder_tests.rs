use qubit_text_codec::{
    Charset,
    TextEncodeErrorKind,
    TextEncoder,
    Utf8,
    Utf8Encoder,
};

#[test]
fn test_utf8_encoder_exposes_charset_and_unit_width() {
    let encoder = Utf8Encoder;

    assert_eq!(Charset::UTF_8, encoder.charset());
    assert_eq!(Utf8::MAX_UNITS_PER_CHAR, encoder.max_units_per_char());
}

#[test]
fn test_utf8_encoder_encodes_chars_and_reports_errors() {
    let encoder = Utf8Encoder;
    let mut buffer = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

    let written = encoder
        .encode_char('中', &mut buffer, 0)
        .expect("encode CJK");
    assert_eq!(3, written);
    assert_eq!("中".as_bytes(), &buffer[..written]);

    let written = encoder
        .encode_code_point(0x1f600, &mut buffer, 0)
        .expect("encode emoji");
    assert_eq!(4, written);
    assert_eq!("😀".as_bytes(), &buffer[..written]);

    let mut small = [0_u8; 2];
    let error = encoder
        .encode_char('中', &mut small, 0)
        .expect_err("small buffer must fail");
    assert_eq!(TextEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(2, error.index());

    let mut tiny = [0_u8; 1];
    let out_of_range = tiny.len() + 1;
    let error = encoder
        .encode_char('A', &mut tiny, out_of_range)
        .expect_err("index beyond output length should fail");
    assert_eq!(TextEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(out_of_range, error.index());

    let error = encoder
        .encode_code_point(0xd800, &mut buffer, 0)
        .expect_err("surrogate is not a scalar value");
    assert_eq!(TextEncodeErrorKind::InvalidCodePoint, error.kind());
}
