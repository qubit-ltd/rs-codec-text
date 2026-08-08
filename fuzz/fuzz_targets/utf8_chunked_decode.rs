// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
#![no_main]

use libfuzzer_sys::fuzz_target;
use qubit_codec::TranscodeStatus;
use qubit_codec_text::CharsetDecoder;
use qubit_codec_text::Utf8Codec;

const MAX_FUZZ_INPUT_LEN: usize = 4_096;

fuzz_target!(|data: &[u8]| {
    let data = &data[..data.len().min(MAX_FUZZ_INPUT_LEN)];
    let mut one_shot = CharsetDecoder::new(Utf8Codec);
    let mut expected = vec!['\0'; data.len()];
    let expected_written = one_shot
        .transcode_complete_into(data, &mut expected)
        .expect("replacement decoder must accept complete UTF-8 input");

    let mut decoder = CharsetDecoder::new(Utf8Codec);
    decoder.reset(&mut [], 0).expect("reset decoder");
    let mut output = vec!['\0'; data.len()];
    let mut output_cursor = 0;
    let mut pending = Vec::new();
    let mut offset = 0;
    let mut seed = data.len().wrapping_add(1);
    while offset < data.len() {
        seed = seed.wrapping_mul(31).wrapping_add(offset);
        let chunk_len = (seed % 7).saturating_add(1).min(data.len() - offset);
        pending.extend_from_slice(&data[offset..offset + chunk_len]);
        offset += chunk_len;
        let progress = decoder
            .transcode(&pending, 0, &mut output, output_cursor)
            .expect("open stream must not reject incomplete UTF-8");
        output_cursor += progress.written();
        pending.drain(..progress.read());
        assert!(matches!(
            progress.status(),
            TranscodeStatus::Complete | TranscodeStatus::NeedInput { .. }
        ));
    }
    let progress = decoder
        .transcode_eof(&pending, 0, &mut output, output_cursor)
        .expect("EOF replacement policy must accept remaining UTF-8 tail");
    output_cursor += progress.written();
    pending.drain(..progress.read());
    assert!(pending.is_empty());
    output_cursor += decoder
        .finish(&mut output, output_cursor)
        .expect("finish decoder");
    assert_eq!(expected[..expected_written], output[..output_cursor]);
});
