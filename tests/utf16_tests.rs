use qubit_unicode::{
    ParsingPosition,
    UnicodeErrorKind,
    Utf16,
};

#[test]
fn test_utf16_classifies_code_units_and_surrogate_pairs() {
    assert!(Utf16::is_single('A' as u16));
    assert!(!Utf16::is_single(0xd83d));
    assert!(Utf16::is_leading(0xd83d));
    assert!(Utf16::is_trailing(0xde00));
    assert!(Utf16::is_surrogate(0xd83d));
    assert!(Utf16::is_surrogate_pair(0xd83d, 0xde00));
    assert_eq!(Some(0x1f600), Utf16::compose(0xd83d, 0xde00));
    assert_eq!(Some(0xd83d), Utf16::decompose_high(0x1f600));
    assert_eq!(Some(0xde00), Utf16::decompose_low(0x1f600));
    assert_eq!(1, Utf16::trailing_count(0xd83d));
    assert_eq!(0, Utf16::trailing_count('A' as u16));
    assert_eq!(Some(1), Utf16::code_unit_count('A' as u32));
    assert_eq!(Some(2), Utf16::code_unit_count(0x1f600));
    assert_eq!(None, Utf16::code_unit_count(0xd800));
}

#[test]
fn test_utf16_get_next_and_get_previous_move_across_code_points() {
    let units = [0x0041, 0x4e2d, 0xd83d, 0xde00];
    let mut pos = ParsingPosition::new(0);

    assert_eq!(
        Some('A'),
        Utf16::get_next(&mut pos, &units, units.len()).expect("ASCII")
    );
    assert_eq!(1, pos.index());
    assert_eq!(
        Some('中'),
        Utf16::get_next(&mut pos, &units, units.len()).expect("BMP")
    );
    assert_eq!(2, pos.index());
    assert_eq!(
        Some('😀'),
        Utf16::get_next(&mut pos, &units, units.len()).expect("supplementary")
    );
    assert_eq!(4, pos.index());
    assert_eq!(
        None,
        Utf16::get_next(&mut pos, &units, units.len()).expect("end")
    );

    assert_eq!(
        Some('😀'),
        Utf16::get_previous(&mut pos, &units, 0).expect("previous supplementary")
    );
    assert_eq!(2, pos.index());
    assert_eq!(
        Some('中'),
        Utf16::get_previous(&mut pos, &units, 0).expect("previous BMP")
    );
    assert_eq!(1, pos.index());
}

#[test]
fn test_utf16_forward_backward_and_boundary_adjustment() {
    let units = [0x0041, 0xd83d, 0xde00, 0x0042];
    let mut pos = ParsingPosition::new(1);

    assert_eq!(
        2,
        Utf16::forward(&mut pos, &units, units.len()).expect("forward")
    );
    assert_eq!(3, pos.index());
    assert_eq!(2, Utf16::backward(&mut pos, &units, 0).expect("backward"));
    assert_eq!(1, pos.index());

    pos.set_index(2);
    assert_eq!(
        1,
        Utf16::set_to_start(&mut pos, &units, 0).expect("set to start")
    );
    assert_eq!(1, pos.index());
    assert_eq!(
        1,
        Utf16::set_to_terminal(&mut pos, &units, units.len()).expect("terminal")
    );
    assert_eq!(2, pos.index());

    pos.set_index(0);
    assert_eq!(
        0,
        Utf16::set_to_start(&mut pos, &units, 0).expect("not trailing")
    );
    assert_eq!(
        0,
        Utf16::set_to_terminal(&mut pos, &units, units.len()).expect("not leading")
    );

    pos.set_index(units.len());
    assert_eq!(
        0,
        Utf16::forward(&mut pos, &units, units.len()).expect("end")
    );
    assert_eq!(
        0,
        Utf16::set_to_terminal(&mut pos, &units, units.len()).expect("end")
    );
    assert_eq!(
        None,
        Utf16::get_next(&mut pos, &units, units.len()).expect("end next")
    );

    let mut start = ParsingPosition::new(0);
    assert_eq!(0, Utf16::backward(&mut start, &units, 0).expect("start"));
    assert_eq!(
        None,
        Utf16::get_previous(&mut start, &units, 0).expect("start previous")
    );

    let mut after_single = ParsingPosition::new(1);
    assert_eq!(
        1,
        Utf16::backward(&mut after_single, &units, 0).expect("single")
    );
    assert_eq!(0, after_single.index());
}

