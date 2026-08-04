// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
#![no_main]

use libfuzzer_sys::fuzz_target;
use qubit_codec_text::{
    CharsetEncoder,
    Utf8Codec,
};

const MAX_FUZZ_INPUT_LEN: usize = 4_096;

fuzz_target!(|data: &[u8]| {
    let data = &data[..data.len().min(MAX_FUZZ_INPUT_LEN)];
    let chars = data
        .chunks(4)
        .map(|chunk| {
            let mut bytes = [0_u8; 4];
            bytes[..chunk.len()].copy_from_slice(chunk);
            char::from_u32(u32::from_le_bytes(bytes))
                .unwrap_or(char::REPLACEMENT_CHARACTER)
        })
        .collect::<Vec<_>>();
    let expected = chars.iter().collect::<String>();
    let mut encoder = CharsetEncoder::new(Utf8Codec);
    let mut output = vec![0_u8; chars.len().saturating_mul(4)];
    let written = encoder
        .transcode_complete_into(&chars, &mut output)
        .expect("UTF-8 encoder must encode Unicode scalar values");
    assert_eq!(expected.as_bytes(), &output[..written]);
});
