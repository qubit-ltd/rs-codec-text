use qubit_codec::{ByteOrder, Codec};
use qubit_codec_text::{
    AsciiCodec, CharsetDecodeErrorKind, Latin1Codec, Utf8Codec, Utf16ByteCodec, Utf16U16Codec,
    Utf32ByteCodec, Utf32U32Codec,
};

#[test]
fn test_core_codec_trait_reports_text_codec_unit_bounds() {
    assert_eq!(1, <AsciiCodec as Codec>::MIN_UNITS_PER_VALUE.get());
    assert_eq!(1, <AsciiCodec as Codec>::MAX_UNITS_PER_VALUE.get());
    assert_eq!(1, <Latin1Codec as Codec>::MIN_UNITS_PER_VALUE.get());
    assert_eq!(1, <Latin1Codec as Codec>::MAX_UNITS_PER_VALUE.get());
    assert_eq!(1, <Utf8Codec as Codec>::MIN_UNITS_PER_VALUE.get());
    assert_eq!(4, <Utf8Codec as Codec>::MAX_UNITS_PER_VALUE.get());

    assert_eq!(2, <Utf16ByteCodec as Codec>::MIN_UNITS_PER_VALUE.get());
    assert_eq!(4, <Utf16ByteCodec as Codec>::MAX_UNITS_PER_VALUE.get());
    assert_eq!(1, <Utf16U16Codec as Codec>::MIN_UNITS_PER_VALUE.get());
    assert_eq!(2, <Utf16U16Codec as Codec>::MAX_UNITS_PER_VALUE.get());

    assert_eq!(4, <Utf32ByteCodec as Codec>::MIN_UNITS_PER_VALUE.get());
    assert_eq!(4, <Utf32ByteCodec as Codec>::MAX_UNITS_PER_VALUE.get());
    assert_eq!(1, <Utf32U32Codec as Codec>::MIN_UNITS_PER_VALUE.get());
    assert_eq!(1, <Utf32U32Codec as Codec>::MAX_UNITS_PER_VALUE.get());
}

fn assert_u8_codec<C>(mut codec: C, value: char, expected: &[u8])
where
    C: Codec<Value = char, Unit = u8>,
    C::DecodeError: core::fmt::Debug,
    C::EncodeError: core::fmt::Debug,
{
    let mut output = [0_u8; 4];
    let written = unsafe {
        codec
            .encode(&value, &mut output, 0)
            .expect("character should encode")
    }
    .get();
    assert_eq!(expected, &output[..written]);

    let (decoded, consumed) = unsafe {
        codec
            .decode(&output[..written], 0)
            .expect("encoded bytes should decode")
    };
    assert_eq!((value, written), (decoded, consumed.get()));
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
    let mut codec = Utf16U16Codec;
    let mut output = [0_u16; 2];

    let written = unsafe {
        codec
            .encode(&'😀', &mut output, 0)
            .expect("supplementary character should encode")
    }
    .get();
    assert_eq!(2, written);
    assert_eq!([0xd83d, 0xde00], output);

    let (decoded, consumed) = unsafe {
        codec
            .decode(&output, 0)
            .expect("surrogate pair should decode")
    };
    assert_eq!('😀', decoded);
    assert_eq!(2, consumed.get());
}

#[test]
fn test_core_codec_trait_is_implemented_for_utf32_units() {
    let mut codec = Utf32U32Codec;
    let mut output = [0_u32; 1];

    let written = unsafe {
        codec
            .encode(&'中', &mut output, 0)
            .expect("character should encode")
    }
    .get();
    assert_eq!(1, written);
    assert_eq!([0x4e2d], output);

    let (decoded, consumed) =
        unsafe { codec.decode(&output, 0).expect("UTF-32 unit should decode") };
    assert_eq!('中', decoded);
    assert_eq!(1, consumed.get());
}

#[test]
fn test_core_codec_trait_reports_text_codec_value_errors() {
    let decode_error = unsafe {
        AsciiCodec
            .decode(&[0x80], 0)
            .expect_err("non-ASCII byte should be malformed")
    };
    let decode_error = super::invalid_source(decode_error);
    assert_eq!(CharsetDecodeErrorKind::malformed(0x80), decode_error.kind(),);
    assert_eq!(0, decode_error.index());

    assert!(!AsciiCodec.can_encode_value(&'é'));
    assert!(!Latin1Codec.can_encode_value(&'\u{0100}'));
}
