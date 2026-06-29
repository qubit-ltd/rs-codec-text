use qubit_codec::Codec;
use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeResult,
    Utf32,
    Utf32U32Codec,
};

type DecodedCharResult = Result<
    (char, core::num::NonZeroUsize),
    qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
>;
type DecodeFn =
    unsafe fn(&mut Utf32U32Codec, &[u32], usize) -> DecodedCharResult;
type EncodeFn = unsafe fn(
    &mut Utf32U32Codec,
    &char,
    &mut [u32],
    usize,
) -> CharsetEncodeResult<core::num::NonZeroUsize>;

#[test]
fn test_utf32_u32_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf32U32Codec;

    assert_eq!(
        Charset::UTF_32,
        <Utf32U32Codec as CharsetCodec>::charset(&codec)
    );
    assert_eq!(1, <Utf32U32Codec as Codec>::MIN_UNITS_PER_VALUE.get());
    assert_eq!(
        Utf32::MAX_UNITS_PER_CHAR,
        <Utf32U32Codec as Codec>::MAX_UNITS_PER_VALUE.get(),
    );
    assert!(codec.can_encode_value(&'A'));
    assert_eq!(1, codec.encode_len(&'A').get());

    assert_eq!(Charset::UTF_32, codec.charset());
}

#[test]
fn test_utf32_u32_codec_encodes_and_decodes_units() {
    let mut codec = Utf32U32Codec;
    let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];

    assert_eq!(1, unsafe {
        codec
            .encode(&'😀', &mut output, 0)
            .expect("encode unit codec")
            .get()
    });
    let (decoded, consumed) =
        unsafe { codec.decode(&output, 0) }.expect("decode unit codec");
    assert_eq!('😀', decoded);
    assert_eq!(1, consumed.get());
}

#[test]
fn test_utf32_u32_codec_reports_closed_tail_and_invalid_units() {
    let mut codec = Utf32U32Codec;

    let error = unsafe { codec.decode(&[0x110000], 0) }
        .expect_err("non-scalar UTF-32 unit should fail");
    let error = super::invalid_source(error);
    assert!(matches!(
        error.kind(),
        CharsetDecodeErrorKind::InvalidCodePoint { .. },
    ));
    assert_eq!(Some(0x110000), error.value());
}

#[test]
fn test_utf32_u32_codec_direct_function_items_cover_trait_methods() {
    let mut codec = Utf32U32Codec;
    let inherent_charset: fn(Utf32U32Codec) -> Charset = Utf32U32Codec::charset;
    let trait_charset: fn(&Utf32U32Codec) -> Charset =
        <Utf32U32Codec as CharsetCodec>::charset;
    let min_units = <Utf32U32Codec as Codec>::MIN_UNITS_PER_VALUE;
    let max_units = <Utf32U32Codec as Codec>::MAX_UNITS_PER_VALUE;
    let encode_len: fn(&Utf32U32Codec, &char) -> core::num::NonZeroUsize =
        <Utf32U32Codec as Codec>::encode_len;
    let decode: DecodeFn = <Utf32U32Codec as Codec>::decode;
    let encode: EncodeFn =
        std::hint::black_box(<Utf32U32Codec as Codec>::encode);

    assert_eq!(Charset::UTF_32, inherent_charset(codec));
    assert_eq!(Charset::UTF_32, trait_charset(&codec));
    assert_eq!(1, min_units.get());
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, max_units.get());
    assert_eq!(1, encode_len(&codec, &'中').get());

    let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];
    assert_eq!(
        1,
        unsafe { encode(&mut codec, &'中', &mut output, 0) }
            .expect("encode unit")
            .get()
    );
    let (decoded, consumed) =
        unsafe { decode(&mut codec, &output, 0) }.expect("decode unit");
    assert_eq!(('中', 1), (decoded, consumed.get()));
}
