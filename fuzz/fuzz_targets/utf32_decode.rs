// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
#![no_main]

use libfuzzer_sys::fuzz_target;
use qubit_codec::ByteOrder;
use qubit_codec_text::{CharsetDecodePolicy, CharsetDecoder, Utf32ByteCodec};

const MAX_FUZZ_INPUT_LEN: usize = 4_096;

fuzz_target!(|data: &[u8]| {
    let data = &data[..data.len().min(MAX_FUZZ_INPUT_LEN)];
    fuzz_order(data, ByteOrder::LittleEndian);
    fuzz_order(data, ByteOrder::BigEndian);
});

fn fuzz_order(data: &[u8], byte_order: ByteOrder) {
    let mut replace = CharsetDecoder::new(Utf32ByteCodec::new(byte_order));
    let mut replace_output = vec!['\0'; data.len()];
    let replace_result = replace.transcode_complete_into(data, &mut replace_output);

    let mut report = CharsetDecoder::with_policy(
        Utf32ByteCodec::new(byte_order),
        CharsetDecodePolicy::report(),
    );
    let mut report_output = vec!['\0'; data.len()];
    let report_result = report.transcode_complete_into(data, &mut report_output);

    let mut malformed = !data.len().is_multiple_of(4);
    let mut expected = Vec::with_capacity(data.len() / 4 + 1);
    for bytes in data.chunks_exact(4) {
        let value = match byte_order {
            ByteOrder::LittleEndian => u32::from_le_bytes(bytes.try_into().expect("four bytes")),
            ByteOrder::BigEndian => u32::from_be_bytes(bytes.try_into().expect("four bytes")),
            ByteOrder::NativeEndian => u32::from_ne_bytes(bytes.try_into().expect("four bytes")),
        };
        match char::from_u32(value) {
            Some(value) => expected.push(value),
            None => {
                malformed = true;
                expected.push(char::REPLACEMENT_CHARACTER);
            }
        }
    }
    if !data.len().is_multiple_of(4) {
        expected.push(char::REPLACEMENT_CHARACTER);
    }

    let replace_written = replace_result.expect("complete UTF-32 input must be replaceable");
    assert_eq!(expected, replace_output[..replace_written]);
    if malformed {
        assert!(report_result.is_err());
    } else {
        let report_written = report_result.expect("well-formed UTF-32 must report success");
        assert_eq!(expected, report_output[..report_written]);
    }
}
