use core::fmt::Debug;

use qubit_codec::TranscodeError;
use qubit_codec_text::{
    AsciiCodec, ByteOrder, CharsetCodec, CharsetConvertError, CharsetConverter,
    CharsetDecodeErrorKind, CharsetDecodePolicy, CharsetDecoder, CharsetEncodePolicy,
    CharsetEncodeProbe, CharsetEncoder, Codec, TranscodeStatus, Transcoder, Utf8Codec,
    Utf16ByteCodec, Utf16U16Codec, Utf32ByteCodec, Utf32U32Codec,
};

#[test]
fn test_utf8_codec_matches_std_boundaries_and_round_trip() {
    let mut codec = Utf8Codec;
    let mut encoder = CharsetEncoder::new(Utf8Codec);
    let samples = [
        "",
        "A",
        "\u{7f}",
        "\u{80}",
        "\u{7ff}",
        "\u{800}",
        "\u{d7ff}",
        "\u{e000}",
        "\u{ffff}",
        "\u{10000}",
        "\u{10ffff}",
        "😀",
    ];

    for text in &samples {
        let bytes = text.as_bytes();
        let expected: Vec<char> = text.chars().collect();
        assert_eq!(expected, decode_all_utf8(&mut codec, bytes));

        let mut output = vec![0_u8; bytes.len()];
        let progress = encoder
            .transcode(&expected, 0, &mut output, 0)
            .expect("utf8 encode should succeed");
        assert_eq!(TranscodeStatus::Complete, progress.status());
        assert_eq!(bytes.len(), progress.written());
        assert_eq!(bytes, &output);
        encoder.reset(&mut output, 0).expect("reset");
    }

    for (input, error_index, value) in [
        (b"\x80" as &[u8], 0, Some(0x80)),
        (b"\xF0\x80\x80\x80", 1, Some(0x80)),
        (b"\xED\xA0\x80", 1, Some(0xA0)),
        (b"\xF4\x90\x80\x80", 1, Some(0x90)),
    ] {
        let std_error = std::str::from_utf8(input).unwrap_err();
        let codec_error =
            unsafe { codec.decode(input, 0) }.expect_err("malformed utf-8 should fail");
        assert_eq!(
            CharsetDecodeErrorKind::MalformedSequence { value },
            codec_error.kind(),
        );
        assert_eq!(0, std_error.valid_up_to());
        assert_eq!(error_index, codec_error.index());
    }

    for (input, required, available) in [
        (&[0xe4][..], 3, 1),
        (&[0xf0, 0x90][..], 4, 2),
        (&[0xf4, 0x80, 0x80][..], 4, 3),
    ] {
        let std_error = std::str::from_utf8(input).unwrap_err();
        assert!(std_error.error_len().is_none());

        let error = unsafe { codec.decode(input, 0) }.expect_err("short input is incomplete");
        assert_eq!(
            CharsetDecodeErrorKind::IncompleteSequence {
                required,
                available
            },
            error.kind(),
        );
    }
}

#[test]
fn test_utf16_codecs_match_std_unit_round_trip() {
    let mut codec = Utf16U16Codec;
    let sample_chars = unicode_boundary_chars();
    let expected_units = encode_utf16_units(&sample_chars);

    assert_eq!(
        sample_chars,
        decode_all_utf16_units(&mut codec, &expected_units)
    );

    let mut encoded = vec![0_u16; expected_units.len()];
    let progress = CharsetEncoder::new(Utf16U16Codec)
        .transcode(&sample_chars, 0, &mut encoded, 0)
        .expect("utf16 u16 encode should succeed");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(expected_units, &encoded[..progress.written()]);

    assert!(
        std::char::decode_utf16([0xd83d].into_iter())
            .next()
            .is_some_and(|result| result.is_err())
    );
    assert!(
        std::char::decode_utf16([0xd83d, 0x0041].into_iter())
            .next()
            .is_some_and(|result| result.is_err())
    );

    for (malformed, offending) in [
        (&[0xdc00_u16][..], 0xdc00),
        (&[0xd83d, 0x0041][..], 0x0041),
        (&[0xdbff, 0x0041][..], 0x0041),
    ] {
        assert!(std::char::decode_utf16(malformed.iter().copied()).any(|result| result.is_err()));
        let decode_result = unsafe { codec.decode(malformed, 0) };
        assert!(matches!(
            decode_result,
            Err(ref error) if matches!(
                error.kind(),
                CharsetDecodeErrorKind::MalformedSequence { value: Some(value) }
                    if value == offending
            ),
        ));
    }

    let partial = [0xd83d];
    let error =
        unsafe { codec.decode(&partial, 0) }.expect_err("partial high surrogate is incomplete");
    assert_eq!(
        CharsetDecodeErrorKind::IncompleteSequence {
            required: 2,
            available: 1,
        },
        error.kind(),
    );
}

