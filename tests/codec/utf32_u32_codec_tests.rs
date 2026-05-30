use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeProbe,
    Codec,
    Utf32,
    Utf32U32Codec,
};

#[test]
fn test_utf32_u32_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf32U32Codec;

    assert_eq!(Charset::UTF_32, <Utf32U32Codec as CharsetCodec>::charset(&codec));
    assert_eq!(1, codec.min_units_per_value().get());
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, codec.max_units_per_value().get());
    assert_eq!(1, codec.encode_len('A', 0).expect("encode utf32 unit"));

    assert_eq!(Charset::UTF_32, codec.charset());
}

#[test]
fn test_utf32_u32_codec_encodes_and_decodes_units() {
    let codec = Utf32U32Codec;
    let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];

    assert_eq!(1, unsafe {
        codec
            .encode_unchecked(&'😀', &mut output, 0)
            .expect("encode unit codec")
    });
    let (decoded, consumed) = unsafe { codec.decode_unchecked(&output, 0) }.expect("decode unit codec");
    assert_eq!('😀', decoded);
    assert_eq!(1, consumed.get());

    let error = unsafe { codec.encode_unchecked(&'A', &mut [], 0) }.expect_err("UTF-32 needs one unit");
    assert_eq!(Some(1), error.required());
    assert_eq!(Some(0), error.available());
}

#[test]
fn test_utf32_u32_codec_reports_closed_tail_and_invalid_units() {
    let codec = Utf32U32Codec;

    let error = unsafe { codec.decode_unchecked(&[], 0) }.expect_err("empty input is incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 1,
            available: 0,
        },
        error.kind()
    );

    let error = unsafe { codec.decode_unchecked(&[], 1) }.expect_err("index outside slice should fail");
    assert_eq!(CharsetDecodeErrorKind::InvalidInputIndex { input_len: 0 }, error.kind());
    assert_eq!(1, error.index());

    let error = unsafe { codec.decode_unchecked(&[0x110000], 0) }.expect_err("non-scalar UTF-32 unit should fail");
    assert!(matches!(error.kind(), CharsetDecodeErrorKind::InvalidCodePoint { .. },));
    assert_eq!(Some(0x110000), error.value());
}
