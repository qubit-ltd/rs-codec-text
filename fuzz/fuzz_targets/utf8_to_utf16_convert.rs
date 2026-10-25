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
use qubit_codec_text::CharsetConverter;
use qubit_codec_text::Utf8Codec;
use qubit_codec_text::Utf16U16Codec;

const MAX_FUZZ_INPUT_LEN: usize = 4_096;

fn run_chunked_backpressure(input: &[u8], controls: &[u8]) -> Vec<u16> {
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    assert_eq!(0, converter.reset(&mut [], 0).expect("reset"));
    let mut control_cursor = 0_usize;
    let mut next_control = || {
        let value = if controls.is_empty() {
            0
        } else {
            controls[control_cursor % controls.len()] as usize
        };
        control_cursor = control_cursor.wrapping_add(1);
        value
    };
    let mut pending = Vec::new();
    let mut result = Vec::new();
    let mut added = 0;
    let mut consumed = 0;
    let mut feed = true;
    let mut eof = false;
    let mut minimum_window = 0;
    let mut steps = 0;
    let max_steps = input.len().saturating_mul(64).saturating_add(128);
    loop {
        steps += 1;
        assert!(steps <= max_steps, "stream did not converge");
        if feed {
            if added == input.len() {
                eof = true;
            } else {
                let count = (next_control() % 7 + 1).min(input.len() - added);
                pending.extend_from_slice(&input[added..added + count]);
                added += count;
            }
            feed = false;
        }
        let window = (next_control() % 3).max(minimum_window);
        minimum_window = 0;
        let mut output = vec![0_u16; window];
        let progress = if eof {
            converter.transcode_eof(&pending, 0, &mut output, 0)
        } else {
            converter.transcode(&pending, 0, &mut output, 0)
        }
        .expect("Replace accepts arbitrary UTF-8");
        assert!(progress.read() <= pending.len());
        assert!(progress.written() <= output.len());
        consumed += progress.read();
        result.extend_from_slice(&output[..progress.written()]);
        pending.drain(..progress.read());
        match progress.status() {
            TranscodeStatus::Complete => {
                assert!(pending.is_empty());
                if eof {
                    break;
                }
                feed = true;
            }
            TranscodeStatus::NeedInput { .. } => {
                assert!(!eof, "EOF must resolve incomplete input");
                feed = true;
            }
            TranscodeStatus::NeedOutput { required } => {
                if progress.read() == 0 && progress.written() == 0 {
                    minimum_window = required.get();
                }
            }
        }
    }
    assert_eq!(input.len(), added);
    assert_eq!(input.len(), consumed);
    assert!(pending.is_empty());
    assert_eq!(0, converter.finish(&mut [], 0).expect("finish"));
    result
}

fuzz_target!(|data: &[u8]| {
    let data = &data[..data.len().min(MAX_FUZZ_INPUT_LEN)];
    let expected = String::from_utf8_lossy(data).encode_utf16().collect::<Vec<_>>();
    let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
    let mut output = vec![0_u16; data.len().saturating_mul(4).saturating_add(4)];
    let written = converter
        .transcode_complete_into(data, &mut output)
        .expect("replacement converter must accept complete UTF-8 input");
    assert_eq!(expected, output[..written]);
    let streamed = run_chunked_backpressure(data, data);
    assert_eq!(expected, streamed);
});
