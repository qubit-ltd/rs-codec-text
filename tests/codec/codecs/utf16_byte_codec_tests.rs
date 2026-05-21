use qubit_text_codec::{
    ByteOrder,
    DecodeStatus,
    TextDecoder,
    TextEncoder,
    TextEncoding,
    Utf16,
    Utf16ByteCodec,
};

#[test]
fn test_utf16_byte_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);

    assert_eq!(ByteOrder::LittleEndian, codec.byte_order());
    assert_eq!(TextEncoding::UTF_16, codec.encoding());
    assert_eq!(Utf16::MAX_BYTES_PER_CHAR, codec.max_units_per_char());
    assert_eq!(TextEncoding::UTF_16, TextEncoder::<u8>::encoding(&codec));
    assert_eq!(TextEncoding::UTF_16, TextDecoder::<u8>::encoding(&codec));
    assert_eq!(
        Utf16::MAX_BYTES_PER_CHAR,
        TextEncoder::<u8>::max_units_per_char(&codec)
    );
    assert_eq!(
        Utf16::MAX_BYTES_PER_CHAR,
        TextDecoder::<u8>::max_units_per_char(&codec)
    );
}

#[test]
fn test_utf16_byte_codec_encodes_and_decodes_bytes() {
    let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];

    assert_eq!(
        4,
        codec
            .encode_char('😀', &mut output)
            .expect("encode pair bytes")
    );
    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 4,
        },
        codec.decode_prefix(&output).expect("decode pair bytes"),
    );
}
