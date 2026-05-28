use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeProbe,
    Codec,
    Utf16,
    Utf16U16Codec,
};

#[test]
fn test_utf16_u16_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf16U16Codec;

    assert_eq!(Charset::UTF_16, <Utf16U16Codec as CharsetCodec>::charset(&codec));
    assert_eq!(1, codec.min_units_per_value());
    assert_eq!(Utf16::MAX_UNITS_PER_CHAR, codec.max_units_per_value());
    assert_eq!(1, codec.encode_len('A', 0).expect("encode UTF-16 BMP"));

    assert_eq!(Charset::UTF_16, codec.charset());
}

#[test]
fn test_utf16_u16_codec_encodes_and_decodes_pairs() {
    let codec = Utf16U16Codec;
    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];

    assert_eq!(2, unsafe {
        codec.encode_unchecked(&'😀', &mut output, 0).expect("encode pair")
    });
    assert_eq!(
        ('😀', 2),
        unsafe { codec.decode_unchecked(&output, 0) }.expect("decode pair"),
    );
}

#[test]
fn test_utf16_u16_codec_decodes_bmp_and_reports_closed_tail_or_malformed_units() {
    let codec = Utf16U16Codec;

    assert_eq!(
        ('A', 1),
        unsafe { codec.decode_unchecked(&['A' as u16], 0) }.expect("BMP scalar"),
    );

    let error = unsafe { codec.decode_unchecked(&[0xd83d], 0) }.expect_err("dangling high surrogate is incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 2,
            available: 1,
        },
        error.kind()
    );

    let error = unsafe { codec.decode_unchecked(&[], 1) }.expect_err("index outside slice should fail");
    assert_eq!(CharsetDecodeErrorKind::InvalidInputIndex { input_len: 0 }, error.kind());
    assert_eq!(1, error.index());

    let error = unsafe { codec.decode_unchecked(&[0xd83d, 'A' as u16], 0) }
        .expect_err("high surrogate followed by non-low-surrogate should fail");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some('A' as u32)
        },
        error.kind()
    );
    assert_eq!(1, error.index());

    let error = unsafe { codec.decode_unchecked(&[0xde00], 0) }.expect_err("isolated low surrogate should fail");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0xde00) },
        error.kind()
    );
    assert_eq!(0, error.index());
}

#[test]
fn test_utf16_u16_codec_encodes_bmp_and_supplementary_scalars() {
    let codec = Utf16U16Codec;
    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];

    assert_eq!(1, unsafe { codec.encode_unchecked(&'A', &mut output, 0).expect("BMP") });
    assert_eq!(2, unsafe {
        codec.encode_unchecked(&'😀', &mut output, 0).expect("surrogate pair")
    });

    let error =
        unsafe { codec.encode_unchecked(&'😀', &mut output[..1], 0) }.expect_err("surrogate pair needs two units");
    assert_eq!(Some(2), error.required());
    assert_eq!(Some(1), error.available());

    let error = unsafe { codec.encode_unchecked(&'A', &mut [], 1) }.expect_err("output index outside slice");
    assert_eq!(Some(2), error.required());
    assert_eq!(Some(0), error.available());
}
