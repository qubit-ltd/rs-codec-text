use qubit_unicode::{
    ParsingPosition,
    UnicodeErrorKind,
    Utf8,
};

#[test]
fn test_utf8_classifies_code_units() {
    assert!(Utf8::is_single(b'A'));
    assert!(!Utf8::is_single(0x80));
    assert!(Utf8::is_leading(0xe4));
    assert!(Utf8::is_leading(0xf0));
    assert!(!Utf8::is_leading(0xc1));
    assert!(Utf8::is_trailing(0xb8));
    assert!(!Utf8::is_trailing(b'A'));
    assert_eq!(Some(1), Utf8::trailing_count(0xc2));
    assert_eq!(Some(2), Utf8::trailing_count(0xe4));
    assert_eq!(Some(3), Utf8::trailing_count(0xf0));
    assert_eq!(None, Utf8::trailing_count(b'A'));
    assert_eq!(Some(1), Utf8::code_unit_count('A' as u32));
    assert_eq!(Some(2), Utf8::code_unit_count('é' as u32));
    assert_eq!(Some(3), Utf8::code_unit_count('中' as u32));
    assert_eq!(Some(4), Utf8::code_unit_count(0x1f600));
    assert_eq!(None, Utf8::code_unit_count(0xd800));
    assert_eq!(None, Utf8::code_unit_count(0x110000));
}

#[test]
fn test_utf8_get_next_and_get_previous_move_across_code_points() {
    let bytes = "A中😀".as_bytes();
    let mut pos = ParsingPosition::new(0);

    assert_eq!(
        Some('A'),
        Utf8::get_next(&mut pos, bytes, bytes.len()).expect("ASCII")
    );
    assert_eq!(1, pos.index());
    assert_eq!(
        Some('中'),
        Utf8::get_next(&mut pos, bytes, bytes.len()).expect("CJK")
    );
    assert_eq!(4, pos.index());
    assert_eq!(
        Some('😀'),
        Utf8::get_next(&mut pos, bytes, bytes.len()).expect("emoji")
    );
    assert_eq!(8, pos.index());
    assert_eq!(
        None,
        Utf8::get_next(&mut pos, bytes, bytes.len()).expect("end")
    );

    assert_eq!(
        Some('😀'),
        Utf8::get_previous(&mut pos, bytes, 0).expect("previous emoji")
    );
    assert_eq!(4, pos.index());
    assert_eq!(
        Some('中'),
        Utf8::get_previous(&mut pos, bytes, 0).expect("previous CJK")
    );
    assert_eq!(1, pos.index());
}

#[test]
fn test_utf8_forward_backward_and_boundary_adjustment() {
    let bytes = "A中😀".as_bytes();
    let mut pos = ParsingPosition::new(1);

    assert_eq!(
        3,
        Utf8::forward(&mut pos, bytes, bytes.len()).expect("forward")
    );
    assert_eq!(4, pos.index());
    assert_eq!(
        4,
        Utf8::forward(&mut pos, bytes, bytes.len()).expect("forward")
    );
    assert_eq!(8, pos.index());

    assert_eq!(4, Utf8::backward(&mut pos, bytes, 0).expect("backward"));
    assert_eq!(4, pos.index());

    pos.set_index(6);
    assert_eq!(
        2,
        Utf8::set_to_start(&mut pos, bytes, 0).expect("set to start")
    );
    assert_eq!(4, pos.index());
    assert_eq!(
        3,
        Utf8::set_to_terminal(&mut pos, bytes, bytes.len()).expect("terminal")
    );
    assert_eq!(7, pos.index());

    pos.set_index(0);
    assert_eq!(
        0,
        Utf8::set_to_start(&mut pos, bytes, 0).expect("not trailing")
    );
    assert_eq!(
        0,
        Utf8::set_to_terminal(&mut pos, bytes, bytes.len()).expect("not leading")
    );

    pos.set_index(bytes.len());
    assert_eq!(0, Utf8::forward(&mut pos, bytes, bytes.len()).expect("end"));
    assert_eq!(
        0,
        Utf8::set_to_terminal(&mut pos, bytes, bytes.len()).expect("end")
    );
    assert_eq!(
        None,
        Utf8::get_next(&mut pos, bytes, bytes.len()).expect("end next")
    );

    let mut start = ParsingPosition::new(0);
    assert_eq!(0, Utf8::backward(&mut start, bytes, 0).expect("start"));
    assert_eq!(
        None,
        Utf8::get_previous(&mut start, bytes, 0).expect("start previous")
    );

    let mut after_ascii = ParsingPosition::new(1);
    assert_eq!(
        1,
        Utf8::backward(&mut after_ascii, bytes, 0).expect("ASCII")
    );
    assert_eq!(0, after_ascii.index());
}

