use qubit_codec_text::prelude::{
    Ascii,
    BufferedConverter,
    BufferedDecoder,
    BufferedEncoder,
    ByteOrder,
    Charset,
    CharsetCodec,
    CharsetDecoder,
    CharsetEncoder,
    Codec,
    CodecBufferedEncoder,
    CodecValueEncoder,
    DecodeErrorInfo,
    DecodeFailure,
    TranscodeStatus,
    Transcoder,
    Unicode,
    UnicodeBom,
    Utf8,
    Utf8Codec,
    Utf16,
    Utf16ByteCodec,
    Utf32,
    Utf32ByteCodec,
};

#[test]
fn test_prelude_reexports_common_types() {
    fn _accept_buffered_encoder<T: BufferedEncoder<char, u8>>() {}
    fn _accept_buffered_decoder<T: BufferedDecoder<u8, char>>() {}
    fn _accept_buffered_converter<T: BufferedConverter<u8, u16>>() {}
    fn _accept_codec_value_encoder<T: qubit_codec::ValueEncoder<char, Output = Vec<u8>>>() {}
    fn _accept_codec_buffered_encoder<T: BufferedEncoder<char, u8>>() {}

    assert!(Ascii::is_ascii_char('A'));
    _accept_codec_value_encoder::<CodecValueEncoder<Utf8Codec, char, u8>>();
    _accept_codec_buffered_encoder::<CodecBufferedEncoder<Utf8Codec>>();
    assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xe4));
    assert_eq!(2, Utf16::unit_len('😀'));
    assert!(Utf32::is_valid_unit('中' as u32));
    assert!(Unicode::is_scalar_value('中' as u32));
    assert_eq!(Some(ByteOrder::LittleEndian), Utf16::detect_bom(&[0xff, 0xfe]));
    assert_eq!(Some(UnicodeBom::Utf8), UnicodeBom::detect(&[0xef, 0xbb, 0xbf]));

    let utf8 = Utf8Codec;
    assert_eq!(Charset::UTF_8, utf8.charset());
    assert_eq!(4, utf8.max_units_per_value());
    assert_eq!(('A', 1), unsafe {
        utf8.decode_unchecked("A".as_bytes(), 0).expect("UTF-8 prefix")
    });
    fn _accept_decode_error_info<T: DecodeErrorInfo>() {}
    _accept_decode_error_info::<qubit_codec_text::CharsetDecodeError>();
    assert_eq!(Some(1), DecodeFailure::Invalid { consumed: 1 }.invalid_consumed());
    let mut decoder = CharsetDecoder::new(utf8);
    let mut chars = ['\0'; 1];
    let progress = decoder
        .transcode("A".as_bytes(), 0, &mut chars, 0)
        .expect("policy decoder");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!('A', chars[0]);

    let utf16 = Utf16ByteCodec::new(ByteOrder::BigEndian);
    assert_eq!(Charset::UTF_16BE, utf16.charset());

    let utf32 = Utf32ByteCodec::new(ByteOrder::LittleEndian);
    let mut encoder = CharsetEncoder::new(utf32);
    let mut output = [0_u8; 4];
    let progress = encoder.transcode(&['A'], 0, &mut output, 0).expect("policy encoder");
    assert_eq!(Charset::UTF_32LE, encoder.codec().charset());
    assert_eq!(TranscodeStatus::Complete, progress.status());
}
