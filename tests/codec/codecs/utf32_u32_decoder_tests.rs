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
        decoder.decode_prefix(&[]).expect("UTF-32 unit needs more"),
        DecodeStatus::NeedMore { .. },
    ));

    for unit in [0xd800, 0x110000] {
        let error = decoder
            .decode_prefix(&[unit])
            .expect_err("invalid UTF-32 unit");
        assert_eq!(TextDecodeErrorKind::InvalidCodePoint, error.kind());
        assert_eq!(0, error.index());
        assert_eq!(Some(unit), error.value());
    }
}