#[test]
fn test_utf16_byte_codecs_match_std_and_round_trip() {
    assert_utf16_byte_codec_round_trip(ByteOrder::LittleEndian);
    assert_utf16_byte_codec_round_trip(ByteOrder::BigEndian);
}

#[test]
fn test_utf32_codecs_match_std_unit_round_trip() {
    let mut codec = Utf32U32Codec;
    let sample_chars = unicode_boundary_chars();
    let expected_units: Vec<u32> = sample_chars.iter().map(|&ch| ch as u32).collect();

    assert_eq!(
        sample_chars,
        decode_all_utf32_units(&mut codec, &expected_units)
    );

    let mut encoded = vec![0_u32; expected_units.len()];
    let progress = CharsetEncoder::new(Utf32U32Codec)
        .transcode(&sample_chars, 0, &mut encoded, 0)
        .expect("utf32 u32 encode should succeed");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(expected_units, &encoded[..progress.written()]);

    let invalid_units = [0xd800u32, 0xdfffu32, 0x110000u32, 0x0011_0000u32];
    for invalid in invalid_units {
        assert_eq!(None, std::char::from_u32(invalid));
        assert!(matches!(
            unsafe { codec.decode(&[invalid], 0) },
            Err(ref error) if matches!(error.kind(), CharsetDecodeErrorKind::InvalidCodePoint { .. }),
        ));
    }
}

#[test]
fn test_utf32_byte_codecs_match_std_and_round_trip() {
    assert_utf32_byte_codec_round_trip(ByteOrder::LittleEndian);
    assert_utf32_byte_codec_round_trip(ByteOrder::BigEndian);
}

#[test]
fn test_charset_decoder_wrappers_decode_unicode_boundaries() {
    let chars = unicode_boundary_chars();
    let utf8 = encode_utf8_bytes(&chars);
    let utf16 = encode_utf16_units(&chars);
    let utf16le = encode_utf16_bytes(&chars, ByteOrder::LittleEndian);
    let utf16be = encode_utf16_bytes(&chars, ByteOrder::BigEndian);
    let utf32 = encode_utf32_units(&chars);
    let utf32le = encode_utf32_bytes(&chars, ByteOrder::LittleEndian);
    let utf32be = encode_utf32_bytes(&chars, ByteOrder::BigEndian);

    assert_decoder_output(CharsetDecoder::new(Utf8Codec), &utf8, &chars);
    assert_decoder_output(CharsetDecoder::new(Utf16U16Codec), &utf16, &chars);
    assert_decoder_output(
        CharsetDecoder::new(Utf16ByteCodec::new(ByteOrder::LittleEndian)),
        &utf16le,
        &chars,
    );
    assert_decoder_output(
        CharsetDecoder::new(Utf16ByteCodec::new(ByteOrder::BigEndian)),
        &utf16be,
        &chars,
    );
    assert_decoder_output(CharsetDecoder::new(Utf32U32Codec), &utf32, &chars);
    assert_decoder_output(
        CharsetDecoder::new(Utf32ByteCodec::new(ByteOrder::LittleEndian)),
        &utf32le,
        &chars,
    );
    assert_decoder_output(
        CharsetDecoder::new(Utf32ByteCodec::new(ByteOrder::BigEndian)),
        &utf32be,
        &chars,
    );
}

