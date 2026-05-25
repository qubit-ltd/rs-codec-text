use qubit_text_codec::{
    ByteOrder,
    Unicode,
    UnicodeBom,
};

#[test]
fn test_unicode_classifies_code_points() {
    assert!(Unicode::is_code_point(0x10ffff));
    assert!(Unicode::is_code_point(0xd800));
    assert!(!Unicode::is_code_point(0x110000));

    assert!(Unicode::is_scalar_value(0x10ffff));
    assert!(!Unicode::is_scalar_value(0xd800));
    assert!(!Unicode::is_scalar_value(0xdfff));
    assert!(!Unicode::is_scalar_value(0x110000));

    assert!(Unicode::is_ascii(0x7f));
    assert!(!Unicode::is_ascii(0x80));
    assert!(Unicode::is_bmp(0xffff));
    assert!(!Unicode::is_bmp(0x10000));
    assert!(Unicode::is_supplementary(0x10000));
    assert!(Unicode::is_supplementary(0x10ffff));
}

#[test]
fn test_unicode_classifies_surrogates_noncharacters_and_controls() {
    assert!(Unicode::is_high_surrogate(0xd800));
    assert!(Unicode::is_low_surrogate(0xdfff));
    assert!(Unicode::is_surrogate(0xd83d));
    assert!(Unicode::is_noncharacter(0xfdd0));
    assert!(Unicode::is_noncharacter(0x10ffff));
    assert!(!Unicode::is_noncharacter('A' as u32));
    assert!(Unicode::is_control(0x00));
    assert!(Unicode::is_control(0x7f));
    assert!(!Unicode::is_control('A' as u32));
}

#[test]
fn test_unicode_converts_to_char_and_reports_plane() {
    assert_eq!(Some('中'), Unicode::to_char('中' as u32));
    assert_eq!(None, Unicode::to_char(0xd800));
    assert_eq!(None, Unicode::to_char(0x110000));
    assert_eq!(Some(0), Unicode::plane('A' as u32));
    assert_eq!(Some(1), Unicode::plane(0x1f600));
    assert_eq!(None, Unicode::plane(0x110000));
}

#[test]
fn test_unicode_bom_detects_longest_prefix_first() {
    assert_eq!(Some(UnicodeBom::Utf8), UnicodeBom::detect(&[0xef, 0xbb, 0xbf]));
    assert_eq!(Some(UnicodeBom::Utf16BigEndian), UnicodeBom::detect(&[0xfe, 0xff]));
    assert_eq!(Some(UnicodeBom::Utf16LittleEndian), UnicodeBom::detect(&[0xff, 0xfe]));
    assert_eq!(
        Some(UnicodeBom::Utf32BigEndian),
        UnicodeBom::detect(&[0x00, 0x00, 0xfe, 0xff]),
    );
    assert_eq!(
        Some(UnicodeBom::Utf32LittleEndian),
        UnicodeBom::detect(&[0xff, 0xfe, 0x00, 0x00]),
    );
    assert_eq!(
        Some(ByteOrder::LittleEndian),
        UnicodeBom::Utf32LittleEndian.byte_order()
    );
    assert_eq!(None, UnicodeBom::Utf8.byte_order());
}
