use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeErrorKind,
    DecodeStatus,
    Utf16,
    Utf16U16Codec,
};

#[test]
fn test_utf16_u16_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf16U16Codec;

    assert_eq!(Charset::UTF_16, <Utf16U16Codec as CharsetCodec>::charset(&codec));
    assert_eq!(
        Utf16::MAX_UNITS_PER_CHAR,
        <Utf16U16Codec as CharsetCodec>::max_units_per_char(&codec)
    );
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 1,
            available: 0,
        },
        <Utf16U16Codec as CharsetCodec>::decode_one(&codec, &[], 0).expect("utf16 need more"),
    );
    assert_eq!(
        1,
        <Utf16U16Codec as CharsetCodec>::encode_one(&codec, 'A', &mut [0_u16; 2], 0).expect("encode utf16 bmp"),
    );

    assert_eq!(Charset::UTF_16, codec.charset());
    assert_eq!(Utf16::MAX_UNITS_PER_CHAR, codec.max_units_per_char());
    assert_eq!(Utf16::MAX_UNITS_PER_CHAR, codec.max_units_per_char());
}

#[test]
fn test_utf16_u16_codec_encodes_and_decodes_pairs() {
    let codec = Utf16U16Codec;
    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];

    assert_eq!(2, codec.encode_one('😀', &mut output, 0).expect("encode pair"));
    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 2,
        },
        codec.decode_one(&output, 0).expect("decode pair"),
    );
    assert!(matches!(
        codec.decode_one(&[], 0).expect("empty input needs more"),
        DecodeStatus::NeedMore { .. },
    ));
}

#[test]
fn test_utf16_u16_codec_decodes_bmp_and_reports_partial_or_malformed_units() {
    let codec = Utf16U16Codec;

    assert_eq!(
        DecodeStatus::Complete {
            value: 'A',
            consumed: 1,
        },
        codec.decode_one(&['A' as u16], 0).expect("BMP scalar"),
    );
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 2,
            available: 1,
        },
        codec
            .decode_one(&[0xd83d], 0)
            .expect("dangling high surrogate needs more"),
    );

    let error = codec.decode_one(&[], 1).expect_err("index outside slice should fail");
    assert_eq!(CharsetDecodeErrorKind::InvalidInputIndex { input_len: 0 }, error.kind());
    assert_eq!(1, error.index());

    let error = codec
        .decode_one(&[0xd83d, 'A' as u16], 0)
        .expect_err("high surrogate followed by non-low-surrogate should fail");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some('A' as u32)
        },
        error.kind()
    );
    assert_eq!(1, error.index());

    let error = codec
        .decode_one(&[0xde00], 0)
        .expect_err("isolated low surrogate should fail");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0xde00) },
        error.kind()
    );
    assert_eq!(0, error.index());
}

#[test]
fn test_utf16_u16_codec_reports_small_output_buffers() {
    let codec = Utf16U16Codec;
    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];

    assert_eq!(1, codec.encode_one('A', &mut output, 0).expect("BMP"));

    let error = codec
        .encode_one('A', &mut output[..0], 1)
        .expect_err("index outside slice should fail");
    assert!(matches!(error.kind(), CharsetEncodeErrorKind::BufferTooSmall { .. },));
    assert_eq!(1, error.index());

    let error = codec
        .encode_one('😀', &mut output[..1], 0)
        .expect_err("surrogate pair needs two units");
    assert!(matches!(error.kind(), CharsetEncodeErrorKind::BufferTooSmall { .. },));
    assert_eq!(0, error.index());
}
