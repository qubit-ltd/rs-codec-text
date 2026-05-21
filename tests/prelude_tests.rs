use qubit_text_codec::prelude::{
    Ascii,
    ByteOrder,
    Charset,
    DecodeStatus,
    TextDecoder,
    TextEncoder,
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
    assert_eq!(
        Some(ByteOrder::LittleEndian),
        Utf16::detect_bom(&[0xff, 0xfe])
    );
    assert_eq!(
        Some(UnicodeBom::Utf8),
        UnicodeBom::detect(&[0xef, 0xbb, 0xbf])
    );

    let utf8 = Utf8Codec;
    assert_eq!(Charset::UTF_8, TextDecoder::<u8>::charset(&utf8));
    assert_eq!(Charset::UTF_8, TextEncoder::<u8>::charset(&utf8));
    assert!(matches!(
        utf8.decode_prefix("A".as_bytes()).expect("UTF-8 prefix"),
        DecodeStatus::Complete { .. },
    ));

    let utf16 = Utf16ByteCodec::new(ByteOrder::BigEndian);
    assert_eq!(Charset::UTF_16BE, TextDecoder::<u8>::charset(&utf16));

    let utf32 = Utf32ByteCodec::new(ByteOrder::LittleEndian);
    assert_eq!(Charset::UTF_32LE, TextEncoder::<u8>::charset(&utf32));
}
