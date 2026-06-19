use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeResult,
    CharsetEncodeResult,
    Codec,
    Latin1,
    Latin1Codec,
};

type DecodedCharResult = CharsetDecodeResult<(char, core::num::NonZeroUsize)>;
type DecodeFn = unsafe fn(&mut Latin1Codec, &[u8], usize) -> DecodedCharResult;
type EncodeFn = unsafe fn(
    &mut Latin1Codec,
    &char,
    &mut [u8],
    usize,
) -> CharsetEncodeResult<core::num::NonZeroUsize>;

#[test]
fn test_latin1_codec_exposes_identity_and_limits() {
    let codec = Latin1Codec;

    assert_eq!(
        Charset::ISO_8859_1,
        <Latin1Codec as CharsetCodec>::charset(&codec)
    );
    assert_eq!(1, codec.min_units_per_value().get());
    assert_eq!(1, codec.max_units_per_value().get());
    assert!(codec.can_encode_value(&'A'));
    assert!(codec.can_encode_value(&'\u{00ff}'));
    assert!(!codec.can_encode_value(&'\u{0100}'));
    assert_eq!(1, codec.encode_len(&'A').get());

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
        Latin1::code_point_to_char(Latin1::MAX_CODE_POINT)
            .expect("valid Latin-1 max"),
        decoded
    );
    assert_eq!(1, consumed.get());
}

#[test]
fn test_latin1_codec_encodes_latin1_and_reports_encodable_domain() {
    let mut codec = Latin1Codec;
    let mut output = [0_u8; 1];

    assert_eq!(1, unsafe {
        codec
            .encode(&'\u{00ff}', &mut output, 0)
            .expect("max valid latin1")
            .get()
    },);
    assert_eq!(0xff, output[0]);

    assert!(!codec.can_encode_value(&'\u{0100}'));
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
    let can_encode_value: fn(&Latin1Codec, &char) -> bool =
        <Latin1Codec as Codec>::can_encode_value;
    let encode_len: fn(&Latin1Codec, &char) -> core::num::NonZeroUsize =
        <Latin1Codec as Codec>::encode_len;
    let decode: DecodeFn = <Latin1Codec as Codec>::decode;
    let encode: EncodeFn = <Latin1Codec as Codec>::encode;

    assert_eq!(Charset::ISO_8859_1, inherent_charset(codec));
    assert_eq!(Charset::ISO_8859_1, trait_charset(&codec));
    assert_eq!(1, min_units(&codec).get());
    assert_eq!(1, max_units(&codec).get());
    assert!(can_encode_value(&codec, &'\u{00ff}'));
    assert_eq!(1, encode_len(&codec, &'\u{00ff}').get());

    let (decoded, consumed) =
        unsafe { decode(&mut codec, &[0xff], 0) }.expect("decode Latin-1");
    assert_eq!(('\u{00ff}', 1), (decoded, consumed.get()));
    let mut output = [0_u8; 1];
    assert_eq!(
        1,
        unsafe { encode(&mut codec, &'\u{00ff}', &mut output, 0) }
            .expect("encode Latin-1")
            .get()
    );
    assert_eq!(0xff, output[0]);
}
