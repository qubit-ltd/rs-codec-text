// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use proptest::collection;
use proptest::prelude::Strategy;
use proptest::prelude::any;
use proptest::prop_assert_eq;
use proptest::proptest;
use qubit_codec::ByteOrder;
use qubit_codec::TranscodeDecodeError;
use qubit_codec::Transcoder;
use qubit_codec_text::CharsetCodec;
use qubit_codec_text::CharsetDecodeError;
use qubit_codec_text::CharsetDecoder;
use qubit_codec_text::CharsetEncoder;
use qubit_codec_text::Utf8Codec;
use qubit_codec_text::Utf16ByteCodec;

fn short_string() -> impl Strategy<Value = String> {
    collection::vec(any::<char>(), 0..128).prop_map(|chars| chars.into_iter().collect())
}

fn encode_string<C>(encoder: &mut CharsetEncoder<C>, input: &str) -> Vec<C::Unit>
where
    C: CharsetCodec,
    C::Unit: Default,
{
    let input = input.chars().collect::<Vec<_>>();
    let capacity = encoder
        .max_total_output_len(input.len())
        .expect("test encoder output bound should be representable");
    let mut output = Vec::new();
    output.resize_with(capacity, C::Unit::default);
    let written = encoder
        .transcode_complete_into(&input, &mut output)
        .expect("test input should encode");
    output.truncate(written);
    output
}

fn decode_string<C>(decoder: &mut CharsetDecoder<C>, input: &[C::Unit]) -> Result<String, CharsetDecodeError>
where
    C: CharsetCodec,
{
    let capacity = decoder
        .max_total_output_len(input.len())
        .expect("test decoder output bound should be representable");
    let mut output = vec!['\0'; capacity];
    let charset = decoder.charset();
    let written = decoder
        .transcode_complete_into(input, &mut output)
        .map_err(|error| match error {
            TranscodeDecodeError::Failure(failure) => CharsetDecodeError::map_transcode_failure(charset, failure),
            TranscodeDecodeError::Domain(error) => error.into_source(),
        })?;
    Ok(output[..written].iter().collect())
}

proptest! {
    #[test]
    fn test_utf8_encode_decode_round_trips_strings(input in short_string()) {
        let mut encoder = CharsetEncoder::new(Utf8Codec);
        let encoded = encode_string(&mut encoder, &input);
        let mut decoder = CharsetDecoder::new(Utf8Codec);
        let decoded = decode_string(&mut decoder, &encoded)
            .expect("encoded UTF-8 should decode as a complete string");

        prop_assert_eq!(input, decoded);
    }

    #[test]
    fn test_utf16le_byte_encode_decode_round_trips_strings(
        input in short_string(),
    ) {
        let mut encoder = CharsetEncoder::new(Utf16ByteCodec::new(
            ByteOrder::LittleEndian,
        ));
        let encoded = encode_string(&mut encoder, &input);
        let mut decoder = CharsetDecoder::new(Utf16ByteCodec::new(
            ByteOrder::LittleEndian,
        ));
        let decoded = decode_string(&mut decoder, &encoded)
            .expect("encoded UTF-16LE should decode as a complete string");

        prop_assert_eq!(input, decoded);
    }

    #[test]
    fn test_utf8_decode_handles_arbitrary_bytes(
        bytes in collection::vec(any::<u8>(), 0..512),
    ) {
        let mut decoder = CharsetDecoder::new(Utf8Codec);
        let capacity = decoder
            .max_total_output_len(bytes.len())
            .expect("UTF-8 decoder output bound should be representable");
        let mut output = vec!['\0'; capacity];
        let _ = decoder.transcode_complete_into(&bytes, &mut output);
    }
}
