use qubit_text_codec::{
    Charset,
    TextEncodeErrorKind,
    TextEncoder,
    Utf16,
    Utf16U16Encoder,
};

#[test]
fn test_utf16_u16_encoder_exposes_charset_and_unit_width() {
    let encoder = Utf16U16Encoder;

    assert_eq!(Charset::UTF_16, encoder.charset());
    assert_eq!(Utf16::MAX_UNITS_PER_CHAR, encoder.max_units_per_char());
}

#[test]
fn test_utf16_u16_encoder_encodes_units() {
    let encoder = Utf16U16Encoder;
    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];

    let written = encoder
        .encode_char('😀', &mut output)
        .expect("encode emoji");
    assert_eq!(2, written);
    assert_eq!([0xd83d, 0xde00], output);

    let mut small = [0_u16; 1];
    let error = encoder
        .encode_char('😀', &mut small)
        .expect_err("UTF-16 unit encoder must reject a too-small output buffer");
    assert_eq!(TextEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(1, error.index());
}
