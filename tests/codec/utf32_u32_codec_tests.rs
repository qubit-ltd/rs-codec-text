use qubit_text_codec::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeErrorKind,
    DecodeStatus,
    Utf32,
    Utf32U32Codec,
};

#[test]
fn test_utf32_u32_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf32U32Codec;

    assert_eq!(Charset::UTF_32, <Utf32U32Codec as CharsetCodec>::charset(&codec));
    assert_eq!(
        Utf32::MAX_UNITS_PER_CHAR,
        <Utf32U32Codec as CharsetCodec>::max_units_per_char(&codec)
    );
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 1,
            available: 0,
        },
        <Utf32U32Codec as CharsetCodec>::decode_one(&codec, &[], 0).expect("utf32 need more"),
    );
    assert_eq!(
        1,
        <Utf32U32Codec as CharsetCodec>::encode_one(&codec, 'A', &mut [0_u32; 1], 0).expect("encode utf32 unit"),
    );

    assert_eq!(Charset::UTF_32, codec.charset());
    assert_eq!(Charset::UTF_32, codec.charset());
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, codec.max_units_per_char());
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, codec.max_units_per_char());
}

#[test]
fn test_utf32_u32_codec_encodes_and_decodes_units() {
    let codec = Utf32U32Codec;
    let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];

    assert_eq!(1, codec.encode_one('😀', &mut output, 0).expect("encode unit codec"));
    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 1,
        },
        codec.decode_one(&output, 0).expect("decode unit codec"),
    );
}

#[test]
fn test_utf32_u32_codec_reports_partial_invalid_and_small_buffers() {
    let codec = Utf32U32Codec;
    let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];

    assert_eq!(
        DecodeStatus::NeedMore {
            required: 1,
            available: 0,
        },
        codec.decode_one(&[], 0).expect("empty input needs more"),
    );

    let error = codec.decode_one(&[], 1).expect_err("index outside slice should fail");
    assert_eq!(CharsetDecodeErrorKind::MalformedSequence { value: None }, error.kind());
    assert_eq!(1, error.index());

    let error = codec
        .decode_one(&[0x110000], 0)
        .expect_err("non-scalar UTF-32 unit should fail");
    assert!(matches!(error.kind(), CharsetDecodeErrorKind::InvalidCodePoint { .. },));
    assert_eq!(Some(0x110000), error.value());

    let error = codec
        .encode_one('A', &mut output[..0], 0)
        .expect_err("empty output should fail");
    assert!(matches!(error.kind(), CharsetEncodeErrorKind::BufferTooSmall { .. },));
    assert_eq!(0, error.index());
}
