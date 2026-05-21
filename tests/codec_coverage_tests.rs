use qubit_unicode::{
    ByteOrder,
    DecodeResult,
    TextDecoder,
    TextDecodingErrorKind,
    TextEncoder,
    TextEncoding,
    TextEncodingErrorKind,
    UnicodeBom,
    Utf8,
    Utf8Codec,
    Utf8Decoder,
    Utf8Encoder,
    Utf16,
    Utf16ByteCodec,
    Utf16ByteDecoder,
    Utf16ByteEncoder,
    Utf16U16Codec,
    Utf16U16Decoder,
    Utf16U16Encoder,
    Utf32,
    Utf32ByteCodec,
    Utf32ByteDecoder,
    Utf32ByteEncoder,
    Utf32U32Codec,
    Utf32U32Decoder,
    Utf32U32Encoder,
};

#[test]
fn test_text_encoding_and_bom_cover_all_variants() {
    assert_eq!("ASCII", TextEncoding::Ascii.name());
    assert_eq!("UTF-8", TextEncoding::Utf8.to_string());
    assert_eq!("UTF-16", TextEncoding::Utf16.name());
    assert_eq!("UTF-32", TextEncoding::Utf32.name());
    assert_eq!("GBK", TextEncoding::Named("GBK").to_string());

    let boms = [
        (
            UnicodeBom::Utf8,
            &[0xef, 0xbb, 0xbf][..],
            TextEncoding::Utf8,
            None,
        ),
        (
            UnicodeBom::Utf16BigEndian,
            &[0xfe, 0xff][..],
            TextEncoding::Utf16,
            Some(ByteOrder::BigEndian),
        ),
        (
            UnicodeBom::Utf16LittleEndian,
            &[0xff, 0xfe][..],
            TextEncoding::Utf16,
            Some(ByteOrder::LittleEndian),
        ),
        (
            UnicodeBom::Utf32BigEndian,
            &[0x00, 0x00, 0xfe, 0xff][..],
            TextEncoding::Utf32,
            Some(ByteOrder::BigEndian),
        ),
        (
            UnicodeBom::Utf32LittleEndian,
            &[0xff, 0xfe, 0x00, 0x00][..],
            TextEncoding::Utf32,
            Some(ByteOrder::LittleEndian),
        ),
    ];

    for (bom, bytes, encoding, byte_order) in boms {
        assert_eq!(bytes, bom.bytes());
        assert_eq!(bytes.len(), bom.byte_len());
        assert_eq!(encoding, bom.encoding());
        assert_eq!(byte_order, bom.byte_order());
        assert_eq!(Some(bom), UnicodeBom::detect(bytes));
    }
    assert_eq!(None, UnicodeBom::detect(&[0, 1, 2, 3]));
    assert_eq!(None, Utf16::detect_bom(&[0xef, 0xbb, 0xbf]));
    assert_eq!(None, Utf32::detect_bom(&[0xfe, 0xff]));
}

#[test]
fn test_byte_order_covers_all_serializers() {
    assert_eq!([0x12, 0x34], ByteOrder::BigEndian.u16_bytes(0x1234));
    assert_eq!([0x34, 0x12], ByteOrder::LittleEndian.u16_bytes(0x1234));
    assert_eq!(
        [0x00, 0x01, 0xf6, 0x00],
        ByteOrder::BigEndian.u32_bytes(0x0001f600),
    );
    assert_eq!(
        [0x00, 0xf6, 0x01, 0x00],
        ByteOrder::LittleEndian.u32_bytes(0x0001f600),
    );
}

