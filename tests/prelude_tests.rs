use qubit_unicode::prelude::{
    Ascii,
    ParsingPosition,
    Unicode,
    UnicodeError,
    UnicodeErrorKind,
    UnicodeResult,
    Utf8,
    Utf16,
};

#[test]
fn test_prelude_reexports_common_types() {
    let mut pos = ParsingPosition::new(0);
    let err = Utf8::get_next(&mut pos, &[0x80], 1).expect_err("invalid leading byte");

    assert!(Ascii::is_ascii_char('A'));
    assert!(Unicode::is_valid_unicode('中' as i32));
    assert_eq!(Some(1), Utf16::code_unit_count('A' as u32));
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    let direct_error: UnicodeError = err;
    let result: UnicodeResult<()> = Err(direct_error);
    assert!(result.is_err());
}
