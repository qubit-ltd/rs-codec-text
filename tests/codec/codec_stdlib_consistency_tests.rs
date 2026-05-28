use qubit_codec_text::{
    ByteOrder,
    CharsetDecodeErrorKind,
    CharsetEncoder,
    Codec,
    TranscodeStatus,
    Transcoder,
    Utf8Codec,
    Utf16ByteCodec,
    Utf16U16Codec,
    Utf32ByteCodec,
    Utf32U32Codec,
};

#[test]
fn test_utf8_codec_matches_std_boundaries_and_round_trip() {
    let codec = Utf8Codec;
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
        assert_eq!(expected, decode_all_utf8(&codec, bytes));

        let mut output = vec![0_u8; bytes.len()];
        let progress = encoder
            .transcode(&expected, 0, &mut output, 0)
            .expect("utf8 encode should succeed");
        assert_eq!(TranscodeStatus::Complete, progress.status());
        assert_eq!(bytes.len(), progress.written());
        assert_eq!(bytes, &output);
        encoder.reset();
    }

    for (input, error_index, value) in [
        (b"\x80" as &[u8], 0, Some(0x80)),
        (b"\xF0\x80\x80\x80", 1, Some(0x80)),
        (b"\xED\xA0\x80", 1, Some(0xA0)),
        (b"\xF4\x90\x80\x80", 1, Some(0x90)),
    ] {
        let std_error = std::str::from_utf8(input).unwrap_err();
        let codec_error = unsafe { codec.decode_unchecked(input, 0) }.expect_err("malformed utf-8 should fail");
        assert_eq!(CharsetDecodeErrorKind::MalformedSequence { value }, codec_error.kind(),);
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

        let error = unsafe { codec.decode_unchecked(input, 0) }.expect_err("short input is incomplete");
        assert_eq!(
            CharsetDecodeErrorKind::IncompleteSequence { required, available },
            error.kind(),
        );
    }
}

#[test]
fn test_utf16_codecs_match_std_unit_round_trip() {
    let codec = Utf16U16Codec;
    let sample_chars: Vec<char> = vec![
        'A',
        '\u{7f}',
        '\u{7ff}',
        '\u{800}',
        '\u{d7ff}',
        '\u{10000}',
        '\u{10ffff}',
    ];
    let expected_units = encode_utf16_units(&sample_chars);

    assert_eq!(sample_chars, decode_all_utf16_units(&codec, &expected_units));

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
        let decode_result = unsafe { codec.decode_unchecked(malformed, 0) };
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
    let error = unsafe { codec.decode_unchecked(&partial, 0) }.expect_err("partial high surrogate is incomplete");
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
    let codec = Utf32U32Codec;
    let sample_chars: Vec<char> = vec![
        'A',
        '\u{7f}',
        '\u{7ff}',
        '\u{800}',
        '\u{d7ff}',
        '\u{10000}',
        '\u{10ffff}',
    ];
    let expected_units: Vec<u32> = sample_chars.iter().map(|&ch| ch as u32).collect();

    assert_eq!(sample_chars, decode_all_utf32_units(&codec, &expected_units));

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
            unsafe { codec.decode_unchecked(&[invalid], 0) },
            Err(ref error) if matches!(error.kind(), CharsetDecodeErrorKind::InvalidCodePoint { .. }),
        ));
    }
}

#[test]
fn test_utf32_byte_codecs_match_std_and_round_trip() {
    assert_utf32_byte_codec_round_trip(ByteOrder::LittleEndian);
    assert_utf32_byte_codec_round_trip(ByteOrder::BigEndian);
}

fn decode_all_utf8(codec: &Utf8Codec, input: &[u8]) -> Vec<char> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < input.len() {
        match unsafe { codec.decode_unchecked(input, index) } {
            Ok((value, consumed)) => {
                output.push(value);
                index += consumed;
            }
            status => panic!("expected complete utf8 decode for valid sequence, got {status:?}"),
        }
    }
    output
}

