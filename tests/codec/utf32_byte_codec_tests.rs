use qubit_text_codec::{
    ByteOrder,
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeErrorKind,
    DecodeStatus,
    Utf32,
    Utf32ByteCodec,
};

#[test]
fn test_utf32_byte_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf32ByteCodec::new(ByteOrder::BigEndian);

    assert_eq!(Charset::UTF_32BE, <Utf32ByteCodec as CharsetCodec>::charset(&codec));
    assert_eq!(
        Utf32::MAX_BYTES_PER_CHAR,
        <Utf32ByteCodec as CharsetCodec>::max_units_per_char(&codec)
    );
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 4,
            available: 0,
        },
        <Utf32ByteCodec as CharsetCodec>::decode_one(&codec, &[], 0).expect("utf32 bytes need more"),
    );
    assert_eq!(
        4,
        <Utf32ByteCodec as CharsetCodec>::encode_one(&codec, 'A', &mut [0_u8; 4], 0).expect("encode utf32 unit bytes"),
    );

    assert_eq!(ByteOrder::BigEndian, codec.byte_order());
    assert_eq!(Charset::UTF_32BE, codec.charset());
    assert_eq!(Utf32::MAX_BYTES_PER_CHAR, codec.max_units_per_char());
    assert_eq!(Utf32::MAX_BYTES_PER_CHAR, codec.max_units_per_char());
}

#[test]
fn test_utf32_byte_codec_encodes_and_decodes_bytes() {
    let codec = Utf32ByteCodec::new(ByteOrder::BigEndian);
    let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];

    assert_eq!(4, codec.encode_one('A', &mut output, 0).expect("encode UTF-32BE A"));
    assert_eq!(
        DecodeStatus::Complete {
            value: 'A',
            consumed: 4,
        },
        codec.decode_one(&output, 0).expect("decode UTF-32BE A"),
    );
}

#[test]
fn test_utf32_byte_codec_reports_partial_invalid_and_small_buffers() {
    let codec = Utf32ByteCodec::new(ByteOrder::LittleEndian);
    let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];

    assert_eq!(
        DecodeStatus::NeedMore {
            required: 4,
            available: 2,
        },
        codec
            .decode_one(&[0x41, 0x00], 0)
            .expect("partial UTF-32 bytes need more input"),
    );

    let error = codec.decode_one(&[], 1).expect_err("index outside slice should fail");
    assert_eq!(CharsetDecodeErrorKind::InvalidInputIndex { input_len: 0 }, error.kind());
    assert_eq!(1, error.index());

    let error = codec
        .decode_one(&[0x00, 0x00, 0x11, 0x00], 0)
        .expect_err("non-scalar UTF-32 unit should fail");
    assert!(matches!(error.kind(), CharsetDecodeErrorKind::InvalidCodePoint { .. },));
    assert_eq!(Some(0x0011_0000), error.value());

    let error = codec
        .encode_one('A', &mut output[..0], 1)
        .expect_err("index outside slice should fail");
    assert!(matches!(error.kind(), CharsetEncodeErrorKind::BufferTooSmall { .. },));
    assert_eq!(1, error.index());

    let error = codec
        .encode_one('A', &mut output[..3], 0)
        .expect_err("UTF-32 byte encoding needs four bytes");
    assert!(matches!(error.kind(), CharsetEncodeErrorKind::BufferTooSmall { .. },));
    assert_eq!(0, error.index());
}