#[test]
fn test_utf8_all_codec_types_and_utf8_error_branches() {
    let encoder = Utf8Encoder;
    let decoder = Utf8Decoder;
    let codec = Utf8Codec;
    let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

    assert_eq!(TextEncoding::Utf8, encoder.encoding());
    assert_eq!(Utf8::MAX_UNITS_PER_CHAR, encoder.max_units_per_char());
    assert_eq!(TextEncoding::Utf8, decoder.encoding());
    assert_eq!(Utf8::MAX_UNITS_PER_CHAR, decoder.max_units_per_char());
    assert_eq!(TextEncoding::Utf8, TextEncoder::<u8>::encoding(&codec));
    assert_eq!(TextEncoding::Utf8, TextDecoder::<u8>::encoding(&codec));
    assert_eq!(
        Utf8::MAX_UNITS_PER_CHAR,
        TextEncoder::<u8>::max_units_per_char(&codec)
    );
    assert_eq!(
        Utf8::MAX_UNITS_PER_CHAR,
        TextDecoder::<u8>::max_units_per_char(&codec)
    );

    assert_eq!(1, encoder.encode_char('A', &mut output).expect("ASCII"));
    assert_eq!(2, codec.encode_char('é', &mut output).expect("Latin-1"));
    assert_eq!(Some('A'), decoder.decode_next(b"A", &mut 0).expect("ASCII"));

    for bytes in [
        &[0xc2, 0x80][..],
        &[0xdf, 0xbf],
        &[0xe0, 0xa0, 0x80],
        &[0xed, 0x9f, 0xbf],
        &[0xee, 0x80, 0x80],
        &[0xf0, 0x90, 0x80, 0x80],
        &[0xf1, 0x80, 0x80, 0x80],
        &[0xf4, 0x8f, 0xbf, 0xbf],
    ] {
        assert!(matches!(
            codec.decode_prefix(bytes).expect("well-formed UTF-8"),
            DecodeResult::Complete(_),
        ));
    }

    for (bytes, index) in [
        (&[0x80][..], 0),
        (&[0xc2, 0x20], 1),
        (&[0xe0, 0x9f, 0x80], 1),
        (&[0xed, 0xa0, 0x80], 1),
        (&[0xe1, 0x80, 0x20], 2),
        (&[0xf0, 0x8f, 0xbf, 0xbf], 1),
        (&[0xf4, 0x90, 0x80, 0x80], 1),
        (&[0xf1, 0x80, 0x20, 0x80], 2),
        (&[0xf1, 0x80, 0x80, 0x20], 3),
    ] {
        let error = codec
            .decode_prefix(bytes)
            .expect_err("malformed UTF-8 must fail");
        assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
        assert_eq!(index, error.index());
    }

    assert!(matches!(
        codec.decode_prefix(&[]).expect("empty prefix needs more"),
        DecodeResult::NeedMore(_),
    ));
    assert!(matches!(
        codec
            .decode_prefix(&[0xe4])
            .expect("short prefix needs more"),
        DecodeResult::NeedMore(_),
    ));
    let mut bad_index = 2;
    let error = codec
        .decode_next(b"A", &mut bad_index)
        .expect_err("out-of-bounds cursor must fail");
    assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
}

