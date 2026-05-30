use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeProbe,
    Codec,
    Utf8,
    Utf8Codec,
};

#[test]
fn test_utf8_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf8Codec;

    assert_eq!(Charset::UTF_8, <Utf8Codec as CharsetCodec>::charset(&codec));
    assert_eq!(1, codec.min_units_per_value().get());
    assert_eq!(Utf8::MAX_UNITS_PER_CHAR, codec.max_units_per_value().get());
    assert_eq!(1, codec.encode_len('A', 0).expect("encode ascii as utf8"));

    assert_eq!(Charset::UTF_8, codec.charset());
}

#[test]
fn test_utf8_codec_encodes_and_decodes() {
    let codec = Utf8Codec;
    let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

    assert_eq!(2, unsafe {
        codec.encode_unchecked(&'é', &mut output, 0).expect("Latin-1")
    });
    let (decoded, consumed) = unsafe { codec.decode_unchecked(&output[..2], 0) }.expect("decode Latin-1");
    assert_eq!('é', decoded);
    assert_eq!(2, consumed.get());

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
fn test_utf8_codec_decodes_all_lengths_and_reports_closed_tail() {
    let codec = Utf8Codec;

    let (decoded, consumed) = unsafe { codec.decode_unchecked(b"A", 0) }.expect("ASCII");
    assert_eq!('A', decoded);
    assert_eq!(1, consumed.get());
    let (decoded, consumed) = unsafe { codec.decode_unchecked("中".as_bytes(), 0) }.expect("three bytes");
    assert_eq!('中', decoded);
    assert_eq!(3, consumed.get());
    let (decoded, consumed) = unsafe { codec.decode_unchecked("😀".as_bytes(), 0) }.expect("four bytes");
    assert_eq!('😀', decoded);
    assert_eq!(4, consumed.get());

    let error = unsafe { codec.decode_unchecked(&[0xe4], 0) }.expect_err("partial three-byte prefix");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 3,
            available: 1,
        },
        error.kind(),
    );

    let error = unsafe { codec.decode_unchecked(&[0xf0, 0x90], 0) }.expect_err("partial four-byte prefix");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 4,
            available: 2,
        },
        error.kind(),
    );
}

#[test]
fn test_utf8_codec_reports_malformed_sequences() {
    let codec = Utf8Codec;

    let cases = [
        (&[0x80][..], 0, Some(0x80)),
        (&[0xc2, b' '][..], 1, Some(b' ' as u32)),
        (&[0xe0, 0x80, 0x80][..], 1, Some(0x80)),
        (&[0xed, 0xa0, 0x80][..], 1, Some(0xa0)),
        (&[0xe1, 0x80, b' '][..], 2, Some(0x20)),
        (&[0xf0, 0x80, 0x80, 0x80][..], 1, Some(0x80)),
        (&[0xf1, 0x80, b' ', 0x80][..], 2, Some(0x20)),
        (&[0xf4, 0xc0, 0x80, 0x80][..], 1, Some(0xc0)),
        (&[0xf4, 0x80, 0x80, b' '][..], 3, Some(0x20)),
        (&[0xe4, b' '][..], 1, Some(0x20)),
        (&[0xe4, 0xb8, b' '][..], 2, Some(0x20)),
        (&[0xf0, 0x90, b' '][..], 2, Some(0x20)),
    ];

    for (input, index, value) in cases {
        let error = unsafe { codec.decode_unchecked(input, 0) }.expect_err("malformed UTF-8 should fail");
        assert_eq!(CharsetDecodeErrorKind::MalformedSequence { value }, error.kind());
        assert_eq!(index, error.index());
    }

    let error = unsafe { codec.decode_unchecked(b"", 1) }.expect_err("input index outside slice should fail");
    assert_eq!(CharsetDecodeErrorKind::InvalidInputIndex { input_len: 0 }, error.kind());
    assert_eq!(1, error.index());
}

#[test]
fn test_utf8_codec_encodes_all_lengths() {
    let codec = Utf8Codec;
    let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

    assert_eq!(1, unsafe {
        codec.encode_unchecked(&'A', &mut output, 0).expect("ASCII")
    });
    assert_eq!(2, unsafe {
        codec.encode_unchecked(&'é', &mut output, 0).expect("two bytes")
    });
    assert_eq!(3, unsafe {
        codec.encode_unchecked(&'中', &mut output, 0).expect("three bytes")
    });
    assert_eq!(4, unsafe {
        codec.encode_unchecked(&'😀', &mut output, 0).expect("four bytes")
    });

    let error = unsafe { codec.encode_unchecked(&'é', &mut output[..1], 0) }.expect_err("short output should fail");
    assert_eq!(Some(2), error.required());
    assert_eq!(Some(1), error.available());

    let error = unsafe { codec.encode_unchecked(&'A', &mut [], 1) }.expect_err("output index outside slice");
    assert_eq!(Some(2), error.required());
    assert_eq!(Some(0), error.available());
}
