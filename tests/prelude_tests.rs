use qubit_codec_text::prelude::{
    Ascii,
    ByteOrder,
    Charset,
    CharsetCodec,
    CharsetDecoder,
    CharsetEncoder,
    Coder,
    CoderStatus,
    DecodeStatus,
    Unicode,
    UnicodeBom,
    Utf8,
    Utf8Codec,
    Utf16,
    Utf16ByteCodec,
    Utf32,
    Utf32ByteCodec,
};

#[test]
fn test_prelude_reexports_common_types() {
    assert!(Ascii::is_ascii_char('A'));
    assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xe4));
    assert_eq!(2, Utf16::unit_len('😀'));
    assert!(Utf32::is_valid_unit('中' as u32));
    assert!(Unicode::is_scalar_value('中' as u32));
    assert_eq!(Some(ByteOrder::LittleEndian), Utf16::detect_bom(&[0xff, 0xfe]));
    assert_eq!(Some(UnicodeBom::Utf8), UnicodeBom::detect(&[0xef, 0xbb, 0xbf]));

    let utf8 = Utf8Codec;
    assert_eq!(Charset::UTF_8, utf8.charset());
    assert!(matches!(
        utf8.decode_one("A".as_bytes(), 0).expect("UTF-8 prefix"),
        DecodeStatus::Complete { .. },
    ));
    let mut decoder = CharsetDecoder::new(utf8);
    let mut chars = ['\0'; 1];
    let progress = decoder
        .convert("A".as_bytes(), 0, &mut chars, 0)
        .expect("policy decoder");
    assert_eq!(CoderStatus::Complete, progress.status());

    let utf16 = Utf16ByteCodec::new(ByteOrder::BigEndian);
    assert_eq!(Charset::UTF_16BE, utf16.charset());

    let utf32 = Utf32ByteCodec::new(ByteOrder::LittleEndian);
    let mut encoder = CharsetEncoder::new(utf32);
    let mut output = [0_u8; 4];
    let progress = encoder.convert(&['A'], 0, &mut output, 0).expect("policy encoder");
    assert_eq!(Charset::UTF_32LE, encoder.codec().charset());
    assert_eq!(CoderStatus::Complete, progress.status());
}