#[test]
fn test_utf8_put_encodes_scalar_values() {
    let mut buffer = [0; Utf8::MAX_CODE_UNIT_COUNT];
    let end_index = buffer.len();

    let count = Utf8::put('中' as u32, 0, &mut buffer, end_index).expect("encode CJK");
    assert_eq!(3, count);
    assert_eq!("中".as_bytes(), &buffer[..count]);

    let count = Utf8::put(0x1f600, 0, &mut buffer, end_index).expect("encode emoji");
    assert_eq!(4, count);
    assert_eq!("😀".as_bytes(), &buffer[..count]);
}

#[test]
fn test_utf8_reports_malformed_incomplete_and_overflow() {
    let mut pos = ParsingPosition::new(0);

    let err = Utf8::get_next(&mut pos, &[0xc0, 0x80], 2).expect_err("overlong NUL");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());
    assert_eq!(Some(0), pos.error_index());

    pos.reset(0);
    let err = Utf8::get_next(&mut pos, &[0xe4, 0xb8], 2).expect_err("truncated CJK");
    assert_eq!(UnicodeErrorKind::Incomplete, err.kind());
    assert_eq!(Some(2), pos.error_index());

    pos.reset(0);
    let err = Utf8::get_next(&mut pos, &[0xed, 0xa0, 0x80], 3).expect_err("surrogate");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    let mut tiny = [0; 2];
    let err = Utf8::put('中' as u32, 0, &mut tiny, 2).expect_err("not enough space");
    assert_eq!(UnicodeErrorKind::BufferOverflow, err.kind());

    let err = Utf8::put(0xd800, 0, &mut tiny, 2).expect_err("surrogate scalar");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(1);
    let err = Utf8::set_to_start(&mut pos, &[0x41, 0x80], 0).expect_err("bad leading");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(0);
    let err = Utf8::set_to_start(&mut pos, &[0x80], 0).expect_err("missing leading");
    assert_eq!(UnicodeErrorKind::Incomplete, err.kind());

    pos.reset(4);
    let err = Utf8::set_to_start(&mut pos, &[0xf0, 0x80, 0x80, 0x80, 0x80], 0)
        .expect_err("too many trailing bytes");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(0);
    let err = Utf8::set_to_terminal(&mut pos, &[0xe4, 0xb8], 2).expect_err("missing trailing");
    assert_eq!(UnicodeErrorKind::Incomplete, err.kind());

    pos.reset(0);
    let err = Utf8::set_to_terminal(&mut pos, &[0xe4, b'A', 0x80], 3).expect_err("bad trailing");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(1);
    let err = Utf8::backward(&mut pos, &[0xe4], 0).expect_err("leading before cursor");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(1);
    let err = Utf8::backward(&mut pos, &[0x80], 0).expect_err("trailing at start");
    assert_eq!(UnicodeErrorKind::Incomplete, err.kind());

    pos.reset(2);
    let err = Utf8::backward(&mut pos, &[0x41, 0x80], 0).expect_err("bad leading before trailing");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(3);
    let err = Utf8::backward(&mut pos, &[0xf0, 0x80, 0x80], 0).expect_err("wrong byte count");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(0);
    let err = Utf8::get_next(&mut pos, &[0xe4, b'A', 0x80], 3).expect_err("bad next trailing");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    pos.reset(3);
    let err = Utf8::get_previous(&mut pos, &[0xed, 0xa0, 0x80], 0)
        .expect_err("surrogate while reading previous");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());
}
