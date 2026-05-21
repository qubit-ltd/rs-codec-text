use qubit_text_codec::{
    DecodeStatus,
    TextDecoder,
    TextEncoder,
    TextEncoding,
    Utf16,
    Utf16U16Codec,
};

#[test]
fn test_utf16_u16_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf16U16Codec;

    assert_eq!(TextEncoding::UTF_16, TextEncoder::<u16>::encoding(&codec));
    assert_eq!(TextEncoding::UTF_16, TextDecoder::<u16>::encoding(&codec));
    assert_eq!(
        Utf16::MAX_UNITS_PER_CHAR,
        TextEncoder::<u16>::max_units_per_char(&codec)
    );
    assert_eq!(
        Utf16::MAX_UNITS_PER_CHAR,
        TextDecoder::<u16>::max_units_per_char(&codec)
    );
}

#[test]
fn test_utf16_u16_codec_encodes_and_decodes_pairs() {
    let codec = Utf16U16Codec;
    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];

    assert_eq!(
        2,
        codec.encode_char('😀', &mut output).expect("encode pair")
    );
    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 2,
        },
        codec.decode_prefix(&output).expect("decode pair"),
    );
    assert!(matches!(
        codec.decode_prefix(&[]).expect("empty input needs more"),
        DecodeStatus::NeedMore { .. },
    ));
}
