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
use qubit_codec_text::{
    CharsetDecodePolicy,
    CharsetDecoder,
    Utf16ByteCodec,
};

const MAX_FUZZ_INPUT_LEN: usize = 4_096;

fuzz_target!(|data: &[u8]| {
    let data = &data[..data.len().min(MAX_FUZZ_INPUT_LEN)];
    fuzz_order(data, ByteOrder::LittleEndian);
    fuzz_order(data, ByteOrder::BigEndian);
});

fn fuzz_order(data: &[u8], byte_order: ByteOrder) {
    let mut replace = CharsetDecoder::new(Utf16ByteCodec::new(byte_order));
    let mut replace_output = vec!['\0'; data.len()];
    let replace_result =
        replace.transcode_complete_into(data, &mut replace_output);

    let mut report = CharsetDecoder::with_policy(
        Utf16ByteCodec::new(byte_order),
        CharsetDecodePolicy::report(),
    );
    let mut report_output = vec!['\0'; data.len()];
    let report_result =
        report.transcode_complete_into(data, &mut report_output);

    if data.len().is_multiple_of(2) {
        let units = data
            .chunks_exact(2)
            .map(|bytes| match byte_order {
                ByteOrder::LittleEndian => {
                    u16::from_le_bytes([bytes[0], bytes[1]])
                }
                ByteOrder::BigEndian => {
                    u16::from_be_bytes([bytes[0], bytes[1]])
                }
                ByteOrder::NativeEndian => {
                    u16::from_ne_bytes([bytes[0], bytes[1]])
                }
            })
            .collect::<Vec<_>>();
        let decoded = core::char::decode_utf16(units).collect::<Vec<_>>();
        let malformed = decoded.iter().any(Result::is_err);
        let expected = decoded
            .into_iter()
            .map(|result| result.unwrap_or(char::REPLACEMENT_CHARACTER))
            .collect::<Vec<_>>();
        let replace_written =
            replace_result.expect("complete UTF-16 input must be replaceable");
        assert_eq!(expected, replace_output[..replace_written]);
        if malformed {
            assert!(report_result.is_err());
        } else {
            let report_written =
                report_result.expect("well-formed UTF-16 must report success");
            assert_eq!(expected, report_output[..report_written]);
        }
    } else {
        let mut units = data
            .chunks_exact(2)
            .map(|bytes| match byte_order {
                ByteOrder::LittleEndian => {
                    u16::from_le_bytes([bytes[0], bytes[1]])
                }
                ByteOrder::BigEndian => {
                    u16::from_be_bytes([bytes[0], bytes[1]])
                }
                ByteOrder::NativeEndian => {
                    u16::from_ne_bytes([bytes[0], bytes[1]])
                }
            })
            .collect::<Vec<_>>();
        // A trailing high surrogate and an odd byte form one maximal
        // incomplete tail. The decoder consumes that tail and emits a single
        // replacement, rather than reporting two independent malformed units.
        let trailing_high_surrogate = units
            .last()
            .is_some_and(|unit| (0xD800..=0xDBFF).contains(unit));
        if trailing_high_surrogate {
            units.pop();
        }
        let mut expected = core::char::decode_utf16(units)
            .map(|result| result.unwrap_or(char::REPLACEMENT_CHARACTER))
            .collect::<Vec<_>>();
        expected.push(char::REPLACEMENT_CHARACTER);
        let replace_written = replace_result
            .expect("odd UTF-16 byte tail at EOF must be replaceable");
        assert_eq!(expected, replace_output[..replace_written]);
        assert!(report_result.is_err());
    }
}
