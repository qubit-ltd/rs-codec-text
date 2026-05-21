use qubit_text_codec::{
    ByteOrder,
    Charset,
    DecodeStatus,
    TextDecodeErrorKind,
    TextDecoder,
    Utf32,
    Utf32ByteDecoder,
};

#[test]
fn test_utf32_byte_decoder_exposes_charset_order_and_unit_width() {
    let decoder = Utf32ByteDecoder::new(ByteOrder::BigEndian);

    assert_eq!(ByteOrder::BigEndian, decoder.byte_order());
    assert_eq!(Charset::UTF_32BE, decoder.charset());
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
            .decode_prefix(&[0x00, 0x01, 0xf6, 0x00], 0)
            .expect("UTF-32BE bytes"),
    );
}

#[test]
fn test_utf32_byte_decoder_reports_need_more_and_invalid_bytes() {
    let decoder = Utf32ByteDecoder::new(ByteOrder::BigEndian);

    assert!(matches!(
        decoder
            .decode_prefix(&[0, 0, 0], 0)
            .expect("UTF-32 bytes need more"),
        DecodeStatus::NeedMore { .. },
    ));

    for bytes in [[0x00, 0x00, 0xd8, 0x00], [0x00, 0x11, 0x00, 0x00]] {
        let error = decoder
            .decode_prefix(&bytes, 0)
            .expect_err("invalid UTF-32 bytes");
        assert_eq!(TextDecodeErrorKind::InvalidCodePoint, error.kind());
        assert_eq!(Some(ByteOrder::BigEndian.read_u32(&bytes)), error.value());
    }
}

#[test]
fn test_utf32_byte_decoder_matches_char_from_u32_boundaries() {
    for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
        let decoder = Utf32ByteDecoder::new(byte_order);

        for unit in [0x0000, 0x0041, 0xd7ff, 0xe000, 0x10ffff] {
            let bytes = byte_order.u32_bytes(unit);
            let expected = char::from_u32(unit).expect("standard library accepts valid scalar");

            assert_eq!(
                DecodeStatus::Complete {
                    value: expected,
                    consumed: 4,
                },
                decoder
                    .decode_prefix(&bytes, 0)
                    .expect("decoder accepts valid UTF-32 bytes"),
            );
        }

        for unit in [0xd800, 0xdfff, 0x110000] {
            let bytes = byte_order.u32_bytes(unit);
            assert!(
                char::from_u32(unit).is_none(),
                "standard library rejects invalid scalar"
            );
            let error = decoder
                .decode_prefix(&bytes, 0)
                .expect_err("decoder rejects invalid UTF-32 bytes");
            assert_eq!(TextDecodeErrorKind::InvalidCodePoint, error.kind());
            assert_eq!(Some(unit), error.value());
        }
    }
}
