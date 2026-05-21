use qubit_unicode::{
    ByteOrder,
    DecodeResult,
    TextDecoder,
    TextDecodingErrorKind,
    TextEncoder,
    TextEncodingErrorKind,
    Utf16,
    Utf16ByteDecoder,
    Utf16ByteEncoder,
    Utf16U16Decoder,
    Utf16U16Encoder,
};

#[test]
fn test_utf16_classifies_units_and_surrogate_pairs() {
    assert!(Utf16::is_single_unit('A' as u16));
    assert!(!Utf16::is_single_unit(0xd83d));
    assert!(Utf16::is_high_surrogate(0xd83d));
    assert!(Utf16::is_low_surrogate(0xde00));
    assert!(Utf16::is_surrogate(0xd83d));
    assert!(Utf16::is_surrogate_pair(0xd83d, 0xde00));
    assert_eq!(Some(0x1f600), Utf16::compose_pair(0xd83d, 0xde00));
    assert_eq!(Some(0xd83d), Utf16::high_surrogate(0x1f600));
    assert_eq!(Some(0xde00), Utf16::low_surrogate(0x1f600));
    assert_eq!(1, Utf16::unit_len('A'));
    assert_eq!(2, Utf16::unit_len('😀'));
    assert_eq!(Some(1), Utf16::unit_len_code_point('中' as u32));
    assert_eq!(Some(2), Utf16::unit_len_code_point(0x1f600));
    assert_eq!(None, Utf16::unit_len_code_point(0xd800));
    assert_eq!(
        Some(ByteOrder::LittleEndian),
        Utf16::detect_bom(&[0xff, 0xfe])
    );
}

#[test]
fn test_utf16_u16_codec_decodes_and_encodes_units() {
    let decoder = Utf16U16Decoder;
    let encoder = Utf16U16Encoder;
    let units = [0x0041, 0x4e2d, 0xd83d, 0xde00];
    let mut index = 0;

    assert_eq!(
        Some('A'),
        decoder.decode_next(&units, &mut index).expect("ASCII")
    );
    assert_eq!(
        Some('中'),
        decoder.decode_next(&units, &mut index).expect("BMP")
    );
    assert_eq!(
        Some('😀'),
        decoder.decode_next(&units, &mut index).expect("pair")
    );
    assert_eq!(None, decoder.decode_next(&units, &mut index).expect("EOF"));

    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];
    let written = encoder
        .encode_char('😀', &mut output)
        .expect("encode emoji");
    assert_eq!(2, written);
    assert_eq!([0xd83d, 0xde00], output);
}

#[test]
fn test_utf16_u16_decoder_reports_need_more_and_malformed_pairs() {
    let decoder = Utf16U16Decoder;

    assert_eq!(
        DecodeResult::NeedMore(qubit_unicode::NeedMore::new(2, 1)),
        decoder
            .decode_prefix(&[0xd83d])
            .expect("high surrogate needs low surrogate"),
    );

    let error = decoder
        .decode_prefix(&[0xde00])
        .expect_err("low surrogate cannot start a scalar");
    assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
    assert_eq!(0, error.index());

    let error = decoder
        .decode_prefix(&[0xd83d, 0x0041])
        .expect_err("bad surrogate pair must fail");
    assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());
}

#[test]
fn test_utf16_byte_codec_uses_byte_order() {
    let decoder = Utf16ByteDecoder::new(ByteOrder::LittleEndian);
    let encoder = Utf16ByteEncoder::new(ByteOrder::LittleEndian);
    let bytes = [0x3d, 0xd8, 0x00, 0xde];

    assert_eq!(
        DecodeResult::Complete(qubit_unicode::Decoded::new('😀', 4)),
        decoder
            .decode_prefix(&bytes)
            .expect("decode UTF-16LE emoji"),
    );

    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];
    let written = encoder
        .encode_char('😀', &mut output)
        .expect("encode UTF-16LE emoji");
    assert_eq!(4, written);
    assert_eq!(bytes, output);

    let mut small = [0_u8; 2];
    let error = encoder
        .encode_char('😀', &mut small)
        .expect_err("small byte buffer must fail");
    assert_eq!(TextEncodingErrorKind::BufferTooSmall, error.kind());
}
