use qubit_text_codec::{
    ByteOrder,
    TextEncoder,
    TextEncoding,
    TextEncodingErrorKind,
    Utf16,
    Utf16ByteEncoder,
};

#[test]
fn test_utf16_byte_encoder_exposes_encoding_order_and_unit_width() {
    let encoder = Utf16ByteEncoder::new(ByteOrder::LittleEndian);

    assert_eq!(ByteOrder::LittleEndian, encoder.byte_order());
    assert_eq!(TextEncoding::UTF_16, encoder.encoding());
    assert_eq!(Utf16::MAX_BYTES_PER_CHAR, encoder.max_units_per_char());
}

#[test]
fn test_utf16_byte_encoder_encodes_bytes_and_reports_small_buffers() {
    let encoder = Utf16ByteEncoder::new(ByteOrder::LittleEndian);
    let bytes = [0x3d, 0xd8, 0x00, 0xde];
    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];

    let written = encoder
        .encode_char('😀', &mut output)
        .expect("encode UTF-16LE emoji");
    assert_eq!(4, written);
    assert_eq!(bytes, output);

    let mut small = [0_u8; 2];
    let error = encoder
        .encode_char('😀', &mut small)
        .expect_err("small byte buffer must fail");
    assert_eq!(TextEncodingErrorKind::BufferTooSmall, error.kind());
    assert_eq!(2, error.index());
}
