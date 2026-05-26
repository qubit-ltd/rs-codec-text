use qubit_codec::{
    ByteOrder,
    Codec,
};
use qubit_codec_text::{
    AsciiCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeErrorKind,
    Latin1Codec,
    Utf8Codec,
    Utf16ByteCodec,
    Utf16U16Codec,
    Utf32ByteCodec,
    Utf32U32Codec,
};

#[test]
fn test_core_codec_trait_reports_text_codec_unit_bounds() {
    assert_eq!(1, AsciiCodec.min_units_per_value());
    assert_eq!(1, AsciiCodec.max_units_per_value());
    assert_eq!(1, Latin1Codec.min_units_per_value());
    assert_eq!(1, Latin1Codec.max_units_per_value());
    assert_eq!(1, Utf8Codec.min_units_per_value());
    assert_eq!(4, Utf8Codec.max_units_per_value());

    let utf16_bytes = Utf16ByteCodec::new(ByteOrder::BigEndian);
    assert_eq!(2, utf16_bytes.min_units_per_value());
    assert_eq!(4, utf16_bytes.max_units_per_value());
    assert_eq!(1, Utf16U16Codec.min_units_per_value());
    assert_eq!(2, Utf16U16Codec.max_units_per_value());

    let utf32_bytes = Utf32ByteCodec::new(ByteOrder::LittleEndian);
    assert_eq!(4, utf32_bytes.min_units_per_value());
    assert_eq!(4, utf32_bytes.max_units_per_value());
    assert_eq!(1, Utf32U32Codec.min_units_per_value());
    assert_eq!(1, Utf32U32Codec.max_units_per_value());
}

fn assert_u8_codec<C>(codec: C, value: char, expected: &[u8])
where
    C: Codec<char, u8>,
    <C as Codec<char, u8>>::DecodeError: core::fmt::Debug,
    <C as Codec<char, u8>>::EncodeError: core::fmt::Debug,
{
    let mut output = [0_u8; 4];
    let written = unsafe {
        codec
            .encode_unchecked(value, &mut output, 0)
            .expect("character should encode")
    };
    assert_eq!(expected, &output[..written]);

    let (decoded, consumed) = unsafe {
        codec
            .decode_unchecked(&output[..written], 0)
            .expect("encoded bytes should decode")
    };
    assert_eq!((value, written), (decoded, consumed));
}

#[test]
fn test_core_codec_trait_is_implemented_for_byte_text_codecs() {
    assert_u8_codec(AsciiCodec, 'A', b"A");
    assert_u8_codec(Latin1Codec, '\u{00a9}', &[0xa9]);
    assert_u8_codec(Utf8Codec, '中', "中".as_bytes());
    assert_u8_codec(
        Utf16ByteCodec::new(ByteOrder::BigEndian),
        '😀',
        &[0xd8, 0x3d, 0xde, 0x00],
    );
    assert_u8_codec(
        Utf32ByteCodec::new(ByteOrder::LittleEndian),
        '中',
        &[0x2d, 0x4e, 0x00, 0x00],
    );
}

#[test]
fn test_core_codec_trait_is_implemented_for_utf16_units() {
    let codec = Utf16U16Codec;
    let mut output = [0_u16; 2];

    let written = unsafe {
        codec
            .encode_unchecked('😀', &mut output, 0)
            .expect("supplementary character should encode")
    };
    assert_eq!(2, written);
    assert_eq!([0xd83d, 0xde00], output);

    let decoded = unsafe {
        codec
            .decode_unchecked(&output, 0)
            .expect("surrogate pair should decode")
    };
    assert_eq!(('😀', 2), decoded);
}

#[test]
fn test_core_codec_trait_is_implemented_for_utf32_units() {
    let codec = Utf32U32Codec;
    let mut output = [0_u32; 1];

    let written = unsafe {
        codec
            .encode_unchecked('中', &mut output, 0)
            .expect("character should encode")
    };
    assert_eq!(1, written);
    assert_eq!([0x4e2d], output);

    let decoded = unsafe { codec.decode_unchecked(&output, 0).expect("UTF-32 unit should decode") };
    assert_eq!(('中', 1), decoded);
}

#[test]
fn test_core_codec_trait_reports_text_codec_value_errors() {
    let decode_error = unsafe {
        AsciiCodec
            .decode_unchecked(&[0x80], 0)
            .expect_err("non-ASCII byte should be malformed")
    };
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0x80) },
        decode_error.kind(),
    );
    assert_eq!(0, decode_error.index());

    let mut ascii_output = [0_u8; 1];
    let encode_error = unsafe {
        AsciiCodec
            .encode_unchecked('é', &mut ascii_output, 0)
            .expect_err("non-ASCII character should be unmappable")
    };
    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter { value: 'é' as u32 },
        encode_error.kind(),
    );
    assert_eq!(0, encode_error.index());

    let mut latin1_output = [0_u8; 1];
    let encode_error = unsafe {
        Latin1Codec
            .encode_unchecked('\u{0100}', &mut latin1_output, 0)
            .expect_err("outside Latin-1 should be unmappable")
    };
    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter { value: 0x0100 },
        encode_error.kind(),
    );
    assert_eq!(0, encode_error.index());
}