#[test]
fn test_utf16_put_and_escape_encode_scalar_values() {
    let mut buffer = [0; Utf16::MAX_CODE_UNIT_COUNT];
    let end_index = buffer.len();

    let count = Utf16::put('中' as u32, 0, &mut buffer, end_index).expect("encode BMP");
    assert_eq!(1, count);
    assert_eq!(0x4e2d, buffer[0]);

    let count = Utf16::put(0x1f600, 0, &mut buffer, end_index).expect("encode emoji");
    assert_eq!(2, count);
    assert_eq!([0xd83d, 0xde00], buffer);

    assert_eq!(Some("\\u0041".to_string()), Utf16::escape_java('A' as u32));
    assert_eq!(Some("\\u4E2D".to_string()), Utf16::escape_java('中' as u32));
    assert_eq!(
        Some("\\uD83D\\uDE00".to_string()),
        Utf16::escape_java(0x1f600)
    );
    assert_eq!(None, Utf16::escape_java(0x110000));

    assert_eq!(Some("\\u{41}".to_string()), Utf16::escape_rust('A' as u32));
    assert_eq!(
        Some("\\u{4E2D}".to_string()),
        Utf16::escape_rust('中' as u32)
    );
    assert_eq!(Some("\\u{1F600}".to_string()), Utf16::escape_rust(0x1f600));
    assert_eq!(None, Utf16::escape_rust(0x110000));
}

#[test]
fn test_utf16_reports_malformed_incomplete_and_overflow() {
    let mut pos = ParsingPosition::new(0);

    let err = Utf16::get_next(&mut pos, &[0xde00], 1).expect_err("low surrogate first");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());
    assert_eq!(Some(0), pos.error_index());

    pos.reset(0);
    let err = Utf16::get_next(&mut pos, &[0xd83d], 1).expect_err("missing low surrogate");
    assert_eq!(UnicodeErrorKind::Incomplete, err.kind());
    assert_eq!(Some(1), pos.error_index());

    let mut tiny = [0; 1];
    let err = Utf16::put(0x1f600, 0, &mut tiny, 1).expect_err("not enough space");
    assert_eq!(UnicodeErrorKind::BufferOverflow, err.kind());

    let err = Utf16::put(0xd800, 0, &mut tiny, 1).expect_err("surrogate scalar");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(1);
    let err = Utf16::set_to_start(&mut pos, &[0x0041, 0xde00], 0).expect_err("bad high");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(0);
    let err = Utf16::set_to_start(&mut pos, &[0xde00], 0).expect_err("missing high");
    assert_eq!(UnicodeErrorKind::Incomplete, err.kind());

    pos.reset(0);
    let err = Utf16::set_to_terminal(&mut pos, &[0xd83d], 1).expect_err("missing low");
    assert_eq!(UnicodeErrorKind::Incomplete, err.kind());

    pos.reset(0);
    let err = Utf16::set_to_terminal(&mut pos, &[0xd83d, 0x0041], 2).expect_err("bad low");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(1);
    let err = Utf16::backward(&mut pos, &[0xd83d], 0).expect_err("high before cursor");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(1);
    let err = Utf16::backward(&mut pos, &[0xde00], 0).expect_err("low at start");
    assert_eq!(UnicodeErrorKind::Incomplete, err.kind());

    pos.reset(2);
    let err = Utf16::backward(&mut pos, &[0x0041, 0xde00], 0).expect_err("bad leading");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(0);
    let err = Utf16::get_next(&mut pos, &[0xd83d, 0x0041], 2).expect_err("bad pair");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    assert_eq!(None, Utf16::escape_java(0xd800));
    assert_eq!(None, Utf16::escape_rust(0xd800));
}