#[test]
fn test_utf16_all_codec_types_and_error_branches() {
    let u16_encoder = Utf16U16Encoder;
    let u16_decoder = Utf16U16Decoder;
    let u16_codec = Utf16U16Codec;
    let byte_encoder = Utf16ByteEncoder::new(ByteOrder::BigEndian);
    let byte_decoder = Utf16ByteDecoder::new(ByteOrder::LittleEndian);
    let byte_codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
    let mut unit_output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];
    let mut byte_output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];

    assert_eq!(TextEncoding::Utf16, u16_encoder.encoding());
    assert_eq!(Utf16::MAX_UNITS_PER_CHAR, u16_encoder.max_units_per_char());
    assert_eq!(TextEncoding::Utf16, u16_decoder.encoding());
    assert_eq!(Utf16::MAX_UNITS_PER_CHAR, u16_decoder.max_units_per_char());
    assert_eq!(
        TextEncoding::Utf16,
        TextEncoder::<u16>::encoding(&u16_codec)
    );
    assert_eq!(
        TextEncoding::Utf16,
        TextDecoder::<u16>::encoding(&u16_codec)
    );
    assert_eq!(
        Utf16::MAX_UNITS_PER_CHAR,
        TextEncoder::<u16>::max_units_per_char(&u16_codec)
    );
    assert_eq!(
        Utf16::MAX_UNITS_PER_CHAR,
        TextDecoder::<u16>::max_units_per_char(&u16_codec)
    );

    assert_eq!(ByteOrder::BigEndian, byte_encoder.byte_order());
    assert_eq!(ByteOrder::LittleEndian, byte_decoder.byte_order());
    assert_eq!(ByteOrder::LittleEndian, byte_codec.byte_order());
    assert_eq!(TextEncoding::Utf16, byte_encoder.encoding());
    assert_eq!(TextEncoding::Utf16, byte_decoder.encoding());
    assert_eq!(
        TextEncoding::Utf16,
        TextEncoder::<u8>::encoding(&byte_codec)
    );
    assert_eq!(
        TextEncoding::Utf16,
        TextDecoder::<u8>::encoding(&byte_codec)
    );
    assert_eq!(Utf16::MAX_BYTES_PER_CHAR, byte_encoder.max_units_per_char());
    assert_eq!(Utf16::MAX_BYTES_PER_CHAR, byte_decoder.max_units_per_char());
    assert_eq!(
        Utf16::MAX_BYTES_PER_CHAR,
        TextEncoder::<u8>::max_units_per_char(&byte_codec)
    );
    assert_eq!(
        Utf16::MAX_BYTES_PER_CHAR,
        TextDecoder::<u8>::max_units_per_char(&byte_codec)
    );

    assert_eq!(
        1,
        u16_encoder
            .encode_char('中', &mut unit_output)
            .expect("BMP")
    );
    assert_eq!(
        2,
        u16_codec.encode_char('😀', &mut unit_output).expect("pair")
    );
    assert_eq!(
        2,
        byte_encoder
            .encode_char('中', &mut byte_output)
            .expect("BMP bytes")
    );
    assert_eq!(
        4,
        byte_codec
            .encode_char('😀', &mut byte_output)
            .expect("pair bytes")
    );

    assert_eq!(
        DecodeResult::Complete(qubit_unicode::Decoded::new('A', 1)),
        u16_decoder.decode_prefix(&[0x0041]).expect("single unit"),
    );
    assert_eq!(
        DecodeResult::Complete(qubit_unicode::Decoded::new('😀', 2)),
        u16_codec.decode_prefix(&[0xd83d, 0xde00]).expect("pair"),
    );
    assert_eq!(
        DecodeResult::Complete(qubit_unicode::Decoded::new('A', 2)),
        byte_decoder.decode_prefix(&[0x41, 0x00]).expect("LE byte"),
    );
    assert_eq!(
        DecodeResult::Complete(qubit_unicode::Decoded::new('😀', 4)),
        byte_codec
            .decode_prefix(&[0x3d, 0xd8, 0x00, 0xde])
            .expect("LE pair bytes"),
    );

    for units in [&[0xd83d][..], &[]] {
        assert!(matches!(
            u16_codec
                .decode_prefix(units)
                .expect("UTF-16 unit prefix needs more"),
            DecodeResult::NeedMore(_),
        ));
    }
    for bytes in [[][..].as_ref(), &[0x3d][..], &[0x3d, 0xd8, 0x00][..]] {
        assert!(matches!(
            byte_codec
                .decode_prefix(bytes)
                .expect("UTF-16 byte prefix needs more"),
            DecodeResult::NeedMore(_),
        ));
    }

    for units in [&[0xde00][..], &[0xd83d, 0x0041]] {
        let error = u16_codec
            .decode_prefix(units)
            .expect_err("malformed UTF-16 units");
        assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
    }
    for bytes in [&[0x00, 0xde][..], &[0x3d, 0xd8, 0x41, 0x00]] {
        let error = byte_codec
            .decode_prefix(bytes)
            .expect_err("malformed UTF-16 bytes");
        assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
    }
}

