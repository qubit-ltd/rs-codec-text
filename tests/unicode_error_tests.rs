use qubit_unicode::{
    UnicodeError,
    UnicodeErrorKind,
};

#[test]
fn test_unicode_error_exposes_kind_and_index() {
    let error = UnicodeError::new(UnicodeErrorKind::Malformed, 7);

    assert_eq!(UnicodeErrorKind::Malformed, error.kind());
    assert_eq!(7, error.index());
    assert_eq!(
        "The Unicode code unit sequence is malformed. at index 7",
        error.to_string()
    );
}
