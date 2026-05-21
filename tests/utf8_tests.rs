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
fn test_utf8_get_next_and_get_previous_accept_table_3_7_boundaries() {
    let cases: &[(&[u8], u32)] = &[
        (&[0x00], 0x0000),
        (&[0x7f], 0x007f),
        (&[0xc2, 0x80], 0x0080),
        (&[0xdf, 0xbf], 0x07ff),
        (&[0xe0, 0xa0, 0x80], 0x0800),
        (&[0xed, 0x9f, 0xbf], 0xd7ff),
        (&[0xee, 0x80, 0x80], 0xe000),
        (&[0xef, 0xbf, 0xbf], 0xffff),
        (&[0xf0, 0x90, 0x80, 0x80], 0x10000),
        (&[0xf1, 0x80, 0x80, 0x80], 0x40000),
        (&[0xf3, 0xbf, 0xbf, 0xbf], 0xfffff),
        (&[0xf4, 0x8f, 0xbf, 0xbf], 0x10ffff),
    ];

    for &(bytes, code_point) in cases {
        let expected = char::from_u32(code_point).expect("test case must be a scalar value");

        let mut next_pos = ParsingPosition::new(0);
        assert_eq!(
            Some(expected),
            Utf8::get_next(&mut next_pos, bytes, bytes.len()).expect("well-formed UTF-8")
        );
        assert_eq!(bytes.len(), next_pos.index());

        let mut previous_pos = ParsingPosition::new(bytes.len());
        assert_eq!(
            Some(expected),
            Utf8::get_previous(&mut previous_pos, bytes, 0).expect("well-formed UTF-8")
        );
        assert_eq!(0, previous_pos.index());
    }
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

    let cases = [
        0x0000, 0x007f, 0x0080, 0x07ff, 0x0800, 0xd7ff, 0xe000, 0xffff, 0x10000, 0x10ffff,
    ];
    for code_point in cases {
        buffer.fill(0);
        let count = Utf8::put(code_point, 0, &mut buffer, end_index).expect("encode scalar");
        let ch = char::from_u32(code_point).expect("test case must be a scalar value");
        let mut expected = [0; Utf8::MAX_CODE_UNIT_COUNT];
        let expected = ch.encode_utf8(&mut expected);
        assert_eq!(expected.as_bytes(), &buffer[..count]);
    }

    let count = Utf8::put('中' as u32, 0, &mut buffer, end_index).expect("encode CJK");
    assert_eq!(3, count);
    assert_eq!("中".as_bytes(), &buffer[..count]);

    let count = Utf8::put(0x1f600, 0, &mut buffer, end_index).expect("encode emoji");
    assert_eq!(4, count);
    assert_eq!("😀".as_bytes(), &buffer[..count]);

    let count = Utf8::put_char('A', 0, &mut buffer, end_index).expect("encode ASCII char");
    assert_eq!(1, count);
    assert_eq!(b"A", &buffer[..count]);

    let count = Utf8::put_char('中', 0, &mut buffer, end_index).expect("encode CJK char");
    assert_eq!(3, count);
    assert_eq!("中".as_bytes(), &buffer[..count]);

    let count = Utf8::put_char('😀', 0, &mut buffer, end_index).expect("encode emoji char");
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
    let err = Utf8::get_next(&mut pos, &[0xc2], 1).expect_err("truncated two-byte sequence");
    assert_eq!(UnicodeErrorKind::Incomplete, err.kind());
    assert_eq!(Some(1), pos.error_index());

    pos.reset(0);
    let err =
        Utf8::get_next(&mut pos, &[0xf0, 0x90, 0x80], 3).expect_err("truncated four-byte sequence");
    assert_eq!(UnicodeErrorKind::Incomplete, err.kind());
    assert_eq!(Some(3), pos.error_index());

    pos.reset(0);
    let err = Utf8::get_next(&mut pos, &[0xed, 0xa0, 0x80], 3).expect_err("surrogate");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    let malformed_next_cases: &[(&[u8], usize, &str)] = &[
        (&[0xc2, 0x20], 1, "invalid two-byte continuation"),
        (&[0xe0, 0x9f, 0x80], 1, "overlong three-byte sequence"),
        (&[0xed, 0xa0, 0x80], 1, "UTF-8 surrogate sequence"),
        (&[0xe1, 0x80, 0x20], 2, "invalid third byte"),
        (&[0xf0, 0x8f, 0xbf, 0xbf], 1, "overlong four-byte sequence"),
        (&[0xf4, 0x90, 0x80, 0x80], 1, "code point above U+10FFFF"),
        (
            &[0xf1, 0x80, 0x20, 0x80],
            2,
            "invalid third byte in four-byte sequence",
        ),
        (&[0xf1, 0x80, 0x80, 0x20], 3, "invalid fourth byte"),
        (&[0xf5, 0x80, 0x80, 0x80], 0, "disallowed leading byte"),
        (&[0xe1, 0xc0, 0x80], 1, "invalid second byte"),
    ];
    for &(bytes, error_index, label) in malformed_next_cases {
        pos.reset(0);
        let err = Utf8::get_next(&mut pos, bytes, bytes.len()).expect_err(label);
        assert_eq!(UnicodeErrorKind::Malformed, err.kind(), "{label}");
        assert_eq!(Some(error_index), pos.error_index(), "{label}");
    }

    let malformed_previous_cases: &[(&[u8], usize, &str)] = &[
        (&[0xe0, 0x9f, 0x80], 1, "overlong three-byte sequence"),
        (&[0xed, 0xa0, 0x80], 1, "UTF-8 surrogate sequence"),
        (&[0xf0, 0x8f, 0xbf, 0xbf], 1, "overlong four-byte sequence"),
        (&[0xf4, 0x90, 0x80, 0x80], 1, "code point above U+10FFFF"),
        (
            &[0xf1, 0x80, 0x20, 0x80],
            2,
            "invalid third byte in four-byte sequence",
        ),
        (&[0xe1, 0xc0, 0x80], 1, "invalid second byte"),
    ];
    for &(bytes, error_index, label) in malformed_previous_cases {
        pos.reset(bytes.len());
        let err = Utf8::get_previous(&mut pos, bytes, 0).expect_err(label);
        assert_eq!(UnicodeErrorKind::Malformed, err.kind(), "{label}");
        assert_eq!(Some(error_index), pos.error_index(), "{label}");
    }

    for &(bytes, label) in &[
        (&[0xc2, 0x20][..], "ASCII after bad two-byte leading"),
        (&[0xe1, 0x80, 0x20][..], "ASCII after bad three-byte tail"),
        (
            &[0xf1, 0x80, 0x80, 0x20][..],
            "ASCII after bad four-byte tail",
        ),
    ] {
        pos.reset(bytes.len());
        assert_eq!(
            Some(' '),
            Utf8::get_previous(&mut pos, bytes, 0).expect(label),
            "{label}"
        );
        assert_eq!(bytes.len() - 1, pos.index(), "{label}");
    }

    let mut tiny = [0; 2];
    let err = Utf8::put('中' as u32, 0, &mut tiny, 2).expect_err("not enough space");
    assert_eq!(UnicodeErrorKind::BufferOverflow, err.kind());

    let err = Utf8::put_char('中', 0, &mut tiny, 2).expect_err("char not enough space");
    assert_eq!(UnicodeErrorKind::BufferOverflow, err.kind());

    let err = Utf8::put(0xd800, 0, &mut tiny, 2).expect_err("surrogate scalar");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    let err = Utf8::put(0x110000, 0, &mut tiny, 2).expect_err("out-of-range scalar");
    assert_eq!(UnicodeErrorKind::Malformed, err.kind());

    let mut one = [0; 1];
    let err = Utf8::put(0x7f, 0, &mut one, 0).expect_err("ASCII overflow");
    assert_eq!(UnicodeErrorKind::BufferOverflow, err.kind());
    assert_eq!(0, one[0]);

    let err = Utf8::put(0x80, 0, &mut one, 1).expect_err("two-byte overflow");
    assert_eq!(UnicodeErrorKind::BufferOverflow, err.kind());

    let mut three = [0; 3];
    let err = Utf8::put(0x10000, 0, &mut three, 3).expect_err("four-byte overflow");
    assert_eq!(UnicodeErrorKind::BufferOverflow, err.kind());

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
