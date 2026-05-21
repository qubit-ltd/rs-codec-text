use qubit_text_codec::{
    Charset,
    DecodeStatus,
    TextDecodeErrorKind,
    TextDecoder,
    Utf8,
    Utf8Decoder,
};

#[test]
fn test_utf8_decoder_exposes_charset_and_unit_width() {
    let decoder = Utf8Decoder;

    assert_eq!(Charset::UTF_8, decoder.charset());
    assert_eq!(Utf8::MAX_UNITS_PER_CHAR, decoder.max_units_per_char());
}

#[test]
fn test_utf8_decoder_decodes_prefix_and_next() {
    let decoder = Utf8Decoder;
    let bytes = "A中😀".as_bytes();

    assert_eq!(
        DecodeStatus::Complete {
            value: 'A',
            consumed: 1,
        },
        decoder.decode_prefix(bytes, 0).expect("ASCII prefix"),
    );

    let mut index = 0;
    assert_eq!(
        Some('A'),
        decoder.decode_next(bytes, &mut index).expect("A")
    );
    assert_eq!(1, index);
    assert_eq!(
        Some('中'),
        decoder.decode_next(bytes, &mut index).expect("CJK")
    );
    assert_eq!(4, index);
    assert_eq!(
        Some('😀'),
        decoder.decode_next(bytes, &mut index).expect("emoji")
    );
    assert_eq!(8, index);
    assert_eq!(None, decoder.decode_next(bytes, &mut index).expect("EOF"));
}

#[test]
fn test_utf8_decoder_reports_need_more_and_malformed_sequences() {
    let decoder = Utf8Decoder;

    assert_eq!(
        DecodeStatus::NeedMore {
            required: 3,
            available: 2,
        },
        decoder
            .decode_prefix(&[0xe4, 0xb8], 0)
            .expect("valid prefix needs more"),
    );

    let error = decoder
        .decode_prefix(&[0xe4, b'A', 0x80], 0)
        .expect_err("bad continuation must fail");
    assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 4,
            available: 2,
        },
        decoder
            .decode_prefix(&[0xf4, 0x90], 0)
            .expect("extended leading byte should need more bytes"),
    );

    let mut index = 0;
    let error = decoder
        .decode_next(&[0xf0, 0x9f], &mut index)
        .expect_err("closed incomplete input must fail");
    assert_eq!(TextDecodeErrorKind::IncompleteSequence, error.kind());
    assert_eq!(2, error.index());
}

#[test]
fn test_utf8_decoder_rejects_malformed_partial_prefixes() {
    let decoder = Utf8Decoder;

    for (bytes, index) in [
        (&[0xe4, b' '][..], 1),
        (&[0xe0, 0x9f][..], 1),
        (&[0xed, 0xa0][..], 1),
        (&[0xf0, b' '][..], 1),
        (&[0xf0, 0x90, b' '][..], 2),
    ] {
        let error = decoder
            .decode_prefix(bytes, 0)
            .expect_err("malformed partial UTF-8 prefix must fail");
        assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
        assert_eq!(index, error.index());
    }

    let mut index = 0;
    let error = decoder
        .decode_next(&[0xe4, b' '], &mut index)
        .expect_err("closed malformed input must not be reported incomplete");
    assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());
    assert_eq!(0, index);
}

#[test]
fn test_utf8_decoder_covers_well_formed_and_malformed_boundaries() {
    let decoder = Utf8Decoder;

    for bytes in [
        &[0xc2, 0x80][..],
        &[0xdf, 0xbf],
        &[0xe0, 0xa0, 0x80],
        &[0xed, 0x9f, 0xbf],
        &[0xee, 0x80, 0x80],
        &[0xf0, 0x90, 0x80, 0x80],
        &[0xf1, 0x80, 0x80, 0x80],
        &[0xf4, 0x8f, 0xbf, 0xbf],
    ] {
        assert!(matches!(
            decoder.decode_prefix(bytes, 0).expect("well-formed UTF-8"),
            DecodeStatus::Complete { .. },
        ));
    }

    for (bytes, index, kind) in [
        (&[0x80][..], 0, TextDecodeErrorKind::MalformedSequence),
        (&[0xc2, 0x20][..], 1, TextDecodeErrorKind::MalformedSequence),
        (
            &[0xe0, 0x9f, 0x80][..],
            1,
            TextDecodeErrorKind::MalformedSequence,
        ),
        (
            &[0xed, 0xa0, 0x80][..],
            1,
            TextDecodeErrorKind::MalformedSequence,
        ),
        (
            &[0xe1, 0x80, 0x20][..],
            2,
            TextDecodeErrorKind::MalformedSequence,
        ),
        (
            &[0xf0, 0x8f, 0xbf, 0xbf][..],
            1,
            TextDecodeErrorKind::MalformedSequence,
        ),
        (
            &[0xf4, 0x90, 0x80, 0x80][..],
            0,
            TextDecodeErrorKind::InvalidCodePoint,
        ),
        (
            &[0xf1, 0x80, 0x20, 0x80][..],
            2,
            TextDecodeErrorKind::MalformedSequence,
        ),
        (
            &[0xf1, 0x80, 0x80, 0x20][..],
            3,
            TextDecodeErrorKind::MalformedSequence,
        ),
    ] {
        let error = decoder
            .decode_prefix(bytes, 0)
            .expect_err("malformed UTF-8 must fail");
        assert_eq!(kind, error.kind());
        assert_eq!(index, error.index());
    }
}
