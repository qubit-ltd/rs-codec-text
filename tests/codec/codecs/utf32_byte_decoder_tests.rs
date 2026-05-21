use qubit_text_codec::{
    ByteOrder,
    DecodeStatus,
    TextDecoder,
    TextDecodingErrorKind,
    TextEncoding,
    Utf32,
    Utf32ByteDecoder,
};

#[test]
fn test_utf32_byte_decoder_exposes_encoding_order_and_unit_width() {
    let decoder = Utf32ByteDecoder::new(ByteOrder::BigEndian);

    assert_eq!(ByteOrder::BigEndian, decoder.byte_order());
    assert_eq!(TextEncoding::UTF_32, decoder.encoding());
    assert_eq!(Utf32::MAX_BYTES_PER_CHAR, decoder.max_units_per_char());
}

#[test]
fn test_utf32_byte_decoder_decodes_bytes() {
    let decoder = Utf32ByteDecoder::new(ByteOrder::BigEndian);

    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 4,
        },
        decoder
            .decode_prefix(&[0x00, 0x01, 0xf6, 0x00])
            .expect("UTF-32BE bytes"),
    );
}

#[test]
fn test_utf32_byte_decoder_reports_need_more_and_invalid_bytes() {
    let decoder = Utf32ByteDecoder::new(ByteOrder::BigEndian);

    assert!(matches!(
        decoder
            .decode_prefix(&[0, 0, 0])
            .expect("UTF-32 bytes need more"),
        DecodeStatus::NeedMore { .. },
    ));

    for bytes in [[0x00, 0x00, 0xd8, 0x00], [0x00, 0x11, 0x00, 0x00]] {
        let error = decoder
            .decode_prefix(&bytes)
            .expect_err("invalid UTF-32 bytes");
        assert_eq!(TextDecodingErrorKind::InvalidCodePoint, error.kind());
    }
}
