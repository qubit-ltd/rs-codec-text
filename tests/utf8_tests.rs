use qubit_unicode::{
    DecodeResult,
    TextDecoder,
    TextDecodingErrorKind,
    TextEncoder,
    TextEncodingErrorKind,
    Utf8,
    Utf8Decoder,
    Utf8Encoder,
};

#[test]
fn test_utf8_classifies_bytes_and_lengths() {
    assert!(Utf8::is_single_byte(b'A'));
    assert!(!Utf8::is_single_byte(0x80));
    assert!(Utf8::is_leading_byte(0xe4));
    assert!(Utf8::is_leading_byte(0xf0));
    assert!(!Utf8::is_leading_byte(0xc1));
    assert!(Utf8::is_continuation_byte(0xb8));
    assert!(!Utf8::is_continuation_byte(b'A'));
    assert_eq!(Some(1), Utf8::byte_len_from_leading_byte(b'A'));
    assert_eq!(Some(2), Utf8::byte_len_from_leading_byte(0xc2));
    assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xe4));
    assert_eq!(Some(4), Utf8::byte_len_from_leading_byte(0xf0));
    assert_eq!(None, Utf8::byte_len_from_leading_byte(0x80));
    assert_eq!(1, Utf8::byte_len('A'));
    assert_eq!(3, Utf8::byte_len('中'));
    assert_eq!(4, Utf8::byte_len('😀'));
}

#[test]
fn test_utf8_decoder_decodes_prefix_and_next() {
    let decoder = Utf8Decoder;
    let bytes = "A中😀".as_bytes();

    assert_eq!(
        DecodeResult::Complete(qubit_unicode::Decoded::new('A', 1)),
        decoder.decode_prefix(bytes).expect("ASCII prefix"),
    );

    let mut index = 0;
    assert_eq!(
        Some('A'),
        decoder.decode_next(bytes, &mut index).expect("A")
    );
    assert_eq!(1, index);
    assert_eq!(
        Some('中'),
        decoder.decode_next(bytes, &mut index).expect("CJK")
    );
    assert_eq!(4, index);
    assert_eq!(
        Some('😀'),
        decoder.decode_next(bytes, &mut index).expect("emoji")
    );
    assert_eq!(8, index);
    assert_eq!(None, decoder.decode_next(bytes, &mut index).expect("EOF"));
}

#[test]
fn test_utf8_decoder_reports_need_more_and_malformed_sequences() {
    let decoder = Utf8Decoder;

    assert_eq!(
        DecodeResult::NeedMore(qubit_unicode::NeedMore::new(3, 2)),
        decoder
            .decode_prefix(&[0xe4, 0xb8])
            .expect("valid prefix needs more"),
    );

    let error = decoder
        .decode_prefix(&[0xe4, b'A', 0x80])
        .expect_err("bad continuation must fail");
    assert_eq!(TextDecodingErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());

    let mut index = 0;
    let error = decoder
        .decode_next(&[0xf0, 0x9f], &mut index)
        .expect_err("closed incomplete input must fail");
    assert_eq!(TextDecodingErrorKind::IncompleteSequence, error.kind());
    assert_eq!(2, error.index());
}

#[test]
fn test_utf8_encoder_encodes_chars_and_reports_small_buffers() {
    let encoder = Utf8Encoder;
    let mut buffer = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

    let written = encoder.encode_char('中', &mut buffer).expect("encode CJK");
    assert_eq!(3, written);
    assert_eq!("中".as_bytes(), &buffer[..written]);

    let written = encoder
        .encode_code_point(0x1f600, &mut buffer)
        .expect("encode emoji");
    assert_eq!(4, written);
    assert_eq!("😀".as_bytes(), &buffer[..written]);

    let mut small = [0_u8; 2];
    let error = encoder
        .encode_char('中', &mut small)
        .expect_err("small buffer must fail");
    assert_eq!(TextEncodingErrorKind::BufferTooSmall, error.kind());

    let error = encoder
        .encode_code_point(0xd800, &mut buffer)
        .expect_err("surrogate is not a scalar value");
    assert_eq!(TextEncodingErrorKind::InvalidCodePoint, error.kind());
}
