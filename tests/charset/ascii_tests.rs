use qubit_codec_text::Ascii;

#[test]
fn test_ascii_classifies_ascii_code_points() {
    assert!(Ascii::is_ascii_byte(b'A'));
    assert!(!Ascii::is_ascii_byte(0x80));
    assert!(Ascii::is_ascii_char('~'));
    assert!(!Ascii::is_ascii_char('中'));
    assert!(Ascii::is_ascii_code_point(0x7f));
    assert!(!Ascii::is_ascii_code_point(u32::MAX));
    assert!(!Ascii::is_ascii_code_point(0x80));
}

#[test]
fn test_ascii_exposes_digit_byte_arrays() {
    assert_eq!(b"0123456789", &Ascii::DIGIT_BYTES);
    assert_eq!(b"0123456789abcdef", &Ascii::LOWERCASE_HEX_DIGIT_BYTES);
    assert_eq!(b"0123456789ABCDEF", &Ascii::UPPERCASE_HEX_DIGIT_BYTES);
}

#[test]
fn test_ascii_classifies_common_character_sets() {
    assert!(Ascii::is_whitespace_byte(b'\n'));
    assert!(Ascii::is_whitespace_char(' '));
    assert!(Ascii::is_whitespace_code_point('\r' as u32));
    assert!(!Ascii::is_whitespace_byte(0xa0));
    assert!(!Ascii::is_whitespace_char('\u{00a0}'));
    assert!(!Ascii::is_whitespace_code_point(u32::MAX));

    assert!(Ascii::is_letter_byte(b'Q'));
    assert!(Ascii::is_letter_char('Q'));
    assert!(Ascii::is_letter_code_point('Q' as u32));
    assert!(!Ascii::is_letter_byte(0x80));
    assert!(!Ascii::is_letter_char('中'));
    assert!(!Ascii::is_letter_code_point(u32::MAX));

    assert!(Ascii::is_uppercase_letter_byte(b'Q'));
    assert!(Ascii::is_uppercase_letter_char('Q'));
    assert!(Ascii::is_uppercase_letter_code_point('Q' as u32));
    assert!(Ascii::is_lowercase_letter_byte(b'q'));
    assert!(Ascii::is_lowercase_letter_char('q'));
    assert!(Ascii::is_lowercase_letter_code_point('q' as u32));

    assert!(Ascii::is_digit_byte(b'7'));
    assert!(Ascii::is_digit_char('7'));
    assert!(Ascii::is_digit_code_point('7' as u32));
    assert!(Ascii::is_hex_digit_byte(b'f'));
    assert!(Ascii::is_hex_digit_char('F'));
    assert!(Ascii::is_hex_digit_code_point('9' as u32));
    assert!(Ascii::is_hex_digit_code_point('f' as u32));
    assert!(Ascii::is_hex_digit_code_point('F' as u32));
    assert!(Ascii::is_octal_digit_byte(b'7'));
    assert!(Ascii::is_octal_digit_char('7'));
    assert!(Ascii::is_octal_digit_code_point('7' as u32));
    assert!(!Ascii::is_octal_digit_byte(b'8'));
    assert!(!Ascii::is_octal_digit_char('8'));
    assert!(!Ascii::is_octal_digit_code_point('8' as u32));

    assert!(Ascii::is_letter_or_digit_byte(b'9'));
    assert!(Ascii::is_letter_or_digit_char('Q'));
    assert!(Ascii::is_letter_or_digit_code_point('q' as u32));
    assert!(Ascii::is_printable_byte(b'~'));
    assert!(Ascii::is_printable_char('~'));
    assert!(Ascii::is_printable_code_point('~' as u32));
    assert!(Ascii::is_control_byte(0x1f));
    assert!(Ascii::is_control_char('\u{001f}'));
    assert!(Ascii::is_control_code_point(0x1f));
}

