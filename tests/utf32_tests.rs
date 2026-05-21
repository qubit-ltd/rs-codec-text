use qubit_unicode::{
    ByteOrder,
    DecodeResult,
    TextDecoder,
    TextDecodingErrorKind,
    TextEncoder,
    TextEncodingErrorKind,
    Utf32,
    Utf32ByteDecoder,
    Utf32ByteEncoder,
    Utf32U32Decoder,
    Utf32U32Encoder,
};

#[test]
fn test_utf32_classifies_units_and_detects_bom() {
    assert!(Utf32::is_valid_unit('中' as u32));
    assert!(!Utf32::is_valid_unit(0xd800));
    assert!(!Utf32::is_valid_unit(0x110000));
    assert_eq!(1, Utf32::unit_len('😀'));
    assert_eq!(
        Some(ByteOrder::BigEndian),
        Utf32::detect_bom(&[0x00, 0x00, 0xfe, 0xff]),
    );
}

#[test]
fn test_utf32_u32_codec_decodes_and_encodes_units() {
    let decoder = Utf32U32Decoder;
    let encoder = Utf32U32Encoder;
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

    let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];
    let written = encoder
        .encode_char('😀', &mut output)
        .expect("encode emoji");
    assert_eq!(1, written);
    assert_eq!('😀' as u32, output[0]);
}

#[test]
fn test_utf32_reports_invalid_units_and_small_buffers() {
    let decoder = Utf32U32Decoder;
    let encoder = Utf32U32Encoder;

    let error = decoder
        .decode_prefix(&[0xd800])
        .expect_err("surrogate UTF-32 unit must fail");
    assert_eq!(TextDecodingErrorKind::InvalidCodePoint, error.kind());
    assert_eq!(0, error.index());

    let mut empty = [];
    let error = encoder
        .encode_char('A', &mut empty)
        .expect_err("empty UTF-32 output must fail");
    assert_eq!(TextEncodingErrorKind::BufferTooSmall, error.kind());
}

#[test]
fn test_utf32_byte_codec_uses_byte_order() {
    let decoder = Utf32ByteDecoder::new(ByteOrder::BigEndian);
    let encoder = Utf32ByteEncoder::new(ByteOrder::BigEndian);
    let bytes = [0x00, 0x01, 0xf6, 0x00];

    assert_eq!(
        DecodeResult::Complete(qubit_unicode::Decoded::new('😀', 4)),
        decoder
            .decode_prefix(&bytes)
            .expect("decode UTF-32BE emoji"),
    );

    let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];
    let written = encoder
        .encode_char('😀', &mut output)
        .expect("encode UTF-32BE emoji");
    assert_eq!(4, written);
    assert_eq!(bytes, output);
}
