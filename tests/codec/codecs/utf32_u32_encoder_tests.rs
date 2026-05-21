use qubit_text_codec::{
    Charset,
    TextEncodeErrorKind,
    TextEncoder,
    Utf32,
    Utf32U32Encoder,
};

#[test]
fn test_utf32_u32_encoder_exposes_charset_and_unit_width() {
    let encoder = Utf32U32Encoder;

    assert_eq!(Charset::UTF_32, encoder.charset());
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, encoder.max_units_per_char());
}

#[test]
fn test_utf32_u32_encoder_encodes_units_and_reports_small_buffers() {
    let encoder = Utf32U32Encoder;
    let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];

    let written = encoder
        .encode_char('😀', &mut output)
        .expect("encode emoji");
    assert_eq!(1, written);
    assert_eq!('😀' as u32, output[0]);

    let mut empty = [];
    let error = encoder
        .encode_char('A', &mut empty)
        .expect_err("empty UTF-32 output must fail");
    assert_eq!(TextEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(0, error.index());
}
