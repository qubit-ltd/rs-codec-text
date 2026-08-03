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
    CharsetDecodePolicy,
    CharsetDecoder,
    Utf8Codec,
};

const MAX_FUZZ_INPUT_LEN: usize = 4_096;

fuzz_target!(|data: &[u8]| {
    let data = &data[..data.len().min(MAX_FUZZ_INPUT_LEN)];
    let mut replace = CharsetDecoder::new(Utf8Codec);
    let mut replace_output = vec!['\0'; data.len()];
    let replace_result =
        replace.transcode_complete_into(data, &mut replace_output);

    let mut report =
        CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());
    let mut report_output = vec!['\0'; data.len()];
    let report_result =
        report.transcode_complete_into(data, &mut report_output);

    match core::str::from_utf8(data) {
        Ok(expected) => {
            let expected = expected.chars().collect::<Vec<_>>();
            let replace_written =
                replace_result.expect("valid UTF-8 must decode");
            let report_written =
                report_result.expect("valid UTF-8 must report success");
            assert_eq!(expected, replace_output[..replace_written]);
            assert_eq!(expected, report_output[..report_written]);
        }
        Err(error) if error.error_len().is_some() => {
            let expected =
                String::from_utf8_lossy(data).chars().collect::<Vec<_>>();
            let replace_written = replace_result
                .expect("complete malformed UTF-8 must be replaceable");
            assert_eq!(expected, replace_output[..replace_written]);
            assert!(report_result.is_err());
        }
        Err(_) => {
            let expected =
                String::from_utf8_lossy(data).chars().collect::<Vec<_>>();
            let replace_written = replace_result
                .expect("incomplete UTF-8 at EOF must be replaceable");
            assert_eq!(expected, replace_output[..replace_written]);
            assert!(report_result.is_err());
        }
    }
});
