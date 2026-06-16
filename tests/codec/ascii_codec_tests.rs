use qubit_codec_text::{
    AsciiCodec,
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeResult,
    Codec,
};

type DecodedCharResult = CharsetDecodeResult<(char, core::num::NonZeroUsize)>;
type DecodeFn = unsafe fn(&mut AsciiCodec, &[u8], usize) -> DecodedCharResult;
type EncodeFn = unsafe fn(
    &mut AsciiCodec,
    &char,
    &mut [u8],
    usize,
) -> CharsetEncodeResult<core::num::NonZeroUsize>;

#[test]
fn test_ascii_codec_exposes_identity_and_limits() {
    let codec = AsciiCodec;

    assert_eq!(
        Charset::ASCII,
        <AsciiCodec as CharsetCodec>::charset(&codec)
    );
    assert_eq!(1, codec.min_units_per_value().get());
    assert_eq!(1, codec.max_units_per_value().get());
    assert!(codec.can_encode_value(&'A'));
    assert!(!codec.can_encode_value(&'é'));
    assert_eq!(1, codec.encode_len(&'A').get());

    assert_eq!(Charset::ASCII, codec.charset());
    assert_eq!(Charset::ASCII, codec.charset());
}

#[test]
fn test_ascii_codec_decodes_ascii_bytes_and_reports_malformed() {
    let mut codec = AsciiCodec;

    let (decoded, consumed) =
        unsafe { codec.decode(b"A", 0) }.expect("ASCII decode");
    assert_eq!('A', decoded);
    assert_eq!(1, consumed.get());

    let error = unsafe { codec.decode(&[0x80], 0) }
        .expect_err("non-ASCII byte is malformed");
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(128) },
        error.kind()
    );
    assert_eq!(0, error.index());
}

#[test]
fn test_ascii_codec_encodes_ascii_and_reports_encodable_domain() {
    let mut codec = AsciiCodec;
    let mut output = [0_u8; 2];

    assert_eq!(1, unsafe {
        codec
            .encode(&'A', &mut output, 0)
            .expect("encode ASCII")
            .get()
    });
    assert_eq!(b'A', output[0]);

    assert!(!codec.can_encode_value(&'é'));
}

#[test]
fn test_ascii_codec_direct_function_items_cover_trait_methods() {
    let mut codec = AsciiCodec;
    let inherent_charset: fn(AsciiCodec) -> Charset = AsciiCodec::charset;
    let trait_charset: fn(&AsciiCodec) -> Charset =
        <AsciiCodec as CharsetCodec>::charset;
    let min_units: fn(&AsciiCodec) -> core::num::NonZeroUsize =
        <AsciiCodec as Codec>::min_units_per_value;
    let max_units: fn(&AsciiCodec) -> core::num::NonZeroUsize =
        <AsciiCodec as Codec>::max_units_per_value;
    let can_encode_value: fn(&AsciiCodec, &char) -> bool =
        <AsciiCodec as Codec>::can_encode_value;
    let encode_len: fn(&AsciiCodec, &char) -> core::num::NonZeroUsize =
        <AsciiCodec as Codec>::encode_len;
    let decode: DecodeFn = <AsciiCodec as Codec>::decode;
    let encode: EncodeFn = <AsciiCodec as Codec>::encode;

    assert_eq!(Charset::ASCII, inherent_charset(codec));
    assert_eq!(Charset::ASCII, trait_charset(&codec));
    assert_eq!(1, min_units(&codec).get());
    assert_eq!(1, max_units(&codec).get());
    assert!(can_encode_value(&codec, &'Z'));
    assert_eq!(1, encode_len(&codec, &'Z').get());

    let (decoded, consumed) =
        unsafe { decode(&mut codec, b"Z", 0) }.expect("decode ASCII");
    assert_eq!(('Z', 1), (decoded, consumed.get()));

    let mut output = [0_u8; 1];
    assert_eq!(
        1,
        unsafe { encode(&mut codec, &'Z', &mut output, 0) }
            .expect("encode ASCII")
            .get()
    );
    assert_eq!(b'Z', output[0]);
    assert!(!can_encode_value(&codec, &'é'));
}
