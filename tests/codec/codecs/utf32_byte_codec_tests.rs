use qubit_text_codec::{
    ByteOrder,
    DecodeStatus,
    TextDecoder,
    TextEncoder,
    TextEncoding,
    Utf32,
    Utf32ByteCodec,
};

#[test]
fn test_utf32_byte_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf32ByteCodec::new(ByteOrder::BigEndian);

    assert_eq!(ByteOrder::BigEndian, codec.byte_order());
    assert_eq!(TextEncoding::UTF_32, TextEncoder::<u8>::encoding(&codec));
    assert_eq!(TextEncoding::UTF_32, TextDecoder::<u8>::encoding(&codec));
    assert_eq!(
        Utf32::MAX_BYTES_PER_CHAR,
        TextEncoder::<u8>::max_units_per_char(&codec)
    );
    assert_eq!(
        Utf32::MAX_BYTES_PER_CHAR,
        TextDecoder::<u8>::max_units_per_char(&codec)
    );
}

#[test]
fn test_utf32_byte_codec_encodes_and_decodes_bytes() {
    let codec = Utf32ByteCodec::new(ByteOrder::BigEndian);
    let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];

    assert_eq!(
        4,
        codec
            .encode_char('A', &mut output)
            .expect("encode UTF-32BE A")
    );
    assert_eq!(
        DecodeStatus::Complete {
            value: 'A',
            consumed: 4,
        },
        codec.decode_prefix(&output).expect("decode UTF-32BE A"),
    );
}
