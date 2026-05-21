use qubit_text_codec::{
    Charset,
    DecodeStatus,
    TextDecodeErrorKind,
    TextDecoder,
    Utf32,
    Utf32U32Decoder,
};

#[test]
fn test_utf32_u32_decoder_exposes_charset_and_unit_width() {
    let decoder = Utf32U32Decoder;

    assert_eq!(Charset::UTF_32, decoder.charset());
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, decoder.max_units_per_char());
}

#[test]
fn test_utf32_u32_decoder_decodes_units() {
    let decoder = Utf32U32Decoder;
    let mut index = 0;
    let units = ['A' as u32, '中' as u32, '😀' as u32];

    assert_eq!(
        Some('A'),
        decoder.decode_next(&units, &mut index).expect("ASCII")
    );
    assert_eq!(
        Some('中'),
        decoder.decode_next(&units, &mut index).expect("CJK")
    );
    assert_eq!(
        Some('😀'),
        decoder.decode_next(&units, &mut index).expect("emoji")
    );
    assert_eq!(None, decoder.decode_next(&units, &mut index).expect("EOF"));
}

#[test]
fn test_utf32_u32_decoder_reports_need_more_and_invalid_units() {
    let decoder = Utf32U32Decoder;

    assert!(matches!(
        decoder
            .decode_prefix(&[], 0)
            .expect("UTF-32 unit needs more"),
        DecodeStatus::NeedMore { .. },
    ));

    for unit in [0xd800, 0x110000] {
        let error = decoder
            .decode_prefix(&[unit], 0)
            .expect_err("invalid UTF-32 unit");
        assert_eq!(TextDecodeErrorKind::InvalidCodePoint, error.kind());
        assert_eq!(0, error.index());
        assert_eq!(Some(unit), error.value());
    }
}

#[test]
fn test_utf32_u32_decoder_matches_char_from_u32_boundaries() {
    let decoder = Utf32U32Decoder;

    for unit in [0x0000, 0x0041, 0xd7ff, 0xe000, 0x10ffff] {
        let expected = char::from_u32(unit).expect("standard library accepts valid scalar");

        assert_eq!(
            DecodeStatus::Complete {
                value: expected,
                consumed: 1,
            },
            decoder
                .decode_prefix(&[unit], 0)
                .expect("decoder accepts valid UTF-32 unit"),
        );
    }

    for unit in [0xd800, 0xdfff, 0x110000] {
        assert!(
            char::from_u32(unit).is_none(),
            "standard library rejects invalid scalar"
        );
        let error = decoder
            .decode_prefix(&[unit], 0)
            .expect_err("decoder rejects invalid UTF-32 unit");
        assert_eq!(TextDecodeErrorKind::InvalidCodePoint, error.kind());
        assert_eq!(Some(unit), error.value());
    }
}