#[test]
fn test_charset_encoder_policies_encode_unicode_boundaries() {
    let chars = unicode_boundary_chars();
    let utf8 = encode_utf8_bytes(&chars);
    let utf16 = encode_utf16_units(&chars);
    let utf16le = encode_utf16_bytes(&chars, ByteOrder::LittleEndian);
    let utf16be = encode_utf16_bytes(&chars, ByteOrder::BigEndian);
    let utf32 = encode_utf32_units(&chars);
    let utf32le = encode_utf32_bytes(&chars, ByteOrder::LittleEndian);
    let utf32be = encode_utf32_bytes(&chars, ByteOrder::BigEndian);

    assert_encoder_policies_output(Utf8Codec, &chars, &utf8);
    assert_encoder_policies_output(Utf16U16Codec, &chars, &utf16);
    assert_encoder_policies_output(
        Utf16ByteCodec::new(ByteOrder::LittleEndian),
        &chars,
        &utf16le,
    );
    assert_encoder_policies_output(Utf16ByteCodec::new(ByteOrder::BigEndian), &chars, &utf16be);
    assert_encoder_policies_output(Utf32U32Codec, &chars, &utf32);
    assert_encoder_policies_output(
        Utf32ByteCodec::new(ByteOrder::LittleEndian),
        &chars,
        &utf32le,
    );
    assert_encoder_policies_output(Utf32ByteCodec::new(ByteOrder::BigEndian), &chars, &utf32be);
}

#[test]
fn test_charset_converter_transcodes_unicode_boundaries_between_codecs() {
    let chars = unicode_boundary_chars();
    let utf8 = encode_utf8_bytes(&chars);
    let utf16 = encode_utf16_units(&chars);
    let utf16le = encode_utf16_bytes(&chars, ByteOrder::LittleEndian);
    let utf16be = encode_utf16_bytes(&chars, ByteOrder::BigEndian);
    let utf32 = encode_utf32_units(&chars);
    let utf32le = encode_utf32_bytes(&chars, ByteOrder::LittleEndian);
    let utf32be = encode_utf32_bytes(&chars, ByteOrder::BigEndian);

    macro_rules! assert_all_unicode_targets {
        ($source:expr, $input:expr) => {{
            assert_converter_output($source, Utf8Codec, $input, &utf8);
            assert_converter_output($source, Utf16U16Codec, $input, &utf16);
            assert_converter_output(
                $source,
                Utf16ByteCodec::new(ByteOrder::LittleEndian),
                $input,
                &utf16le,
            );
            assert_converter_output(
                $source,
                Utf16ByteCodec::new(ByteOrder::BigEndian),
                $input,
                &utf16be,
            );
            assert_converter_output($source, Utf32U32Codec, $input, &utf32);
            assert_converter_output(
                $source,
                Utf32ByteCodec::new(ByteOrder::LittleEndian),
                $input,
                &utf32le,
            );
            assert_converter_output(
                $source,
                Utf32ByteCodec::new(ByteOrder::BigEndian),
                $input,
                &utf32be,
            );
        }};
    }

    assert_all_unicode_targets!(Utf8Codec, &utf8);
    assert_all_unicode_targets!(Utf16U16Codec, &utf16);
    assert_all_unicode_targets!(Utf16ByteCodec::new(ByteOrder::LittleEndian), &utf16le);
    assert_all_unicode_targets!(Utf16ByteCodec::new(ByteOrder::BigEndian), &utf16be);
    assert_all_unicode_targets!(Utf32U32Codec, &utf32);
    assert_all_unicode_targets!(Utf32ByteCodec::new(ByteOrder::LittleEndian), &utf32le);
    assert_all_unicode_targets!(Utf32ByteCodec::new(ByteOrder::BigEndian), &utf32be);
}

