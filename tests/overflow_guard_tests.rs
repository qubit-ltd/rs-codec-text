use qubit_unicode::{
    ByteOrder,
    TextEncoder,
    TextEncodingErrorKind,
    Utf8Encoder,
    Utf16ByteEncoder,
    Utf16U16Encoder,
    Utf32ByteEncoder,
    Utf32U32Encoder,
};

#[test]
fn test_utf8_encoder_rejects_small_output_buffer() {
    let encoder = Utf8Encoder;
    let mut output = [0_u8; 2];

    let error = encoder
        .encode_char('中', &mut output)
        .expect_err("UTF-8 encoder must reject a too-small output buffer");

    assert_eq!(TextEncodingErrorKind::BufferTooSmall, error.kind());
    assert_eq!(2, error.index());
}

#[test]
fn test_utf16_encoders_reject_small_output_buffers() {
    let unit_encoder = Utf16U16Encoder;
    let byte_encoder = Utf16ByteEncoder::new(ByteOrder::LittleEndian);
    let mut unit_output = [0_u16; 1];
    let mut byte_output = [0_u8; 2];

    let unit_error = unit_encoder
        .encode_char('😀', &mut unit_output)
        .expect_err("UTF-16 unit encoder must reject a too-small output buffer");
    let byte_error = byte_encoder
        .encode_char('😀', &mut byte_output)
        .expect_err("UTF-16 byte encoder must reject a too-small output buffer");

    assert_eq!(TextEncodingErrorKind::BufferTooSmall, unit_error.kind());
    assert_eq!(1, unit_error.index());
    assert_eq!(TextEncodingErrorKind::BufferTooSmall, byte_error.kind());
    assert_eq!(2, byte_error.index());
}

#[test]
fn test_utf32_encoders_reject_small_output_buffers() {
    let unit_encoder = Utf32U32Encoder;
    let byte_encoder = Utf32ByteEncoder::new(ByteOrder::BigEndian);
    let mut unit_output = [];
    let mut byte_output = [0_u8; 3];

    let unit_error = unit_encoder
        .encode_char('A', &mut unit_output)
        .expect_err("UTF-32 unit encoder must reject an empty output buffer");
    let byte_error = byte_encoder
        .encode_char('A', &mut byte_output)
        .expect_err("UTF-32 byte encoder must reject a too-small output buffer");

    assert_eq!(TextEncodingErrorKind::BufferTooSmall, unit_error.kind());
    assert_eq!(0, unit_error.index());
    assert_eq!(TextEncodingErrorKind::BufferTooSmall, byte_error.kind());
    assert_eq!(3, byte_error.index());
}
