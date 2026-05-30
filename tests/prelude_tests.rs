use qubit_codec_text::prelude::{
    Ascii,
    BufferedConvertEngine,
    BufferedConvertHooks,
    BufferedConverter,
    BufferedDecoder,
    BufferedEncoder,
    ByteOrder,
    Charset,
    CharsetCodec,
    CharsetDecoder,
    CharsetEncodePlan,
    CharsetEncoder,
    Codec,
    CodecBufferedDecoder,
    CodecBufferedEncoder,
    CodecDecodeError,
    CodecEncodeError,
    CodecValueEncoder,
    ConvertErrorFactory,
    ConvertState,
    DecodeErrorFactory,
    DecodeErrorInfo,
    DecodeFailure,
    EncodeErrorFactory,
    EncodePlan,
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
    fn _accept_codec_buffered_decoder<T: BufferedDecoder<u8, char>>() {}
    fn _accept_codec_buffered_encoder<T: BufferedEncoder<char, u8>>() {}
    fn _accept_buffered_decode_engine<T>() {}
    fn _accept_buffered_encode_engine<T>() {}
    fn _accept_buffered_convert_engine<T>() {}
    fn _accept_buffered_convert_hooks<
        T: BufferedConvertHooks<CharsetDecoder<Utf8Codec>, CharsetEncoder<Utf8Codec>, u8, char, u8>,
    >() {
    }

    assert!(Ascii::is_ascii_char('A'));
    _accept_codec_value_encoder::<CodecValueEncoder<Utf8Codec, char, u8>>();
    _accept_codec_buffered_decoder::<CodecBufferedDecoder<Utf8Codec, u8>>();
    _accept_codec_buffered_encoder::<CodecBufferedEncoder<Utf8Codec>>();
    assert_eq!(Some(3), Utf8::byte_len_from_leading_byte(0xe4));
    assert_eq!(2, Utf16::unit_len('😀'));
    assert!(Utf32::is_valid_unit('中' as u32));
    assert!(Unicode::is_scalar_value('中' as u32));
    assert_eq!(Some(ByteOrder::LittleEndian), Utf16::detect_bom(&[0xff, 0xfe]));
    assert_eq!(Some(UnicodeBom::Utf8), UnicodeBom::detect(&[0xef, 0xbb, 0xbf]));

    let utf8 = Utf8Codec;
    _accept_buffered_decode_engine::<qubit_codec_text::BufferedDecodeEngine<Utf8Codec, (), u8>>();
    _accept_buffered_encode_engine::<qubit_codec_text::BufferedEncodeEngine<Utf8Codec, ()>>();
    _accept_buffered_convert_engine::<
        BufferedConvertEngine<CharsetDecoder<Utf8Codec>, CharsetEncoder<Utf8Codec>, (), u8>,
    >();
    let mut convert_output = [0_u8; 1];
    let convert_state = ConvertState::new(&[1_u8], 0, &mut convert_output, 0);
    assert_eq!(1, convert_state.available_input());
    let plan = EncodePlan::new(4, CharsetEncodePlan::Original);
    assert_eq!(4, plan.max_output_units);
    assert_eq!(CharsetEncodePlan::Original, plan.payload);
    let encode_error =
        <CodecEncodeError<core::convert::Infallible> as EncodeErrorFactory<Utf8Codec>>::invalid_input_index(
            &utf8, 2, 1,
        );
    assert!(matches!(encode_error, CodecEncodeError::InvalidInputIndex { .. }));
    let decode_error =
        <CodecDecodeError<core::convert::Infallible> as DecodeErrorFactory<Utf8Codec>>::invalid_input_index(
            &utf8, 2, 1,
        );
    assert!(matches!(decode_error, CodecDecodeError::InvalidInputIndex { .. }));
    let convert_error =
        <qubit_codec_text::CharsetConvertError as ConvertErrorFactory<CharsetDecoder<Utf8Codec>>>::invalid_input_index(
            &CharsetDecoder::new(utf8),
            2,
            1,
        );
    assert!(matches!(
        convert_error,
        qubit_codec_text::CharsetConvertError::Decode(_)
    ));
    assert_eq!(Charset::UTF_8, utf8.charset());
    assert_eq!(4, utf8.max_units_per_value().get());
    let (decoded, consumed) = unsafe { utf8.decode_unchecked("A".as_bytes(), 0).expect("UTF-8 prefix") };
    assert_eq!('A', decoded);
    assert_eq!(1, consumed.get());
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