#[test]
fn test_charset_decoder_policies_handle_malformed_unicode_codecs() {
    let utf16_low_surrogate = 0xdc00_u16;
    let utf32_invalid = 0x110000_u32;

    assert_decoder_malformed_policies(
        Utf8Codec,
        &[0x80, b'A'],
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0x80) },
        0,
    );
    assert_decoder_malformed_policies(
        Utf16U16Codec,
        &[utf16_low_surrogate, 'A' as u16],
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some(utf16_low_surrogate as u32),
        },
        0,
    );
    assert_decoder_malformed_policies(
        Utf16ByteCodec::new(ByteOrder::LittleEndian),
        &[
            utf16_low_surrogate.to_le_bytes()[0],
            utf16_low_surrogate.to_le_bytes()[1],
            b'A',
            0,
        ],
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some(utf16_low_surrogate as u32),
        },
        0,
    );
    assert_decoder_malformed_policies(
        Utf16ByteCodec::new(ByteOrder::BigEndian),
        &[
            utf16_low_surrogate.to_be_bytes()[0],
            utf16_low_surrogate.to_be_bytes()[1],
            0,
            b'A',
        ],
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some(utf16_low_surrogate as u32),
        },
        0,
    );
    assert_decoder_malformed_policies(
        Utf32U32Codec,
        &[utf32_invalid, 'A' as u32],
        CharsetDecodeErrorKind::InvalidCodePoint {
            value: utf32_invalid,
        },
        0,
    );
    assert_decoder_malformed_policies(
        Utf32ByteCodec::new(ByteOrder::LittleEndian),
        &[
            utf32_invalid.to_le_bytes()[0],
            utf32_invalid.to_le_bytes()[1],
            utf32_invalid.to_le_bytes()[2],
            utf32_invalid.to_le_bytes()[3],
            b'A',
            0,
            0,
            0,
        ],
        CharsetDecodeErrorKind::InvalidCodePoint {
            value: utf32_invalid,
        },
        0,
    );
    assert_decoder_malformed_policies(
        Utf32ByteCodec::new(ByteOrder::BigEndian),
        &[
            utf32_invalid.to_be_bytes()[0],
            utf32_invalid.to_be_bytes()[1],
            utf32_invalid.to_be_bytes()[2],
            utf32_invalid.to_be_bytes()[3],
            0,
            0,
            0,
            b'A',
        ],
        CharsetDecodeErrorKind::InvalidCodePoint {
            value: utf32_invalid,
        },
        0,
    );
}

#[test]
fn test_charset_converter_policies_handle_unicode_decode_and_encode_paths() {
    let utf16_low_surrogate = 0xdc00_u16;
    let utf32_invalid = 0x110000_u32;

    assert_converter_decode_policies(
        Utf8Codec,
        &[0x80, b'A'],
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0x80) },
        0,
    );
    assert_converter_decode_policies(
        Utf16U16Codec,
        &[utf16_low_surrogate, 'A' as u16],
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some(utf16_low_surrogate as u32),
        },
        0,
    );
    assert_converter_decode_policies(
        Utf16ByteCodec::new(ByteOrder::LittleEndian),
        &[
            utf16_low_surrogate.to_le_bytes()[0],
            utf16_low_surrogate.to_le_bytes()[1],
            b'A',
            0,
        ],
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some(utf16_low_surrogate as u32),
        },
        0,
    );
    assert_converter_decode_policies(
        Utf16ByteCodec::new(ByteOrder::BigEndian),
        &[
            utf16_low_surrogate.to_be_bytes()[0],
            utf16_low_surrogate.to_be_bytes()[1],
            0,
            b'A',
        ],
        CharsetDecodeErrorKind::MalformedSequence {
            value: Some(utf16_low_surrogate as u32),
        },
        0,
    );
    assert_converter_decode_policies(
        Utf32U32Codec,
        &[utf32_invalid, 'A' as u32],
        CharsetDecodeErrorKind::InvalidCodePoint {
            value: utf32_invalid,
        },
        0,
    );
    assert_converter_decode_policies(
        Utf32ByteCodec::new(ByteOrder::LittleEndian),
        &[
            utf32_invalid.to_le_bytes()[0],
            utf32_invalid.to_le_bytes()[1],
            utf32_invalid.to_le_bytes()[2],
            utf32_invalid.to_le_bytes()[3],
            b'A',
            0,
            0,
            0,
        ],
        CharsetDecodeErrorKind::InvalidCodePoint {
            value: utf32_invalid,
        },
        0,
    );
    assert_converter_decode_policies(
        Utf32ByteCodec::new(ByteOrder::BigEndian),
        &[
            utf32_invalid.to_be_bytes()[0],
            utf32_invalid.to_be_bytes()[1],
            utf32_invalid.to_be_bytes()[2],
            utf32_invalid.to_be_bytes()[3],
            0,
            0,
            0,
            b'A',
        ],
        CharsetDecodeErrorKind::InvalidCodePoint {
            value: utf32_invalid,
        },
        0,
    );

    let input = "A中B".as_bytes();
    assert_converter_output_with_policies(
        Utf8Codec,
        AsciiCodec,
        CharsetDecodePolicy::default(),
        CharsetEncodePolicy::replace('?'),
        input,
        b"A?B",
    );
    assert_converter_output_with_policies(
        Utf8Codec,
        AsciiCodec,
        CharsetDecodePolicy::default(),
        CharsetEncodePolicy::ignore(),
        input,
        b"AB",
    );

    let mut converter = CharsetConverter::from_codecs_with_policies(
        Utf8Codec,
        AsciiCodec,
        CharsetDecodePolicy::default(),
        CharsetEncodePolicy::report(),
    )
    .expect("report policy should not pre-encode replacement");
    let mut output = [0_u8; 3];
    let error = converter
        .transcode(input, 0, &mut output, 0)
        .expect_err("unmappable target character should be reported");
    match error {
        TranscodeError::Domain(CharsetConvertError::Encode(error)) => {
            assert_eq!(Some('中' as u32), error.value());
            assert_eq!(1, error.index());
        }
        TranscodeError::Domain(CharsetConvertError::Decode(error)) => {
            panic!("expected encode error, got {error:?}")
        }
        other => panic!("expected encode domain error, got {other:?}"),
    }
}

