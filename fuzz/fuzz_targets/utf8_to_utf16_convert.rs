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
    CharsetConverter,
    Utf8Codec,
    Utf16U16Codec,
};

const MAX_FUZZ_INPUT_LEN: usize = 4_096;

fuzz_target!(|data: &[u8]| {
    let data = &data[..data.len().min(MAX_FUZZ_INPUT_LEN)];
    let expected = String::from_utf8_lossy(data)
        .encode_utf16()
        .collect::<Vec<_>>();
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let mut output =
        vec![0_u16; data.len().saturating_mul(4).saturating_add(4)];
    let written = converter
        .transcode_complete_into(data, &mut output)
        .expect("replacement converter must accept complete UTF-8 input");
    assert_eq!(expected, output[..written]);
});
