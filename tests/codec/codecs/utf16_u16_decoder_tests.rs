use qubit_text_codec::{
    Charset,
    DecodeStatus,
    TextDecodeErrorKind,
    TextDecoder,
    Utf16,
    Utf16U16Decoder,
};

#[test]
fn test_utf16_u16_decoder_exposes_charset_and_unit_width() {
    let decoder = Utf16U16Decoder;

    assert_eq!(Charset::UTF_16, decoder.charset());
    assert_eq!(Utf16::MAX_UNITS_PER_CHAR, decoder.max_units_per_char());
}

#[test]
fn test_utf16_u16_decoder_decodes_units() {
    let decoder = Utf16U16Decoder;
    let units = [0x0041, 0x4e2d, 0xd83d, 0xde00];
    let mut index = 0;

    assert_eq!(
        Some('A'),
        decoder.decode_next(&units, &mut index).expect("ASCII")
    );
    assert_eq!(
        Some('中'),
        decoder.decode_next(&units, &mut index).expect("BMP")
    );
    assert_eq!(
        Some('😀'),
        decoder.decode_next(&units, &mut index).expect("pair")
    );
    assert_eq!(None, decoder.decode_next(&units, &mut index).expect("EOF"));
}

#[test]
fn test_utf16_u16_decoder_reports_need_more_and_malformed_pairs() {
    let decoder = Utf16U16Decoder;

    assert_eq!(
        DecodeStatus::NeedMore {
            required: 2,
            available: 1,
        },
        decoder
            .decode_prefix(&[0xd83d], 0)
            .expect("high surrogate needs low surrogate"),
    );

    let error = decoder
        .decode_prefix(&[0xde00], 0)
        .expect_err("low surrogate cannot start a scalar");
    assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(0, error.index());

    let error = decoder
        .decode_prefix(&[0xd83d, 0x0041], 0)
        .expect_err("bad surrogate pair must fail");
    assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());
}

#[test]
fn test_utf16_u16_decoder_matches_std_decode_utf16_boundaries() {
    let decoder = Utf16U16Decoder;

    for units in [
        &[0x0041][..],
        &[0x4e2d],
        &[0xd7ff],
        &[0xe000],
        &[0xd800, 0xdc00],
        &[0xd83d, 0xde00],
        &[0xdbff, 0xdfff],
    ] {
        let expected = char::decode_utf16(units.iter().copied())
            .next()
            .expect("sample contains one scalar")
            .expect("standard library accepts valid UTF-16");

        assert_eq!(
            DecodeStatus::Complete {
                value: expected,
                consumed: expected.len_utf16(),
            },
            decoder
                .decode_prefix(units, 0)
                .expect("decoder accepts valid UTF-16"),
        );
    }

    for (units, index) in [
        (&[0xde00][..], 0),
        (&[0xd83d, 0x0041][..], 1),
        (&[0xd83d, 0xd83d][..], 1),
    ] {
        assert!(
            char::decode_utf16(units.iter().copied())
                .next()
                .expect("sample contains one result")
                .is_err(),
            "standard library rejects malformed UTF-16"
        );
        let error = decoder
            .decode_prefix(units, 0)
            .expect_err("decoder rejects malformed UTF-16");
        assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
        assert_eq!(index, error.index());
    }
}