fn decode_all_utf8(codec: &mut Utf8Codec, input: &[u8]) -> Vec<char> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < input.len() {
        match unsafe { codec.decode(input, index) } {
            Ok((value, consumed)) => {
                output.push(value);
                index += consumed.get();
            }
            status => panic!("expected complete utf8 decode for valid sequence, got {status:?}"),
        }
    }
    output
}

fn decode_all_utf16_units(codec: &mut Utf16U16Codec, input: &[u16]) -> Vec<char> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < input.len() {
        match unsafe { codec.decode(input, index) } {
            Ok((value, consumed)) => {
                output.push(value);
                index += consumed.get();
            }
            status => panic!("expected complete utf16 decode for valid sequence, got {status:?}"),
        }
    }
    output
}

fn decode_all_utf16_bytes(codec: &mut Utf16ByteCodec, input: &[u8]) -> Vec<char> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < input.len() {
        match unsafe { codec.decode(input, index) } {
            Ok((value, consumed)) => {
                output.push(value);
                index += consumed.get();
            }
            status => {
                panic!("expected complete utf16 byte decode for valid sequence, got {status:?}")
            }
        }
    }
    output
}

fn decode_all_utf32_units(codec: &mut Utf32U32Codec, input: &[u32]) -> Vec<char> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < input.len() {
        match unsafe { codec.decode(input, index) } {
            Ok((value, consumed)) => {
                output.push(value);
                index += consumed.get();
            }
            status => panic!("expected complete utf32 decode for valid sequence, got {status:?}"),
        }
    }
    output
}

fn decode_all_utf32_bytes(codec: &mut Utf32ByteCodec, input: &[u8]) -> Vec<char> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < input.len() {
        match unsafe { codec.decode(input, index) } {
            Ok((value, consumed)) => {
                output.push(value);
                index += consumed.get();
            }
            status => {
                panic!("expected complete utf32 byte decode for valid sequence, got {status:?}")
            }
        }
    }
    output
}

fn assert_utf16_byte_codec_round_trip(order: ByteOrder) {
    let mut codec = Utf16ByteCodec::new(order);
    let chars = unicode_boundary_chars();
    let units = encode_utf16_units(&chars);
    let expected: Vec<u8> = units
        .iter()
        .copied()
        .flat_map(|unit| match order {
            ByteOrder::LittleEndian => unit.to_le_bytes().to_vec(),
            ByteOrder::BigEndian => unit.to_be_bytes().to_vec(),
        })
        .collect();

    assert_eq!(chars, decode_all_utf16_bytes(&mut codec, &expected));

    let mut output = vec![0_u8; expected.len()];
    let progress = CharsetEncoder::new(codec)
        .transcode(&chars, 0, &mut output, 0)
        .expect("utf16 byte encode should succeed");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(expected, &output[..progress.written()]);
}