#[test]
fn test_ascii_converts_case_and_digits() {
    assert!(Ascii::equals_ignore_case_byte(b'A', b'a'));
    assert!(Ascii::equals_ignore_case_byte(b'A', b'A'));
    assert!(Ascii::equals_ignore_case_char('A', 'a'));
    assert!(Ascii::equals_ignore_case_code_point('A' as u32, 'a' as u32));
    assert!(Ascii::equals_ignore_case_code_point('A' as u32, 'A' as u32));
    assert!(Ascii::equals_ignore_case_char('A', 'A'));
    assert!(!Ascii::equals_ignore_case_byte(b'A', b'B'));

    assert_eq!(b'Q', Ascii::byte_to_uppercase(b'q'));
    assert_eq!(b'Q', Ascii::byte_to_uppercase(b'Q'));
    assert_eq!('Q', Ascii::char_to_uppercase('q'));
    assert_eq!('Q', Ascii::char_to_uppercase('Q'));
    assert_eq!('Q' as u32, Ascii::code_point_to_uppercase('q' as u32));
    assert_eq!('Q' as u32, Ascii::code_point_to_uppercase('Q' as u32));
    assert_eq!(b'q', Ascii::byte_to_lowercase(b'Q'));
    assert_eq!(b'q', Ascii::byte_to_lowercase(b'q'));
    assert_eq!('q', Ascii::char_to_lowercase('Q'));
    assert_eq!('q', Ascii::char_to_lowercase('q'));
    assert_eq!('q' as u32, Ascii::code_point_to_lowercase('Q' as u32));

    assert_eq!(Some(7), Ascii::byte_to_digit(b'7'));
    assert_eq!(Some(7), Ascii::char_to_digit('7'));
    assert_eq!(Some(7), Ascii::code_point_to_digit('7' as u32));
    assert_eq!(None, Ascii::byte_to_digit(b'x'));
    assert_eq!(None, Ascii::char_to_digit('x'));
    assert_eq!(None, Ascii::code_point_to_digit(u32::MAX));
    assert_eq!(Some(9), Ascii::byte_to_hex_digit(b'9'));
    assert_eq!(Some(15), Ascii::byte_to_hex_digit(b'F'));
    assert_eq!(Some(15), Ascii::byte_to_hex_digit(b'f'));
    assert_eq!(Some(9), Ascii::char_to_hex_digit('9'));
    assert_eq!(Some(15), Ascii::char_to_hex_digit('F'));
    assert_eq!(Some(15), Ascii::char_to_hex_digit('f'));
    assert_eq!(Some(9), Ascii::code_point_to_hex_digit('9' as u32));
    assert_eq!(Some(15), Ascii::code_point_to_hex_digit('F' as u32));
    assert_eq!(Some(15), Ascii::code_point_to_hex_digit('f' as u32));
    assert_eq!(None, Ascii::byte_to_hex_digit(b'x'));
    assert_eq!(None, Ascii::char_to_hex_digit('x'));
    assert_eq!(None, Ascii::code_point_to_hex_digit(u32::MAX));
}

#[test]
fn test_ascii_fold_matches_java_ascii_fold_examples() {
    let mut buffer = ['\0'; Ascii::MAX_FOLDING_COUNT];

    let count = Ascii::fold('Æ', &mut buffer, 0);
    assert_eq!(2, count);
    assert_eq!(&['A', 'E'], &buffer[..count]);

    let count = Ascii::fold('⒑', &mut buffer, 0);
    assert_eq!(3, count);
    assert_eq!(&['1', '0', '.'], &buffer[..count]);

    let count = Ascii::fold('⑽', &mut buffer, 0);
    assert_eq!(4, count);
    assert_eq!(&['(', '1', '0', ')'], &buffer[..count]);

    let count = Ascii::fold('中', &mut buffer, 0);
    assert_eq!(1, count);
    assert_eq!('中', buffer[0]);

    let count = Ascii::fold('A', &mut buffer, 0);
    assert_eq!(1, count);
    assert_eq!('A', buffer[0]);

    assert_eq!("AE", Ascii::fold_to_string('Æ'));
}
