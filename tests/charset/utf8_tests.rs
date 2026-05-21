use qubit_text_codec::Utf8;

#[test]
fn test_utf8_classifies_bytes_and_lengths() {
    assert!(Utf8::is_single_byte(b'A'));
    assert!(!Utf8::is_single_byte(0x80));
    assert!(Utf8::is_leading_byte(0xe4));
    assert!(Utf8::is_leading_byte(0xf0));
    assert!(!Utf8::is_leading_byte(0xc1));
    assert!(Utf8::is_continuation_byte(0xb8));
    assert!(!Utf8::is_continuation_byte(b'A'));
    assert_eq!(Some(1), Utf8::byte_len_from_leading_byte(b'A'));
    assert_eq!(Some(2), Utf8::byte_len_from_leading_byte(0xc2));
    assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xe4));
    assert_eq!(Some(4), Utf8::byte_len_from_leading_byte(0xf0));
    assert_eq!(None, Utf8::byte_len_from_leading_byte(0x80));
    assert_eq!(1, Utf8::byte_len('A'));
    assert_eq!(3, Utf8::byte_len('中'));
    assert_eq!(4, Utf8::byte_len('😀'));
}