fn assert_utf32_byte_codec_round_trip(order: ByteOrder) {
    let mut codec = Utf32ByteCodec::new(order);
    let chars = unicode_boundary_chars();
    let units: Vec<u32> = chars.iter().copied().map(|ch| ch as u32).collect();
    let expected: Vec<u8> = units
        .iter()
        .copied()
        .flat_map(|value| match order {
            ByteOrder::LittleEndian => value.to_le_bytes(),
            ByteOrder::BigEndian => value.to_be_bytes(),
        })
        .collect();

    assert_eq!(chars, decode_all_utf32_bytes(&mut codec, &expected));

    let mut output = vec![0_u8; expected.len()];
    let progress = CharsetEncoder::new(codec)
        .transcode(&chars, 0, &mut output, 0)
        .expect("utf32 byte encode should succeed");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(expected, &output[..progress.written()]);
}

fn assert_decoder_output<C>(mut decoder: CharsetDecoder<C>, input: &[C::Unit], expected: &[char])
where
    C: CharsetCodec,
{
    let mut output = vec!['\0'; expected.len()];
    let progress = decoder
        .transcode(input, 0, &mut output, 0)
        .expect("decoder should decode valid Unicode input");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(input.len(), progress.read());
    assert_eq!(expected.len(), progress.written());
    assert_eq!(expected, &output[..progress.written()]);

    let written = decoder
        .finish(&mut output, progress.written())
        .expect("decoder finish should not emit extra output");
    assert_eq!(0, written);
}

fn assert_encoder_policies_output<C>(codec: C, input: &[char], expected: &[C::Unit])
where
    C: CharsetEncodeProbe + Copy,
    C::Unit: Clone + Debug + Default + PartialEq,
{
    for policy in [
        CharsetEncodePolicy::replace('!'),
        CharsetEncodePolicy::ignore(),
        CharsetEncodePolicy::report(),
    ] {
        assert_encoder_output_with_policy(codec, policy, input, expected);
    }
}

fn assert_encoder_output_with_policy<C>(
    codec: C,
    policy: CharsetEncodePolicy,
    input: &[char],
    expected: &[C::Unit],
) where
    C: CharsetEncodeProbe,
    C::Unit: Clone + Debug + Default + PartialEq,
{
    let mut encoder = CharsetEncoder::with_policy(codec, policy)
        .expect("Unicode codec policy should be constructible");
    let mut output = vec![C::Unit::default(); expected.len()];
    let progress = encoder
        .transcode(input, 0, &mut output, 0)
        .expect("Unicode codec should encode every scalar value");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(input.len(), progress.read());
    assert_eq!(expected.len(), progress.written());
    assert_eq!(expected, &output[..progress.written()]);

    let written = encoder
        .finish(&mut output, progress.written())
        .expect("encoder finish should not emit extra output");
    assert_eq!(0, written);
}

fn assert_converter_output<D, E>(source: D, target: E, input: &[D::Unit], expected: &[E::Unit])
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
    E::Unit: Clone + Debug + Default + PartialEq,
{
    assert_converter_output_with_policies(
        source,
        target,
        CharsetDecodePolicy::default(),
        CharsetEncodePolicy::default(),
        input,
        expected,
    );
}

fn assert_converter_output_with_policies<D, E>(
    source: D,
    target: E,
    decode_policy: CharsetDecodePolicy,
    encode_policy: CharsetEncodePolicy,
    input: &[D::Unit],
    expected: &[E::Unit],
) where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
    E::Unit: Clone + Debug + Default + PartialEq,
{
    let mut converter =
        CharsetConverter::from_codecs_with_policies(source, target, decode_policy, encode_policy)
            .expect("converter policies should be constructible");
    let mut output = vec![E::Unit::default(); expected.len()];
    let progress = converter
        .transcode(input, 0, &mut output, 0)
        .expect("converter should transcode input");

    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(input.len(), progress.read());
    assert_eq!(expected.len(), progress.written());
    assert_eq!(expected, &output[..progress.written()]);

    let written = converter
        .finish(&mut output, progress.written())
        .expect("converter finish should not emit extra output");
    assert_eq!(0, written);
}

