use qubit_codec_text::Latin1;

#[test]
fn test_latin1_classifies_latin1_values() {
    assert_eq!('\u{00ff}', Latin1::MAX_CHAR);
    assert_eq!(0xff, Latin1::MAX_BYTE);
    assert_eq!(0xff, Latin1::MAX_CODE_POINT);

    assert!(Latin1::is_latin1_byte(0xff));
    assert!(Latin1::is_latin1_char('\u{00ff}'));
    assert!(!Latin1::is_latin1_char('\u{0100}'));
    assert!(Latin1::is_latin1_code_point(0xff));
    assert!(!Latin1::is_latin1_code_point(0x100));
}

#[test]
fn test_latin1_converts_between_bytes_chars_and_code_points() {
    assert_eq!('\u{00ff}', Latin1::byte_to_char(0xff));
    assert_eq!(0xff, Latin1::byte_to_code_point(0xff));
    assert_eq!(Some(0xff), Latin1::char_to_byte('\u{00ff}'));
    assert_eq!(None, Latin1::char_to_byte('\u{0100}'));
    assert_eq!(Some(0xff), Latin1::code_point_to_byte(0xff));
    assert_eq!(None, Latin1::code_point_to_byte(0x100));
    assert_eq!(Some('\u{00ff}'), Latin1::code_point_to_char(0xff));
    assert_eq!(None, Latin1::code_point_to_char(0x100));
}

#[test]
fn test_latin1_classifies_common_character_sets() {
    assert!(Latin1::is_whitespace_byte(b'\n'));
    assert!(Latin1::is_whitespace_char(' '));
    assert!(!Latin1::is_whitespace_char('\u{00a0}'));
    assert!(!Latin1::is_whitespace_char('\u{0100}'));
    assert!(Latin1::is_whitespace_code_point(' ' as u32));
    assert!(!Latin1::is_whitespace_code_point(0x100));

    assert!(Latin1::is_letter_byte(b'Q'));
    assert!(Latin1::is_letter_byte(0xe9));
    assert!(Latin1::is_letter_char('\u{00c0}'));
    assert!(Latin1::is_letter_code_point(0xfe));
    assert!(!Latin1::is_letter_byte(0xd7));
    assert!(!Latin1::is_letter_char('\u{00f7}'));
    assert!(!Latin1::is_letter_code_point(0x100));
    assert!(!Latin1::is_letter_char('\u{0100}'));

    assert!(Latin1::is_uppercase_letter_byte(0xc0));
    assert!(Latin1::is_uppercase_letter_byte(0xd8));
    assert!(!Latin1::is_uppercase_letter_byte(0xd7));
    assert!(Latin1::is_uppercase_letter_char('\u{00de}'));
    assert!(Latin1::is_uppercase_letter_code_point('Z' as u32));
    assert!(!Latin1::is_uppercase_letter_char('\u{0100}'));
    assert!(!Latin1::is_uppercase_letter_code_point(0x100));
    assert!(Latin1::is_lowercase_letter_byte(0xe0));
    assert!(Latin1::is_lowercase_letter_byte(0xb5));
    assert!(Latin1::is_lowercase_letter_byte(0xf8));
    assert!(!Latin1::is_lowercase_letter_byte(0xf7));
    assert!(Latin1::is_lowercase_letter_char('\u{00df}'));
    assert!(Latin1::is_lowercase_letter_code_point(0xff));
    assert!(!Latin1::is_lowercase_letter_char('\u{0100}'));
    assert!(!Latin1::is_lowercase_letter_code_point(0x100));

    assert!(Latin1::is_digit_byte(b'7'));
    assert!(Latin1::is_digit_code_point('7' as u32));
    assert!(!Latin1::is_digit_char('\u{0100}'));
    assert!(!Latin1::is_digit_code_point(0x100));
    assert!(Latin1::is_hex_digit_char('F'));
    assert!(Latin1::is_hex_digit_code_point('f' as u32));
    assert!(!Latin1::is_hex_digit_char('\u{0100}'));
    assert!(!Latin1::is_hex_digit_code_point(0x100));
    assert!(Latin1::is_octal_digit_char('7'));
    assert!(!Latin1::is_octal_digit_char('\u{0100}'));
    assert!(Latin1::is_octal_digit_code_point('7' as u32));
    assert!(!Latin1::is_octal_digit_code_point(0x100));
    assert!(Latin1::is_letter_or_digit_byte(b'7'));
    assert!(Latin1::is_letter_or_digit_char('\u{00e9}'));
    assert!(Latin1::is_letter_or_digit_code_point(0xe9));
    assert!(!Latin1::is_letter_or_digit_char('\u{0100}'));
    assert!(!Latin1::is_letter_or_digit_code_point(0x100));
    assert!(!Latin1::is_digit_char('\u{00b2}'));

    assert!(Latin1::is_printable_byte(0xa0));
    assert!(Latin1::is_printable_char('\u{00ff}'));
    assert!(Latin1::is_printable_code_point(0xa0));
    assert!(!Latin1::is_printable_byte(0x85));
    assert!(!Latin1::is_printable_code_point(0x85));
    assert!(!Latin1::is_printable_char('\u{0100}'));
    assert!(!Latin1::is_printable_code_point(0x100));
    assert!(Latin1::is_control_byte(0x85));
    assert!(Latin1::is_control_char('\u{009f}'));
    assert!(Latin1::is_control_code_point(0x85));
    assert!(!Latin1::is_control_char('\u{0100}'));
    assert!(!Latin1::is_control_code_point(0x100));
}

