use qubit_codec_text::{
    ByteOrder,
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeProbe,
    Codec,
    Utf32,
    Utf32ByteCodec,
};

#[test]
fn test_utf32_byte_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf32ByteCodec::new(ByteOrder::BigEndian);

    assert_eq!(Charset::UTF_32BE, <Utf32ByteCodec as CharsetCodec>::charset(&codec));
    assert_eq!(4, codec.min_units_per_value());
    assert_eq!(Utf32::MAX_BYTES_PER_CHAR, codec.max_units_per_value());
    assert_eq!(4, codec.encode_len('A', 0).expect("encode utf32 unit bytes"));

    assert_eq!(ByteOrder::BigEndian, codec.byte_order());
    assert_eq!(Charset::UTF_32BE, codec.charset());
}

#[test]
fn test_utf32_byte_codec_encodes_and_decodes_bytes() {
    let codec = Utf32ByteCodec::new(ByteOrder::BigEndian);
    let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];

    assert_eq!(4, unsafe {
        codec.encode_unchecked(&'A', &mut output, 0).expect("encode UTF-32BE A")
    });
    assert_eq!(
        ('A', 4),
        unsafe { codec.decode_unchecked(&output, 0) }.expect("decode UTF-32BE A"),
    );

    let error = unsafe { codec.encode_unchecked(&'A', &mut output[..2], 0) }.expect_err("UTF-32 bytes need four bytes");
    assert_eq!(Some(4), error.required());
    assert_eq!(Some(2), error.available());

    let error = unsafe { codec.encode_unchecked(&'A', &mut [], 1) }.expect_err("output index outside slice");
    assert_eq!(Some(5), error.required());
    assert_eq!(Some(0), error.available());
}

#[test]
fn test_utf32_byte_codec_reports_closed_tail_and_invalid_units() {
    let codec = Utf32ByteCodec::new(ByteOrder::LittleEndian);

    let error = unsafe { codec.decode_unchecked(&[0x41, 0x00], 0) }.expect_err("partial UTF-32 bytes are incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 4,
            available: 2,
        },
        error.kind()
    );

    let error = unsafe { codec.decode_unchecked(&[], 1) }.expect_err("index outside slice should fail");
    assert_eq!(CharsetDecodeErrorKind::InvalidInputIndex { input_len: 0 }, error.kind());
    assert_eq!(1, error.index());

    let error = unsafe { codec.decode_unchecked(&[0x00, 0x00, 0x11, 0x00], 0) }
        .expect_err("non-scalar UTF-32 unit should fail");
    assert!(matches!(error.kind(), CharsetDecodeErrorKind::InvalidCodePoint { .. },));
    assert_eq!(Some(0x0011_0000), error.value());
}