fn assert_decoder_malformed_policies<C>(
    codec: C,
    input: &[C::Unit],
    expected_kind: CharsetDecodeErrorKind,
    expected_index: usize,
) where
    C: CharsetCodec + Copy,
{
    let mut decoder = CharsetDecoder::with_policy(codec, CharsetDecodePolicy::replace('!'));
    let mut replaced = ['\0'; 2];
    let progress = decoder
        .transcode(input, 0, &mut replaced, 0)
        .expect("replace policy should emit replacement and continue");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(input.len(), progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(['!', 'A'], replaced);

    let mut decoder = CharsetDecoder::with_policy(codec, CharsetDecodePolicy::ignore());
    let mut ignored = ['\0'; 1];
    let progress = decoder
        .transcode(input, 0, &mut ignored, 0)
        .expect("ignore policy should skip malformed input and continue");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(input.len(), progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(['A'], ignored);

    let mut decoder = CharsetDecoder::with_policy(codec, CharsetDecodePolicy::report());
    let mut output = ['\0'; 2];
    let error = decoder
        .transcode(input, 0, &mut output, 0)
        .expect_err("report policy should surface malformed input");
    match error {
        TranscodeError::Domain(error) => {
            assert_eq!(expected_kind, error.kind());
            assert_eq!(expected_index, error.index());
        }
        other => panic!("expected decode domain error, got {other:?}"),
    }
}

fn assert_converter_decode_policies<D>(
    source: D,
    input: &[D::Unit],
    expected_kind: CharsetDecodeErrorKind,
    expected_index: usize,
) where
    D: CharsetCodec + Copy,
{
    assert_converter_output_with_policies(
        source,
        Utf8Codec,
        CharsetDecodePolicy::replace('!'),
        CharsetEncodePolicy::default(),
        input,
        b"!A",
    );
    assert_converter_output_with_policies(
        source,
        Utf8Codec,
        CharsetDecodePolicy::ignore(),
        CharsetEncodePolicy::default(),
        input,
        b"A",
    );

    let mut converter = CharsetConverter::from_codecs_with_policies(
        source,
        Utf8Codec,
        CharsetDecodePolicy::report(),
        CharsetEncodePolicy::default(),
    )
    .expect("report decode policy should be constructible");
    let mut output = [0_u8; 2];
    let error = converter
        .transcode(input, 0, &mut output, 0)
        .expect_err("report decode policy should surface source error");
    match error {
        TranscodeError::Domain(CharsetConvertError::Decode(error)) => {
            assert_eq!(expected_kind, error.kind());
            assert_eq!(expected_index, error.index());
        }
        TranscodeError::Domain(CharsetConvertError::Encode(error)) => {
            panic!("expected decode error, got {error:?}")
        }
        other => panic!("expected decode domain error, got {other:?}"),
    }
}

fn unicode_boundary_chars() -> Vec<char> {
    vec![
        '\0',
        'A',
        '\u{7f}',
        '\u{80}',
        '\u{7ff}',
        '\u{800}',
        '\u{d7ff}',
        '\u{e000}',
        '\u{ffff}',
        '\u{10000}',
        '\u{10ffff}',
        '中',
        '😀',
    ]
}

fn encode_utf8_bytes(chars: &[char]) -> Vec<u8> {
    chars.iter().collect::<String>().into_bytes()
}

fn encode_utf16_units(chars: &[char]) -> Vec<u16> {
    let mut units = Vec::new();
    for ch in chars {
        let mut buffer = [0_u16; 2];
        units.extend_from_slice(ch.encode_utf16(&mut buffer));
    }
    units
}

fn encode_utf16_bytes(chars: &[char], order: ByteOrder) -> Vec<u8> {
    encode_utf16_units(chars)
        .into_iter()
        .flat_map(|unit| match order {
            ByteOrder::LittleEndian => unit.to_le_bytes(),
            ByteOrder::BigEndian => unit.to_be_bytes(),
        })
        .collect()
}

fn encode_utf32_units(chars: &[char]) -> Vec<u32> {
    chars.iter().copied().map(|ch| ch as u32).collect()
}

fn encode_utf32_bytes(chars: &[char], order: ByteOrder) -> Vec<u8> {
    encode_utf32_units(chars)
        .into_iter()
        .flat_map(|unit| match order {
            ByteOrder::LittleEndian => unit.to_le_bytes(),
            ByteOrder::BigEndian => unit.to_be_bytes(),
        })
        .collect()
}
