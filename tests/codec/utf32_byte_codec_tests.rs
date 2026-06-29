use qubit_codec::{
    ByteOrder,
    Codec,
};
use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeResult,
    Utf32,
    Utf32ByteCodec,
};

type DecodedCharResult = Result<
    (char, core::num::NonZeroUsize),
    qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
>;
type DecodeFn =
    unsafe fn(&mut Utf32ByteCodec, &[u8], usize) -> DecodedCharResult;
type EncodeFn = unsafe fn(
    &mut Utf32ByteCodec,
    &char,
    &mut [u8],
    usize,
) -> CharsetEncodeResult<core::num::NonZeroUsize>;

#[test]
fn test_utf32_byte_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf32ByteCodec::new(ByteOrder::BigEndian);

    assert_eq!(
        Charset::UTF_32BE,
        <Utf32ByteCodec as CharsetCodec>::charset(&codec)
    );
    assert_eq!(4, <Utf32ByteCodec as Codec>::MIN_UNITS_PER_VALUE.get(),);
    assert_eq!(
        Utf32::MAX_BYTES_PER_CHAR,
        <Utf32ByteCodec as Codec>::MAX_UNITS_PER_VALUE.get(),
    );
    assert!(codec.can_encode_value(&'A'));
    assert_eq!(4, codec.encode_len(&'A').get());

    assert_eq!(ByteOrder::BigEndian, codec.byte_order());
    assert_eq!(Charset::UTF_32BE, codec.charset());
}

#[test]
fn test_utf32_byte_codec_encodes_and_decodes_bytes() {
    let mut codec = Utf32ByteCodec::new(ByteOrder::BigEndian);
    let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];

    assert_eq!(4, unsafe {
        codec
            .encode(&'A', &mut output, 0)
            .expect("encode UTF-32BE A")
            .get()
    });
    let (decoded, consumed) =
        unsafe { codec.decode(&output, 0) }.expect("decode UTF-32BE A");
    assert_eq!('A', decoded);
    assert_eq!(4, consumed.get());
}

#[test]
fn test_utf32_byte_codec_reports_closed_tail_and_invalid_units() {
    let mut codec = Utf32ByteCodec::new(ByteOrder::LittleEndian);

    let error = unsafe { codec.decode(&[0x00, 0x00, 0x11, 0x00], 0) }
        .expect_err("non-scalar UTF-32 unit should fail");
    let error = super::invalid_source(error);
    assert!(matches!(
        error.kind(),
        CharsetDecodeErrorKind::InvalidCodePoint { .. },
    ));
    assert_eq!(Some(0x0011_0000), error.value());
}

#[test]
fn test_utf32_byte_codec_direct_function_items_cover_trait_methods() {
    let mut codec = Utf32ByteCodec::new(ByteOrder::LittleEndian);
    let new_fn: fn(ByteOrder) -> Utf32ByteCodec = Utf32ByteCodec::new;
    let byte_order: fn(Utf32ByteCodec) -> ByteOrder =
        Utf32ByteCodec::byte_order;
    let inherent_charset: fn(Utf32ByteCodec) -> Charset =
        std::hint::black_box(Utf32ByteCodec::charset);
    let trait_charset: fn(&Utf32ByteCodec) -> Charset =
        std::hint::black_box(<Utf32ByteCodec as CharsetCodec>::charset);
    let min_units = <Utf32ByteCodec as Codec>::MIN_UNITS_PER_VALUE;
    let max_units = <Utf32ByteCodec as Codec>::MAX_UNITS_PER_VALUE;
    let encode_len: fn(&Utf32ByteCodec, &char) -> core::num::NonZeroUsize =
        <Utf32ByteCodec as Codec>::encode_len;
    let decode: DecodeFn = <Utf32ByteCodec as Codec>::decode;
    let encode: EncodeFn = <Utf32ByteCodec as Codec>::encode;

    assert_eq!(
        ByteOrder::BigEndian,
        byte_order(new_fn(ByteOrder::BigEndian))
    );
    assert_eq!(Charset::UTF_32LE, inherent_charset(codec));
    assert_eq!(Charset::UTF_32LE, trait_charset(&codec));
    assert_eq!(4, min_units.get());
    assert_eq!(Utf32::MAX_BYTES_PER_CHAR, max_units.get());
    assert_eq!(4, encode_len(&codec, &'中').get());

    let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];
    assert_eq!(
        4,
        unsafe { encode(&mut codec, &'中', &mut output, 0) }
            .expect("encode bytes")
            .get()
    );
    let (decoded, consumed) =
        unsafe { decode(&mut codec, &output, 0) }.expect("decode bytes");
    assert_eq!(('中', 4), (decoded, consumed.get()));
}
