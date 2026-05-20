use qubit_unicode::UnicodeErrorKind;

#[test]
fn test_unicode_error_kind_displays_messages() {
    assert_eq!(
        "The buffer overflows.",
        UnicodeErrorKind::BufferOverflow.to_string()
    );
    assert_eq!(
        "The Unicode code unit sequence is malformed.",
        UnicodeErrorKind::Malformed.to_string()
    );
    assert_eq!(
        "The Unicode code unit sequence is incomplete.",
        UnicodeErrorKind::Incomplete.to_string()
    );
}
