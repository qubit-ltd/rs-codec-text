use qubit_text_codec::{
    ByteOrder,
    Utf16,
};

#[test]
fn test_utf16_classifies_units_and_surrogate_pairs() {
    assert!(Utf16::is_single_unit('A' as u16));
    assert!(!Utf16::is_single_unit(0xd83d));
    assert!(Utf16::is_high_surrogate(0xd83d));
    assert!(Utf16::is_low_surrogate(0xde00));
    assert!(Utf16::is_surrogate(0xd83d));
    assert!(Utf16::is_surrogate_pair(0xd83d, 0xde00));
    assert_eq!(Some(0x1f600), Utf16::compose_pair(0xd83d, 0xde00));
    assert_eq!(Some(0xd83d), Utf16::high_surrogate(0x1f600));
    assert_eq!(Some(0xde00), Utf16::low_surrogate(0x1f600));
    assert_eq!(1, Utf16::unit_len('A'));
    assert_eq!(2, Utf16::unit_len('😀'));
    assert_eq!(Some(1), Utf16::unit_len_code_point('中' as u32));
    assert_eq!(Some(2), Utf16::unit_len_code_point(0x1f600));
    assert_eq!(None, Utf16::unit_len_code_point(0xd800));
    assert_eq!(
        Some(ByteOrder::LittleEndian),
        Utf16::detect_bom(&[0xff, 0xfe])
    );
    assert_eq!(None, Utf16::compose_pair(0xde00, 0xd83d));
    assert_eq!(None, Utf16::high_surrogate('A' as u32));
    assert_eq!(None, Utf16::low_surrogate('A' as u32));
    assert_eq!(None, Utf16::unit_len_code_point(0x110000));
    assert_eq!(Some(ByteOrder::BigEndian), Utf16::detect_bom(&[0xfe, 0xff]));
    assert_eq!(None, Utf16::detect_bom(&[0, 0]));
}