#[test]
fn test_latin1_converts_case_and_digits() {
    assert!(Latin1::equals_ignore_case_byte(0xc0, 0xe0));
    assert!(Latin1::equals_ignore_case_byte(0xe0, 0xe0));
    assert!(Latin1::equals_ignore_case_char('\u{00de}', '\u{00fe}'));
    assert!(Latin1::equals_ignore_case_char('\u{00e0}', '\u{00e0}'));
    assert!(Latin1::equals_ignore_case_code_point(0xc7, 0xe7));
    assert!(Latin1::equals_ignore_case_code_point(0xe0, 0xe0));
    assert!(!Latin1::equals_ignore_case_char('\u{00d7}', '\u{00f7}'));

    assert_eq!(b'A', Latin1::byte_to_uppercase(b'a'));
    assert_eq!(0xc0, Latin1::byte_to_uppercase(0xe0));
    assert_eq!(0xd8, Latin1::byte_to_uppercase(0xf8));
    assert_eq!('\u{00c0}', Latin1::char_to_uppercase('\u{00e0}'));
    assert_eq!('\u{0100}', Latin1::char_to_uppercase('\u{0100}'));
    assert_eq!(0xde, Latin1::code_point_to_uppercase(0xfe));
    assert_eq!(0x100, Latin1::code_point_to_uppercase(0x100));
    assert_eq!(0xdf, Latin1::byte_to_uppercase(0xdf));

    assert_eq!(0xe0, Latin1::byte_to_lowercase(0xc0));
    assert_eq!(0xf8, Latin1::byte_to_lowercase(0xd8));
    assert_eq!('\u{00fe}', Latin1::char_to_lowercase('\u{00de}'));
    assert_eq!('\u{0100}', Latin1::char_to_lowercase('\u{0100}'));
    assert_eq!(0xe7, Latin1::code_point_to_lowercase(0xc7));
    assert_eq!(0x100, Latin1::code_point_to_lowercase(0x100));

    assert_eq!(Some(7), Latin1::byte_to_digit(b'7'));
    assert_eq!(Some(7), Latin1::char_to_digit('7'));
    assert_eq!(Some(7), Latin1::code_point_to_digit('7' as u32));
    assert_eq!(Some(15), Latin1::char_to_hex_digit('f'));
    assert_eq!(Some(15), Latin1::code_point_to_hex_digit('F' as u32));
    assert_eq!(None, Latin1::byte_to_digit(0xb2));
    assert_eq!(None, Latin1::char_to_digit('\u{0100}'));
    assert_eq!(None, Latin1::char_to_hex_digit('\u{00e9}'));
    assert_eq!(None, Latin1::char_to_hex_digit('\u{0100}'));
    assert_eq!(None, Latin1::code_point_to_digit(0x100));
    assert_eq!(None, Latin1::code_point_to_hex_digit(0x100));
}
