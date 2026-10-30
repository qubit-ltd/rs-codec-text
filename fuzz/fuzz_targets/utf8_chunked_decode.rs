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
const SENTINEL: char = '\u{10ffff}';

/// Reads one bounded scheduling byte, independently of the payload.
fn next_control(controls: &[u8], cursor: &mut usize) -> usize {
    let value = if controls.is_empty() {
        *cursor % 256
    } else {
        usize::from(controls[*cursor % controls.len()])
    };
    *cursor += 1;
    value
}

/// Drains one available input prefix or EOF with bounded output windows.
///
/// `counters` holds the total consumed bytes and whether backpressure occurred.
fn drain(
    decoder: &mut CharsetDecoder<Utf8Codec>,
    pending: &mut Vec<u8>,
    eof: bool,
    controls: &[u8],
    window_cursor: &mut usize,
    result: &mut Vec<char>,
    counters: &mut (usize, bool),
) {
    let mut force_capacity = Some(0);
    let mut remaining_calls = pending.len() * 4 + 16;
    loop {
        assert!(remaining_calls > 0, "UTF-8 drain exceeded its progress budget");
        remaining_calls -= 1;
        let capacity = force_capacity
            .take()
            .unwrap_or_else(|| next_control(controls, window_cursor) % 4);
        let mut storage = [SENTINEL; 5];
        let input_len = pending.len();
        let output = &mut storage[..capacity + 1];
        let progress = if eof {
            decoder.transcode_eof(pending, 0, output, 1)
        } else {
            decoder.transcode(pending, 0, output, 1)
        }
        .expect("Replace must accept arbitrary UTF-8 bytes");
        assert!(progress.read() <= input_len, "read exceeded visible input");
        assert!(progress.written() <= capacity, "written exceeded output window");
        assert_eq!(SENTINEL, storage[0], "output prefix was overwritten");
        assert!(
            storage[1 + progress.written()..].iter().all(|value| *value == SENTINEL),
            "output beyond reported progress was overwritten",
        );
        result.extend_from_slice(&storage[1..1 + progress.written()]);
        counters.0 += progress.read();
        pending.drain(..progress.read());
        match progress.status() {
            TranscodeStatus::Complete => {
                assert!(pending.is_empty(), "Complete left unread input");
                return;
            }
            TranscodeStatus::NeedInput { required } => {
                assert!(!eof, "EOF must resolve an incomplete UTF-8 tail");
                assert!(required.get() > pending.len(), "NeedInput must require more input");
                return;
            }
            TranscodeStatus::NeedOutput { required } => {
                counters.1 = true;
                assert_eq!(1, required.get(), "a decoded char needs one output slot");
                if progress.read() == 0 && progress.written() == 0 {
                    assert_eq!(0, capacity, "a nonempty char window must make progress");
                    force_capacity = Some(1);
                }
            }
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let data = &data[..data.len().min(MAX_FUZZ_INPUT_LEN)];
    let (controls, payload) = match data.split_first() {
        None => (&[][..], &[][..]),
        Some((&first, remaining)) => {
            let control_len = (usize::from(first) % 17).min(remaining.len());
            remaining.split_at(control_len)
        }
    };
    let expected: Vec<char> = String::from_utf8_lossy(payload).chars().collect();
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    assert_eq!(0, decoder.reset(&mut [], 0).expect("reset decoder"));
    let mut pending = Vec::new();
    let mut result = Vec::new();
    let mut offset = 0;
    let mut chunk_cursor = 0;
    let mut window_cursor = 0;
    let mut counters = (0, false);
    while offset < payload.len() {
        let chunk_len = (next_control(controls, &mut chunk_cursor) % 7 + 1).min(payload.len() - offset);
        pending.extend_from_slice(&payload[offset..offset + chunk_len]);
        offset += chunk_len;
        drain(
            &mut decoder,
            &mut pending,
            false,
            controls,
            &mut window_cursor,
            &mut result,
            &mut counters,
        );
    }
    drain(
        &mut decoder,
        &mut pending,
        true,
        controls,
        &mut window_cursor,
        &mut result,
        &mut counters,
    );
    assert!(pending.is_empty(), "EOF left unread bytes");
    assert_eq!(payload.len(), counters.0, "input was skipped or replayed");
    assert_eq!(0, decoder.finish(&mut [], 0).expect("finish decoder"));
    assert_eq!(expected, result, "chunking/backpressure changed UTF-8 recovery");
    assert!(
        payload.is_empty() || counters.1,
        "nonempty input never exercised backpressure"
    );
});
