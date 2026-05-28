use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeErrorKind,
    CharsetEncodeProbe,
    Codec,
    Latin1Codec,
    Unicode,
};

#[test]
fn test_latin1_codec_exposes_identity_and_limits() {
    let codec = Latin1Codec;

    assert_eq!(Charset::ISO_8859_1, <Latin1Codec as CharsetCodec>::charset(&codec));
    assert_eq!(1, codec.min_units_per_value());
    assert_eq!(1, codec.max_units_per_value());
    assert_eq!(1, codec.encode_len('A', 0).expect("Latin-1 ASCII is mappable"));

    assert_eq!(Charset::ISO_8859_1, codec.charset());
    assert_eq!(Charset::ISO_8859_1, codec.charset());
}

#[test]
fn test_latin1_codec_decodes_all_byte_values() {
    let codec = Latin1Codec;
    let input = [0u8, 0x7f, 0xff];

    assert_eq!(
        ('\u{0000}', 1),
        unsafe { codec.decode_unchecked(&input, 0) }.expect("decode zero"),
    );
    assert_eq!(
        ('\u{007f}', 1),
        unsafe { codec.decode_unchecked(&input, 1) }.expect("decode DEL"),
    );
    assert_eq!(
        (Unicode::to_char(Unicode::LATIN1_MAX).expect("valid Latin-1 max"), 1),
        unsafe { codec.decode_unchecked(&input, 2) }.expect("decode 0xFF"),
    );

    let error = unsafe { codec.decode_unchecked(&[], 0) }.expect_err("empty closed input is incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 1,
            available: 0,
        },
        error.kind(),
    );
}

#[test]
fn test_latin1_codec_reports_errors_for_invalid_indices_and_unmappable_characters() {
    let codec = Latin1Codec;
    let mut output = [0_u8; 1];

    let error = unsafe { codec.decode_unchecked(&[0x41], 2) }.expect_err("index out of range is invalid");
    assert_eq!(CharsetDecodeErrorKind::InvalidInputIndex { input_len: 1 }, error.kind());

    assert_eq!(1, unsafe {
        codec
            .encode_unchecked(&'\u{00ff}', &mut output, 0)
            .expect("max valid latin1")
    },);
    assert_eq!(0xff, output[0]);

    let error = codec
        .encode_len('\u{0100}', 0)
        .expect_err("above Latin-1 is unmappable");
    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::UnmappableCharacter { .. },
    ));
    assert_eq!(Some('\u{0100}' as u32), error.value());
}
