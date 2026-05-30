use qubit_codec_text::{
    AsciiCodec,
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeErrorKind,
    CharsetEncodeProbe,
    Codec,
};

#[test]
fn test_ascii_codec_exposes_identity_and_limits() {
    let codec = AsciiCodec;

    assert_eq!(Charset::ASCII, <AsciiCodec as CharsetCodec>::charset(&codec));
    assert_eq!(1, codec.min_units_per_value().get());
    assert_eq!(1, codec.max_units_per_value().get());
    assert_eq!(1, codec.encode_len('A', 0).expect("ASCII is mappable"));

    assert_eq!(Charset::ASCII, codec.charset());
    assert_eq!(Charset::ASCII, codec.charset());
}

#[test]
fn test_ascii_codec_decodes_ascii_bytes_and_reports_closed_tail_and_malformed() {
    let codec = AsciiCodec;

    let (decoded, consumed) = unsafe { codec.decode_unchecked(b"A", 0) }.expect("ASCII decode");
    assert_eq!('A', decoded);
    assert_eq!(1, consumed.get());

    let error = unsafe { codec.decode_unchecked(&[], 0) }.expect_err("empty closed input is incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 1,
            available: 0,
        },
        error.kind()
    );

    let error = unsafe { codec.decode_unchecked(&[0x80], 0) }.expect_err("non-ASCII byte is malformed");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(128) },
        error.kind()
    );
    assert_eq!(0, error.index());

    let error = unsafe { codec.decode_unchecked(&[0x41], 2) }.expect_err("index out of range is invalid");
    assert_eq!(CharsetDecodeErrorKind::InvalidInputIndex { input_len: 1 }, error.kind());
    assert_eq!(2, error.index());
}

#[test]
fn test_ascii_codec_encodes_ascii_and_reports_limits_and_unmappable_chars() {
    let codec = AsciiCodec;
    let mut output = [0_u8; 2];

    assert_eq!(1, unsafe {
        codec.encode_unchecked(&'A', &mut output, 0).expect("encode ASCII")
    });
    assert_eq!(b'A', output[0]);

    let error = codec.encode_len('é', 0).expect_err("non-ASCII is unmappable");
    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::UnmappableCharacter { value: _ },
    ));
    assert_eq!(Some('é' as u32), error.value());
}
