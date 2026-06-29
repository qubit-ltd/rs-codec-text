use qubit_codec::Codec;
use qubit_codec_text::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeResult,
    Utf8,
    Utf8Codec,
};

type DecodedCharResult = Result<
    (char, core::num::NonZeroUsize),
    qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
>;
type DecodeFn = unsafe fn(&mut Utf8Codec, &[u8], usize) -> DecodedCharResult;
type EncodeFn = unsafe fn(
    &mut Utf8Codec,
    &char,
    &mut [u8],
    usize,
) -> CharsetEncodeResult<core::num::NonZeroUsize>;

#[test]
fn test_utf8_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf8Codec;

    assert_eq!(Charset::UTF_8, <Utf8Codec as CharsetCodec>::charset(&codec));
    assert_eq!(1, <Utf8Codec as Codec>::MIN_UNITS_PER_VALUE.get());
    assert_eq!(
        Utf8::MAX_UNITS_PER_CHAR,
        <Utf8Codec as Codec>::MAX_UNITS_PER_VALUE.get(),
    );
    assert!(codec.can_encode_value(&'A'));
    assert_eq!(1, codec.encode_len(&'A').get());

    assert_eq!(Charset::UTF_8, codec.charset());
}

#[test]
fn test_utf8_codec_encodes_and_decodes() {
    let mut codec = Utf8Codec;
    let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

    assert_eq!(2, unsafe {
        codec.encode(&'é', &mut output, 0).expect("Latin-1").get()
    });
    let (decoded, consumed) =
        unsafe { codec.decode(&output[..2], 0) }.expect("decode Latin-1");
    assert_eq!('é', decoded);
    assert_eq!(2, consumed.get());
}

#[test]
fn test_utf8_codec_decodes_all_lengths_and_reports_closed_tail() {
    let mut codec = Utf8Codec;

    let (decoded, consumed) = unsafe { codec.decode(b"A", 0) }.expect("ASCII");
    assert_eq!('A', decoded);
    assert_eq!(1, consumed.get());
    let (decoded, consumed) =
        unsafe { codec.decode("中".as_bytes(), 0) }.expect("three bytes");
    assert_eq!('中', decoded);
    assert_eq!(3, consumed.get());
    let (decoded, consumed) =
        unsafe { codec.decode("😀".as_bytes(), 0) }.expect("four bytes");
    assert_eq!('😀', decoded);
    assert_eq!(4, consumed.get());

    let error = unsafe { codec.decode(&[0xe4], 0) }
        .expect_err("partial three-byte prefix");
    assert_eq!(3, super::incomplete_required(error));

    let error = unsafe { codec.decode(&[0xf0, 0x90], 0) }
        .expect_err("partial four-byte prefix");
    assert_eq!(4, super::incomplete_required(error));
}

#[test]
fn test_utf8_codec_reports_malformed_sequences() {
    let mut codec = Utf8Codec;

    let cases = [
        (&[0x80][..], 0, Some(0x80)),
        (&[0xc2, b' '][..], 1, Some(b' ' as u32)),
        (&[0xe0, 0x80, 0x80][..], 1, Some(0x80)),
        (&[0xed, 0xa0, 0x80][..], 1, Some(0xa0)),
        (&[0xe1, 0x80, b' '][..], 2, Some(0x20)),
        (&[0xf0, 0x80, 0x80, 0x80][..], 1, Some(0x80)),
        (&[0xf1, 0x80, b' ', 0x80][..], 2, Some(0x20)),
        (&[0xf4, 0xc0, 0x80, 0x80][..], 1, Some(0xc0)),
        (&[0xf4, 0x80, 0x80, b' '][..], 3, Some(0x20)),
        (&[0xe4, b' '][..], 1, Some(0x20)),
        (&[0xe4, 0xb8, b' '][..], 2, Some(0x20)),
        (&[0xf0, 0x90, b' '][..], 2, Some(0x20)),
    ];

    for (input, index, value) in cases {
        let error = unsafe { codec.decode(input, 0) }
            .expect_err("malformed UTF-8 should fail");
        let error = super::invalid_source(error);
        assert_eq!(
            CharsetDecodeErrorKind::MalformedSequence { value },
            error.kind()
        );
        assert_eq!(index, error.index());
    }
}

#[test]
fn test_utf8_codec_encodes_all_lengths() {
    let mut codec = Utf8Codec;
    let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

    assert_eq!(1, unsafe {
        codec.encode(&'A', &mut output, 0).expect("ASCII").get()
    });
    assert_eq!(2, unsafe {
        codec.encode(&'é', &mut output, 0).expect("two bytes").get()
    });
    assert_eq!(3, unsafe {
        codec
            .encode(&'中', &mut output, 0)
            .expect("three bytes")
            .get()
    });
    assert_eq!(4, unsafe {
        codec
            .encode(&'😀', &mut output, 0)
            .expect("four bytes")
            .get()
    });

    assert!(codec.can_encode_value(&'😀'));
}

#[test]
fn test_utf8_codec_direct_function_items_cover_trait_methods() {
    let mut codec = Utf8Codec;
    let inherent_charset: fn(Utf8Codec) -> Charset = Utf8Codec::charset;
    let trait_charset: fn(&Utf8Codec) -> Charset =
        <Utf8Codec as CharsetCodec>::charset;
    let min_units = <Utf8Codec as Codec>::MIN_UNITS_PER_VALUE;
    let max_units = <Utf8Codec as Codec>::MAX_UNITS_PER_VALUE;
    let encode_len: fn(&Utf8Codec, &char) -> core::num::NonZeroUsize =
        <Utf8Codec as Codec>::encode_len;
    let decode: DecodeFn = <Utf8Codec as Codec>::decode;
    let encode: EncodeFn = std::hint::black_box(<Utf8Codec as Codec>::encode);

    assert_eq!(Charset::UTF_8, inherent_charset(codec));
    assert_eq!(Charset::UTF_8, trait_charset(&codec));
    assert_eq!(1, min_units.get());
    assert_eq!(Utf8::MAX_UNITS_PER_CHAR, max_units.get());
    assert_eq!(4, encode_len(&codec, &'😀').get());

    let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
    assert_eq!(
        4,
        unsafe { encode(&mut codec, &'😀', &mut output, 0) }
            .expect("encode UTF-8")
            .get()
    );
    let (decoded, consumed) =
        unsafe { decode(&mut codec, &output, 0) }.expect("decode UTF-8");
    assert_eq!(('😀', 4), (decoded, consumed.get()));
}
