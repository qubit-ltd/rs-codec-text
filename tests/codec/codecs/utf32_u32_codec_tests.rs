use qubit_text_codec::{
    DecodeStatus,
    TextDecoder,
    TextEncoder,
    TextEncoding,
    Utf32,
    Utf32U32Codec,
};

#[test]
fn test_utf32_u32_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf32U32Codec;

    assert_eq!(TextEncoding::UTF_32, codec.encoding());
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, codec.max_units_per_char());
    assert_eq!(TextEncoding::UTF_32, TextEncoder::<u32>::encoding(&codec));
    assert_eq!(TextEncoding::UTF_32, TextDecoder::<u32>::encoding(&codec));
    assert_eq!(
        Utf32::MAX_UNITS_PER_CHAR,
        TextEncoder::<u32>::max_units_per_char(&codec)
    );
    assert_eq!(
        Utf32::MAX_UNITS_PER_CHAR,
        TextDecoder::<u32>::max_units_per_char(&codec)
    );
}

#[test]
fn test_utf32_u32_codec_encodes_and_decodes_units() {
    let codec = Utf32U32Codec;
    let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];

    assert_eq!(
        1,
        codec
            .encode_char('😀', &mut output)
            .expect("encode unit codec")
    );
    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 1,
        },
        codec.decode_prefix(&output).expect("decode unit codec"),
    );
}
