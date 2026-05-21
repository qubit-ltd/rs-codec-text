use qubit_text_codec::{
    Charset,
    DecodeStatus,
    TextDecodeErrorKind,
    TextDecoder,
    Utf16,
    Utf16U16Decoder,
};

#[test]
fn test_utf16_u16_decoder_exposes_charset_and_unit_width() {
    let decoder = Utf16U16Decoder;

    assert_eq!(Charset::UTF_16, decoder.charset());
    assert_eq!(Utf16::MAX_UNITS_PER_CHAR, decoder.max_units_per_char());
}

#[test]
fn test_utf16_u16_decoder_decodes_units() {
    let decoder = Utf16U16Decoder;
    let units = [0x0041, 0x4e2d, 0xd83d, 0xde00];
    let mut index = 0;

    assert_eq!(
        Some('A'),
        decoder.decode_next(&units, &mut index).expect("ASCII")
    );
    assert_eq!(
        Some('中'),
        decoder.decode_next(&units, &mut index).expect("BMP")
    );
    assert_eq!(
        Some('😀'),
        decoder.decode_next(&units, &mut index).expect("pair")
    );
    assert_eq!(None, decoder.decode_next(&units, &mut index).expect("EOF"));
}

#[test]
fn test_utf16_u16_decoder_reports_need_more_and_malformed_pairs() {
    let decoder = Utf16U16Decoder;

    assert_eq!(
        DecodeStatus::NeedMore {
            required: 2,
            available: 1,
        },
        decoder
            .decode_prefix(&[0xd83d])
            .expect("high surrogate needs low surrogate"),
    );

    let error = decoder
        .decode_prefix(&[0xde00])
        .expect_err("low surrogate cannot start a scalar");
    assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(0, error.index());

    let error = decoder
        .decode_prefix(&[0xd83d, 0x0041])
        .expect_err("bad surrogate pair must fail");
    assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());
}
