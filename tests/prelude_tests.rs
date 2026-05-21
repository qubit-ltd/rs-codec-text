use qubit_unicode::prelude::{
    Ascii,
    ByteOrder,
    DecodeResult,
    TextDecoder,
    TextEncoder,
    TextEncoding,
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
    assert_eq!(TextEncoding::Utf8, TextDecoder::<u8>::encoding(&utf8));
    assert_eq!(TextEncoding::Utf8, TextEncoder::<u8>::encoding(&utf8));
    assert!(matches!(
        utf8.decode_prefix("A".as_bytes()).expect("UTF-8 prefix"),
        DecodeResult::Complete(_),
    ));

    let utf16 = Utf16ByteCodec::new(ByteOrder::BigEndian);
    assert_eq!(TextEncoding::Utf16, TextDecoder::<u8>::encoding(&utf16));

    let utf32 = Utf32ByteCodec::new(ByteOrder::LittleEndian);
    assert_eq!(TextEncoding::Utf32, TextEncoder::<u8>::encoding(&utf32));
}
