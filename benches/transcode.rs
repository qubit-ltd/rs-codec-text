// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Throughput benchmarks for charset encode, decode, and conversion engines.

use std::hint::black_box;
use std::time::Duration;

use criterion::{
    BenchmarkGroup,
    Criterion,
    Throughput,
    criterion_group,
    criterion_main,
    measurement::WallTime,
};
use qubit_codec::Transcoder;
use qubit_codec_text::{
    CharsetCodec,
    CharsetConverter,
    CharsetDecoder,
    CharsetEncoder,
    Utf8Codec,
    Utf16U16Codec,
    Utf32U32Codec,
};

const FIXTURE_REPEAT: usize = 2_048;
const SAMPLE_SIZE: usize = 20;

fn fixture() -> String {
    "ASCII codec throughput 0123456789 — 中文字符 — Ελληνικά — 🦀🚀\n"
        .repeat(FIXTURE_REPEAT)
}

fn bench_encode<C>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    name: &str,
    codec: C,
    chars: &[char],
    logical_bytes: u64,
) where
    C: CharsetCodec,
    C::Unit: Clone + Default,
{
    let mut encoder = CharsetEncoder::new(codec);
    let capacity = encoder
        .max_total_output_len(chars.len())
        .expect("encode output bound should fit usize");
    let mut output = vec![C::Unit::default(); capacity];
    group.throughput(Throughput::Bytes(logical_bytes));
    group.bench_function(name, |bencher| {
        bencher.iter(|| {
            let written = encoder
                .transcode_complete_into(
                    black_box(chars),
                    output.as_mut_slice(),
                )
                .expect("valid fixture should encode");
            black_box((written, output[0..written].as_ptr()));
        });
    });
}

fn bench_decode<C>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    name: &str,
    codec: C,
    input: &[C::Unit],
    logical_bytes: u64,
) where
    C: CharsetCodec,
{
    let mut decoder = CharsetDecoder::new(codec);
    let capacity = decoder
        .max_total_output_len(input.len())
        .expect("decode output bound should fit usize");
    let mut output = vec!['\0'; capacity];
    group.throughput(Throughput::Bytes(logical_bytes));
    group.bench_function(name, |bencher| {
        bencher.iter(|| {
            let written = decoder
                .transcode_complete_into(
                    black_box(input),
                    output.as_mut_slice(),
                )
                .expect("valid fixture should decode");
            black_box((written, output[0..written].as_ptr()));
        });
    });
}

/// Decodes UTF-8 through deliberately short output windows.
fn decode_utf8_window(
    decoder: &mut CharsetDecoder<Utf8Codec>,
    input: &[u8],
    output: &mut [char],
) -> (u64, usize, usize) {
    decoder
        .reset(&mut [], 0)
        .expect("UTF-8 reset should be infallible");
    let mut input_index = 0;
    let mut checksum = 0_u64;
    while input_index < input.len() {
        let progress = decoder
            .transcode(input, input_index, output, 0)
            .expect("valid UTF-8 fixture should decode");
        input_index += progress.read();
        for &character in &output[..progress.written()] {
            checksum = checksum
                .rotate_left(5)
                .wrapping_add(u64::from(character as u32));
        }
        if progress.is_complete() {
            break;
        }
    }
    let finished = decoder.finish(&mut [], 0).expect("UTF-8 finish");
    (checksum, input_index, finished)
}

fn bench_convert<D, E>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    name: &str,
    decoder: D,
    encoder: E,
    input: &[D::Unit],
    logical_bytes: u64,
) where
    D: CharsetCodec,
    E: CharsetCodec,
    E::Unit: Clone + Default,
{
    let mut converter = CharsetConverter::from_codecs(decoder, encoder);
    let capacity = converter
        .max_total_output_len(input.len())
        .expect("convert output bound should fit usize");
    let mut output = vec![E::Unit::default(); capacity];
    group.throughput(Throughput::Bytes(logical_bytes));
    group.bench_function(name, |bencher| {
        bencher.iter(|| {
            let written = converter
                .transcode_complete_into(
                    black_box(input),
                    output.as_mut_slice(),
                )
                .expect("valid fixture should convert");
            black_box((written, output[0..written].as_ptr()));
        });
    });
}

