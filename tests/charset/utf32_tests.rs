use qubit_codec_text::{ByteOrder, Utf32};

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
    assert_eq!(
        Some(ByteOrder::LittleEndian),
        Utf32::detect_bom(&[0xff, 0xfe, 0x00, 0x00])
    );
    assert_eq!(None, Utf32::detect_bom(&[0xff, 0xfe]));
}
