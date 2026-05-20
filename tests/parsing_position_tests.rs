use qubit_unicode::{
    ParsingPosition,
    UnicodeErrorKind,
    Utf8,
};

#[test]
fn test_parsing_position_moves_and_resets() {
    let mut pos = ParsingPosition::new(3);

    pos.increase();
    assert_eq!(4, pos.index());
    pos.increase_by(2);
    assert_eq!(6, pos.index());
    pos.decrease();
    assert_eq!(5, pos.index());
    pos.decrease_by(3);
    assert_eq!(2, pos.index());

    pos.reset(9);
    assert_eq!(9, pos.index());
    assert!(pos.success());

    let default_pos = ParsingPosition::default();
    assert_eq!(0, default_pos.index());
    assert!(default_pos.success());
}

#[test]
fn test_parsing_position_records_and_clears_errors() {
    let mut pos = ParsingPosition::new(0);

    let error = Utf8::get_next(&mut pos, &[0x80], 1).expect_err("invalid leading byte");
    assert_eq!(UnicodeErrorKind::Malformed, error.kind());
    assert_eq!(Some(0), pos.error_index());
    assert_eq!(Some(UnicodeErrorKind::Malformed), pos.error_kind());
    assert!(pos.fail());

    pos.clear_error();
    assert_eq!(None, pos.error_index());
    assert_eq!(None, pos.error_kind());
    assert!(pos.success());
}
