use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeProbe,
    CharsetEncodeResult,
    Codec,
    Utf16,
    Utf16U16Codec,
};

type DecodedCharResult = CharsetDecodeResult<(char, core::num::NonZeroUsize)>;
type DecodeFn =
    unsafe fn(&mut Utf16U16Codec, &[u16], usize) -> DecodedCharResult;
type EncodeFn = unsafe fn(
    &mut Utf16U16Codec,
    &char,
    &mut [u16],
    usize,
) -> CharsetEncodeResult<usize>;

#[test]
fn test_utf16_u16_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf16U16Codec;

    assert_eq!(
        Charset::UTF_16,
        <Utf16U16Codec as CharsetCodec>::charset(&codec)
    );
    assert_eq!(1, codec.min_units_per_value().get());
    assert_eq!(Utf16::MAX_UNITS_PER_CHAR, codec.max_units_per_value().get());
    assert_eq!(1, codec.encode_len('A', 0).expect("encode UTF-16 BMP"));

    assert_eq!(Charset::UTF_16, codec.charset());
}

#[test]
fn test_utf16_u16_codec_encodes_and_decodes_pairs() {
    let mut codec = Utf16U16Codec;
    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];

    assert_eq!(2, unsafe {
        codec.encode(&'😀', &mut output, 0).expect("encode pair")
    });
    let (decoded, consumed) =
        unsafe { codec.decode(&output, 0) }.expect("decode pair");
    assert_eq!('😀', decoded);
    assert_eq!(2, consumed.get());
}

#[test]
fn test_utf16_u16_codec_decodes_bmp_and_reports_closed_tail_or_malformed_units()
{
    let mut codec = Utf16U16Codec;

    let (decoded, consumed) =
        unsafe { codec.decode(&['A' as u16], 0) }.expect("BMP scalar");
    assert_eq!('A', decoded);
    assert_eq!(1, consumed.get());

    let error = unsafe { codec.decode(&[0xd83d], 0) }
        .expect_err("dangling high surrogate is incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 2,
            available: 1,
        },
        error.kind()
    );

    let error = unsafe { codec.decode(&[], 1) }
        .expect_err("index outside slice should fail");
    assert_eq!(
        CharsetDecodeErrorKind::InvalidInputIndex { input_len: 0 },
        error.kind()
    );
    assert_eq!(1, error.index());

    let error = unsafe { codec.decode(&[], 0) }
        .expect_err("empty closed input should be incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 1,
            available: 0,
        },
        error.kind()
    );
    assert_eq!(0, error.index());

    let error = unsafe { codec.decode(&[0xd83d, 'A' as u16], 0) }
        .expect_err("high surrogate followed by non-low-surrogate should fail");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some('A' as u32)
        },
        error.kind()
    );
    assert_eq!(1, error.index());

    let error = unsafe { codec.decode(&[0xde00], 0) }
        .expect_err("isolated low surrogate should fail");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some(0xde00)
        },
        error.kind()
    );
    assert_eq!(0, error.index());
}

#[test]
fn test_utf16_u16_codec_encodes_bmp_and_supplementary_scalars() {
    let mut codec = Utf16U16Codec;
    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];

    assert_eq!(1, unsafe {
        codec.encode(&'A', &mut output, 0).expect("BMP")
    });
    assert_eq!(2, unsafe {
        codec.encode(&'😀', &mut output, 0).expect("surrogate pair")
    });

    let error = unsafe { codec.encode(&'😀', &mut output[..1], 0) }
        .expect_err("surrogate pair needs two units");
    assert_eq!(Some(2), error.required());
    assert_eq!(Some(1), error.available());

    let error = unsafe { codec.encode(&'A', &mut [], 1) }
        .expect_err("output index outside slice");
    assert_eq!(Some(2), error.required());
    assert_eq!(Some(0), error.available());

    let error = unsafe { codec.encode(&'A', &mut [], usize::MAX) }
        .expect_err("overflowing output index should fail without panicking");
    assert_eq!(Some(usize::MAX), error.required());
    assert_eq!(Some(0), error.available());
}

#[test]
fn test_utf16_u16_codec_direct_function_items_cover_trait_methods() {
    let mut codec = Utf16U16Codec;
    let inherent_charset: fn(Utf16U16Codec) -> Charset = Utf16U16Codec::charset;
    let trait_charset: fn(&Utf16U16Codec) -> Charset =
        <Utf16U16Codec as CharsetCodec>::charset;
    let min_units: fn(&Utf16U16Codec) -> core::num::NonZeroUsize =
        <Utf16U16Codec as Codec>::min_units_per_value;
    let max_units: fn(&Utf16U16Codec) -> core::num::NonZeroUsize =
        <Utf16U16Codec as Codec>::max_units_per_value;
    let encode_len: fn(
        &Utf16U16Codec,
        char,
        usize,
    ) -> CharsetEncodeResult<usize> =
        <Utf16U16Codec as CharsetEncodeProbe>::encode_len;
    let decode: DecodeFn = <Utf16U16Codec as Codec>::decode;
    let encode: EncodeFn = <Utf16U16Codec as Codec>::encode;

    assert_eq!(Charset::UTF_16, inherent_charset(codec));
    assert_eq!(Charset::UTF_16, trait_charset(&codec));
    assert_eq!(1, min_units(&codec).get());
    assert_eq!(Utf16::MAX_UNITS_PER_CHAR, max_units(&codec).get());
    assert_eq!(2, encode_len(&codec, '😀', 0).expect("UTF-16 pair length"));

    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];
    assert_eq!(
        2,
        unsafe { encode(&mut codec, &'😀', &mut output, 0) }
            .expect("encode pair")
    );
    let (decoded, consumed) =
        unsafe { decode(&mut codec, &output, 0) }.expect("decode pair");
    assert_eq!(('😀', 2), (decoded, consumed.get()));
}
