// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::Codec;
use qubit_codec::DecodeFailure;
use qubit_codec_text::Charset;
use qubit_codec_text::CharsetCodec;
use qubit_codec_text::CharsetDecodeError;
use qubit_codec_text::CharsetDecodeErrorKind;
use qubit_codec_text::CharsetEncodeResult;
use qubit_codec_text::Utf16;
use qubit_codec_text::Utf16U16Codec;

type DecodedCharResult = Result<(char, core::num::NonZeroUsize), DecodeFailure<CharsetDecodeError>>;
type DecodeFn = unsafe fn(&mut Utf16U16Codec, &[u16], usize) -> DecodedCharResult;
type EncodeFn = unsafe fn(&mut Utf16U16Codec, &char, &mut [u16], usize) -> CharsetEncodeResult<usize>;

#[test]
fn test_utf16_u16_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf16U16Codec;

    assert_eq!(Charset::UTF_16, <Utf16U16Codec as CharsetCodec>::charset(&codec));
    assert_eq!(1, <Utf16U16Codec as Codec>::MIN_UNITS_PER_VALUE);
    assert_eq!(
        Utf16::MAX_UNITS_PER_CHAR,
        <Utf16U16Codec as Codec>::MAX_ENCODE_UNITS_PER_VALUE,
    );
    assert!(codec.can_encode_value(&'A'));
    assert_eq!(1, codec.encode_len(&'A'));

    assert_eq!(Charset::UTF_16, codec.charset());
}

#[test]
fn test_utf16_u16_codec_encodes_and_decodes_pairs() {
    let mut codec = Utf16U16Codec;
    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];

    assert_eq!(2, unsafe {
        codec.encode(&'😀', &mut output, 0).expect("encode pair")
    });
    let (decoded, consumed) = unsafe { codec.decode(&output, 0) }.expect("decode pair");
    assert_eq!('😀', decoded);
    assert_eq!(2, consumed.get());
}

#[test]
fn test_utf16_u16_codec_decodes_bmp_and_reports_closed_tail_or_malformed_units() {
    let mut codec = Utf16U16Codec;

    let (decoded, consumed) = unsafe { codec.decode(&['A' as u16], 0) }.expect("BMP scalar");
    assert_eq!('A', decoded);
    assert_eq!(1, consumed.get());

    let error = unsafe { codec.decode(&[0xd83d], 0) }.expect_err("dangling high surrogate is incomplete");
    assert_eq!(2, super::incomplete_required(error));

    let error = unsafe { codec.decode(&[0xd83d, 'A' as u16], 0) }
        .expect_err("high surrogate followed by non-low-surrogate should fail");
    let error = super::invalid_source(error);
    assert_eq!(CharsetDecodeErrorKind::malformed('A' as u32), error.kind());
    assert_eq!(1, error.index());

    let error = unsafe { codec.decode(&[0xde00], 0) }.expect_err("isolated low surrogate should fail");
    let error = super::invalid_source(error);
    assert_eq!(CharsetDecodeErrorKind::malformed(0xde00), error.kind());
    assert_eq!(0, error.index());
}

#[test]
fn test_utf16_u16_codec_encodes_bmp_and_supplementary_scalars() {
    let mut codec = Utf16U16Codec;
    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];

    assert_eq!(1, unsafe { codec.encode(&'A', &mut output, 0).expect("BMP") });
    assert_eq!(2, unsafe {
        codec.encode(&'😀', &mut output, 0).expect("surrogate pair")
    });

    assert!(codec.can_encode_value(&'😀'));
}

#[test]
fn test_utf16_u16_codec_direct_function_items_cover_trait_methods() {
    let mut codec = Utf16U16Codec;
    let inherent_charset: fn(Utf16U16Codec) -> Charset = Utf16U16Codec::charset;
    let trait_charset: fn(&Utf16U16Codec) -> Charset = <Utf16U16Codec as CharsetCodec>::charset;
    let min_units = <Utf16U16Codec as Codec>::MIN_UNITS_PER_VALUE;
    let max_units = <Utf16U16Codec as Codec>::MAX_ENCODE_UNITS_PER_VALUE;
    let encode_len: fn(&Utf16U16Codec, &char) -> usize = <Utf16U16Codec as Codec>::encode_len;
    let decode: DecodeFn = <Utf16U16Codec as Codec>::decode;
    let encode: EncodeFn = <Utf16U16Codec as Codec>::encode;

    assert_eq!(Charset::UTF_16, inherent_charset(codec));
    assert_eq!(Charset::UTF_16, trait_charset(&codec));
    assert_eq!(1, min_units);
    assert_eq!(Utf16::MAX_UNITS_PER_CHAR, max_units);
    assert_eq!(2, encode_len(&codec, &'😀'));

    let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];
    assert_eq!(
        2,
        unsafe { encode(&mut codec, &'😀', &mut output, 0) }.expect("encode pair")
    );
    let (decoded, consumed) = unsafe { decode(&mut codec, &output, 0) }.expect("decode pair");
    assert_eq!(('😀', 2), (decoded, consumed.get()));
}

#[test]
fn test_utf16_u16_codec_reports_consumption_at_nonzero_input_index() {
    let mut codec = Utf16U16Codec;
    let input = [0x0058, 0xd800, 0x0041];
    let start = 1;
    // SAFETY: Two u16 units are readable at start; the minimum is one.
    let failure = unsafe { codec.decode(&input, start) }.expect_err("high surrogate before ASCII");
    let DecodeFailure::Invalid { source, consumed, .. } = failure else {
        panic!("expected malformed UTF-16 pair");
    };
    let width = consumed.expect("known malformed prefix width").get();
    assert_eq!(Charset::UTF_16, source.charset());
    assert_eq!(
        CharsetDecodeErrorKind::MalformedSequence { value: Some(0x41) },
        source.kind()
    );
    assert_eq!(2, source.index());
    assert_eq!(1, width);
    assert_eq!(Some(width), source.consumed().map(|value| value.get()));
    // SAFETY: The recovery position has one readable non-surrogate u16 unit.
    let (ch, used) = unsafe { codec.decode(&input, start + width) }.expect("following ASCII is preserved");
    assert_eq!(('A', 1), (ch, used.get()));
}