fn decode_all_utf16_units(codec: &Utf16U16Codec, input: &[u16]) -> Vec<char> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < input.len() {
        match unsafe { codec.decode_unchecked(input, index) } {
            Ok((value, consumed)) => {
                output.push(value);
                index += consumed;
            }
            status => panic!("expected complete utf16 decode for valid sequence, got {status:?}"),
        }
    }
    output
}

fn decode_all_utf16_bytes(codec: &Utf16ByteCodec, input: &[u8]) -> Vec<char> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < input.len() {
        match unsafe { codec.decode_unchecked(input, index) } {
            Ok((value, consumed)) => {
                output.push(value);
                index += consumed;
            }
            status => {
                panic!("expected complete utf16 byte decode for valid sequence, got {status:?}")
            }
        }
    }
    output
}

fn decode_all_utf32_units(codec: &Utf32U32Codec, input: &[u32]) -> Vec<char> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < input.len() {
        match unsafe { codec.decode_unchecked(input, index) } {
            Ok((value, consumed)) => {
                output.push(value);
                index += consumed;
            }
            status => panic!("expected complete utf32 decode for valid sequence, got {status:?}"),
        }
    }
    output
}

fn decode_all_utf32_bytes(codec: &Utf32ByteCodec, input: &[u8]) -> Vec<char> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < input.len() {
        match unsafe { codec.decode_unchecked(input, index) } {
            Ok((value, consumed)) => {
                output.push(value);
                index += consumed;
            }
            status => {
                panic!("expected complete utf32 byte decode for valid sequence, got {status:?}")
            }
        }
    }
    output
}

fn assert_utf16_byte_codec_round_trip(order: ByteOrder) {
    let codec = Utf16ByteCodec::new(order);
    let chars: Vec<char> = vec![
        'A',
        '\u{7f}',
        '\u{7ff}',
        '\u{800}',
        '\u{d7ff}',
        '\u{10000}',
        '\u{10ffff}',
    ];
    let units = encode_utf16_units(&chars);
    let expected: Vec<u8> = units
        .iter()
        .copied()
        .flat_map(|unit| match order {
            ByteOrder::LittleEndian => unit.to_le_bytes().to_vec(),
            ByteOrder::BigEndian => unit.to_be_bytes().to_vec(),
        })
        .collect();

    assert_eq!(chars, decode_all_utf16_bytes(&codec, &expected));

    let mut output = vec![0_u8; expected.len()];
    let progress = CharsetEncoder::new(codec)
        .transcode(&chars, 0, &mut output, 0)
        .expect("utf16 byte encode should succeed");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(expected, &output[..progress.written()]);
}

fn assert_utf32_byte_codec_round_trip(order: ByteOrder) {
    let codec = Utf32ByteCodec::new(order);
    let chars: Vec<char> = vec![
        'A',
        '\u{7f}',
        '\u{7ff}',
        '\u{800}',
        '\u{d7ff}',
        '\u{10000}',
        '\u{10ffff}',
    ];
    let units: Vec<u32> = chars.iter().copied().map(|ch| ch as u32).collect();
    let expected: Vec<u8> = units
        .iter()
        .copied()
        .flat_map(|value| match order {
            ByteOrder::LittleEndian => value.to_le_bytes(),
            ByteOrder::BigEndian => value.to_be_bytes(),
        })
        .collect();

    assert_eq!(chars, decode_all_utf32_bytes(&codec, &expected));

    let mut output = vec![0_u8; expected.len()];
    let progress = CharsetEncoder::new(codec)
        .transcode(&chars, 0, &mut output, 0)
        .expect("utf32 byte encode should succeed");
    assert_eq!(TranscodeStatus::Complete, progress.status());
    assert_eq!(expected, &output[..progress.written()]);
}

fn encode_utf16_units(chars: &[char]) -> Vec<u16> {
    let mut units = Vec::new();
    for ch in chars {
        let mut buffer = [0_u16; 2];
        units.extend_from_slice(ch.encode_utf16(&mut buffer));
    }
    units
}
