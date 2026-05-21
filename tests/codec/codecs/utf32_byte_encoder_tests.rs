use qubit_text_codec::{
    ByteOrder,
    TextEncoder,
    TextEncoding,
    TextEncodingErrorKind,
    Utf32,
    Utf32ByteEncoder,
};

#[test]
fn test_utf32_byte_encoder_exposes_encoding_order_and_unit_width() {
    let encoder = Utf32ByteEncoder::new(ByteOrder::BigEndian);

    assert_eq!(ByteOrder::BigEndian, encoder.byte_order());
    assert_eq!(TextEncoding::UTF_32, encoder.encoding());
    assert_eq!(Utf32::MAX_BYTES_PER_CHAR, encoder.max_units_per_char());
}

#[test]
fn test_utf32_byte_encoder_encodes_bytes() {
    let encoder = Utf32ByteEncoder::new(ByteOrder::BigEndian);
    let bytes = [0x00, 0x01, 0xf6, 0x00];
    let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];

    let written = encoder
        .encode_char('😀', &mut output)
        .expect("encode UTF-32BE emoji");
    assert_eq!(4, written);
    assert_eq!(bytes, output);

    let mut small = [0_u8; 3];
    let error = encoder
        .encode_char('A', &mut small)
        .expect_err("UTF-32 byte encoder must reject a too-small output buffer");
    assert_eq!(TextEncodingErrorKind::BufferTooSmall, error.kind());
    assert_eq!(3, error.index());
}
