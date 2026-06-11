use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeErrorKind,
    CharsetEncodeProbe,
    CharsetEncodeResult,
    Codec,
    Latin1Codec,
    Unicode,
};

type DecodedCharResult = CharsetDecodeResult<(char, core::num::NonZeroUsize)>;
type DecodeFn = unsafe fn(&mut Latin1Codec, &[u8], usize) -> DecodedCharResult;
type EncodeFn = unsafe fn(
    &mut Latin1Codec,
    &char,
    &mut [u8],
    usize,
) -> CharsetEncodeResult<usize>;

#[test]
fn test_latin1_codec_exposes_identity_and_limits() {
    let codec = Latin1Codec;

    assert_eq!(
        Charset::ISO_8859_1,
        <Latin1Codec as CharsetCodec>::charset(&codec)
    );
    assert_eq!(1, codec.min_units_per_value().get());
    assert_eq!(1, codec.max_units_per_value().get());
    assert_eq!(
        1,
        codec.encode_len('A', 0).expect("Latin-1 ASCII is mappable")
    );

    assert_eq!(Charset::ISO_8859_1, codec.charset());
    assert_eq!(Charset::ISO_8859_1, codec.charset());
}

#[test]
fn test_latin1_codec_decodes_all_byte_values() {
    let mut codec = Latin1Codec;
    let input = [0u8, 0x7f, 0xff];

    let (decoded, consumed) =
        unsafe { codec.decode(&input, 0) }.expect("decode zero");
    assert_eq!('\u{0000}', decoded);
    assert_eq!(1, consumed.get());
    let (decoded, consumed) =
        unsafe { codec.decode(&input, 1) }.expect("decode DEL");
    assert_eq!('\u{007f}', decoded);
    assert_eq!(1, consumed.get());
    let (decoded, consumed) =
        unsafe { codec.decode(&input, 2) }.expect("decode 0xFF");
    assert_eq!(
        Unicode::to_char(Unicode::LATIN1_MAX).expect("valid Latin-1 max"),
        decoded
    );
    assert_eq!(1, consumed.get());

    let error = unsafe { codec.decode(&[], 0) }
        .expect_err("empty closed input is incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 1,
            available: 0,
        },
        error.kind(),
    );
}

#[test]
fn test_latin1_codec_reports_errors_for_invalid_indices_and_unmappable_characters()
 {
    let mut codec = Latin1Codec;
    let mut output = [0_u8; 1];

    let error = unsafe { codec.decode(&[0x41], 2) }
        .expect_err("index out of range is invalid");
    assert_eq!(
        CharsetDecodeErrorKind::InvalidInputIndex { input_len: 1 },
        error.kind()
    );

    assert_eq!(1, unsafe {
        codec
            .encode(&'\u{00ff}', &mut output, 0)
            .expect("max valid latin1")
    },);
    assert_eq!(0xff, output[0]);

    let error = codec
        .encode_len('\u{0100}', 0)
        .expect_err("above Latin-1 is unmappable");
    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::UnmappableCharacter { .. },
    ));
    assert_eq!(Some('\u{0100}' as u32), error.value());

    let error = unsafe { codec.encode(&'A', &mut [], 0) }
        .expect_err("empty output should fail");
    assert_eq!(
        CharsetEncodeErrorKind::BufferTooSmall {
            required: 1,
            available: 0,
        },
        error.kind()
    );

    let error = unsafe { codec.encode(&'A', &mut [], usize::MAX) }
        .expect_err("overflowing output index should fail without panicking");
    assert_eq!(
        CharsetEncodeErrorKind::BufferTooSmall {
            required: usize::MAX,
            available: 0,
        },
        error.kind()
    );
}

#[test]
fn test_latin1_codec_direct_function_items_cover_trait_methods() {
    let mut codec = Latin1Codec;
    let inherent_charset: fn(Latin1Codec) -> Charset = Latin1Codec::charset;
    let trait_charset: fn(&Latin1Codec) -> Charset =
        <Latin1Codec as CharsetCodec>::charset;
    let min_units: fn(&Latin1Codec) -> core::num::NonZeroUsize =
        <Latin1Codec as Codec>::min_units_per_value;
    let max_units: fn(&Latin1Codec) -> core::num::NonZeroUsize =
        <Latin1Codec as Codec>::max_units_per_value;
    let encode_len: fn(
        &Latin1Codec,
        char,
        usize,
    ) -> CharsetEncodeResult<usize> =
        <Latin1Codec as CharsetEncodeProbe>::encode_len;
    let decode: DecodeFn = <Latin1Codec as Codec>::decode;
    let encode: EncodeFn = <Latin1Codec as Codec>::encode;

    assert_eq!(Charset::ISO_8859_1, inherent_charset(codec));
    assert_eq!(Charset::ISO_8859_1, trait_charset(&codec));
    assert_eq!(1, min_units(&codec).get());
    assert_eq!(1, max_units(&codec).get());
    assert_eq!(
        1,
        encode_len(&codec, '\u{00ff}', 0).expect("Latin-1 is mappable")
    );

    let (decoded, consumed) =
        unsafe { decode(&mut codec, &[0xff], 0) }.expect("decode Latin-1");
    assert_eq!(('\u{00ff}', 1), (decoded, consumed.get()));
    let mut output = [0_u8; 1];
    assert_eq!(
        1,
        unsafe { encode(&mut codec, &'\u{00ff}', &mut output, 0) }
            .expect("encode Latin-1")
    );
    assert_eq!(0xff, output[0]);
}