fn bench_charset_transcode(criterion: &mut Criterion) {
    let text = fixture();
    let chars: Vec<char> = text.chars().collect();
    let utf8 = text.as_bytes().to_vec();
    let utf16: Vec<u16> = text.encode_utf16().collect();
    let utf32: Vec<u32> = chars.iter().copied().map(u32::from).collect();
    let logical_bytes = text.len() as u64;

    let mut encode = criterion.benchmark_group("charset_encode");
    encode.sample_size(SAMPLE_SIZE);
    encode.warm_up_time(Duration::from_secs(2));
    encode.measurement_time(Duration::from_secs(5));
    bench_encode(&mut encode, "utf8", Utf8Codec, &chars, logical_bytes);
    bench_encode(
        &mut encode,
        "utf16_u16",
        Utf16U16Codec,
        &chars,
        logical_bytes,
    );
    bench_encode(
        &mut encode,
        "utf32_u32",
        Utf32U32Codec,
        &chars,
        logical_bytes,
    );
    encode.finish();

    let mut decode = criterion.benchmark_group("charset_decode");
    decode.sample_size(SAMPLE_SIZE);
    decode.warm_up_time(Duration::from_secs(2));
    decode.measurement_time(Duration::from_secs(5));
    bench_decode(&mut decode, "utf8", Utf8Codec, &utf8, logical_bytes);
    bench_decode(
        &mut decode,
        "utf16_u16",
        Utf16U16Codec,
        &utf16,
        logical_bytes,
    );
    bench_decode(
        &mut decode,
        "utf32_u32",
        Utf32U32Codec,
        &utf32,
        logical_bytes,
    );
    for window in [7_usize, 31] {
        let name = format!("utf8_output_window_{window}");
        let mut validation_decoder = CharsetDecoder::new(Utf8Codec);
        let mut validation_output = vec!['\0'; window];
        let (_, consumed, finished) = decode_utf8_window(
            &mut validation_decoder,
            &utf8,
            validation_output.as_mut_slice(),
        );
        assert_eq!(utf8.len(), consumed);
        assert_eq!(0, finished);

        let mut decoder = CharsetDecoder::new(Utf8Codec);
        let mut output = vec!['\0'; window];
        decode.throughput(Throughput::Bytes(logical_bytes));
        decode.bench_function(name, |bencher| {
            bencher.iter(|| {
                black_box(decode_utf8_window(
                    &mut decoder,
                    black_box(&utf8),
                    black_box(output.as_mut_slice()),
                ))
            });
        });
    }
    decode.finish();

    let mut convert = criterion.benchmark_group("charset_convert");
    convert.sample_size(SAMPLE_SIZE);
    convert.warm_up_time(Duration::from_secs(2));
    convert.measurement_time(Duration::from_secs(5));
    bench_convert(
        &mut convert,
        "utf8_to_utf16",
        Utf8Codec,
        Utf16U16Codec,
        &utf8,
        logical_bytes,
    );
    bench_convert(
        &mut convert,
        "utf16_to_utf8",
        Utf16U16Codec,
        Utf8Codec,
        &utf16,
        logical_bytes,
    );
    bench_convert(
        &mut convert,
        "utf8_to_utf32",
        Utf8Codec,
        Utf32U32Codec,
        &utf8,
        logical_bytes,
    );
    bench_convert(
        &mut convert,
        "utf32_to_utf8",
        Utf32U32Codec,
        Utf8Codec,
        &utf32,
        logical_bytes,
    );
    convert.finish();
}

criterion_group!(benches, bench_charset_transcode);
criterion_main!(benches);