#[test]
fn test_utf32_all_codec_types_and_error_branches() {
    let u32_encoder = Utf32U32Encoder;
    let u32_decoder = Utf32U32Decoder;
    let u32_codec = Utf32U32Codec;
    let byte_encoder = Utf32ByteEncoder::new(ByteOrder::LittleEndian);
    let byte_decoder = Utf32ByteDecoder::new(ByteOrder::BigEndian);
    let byte_codec = Utf32ByteCodec::new(ByteOrder::BigEndian);
    let mut unit_output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];
    let mut byte_output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];

    assert_eq!(TextEncoding::Utf32, u32_encoder.encoding());
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, u32_encoder.max_units_per_char());
    assert_eq!(TextEncoding::Utf32, u32_decoder.encoding());
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, u32_decoder.max_units_per_char());
    assert_eq!(
        TextEncoding::Utf32,
        TextEncoder::<u32>::encoding(&u32_codec)
    );
    assert_eq!(
        TextEncoding::Utf32,
        TextDecoder::<u32>::encoding(&u32_codec)
    );
    assert_eq!(
        Utf32::MAX_UNITS_PER_CHAR,
        TextEncoder::<u32>::max_units_per_char(&u32_codec)
    );
    assert_eq!(
        Utf32::MAX_UNITS_PER_CHAR,
        TextDecoder::<u32>::max_units_per_char(&u32_codec)
    );

    assert_eq!(ByteOrder::LittleEndian, byte_encoder.byte_order());
    assert_eq!(ByteOrder::BigEndian, byte_decoder.byte_order());
    assert_eq!(ByteOrder::BigEndian, byte_codec.byte_order());
    assert_eq!(TextEncoding::Utf32, byte_encoder.encoding());
    assert_eq!(TextEncoding::Utf32, byte_decoder.encoding());
    assert_eq!(
        TextEncoding::Utf32,
        TextEncoder::<u8>::encoding(&byte_codec)
    );
    assert_eq!(
        TextEncoding::Utf32,
        TextDecoder::<u8>::encoding(&byte_codec)
    );
    assert_eq!(Utf32::MAX_BYTES_PER_CHAR, byte_encoder.max_units_per_char());
    assert_eq!(Utf32::MAX_BYTES_PER_CHAR, byte_decoder.max_units_per_char());
    assert_eq!(
        Utf32::MAX_BYTES_PER_CHAR,
        TextEncoder::<u8>::max_units_per_char(&byte_codec)
    );
    assert_eq!(
        Utf32::MAX_BYTES_PER_CHAR,
        TextDecoder::<u8>::max_units_per_char(&byte_codec)
    );

    assert_eq!(
        1,
        u32_encoder
            .encode_char('A', &mut unit_output)
            .expect("unit")
    );
    assert_eq!(
        1,
        u32_codec
            .encode_char('😀', &mut unit_output)
            .expect("unit codec")
    );
    assert_eq!(
        4,
        byte_encoder
            .encode_char('A', &mut byte_output)
            .expect("byte")
    );
    assert_eq!(
        4,
        byte_codec
            .encode_char('😀', &mut byte_output)
            .expect("byte codec")
    );

    assert_eq!(
        DecodeResult::Complete(qubit_unicode::Decoded::new('A', 1)),
        u32_decoder.decode_prefix(&['A' as u32]).expect("u32 unit"),
    );
    assert_eq!(
        DecodeResult::Complete(qubit_unicode::Decoded::new('😀', 1)),
        u32_codec
            .decode_prefix(&['😀' as u32])
            .expect("u32 unit codec"),
    );
    assert_eq!(
        DecodeResult::Complete(qubit_unicode::Decoded::new('😀', 4)),
        byte_decoder
            .decode_prefix(&[0x00, 0x01, 0xf6, 0x00])
            .expect("UTF-32BE bytes"),
    );
    assert_eq!(
        DecodeResult::Complete(qubit_unicode::Decoded::new('A', 4)),
        byte_codec
            .decode_prefix(&[0x00, 0x00, 0x00, 0x41])
            .expect("UTF-32BE A"),
    );

    assert!(matches!(
        u32_codec
            .decode_prefix(&[])
            .expect("UTF-32 unit needs more"),
        DecodeResult::NeedMore(_),
    ));
    assert!(matches!(
        byte_codec
            .decode_prefix(&[0, 0, 0])
            .expect("UTF-32 bytes need more"),
        DecodeResult::NeedMore(_),
    ));

    for unit in [0xd800, 0x110000] {
        let error = u32_codec
            .decode_prefix(&[unit])
            .expect_err("invalid UTF-32 unit");
        assert_eq!(TextDecodingErrorKind::InvalidCodePoint, error.kind());
    }
    for bytes in [[0x00, 0x00, 0xd8, 0x00], [0x00, 0x11, 0x00, 0x00]] {
        let error = byte_codec
            .decode_prefix(&bytes)
            .expect_err("invalid UTF-32 bytes");
        assert_eq!(TextDecodingErrorKind::InvalidCodePoint, error.kind());
    }
}

