use qubit_text_codec::{
    DecodeStatus,
    TextDecoder,
    TextEncoder,
    TextEncoding,
    Utf8,
    Utf8Codec,
};

#[test]
fn test_utf8_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf8Codec;

    assert_eq!(TextEncoding::UTF_8, TextEncoder::<u8>::encoding(&codec));
    assert_eq!(TextEncoding::UTF_8, TextDecoder::<u8>::encoding(&codec));
    assert_eq!(
        Utf8::MAX_UNITS_PER_CHAR,
        TextEncoder::<u8>::max_units_per_char(&codec)
    );
    assert_eq!(
        Utf8::MAX_UNITS_PER_CHAR,
        TextDecoder::<u8>::max_units_per_char(&codec)
    );
}

#[test]
fn test_utf8_codec_encodes_and_decodes() {
    let codec = Utf8Codec;
    let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

    assert_eq!(2, codec.encode_char('é', &mut output).expect("Latin-1"));
    assert!(matches!(
        codec.decode_prefix(&output[..2]).expect("decode Latin-1"),
        DecodeStatus::Complete {
            value: 'é',
            consumed: 2,
        },
    ));
    assert!(matches!(
        codec.decode_prefix(&[]).expect("empty prefix needs more"),
        DecodeStatus::NeedMore { .. },
    ));
}
