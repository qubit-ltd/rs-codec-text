use qubit_text_codec::{
    AsciiCodec,
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeErrorKind,
    DecodeStatus,
};

#[test]
fn test_ascii_codec_exposes_identity_and_limits() {
    let codec = AsciiCodec;

    assert_eq!(Charset::ASCII, <AsciiCodec as CharsetCodec>::charset(&codec));
    assert_eq!(1, <AsciiCodec as CharsetCodec>::max_units_per_char(&codec));
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 1,
            available: 0,
        },
        <AsciiCodec as CharsetCodec>::decode_one(&codec, &[], 0).expect("ascii need more"),
    );
    assert_eq!(
        1,
        <AsciiCodec as CharsetCodec>::encode_one(&codec, 'A', &mut [0_u8; 1], 0).expect("encode ascii"),
    );

    assert_eq!(Charset::ASCII, codec.charset());
    assert_eq!(1, codec.max_units_per_char());
    assert_eq!(1, codec.max_units_per_char());
    assert_eq!(Charset::ASCII, codec.charset());
}

#[test]
fn test_ascii_codec_decodes_ascii_bytes_and_reports_need_more_and_malformed() {
    let codec = AsciiCodec;

    assert_eq!(
        DecodeStatus::Complete {
            value: 'A',
            consumed: 1,
        },
        codec.decode_one(b"A", 0).expect("ASCII decode"),
    );
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 1,
            available: 0,
        },
        codec.decode_one(&[], 0).expect("need more for empty input"),
    );
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 2,
            available: 0,
        },
        codec.decode_one(&[0x41], 1).expect("need more at exact boundary"),
    );

    let error = codec.decode_one(&[0x80], 0).expect_err("non-ASCII byte is malformed");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(128) },
        error.kind()
    );
    assert_eq!(0, error.index());

    let error = codec.decode_one(&[0x41], 2).expect_err("index out of range is invalid");
    assert_eq!(CharsetDecodeErrorKind::InvalidInputIndex { input_len: 1 }, error.kind());
    assert_eq!(2, error.index());
}

#[test]
fn test_ascii_codec_encodes_ascii_and_reports_limits_and_unmappable_chars() {
    let codec = AsciiCodec;
    let mut output = [0_u8; 2];

    assert_eq!(1, codec.encode_one('A', &mut output, 0).expect("encode ASCII"));
    assert_eq!(b'A', output[0]);

    let error = codec
        .encode_one('é', &mut output, 0)
        .expect_err("non-ASCII is unmappable");
    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::UnmappableCharacter { value: _ },
    ));
    assert_eq!(Some('é' as u32), error.value());

    let short_error = codec
        .encode_one('A', &mut output, 2)
        .expect_err("output index out of range should fail");
    assert!(matches!(
        short_error.kind(),
        CharsetEncodeErrorKind::BufferTooSmall { .. },
    ));
    assert_eq!(2, short_error.index());
}