#[test]
fn test_encoding_error_variants_are_constructible() {
    let error = Utf8Encoder
        .encode_code_point(0x110000, &mut [0_u8; 4])
        .expect_err("invalid code point must fail");
    assert_eq!(TextEncodingErrorKind::InvalidCodePoint, error.kind());

    assert_eq!(
        "The character cannot be represented by the target encoding.",
        TextEncodingErrorKind::UnmappableCharacter.to_string(),
    );
}

#[test]
fn test_text_decoder_default_decode_next_covers_all_branches() {
    let decoder = Utf8Decoder;

    let mut index = 0;
    assert_eq!(
        Some('A'),
        decoder.decode_next(b"A", &mut index).expect("complete")
    );
    assert_eq!(1, index);
    assert_eq!(None, decoder.decode_next(b"A", &mut index).expect("EOF"));

    let mut incomplete_index = 0;
    let error = decoder
        .decode_next(&[0xe4], &mut incomplete_index)
        .expect_err("closed incomplete input must fail");
    assert_eq!(TextDecodingErrorKind::IncompleteSequence, error.kind());
    assert_eq!(1, error.index());

    let mut malformed_index = 1;
    let error = decoder
        .decode_next(&[b'A', 0x80], &mut malformed_index)
        .expect_err("malformed input must be offset by the cursor");
    assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());

    let mut out_of_bounds = 2;
    let error = decoder
        .decode_next(b"A", &mut out_of_bounds)
        .expect_err("out-of-bounds input index must fail");
    assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
    assert_eq!(2, error.index());
}

#[test]
fn test_utf16_namespace_covers_none_and_big_endian_branches() {
    assert_eq!(None, Utf16::compose_pair(0xde00, 0xd83d));
    assert_eq!(None, Utf16::high_surrogate('A' as u32));
    assert_eq!(None, Utf16::low_surrogate('A' as u32));
    assert_eq!(None, Utf16::unit_len_code_point(0x110000));
    assert_eq!(Some(ByteOrder::BigEndian), Utf16::detect_bom(&[0xfe, 0xff]));
    assert_eq!(None, Utf16::detect_bom(&[0, 0]));
}

#[test]
fn test_utf32_namespace_covers_all_detect_bom_branches() {
    assert!(Utf32::is_valid_unit(0));
    assert!(!Utf32::is_valid_unit(0xd800));
    assert_eq!(
        Some(ByteOrder::LittleEndian),
        Utf32::detect_bom(&[0xff, 0xfe, 0x00, 0x00])
    );
    assert_eq!(
        Some(ByteOrder::BigEndian),
        Utf32::detect_bom(&[0x00, 0x00, 0xfe, 0xff])
    );
    assert_eq!(None, Utf32::detect_bom(&[0xff, 0xfe]));
}
