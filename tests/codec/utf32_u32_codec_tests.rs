use qubit_codec_text::{
    Charset, CharsetCodec, CharsetDecodeErrorKind, CharsetDecodeResult, CharsetEncodeProbe,
    CharsetEncodeResult, Codec, Utf32, Utf32U32Codec,
};

type DecodedCharResult = CharsetDecodeResult<(char, core::num::NonZeroUsize)>;
type DecodeFn = unsafe fn(&mut Utf32U32Codec, &[u32], usize) -> DecodedCharResult;
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
    assert_eq!(1, codec.min_units_per_value().get());
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, codec.max_units_per_value().get());
    assert_eq!(
        1,
        CharsetEncodeProbe::encode_len(&codec, 'A', 0).expect("encode utf32 unit")
    );

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
    let (decoded, consumed) = unsafe { codec.decode(&output, 0) }.expect("decode unit codec");
    assert_eq!('😀', decoded);
    assert_eq!(1, consumed.get());

    let error = unsafe { codec.encode(&'A', &mut [], 0) }.expect_err("UTF-32 needs one unit");
    assert_eq!(Some(1), error.required());
    assert_eq!(Some(0), error.available());

    let error = unsafe { codec.encode(&'A', &mut [], usize::MAX) }
        .expect_err("overflowing output index should fail without panicking");
    assert_eq!(Some(usize::MAX), error.required());
    assert_eq!(Some(0), error.available());
}

#[test]
fn test_utf32_u32_codec_reports_closed_tail_and_invalid_units() {
    let mut codec = Utf32U32Codec;

    let error = unsafe { codec.decode(&[], 0) }.expect_err("empty input is incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 1,
            available: 0,
        },
        error.kind()
    );

    let error = unsafe { codec.decode(&[], 1) }.expect_err("index outside slice should fail");
    assert_eq!(
        CharsetDecodeErrorKind::InvalidInputIndex { input_len: 0 },
        error.kind()
    );
    assert_eq!(1, error.index());

    let error =
        unsafe { codec.decode(&[0x110000], 0) }.expect_err("non-scalar UTF-32 unit should fail");
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
    let trait_charset: fn(&Utf32U32Codec) -> Charset = <Utf32U32Codec as CharsetCodec>::charset;
    let min_units: fn(&Utf32U32Codec) -> core::num::NonZeroUsize =
        <Utf32U32Codec as Codec>::min_units_per_value;
    let max_units: fn(&Utf32U32Codec) -> core::num::NonZeroUsize =
        <Utf32U32Codec as Codec>::max_units_per_value;
    let encode_len: fn(&Utf32U32Codec, char, usize) -> CharsetEncodeResult<usize> =
        <Utf32U32Codec as CharsetEncodeProbe>::encode_len;
    let decode: DecodeFn = <Utf32U32Codec as Codec>::decode;
    let encode: EncodeFn = std::hint::black_box(<Utf32U32Codec as Codec>::encode);

    assert_eq!(Charset::UTF_32, inherent_charset(codec));
    assert_eq!(Charset::UTF_32, trait_charset(&codec));
    assert_eq!(1, min_units(&codec).get());
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, max_units(&codec).get());
    assert_eq!(1, encode_len(&codec, '中', 0).expect("UTF-32 unit length"));

    let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];
    assert_eq!(
        1,
        unsafe { encode(&mut codec, &'中', &mut output, 0) }
            .expect("encode unit")
            .get()
    );
    let (decoded, consumed) = unsafe { decode(&mut codec, &output, 0) }.expect("decode unit");
    assert_eq!(('中', 1), (decoded, consumed.get()));
}
