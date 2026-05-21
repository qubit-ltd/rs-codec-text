use qubit_text_codec::{
    ByteOrder,
    Charset,
    DecodeStatus,
    TextDecodeErrorKind,
    TextDecoder,
    Utf16,
    Utf16ByteDecoder,
};

#[test]
fn test_utf16_byte_decoder_exposes_charset_order_and_unit_width() {
    let decoder = Utf16ByteDecoder::new(ByteOrder::LittleEndian);

    assert_eq!(ByteOrder::LittleEndian, decoder.byte_order());
    assert_eq!(Charset::UTF_16LE, decoder.charset());
    assert_eq!(Utf16::MAX_BYTES_PER_CHAR, decoder.max_units_per_char());
}

#[test]
fn test_utf16_byte_decoder_decodes_bytes() {
    let decoder = Utf16ByteDecoder::new(ByteOrder::LittleEndian);

    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 4,
        },
        decoder
            .decode_prefix(&[0x3d, 0xd8, 0x00, 0xde])
            .expect("decode UTF-16LE emoji"),
    );
}

#[test]
fn test_utf16_byte_decoder_reports_need_more_and_malformed_bytes() {
    let decoder = Utf16ByteDecoder::new(ByteOrder::LittleEndian);

    for bytes in [[][..].as_ref(), &[0x3d][..], &[0x3d, 0xd8, 0x00][..]] {
        assert!(matches!(
            decoder
                .decode_prefix(bytes)
                .expect("UTF-16 byte prefix needs more"),
            DecodeStatus::NeedMore { .. },
        ));
    }

    for bytes in [&[0x00, 0xde][..], &[0x3d, 0xd8, 0x41, 0x00]] {
        let error = decoder
            .decode_prefix(bytes)
            .expect_err("malformed UTF-16 bytes");
        assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    }
}
