use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeErrorKind,
    DecodeStatus,
    Latin1Codec,
    Unicode,
};

#[test]
fn test_latin1_codec_exposes_identity_and_limits() {
    let codec = Latin1Codec;

    assert_eq!(Charset::ISO_8859_1, <Latin1Codec as CharsetCodec>::charset(&codec));
    assert_eq!(1, <Latin1Codec as CharsetCodec>::max_units_per_char(&codec));
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 1,
            available: 0,
        },
        <Latin1Codec as CharsetCodec>::decode_one(&codec, &[], 0).expect("latin1 need more"),
    );
    assert_eq!(
        1,
        <Latin1Codec as CharsetCodec>::encode_one(&codec, 'A', &mut [0_u8; 1], 0).expect("encode latin1"),
    );

    assert_eq!(Charset::ISO_8859_1, codec.charset());
    assert_eq!(1, codec.max_units_per_char());
    assert_eq!(1, codec.max_units_per_char());
    assert_eq!(Charset::ISO_8859_1, codec.charset());
}

#[test]
fn test_latin1_codec_decodes_all_byte_values() {
    let codec = Latin1Codec;
    let input = [0u8, 0x7f, 0xff];

    assert_eq!(
        DecodeStatus::Complete {
            value: '\u{0000}',
            consumed: 1,
        },
        codec.decode_one(&input, 0).expect("decode zero"),
    );
    assert_eq!(
        DecodeStatus::Complete {
            value: '\u{007f}',
            consumed: 1,
        },
        codec.decode_one(&input, 1).expect("decode DEL"),
    );
    assert_eq!(
        DecodeStatus::Complete {
            value: Unicode::to_char(Unicode::LATIN1_MAX).expect("valid Latin-1 max"),
            consumed: 1,
        },
        codec.decode_one(&input, 2).expect("decode 0xFF"),
    );
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 1,
            available: 0,
        },
        codec.decode_one(&[], 0).expect("need more for empty input"),
    );
}

#[test]
fn test_latin1_codec_reports_errors_for_invalid_indices_and_unmappable_characters() {
    let codec = Latin1Codec;
    let mut output = [0_u8; 1];

    let error = codec.decode_one(&[0x41], 2).expect_err("index out of range is invalid");
    assert_eq!(CharsetDecodeErrorKind::InvalidInputIndex { input_len: 1 }, error.kind());

    assert_eq!(
        1,
        codec.encode_one('\u{00ff}', &mut output, 0).expect("max valid latin1"),
    );
    assert_eq!(0xff, output[0]);

    let error = codec
        .encode_one('\u{0100}', &mut output, 0)
        .expect_err("above Latin-1 is unmappable");
    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::UnmappableCharacter { .. },
    ));
    assert_eq!(Some('\u{0100}' as u32), error.value());

    let error = codec
        .encode_one('\u{00a9}', &mut output, 1)
        .expect_err("output index out of range");
    assert!(matches!(error.kind(), CharsetEncodeErrorKind::BufferTooSmall { .. },));
    assert_eq!(1, error.index());
}
