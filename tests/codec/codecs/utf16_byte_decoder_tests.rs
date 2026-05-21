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
            .decode_prefix(&[0x3d, 0xd8, 0x00, 0xde], 0)
            .expect("decode UTF-16LE emoji"),
    );
}

#[test]
fn test_utf16_byte_decoder_reports_need_more_and_malformed_bytes() {
    let decoder = Utf16ByteDecoder::new(ByteOrder::LittleEndian);

    for bytes in [[][..].as_ref(), &[0x3d][..], &[0x3d, 0xd8, 0x00][..]] {
        assert!(matches!(
            decoder
                .decode_prefix(bytes, 0)
                .expect("UTF-16 byte prefix needs more"),
            DecodeStatus::NeedMore { .. },
        ));
    }

    for bytes in [&[0x00, 0xde][..], &[0x3d, 0xd8, 0x41, 0x00]] {
        let error = decoder
            .decode_prefix(bytes, 0)
            .expect_err("malformed UTF-16 bytes");
        assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    }
}

#[test]
fn test_utf16_byte_decoder_matches_std_decode_utf16_boundaries() {
    for byte_order in [ByteOrder::BigEndian, ByteOrder::LittleEndian] {
        let decoder = Utf16ByteDecoder::new(byte_order);

        for units in [
            &[0x0041][..],
            &[0xd7ff],
            &[0xe000],
            &[0xd800, 0xdc00],
            &[0xd83d, 0xde00],
            &[0xdbff, 0xdfff],
        ] {
            let bytes: Vec<u8> = units
                .iter()
                .flat_map(|unit| byte_order.u16_bytes(*unit))
                .collect();
            let expected = char::decode_utf16(units.iter().copied())
                .next()
                .expect("sample contains one scalar")
                .expect("standard library accepts valid UTF-16");

            assert_eq!(
                DecodeStatus::Complete {
                    value: expected,
                    consumed: expected.len_utf16() * 2,
                },
                decoder
                    .decode_prefix(&bytes, 0)
                    .expect("decoder accepts valid UTF-16 bytes"),
            );
        }

        for (units, index) in [
            (&[0xde00][..], 0),
            (&[0xd83d, 0x0041][..], 2),
            (&[0xd83d, 0xd83d][..], 2),
        ] {
            let bytes: Vec<u8> = units
                .iter()
                .flat_map(|unit| byte_order.u16_bytes(*unit))
                .collect();
            assert!(
                char::decode_utf16(units.iter().copied())
                    .next()
                    .expect("sample contains one result")
                    .is_err(),
                "standard library rejects malformed UTF-16"
            );
            let error = decoder
                .decode_prefix(&bytes, 0)
                .expect_err("decoder rejects malformed UTF-16 bytes");
            assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
            assert_eq!(index, error.index());
        }
    }
}
