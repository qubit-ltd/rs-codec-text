use qubit_codec_text::{
    ByteOrder,
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeProbe,
    Codec,
    Utf16,
    Utf16ByteCodec,
};

#[test]
fn test_utf16_byte_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);

    assert_eq!(Charset::UTF_16LE, <Utf16ByteCodec as CharsetCodec>::charset(&codec));
    assert_eq!(2, codec.min_units_per_value());
    assert_eq!(Utf16::MAX_BYTES_PER_CHAR, codec.max_units_per_value());
    assert_eq!(2, codec.encode_len('A', 0).expect("encode UTF-16 bytes"));

    assert_eq!(ByteOrder::LittleEndian, codec.byte_order());
    assert_eq!(Charset::UTF_16LE, codec.charset());
}

#[test]
fn test_utf16_byte_codec_encodes_and_decodes_bytes() {
    let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];

    assert_eq!(4, unsafe {
        codec
            .encode_unchecked(&'😀', &mut output, 0)
            .expect("encode pair bytes")
    });
    assert_eq!(
        ('😀', 4),
        unsafe { codec.decode_unchecked(&output, 0) }.expect("decode pair bytes"),
    );
}

#[test]
fn test_utf16_byte_codec_decodes_bmp_and_reports_closed_tail_or_malformed_bytes() {
    let codec = Utf16ByteCodec::new(ByteOrder::BigEndian);

    assert_eq!(
        ('A', 2),
        unsafe { codec.decode_unchecked(&[0x00, 0x41], 0) }.expect("BMP bytes"),
    );

    let error = unsafe { codec.decode_unchecked(&[0x00], 0) }.expect_err("partial unit is incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 2,
            available: 1,
        },
        error.kind()
    );

    let error = unsafe { codec.decode_unchecked(&[0xd8, 0x3d], 0) }.expect_err("partial surrogate pair is incomplete");
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

    let error = unsafe { codec.decode_unchecked(&[0xd8, 0x3d, 0x00, 0x41], 0) }
        .expect_err("high surrogate followed by BMP unit should fail");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0x0041) },
        error.kind()
    );
    assert_eq!(2, error.index());

    let error = unsafe { codec.decode_unchecked(&[0xde, 0x00], 0) }.expect_err("isolated low surrogate should fail");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0xde00) },
        error.kind()
    );
    assert_eq!(0, error.index());
}

#[test]
fn test_utf16_byte_codec_encodes_bmp_and_supplementary_scalars() {
    let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];

    assert_eq!(2, unsafe {
        codec.encode_unchecked(&'A', &mut output, 0).expect("BMP byte encoding")
    });
    assert_eq!(4, unsafe {
        codec
            .encode_unchecked(&'😀', &mut output, 0)
            .expect("surrogate pair bytes")
    });

    let error =
        unsafe { codec.encode_unchecked(&'😀', &mut output[..2], 0) }.expect_err("surrogate pair needs four bytes");
    assert_eq!(Some(4), error.required());
    assert_eq!(Some(2), error.available());

    let error = unsafe { codec.encode_unchecked(&'A', &mut [], 1) }.expect_err("output index outside slice");
    assert_eq!(Some(3), error.required());
    assert_eq!(Some(0), error.available());
}
