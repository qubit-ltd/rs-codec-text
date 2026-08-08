// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::ByteOrder;
use qubit_codec::Codec;
use qubit_codec::DecodeFailure;
use qubit_codec_text::Charset;
use qubit_codec_text::CharsetCodec;
use qubit_codec_text::CharsetDecodeError;
use qubit_codec_text::CharsetDecodeErrorKind;
use qubit_codec_text::CharsetEncodeResult;
use qubit_codec_text::Utf16;
use qubit_codec_text::Utf16ByteCodec;

type DecodedCharResult =
    Result<(char, core::num::NonZeroUsize), DecodeFailure<CharsetDecodeError>>;
type DecodeFn =
    unsafe fn(&mut Utf16ByteCodec, &[u8], usize) -> DecodedCharResult;
type EncodeFn = unsafe fn(
    &mut Utf16ByteCodec,
    &char,
    &mut [u8],
    usize,
) -> CharsetEncodeResult<usize>;

#[test]
fn test_utf16_byte_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);

    assert_eq!(
        Charset::UTF_16LE,
        <Utf16ByteCodec as CharsetCodec>::charset(&codec)
    );
    assert_eq!(2, <Utf16ByteCodec as Codec>::MIN_UNITS_PER_VALUE,);
    assert_eq!(
        Utf16::MAX_BYTES_PER_CHAR,
        <Utf16ByteCodec as Codec>::MAX_ENCODE_UNITS_PER_VALUE,
    );
    assert!(codec.can_encode_value(&'A'));
    assert_eq!(2, codec.encode_len(&'A'));

    assert_eq!(ByteOrder::LittleEndian, codec.byte_order());
    assert_eq!(Charset::UTF_16LE, codec.charset());
}

#[test]
fn test_utf16_byte_codec_encodes_and_decodes_bytes() {
    let mut codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];

    assert_eq!(4, unsafe {
        codec
            .encode(&'😀', &mut output, 0)
            .expect("encode pair bytes")
    });
    let (decoded, consumed) =
        unsafe { codec.decode(&output, 0) }.expect("decode pair bytes");
    assert_eq!('😀', decoded);
    assert_eq!(4, consumed.get());
}

#[test]
fn test_utf16_byte_codec_decodes_bmp_and_reports_closed_tail_or_malformed_bytes()
 {
    let mut codec = Utf16ByteCodec::new(ByteOrder::BigEndian);

    let (decoded, consumed) =
        unsafe { codec.decode(&[0x00, 0x41], 0) }.expect("BMP bytes");
    assert_eq!('A', decoded);
    assert_eq!(2, consumed.get());

    let error = unsafe { codec.decode(&[0xd8, 0x3d], 0) }
        .expect_err("partial surrogate pair is incomplete");
    assert_eq!(4, super::incomplete_required(error));

    let error = unsafe { codec.decode(&[0xd8, 0x3d, 0x00, 0x41], 0) }
        .expect_err("high surrogate followed by BMP unit should fail");
    let error = super::invalid_source(error);
    assert_eq!(CharsetDecodeErrorKind::malformed(0x0041), error.kind());
    assert_eq!(2, error.index());

    let error = unsafe { codec.decode(&[0xde, 0x00], 0) }
        .expect_err("isolated low surrogate should fail");
    let error = super::invalid_source(error);
    assert_eq!(CharsetDecodeErrorKind::malformed(0xde00), error.kind());
    assert_eq!(0, error.index());
}

#[test]
fn test_utf16_byte_codec_encodes_bmp_and_supplementary_scalars() {
    let mut codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];

    assert_eq!(2, unsafe {
        codec
            .encode(&'A', &mut output, 0)
            .expect("BMP byte encoding")
    });
    assert_eq!(4, unsafe {
        codec
            .encode(&'😀', &mut output, 0)
            .expect("surrogate pair bytes")
    });

    assert!(codec.can_encode_value(&'😀'));
}

#[test]
fn test_utf16_byte_codec_direct_function_items_cover_trait_methods() {
    let mut codec = Utf16ByteCodec::new(ByteOrder::BigEndian);
    let new_fn: fn(ByteOrder) -> Utf16ByteCodec = Utf16ByteCodec::new;
    let byte_order: fn(Utf16ByteCodec) -> ByteOrder =
        Utf16ByteCodec::byte_order;
    let inherent_charset: fn(Utf16ByteCodec) -> Charset =
        std::hint::black_box(Utf16ByteCodec::charset);
    let trait_charset: fn(&Utf16ByteCodec) -> Charset =
        <Utf16ByteCodec as CharsetCodec>::charset;
    let min_units = <Utf16ByteCodec as Codec>::MIN_UNITS_PER_VALUE;
    let max_units = <Utf16ByteCodec as Codec>::MAX_ENCODE_UNITS_PER_VALUE;
    let encode_len: fn(&Utf16ByteCodec, &char) -> usize =
        <Utf16ByteCodec as Codec>::encode_len;
    let decode: DecodeFn = <Utf16ByteCodec as Codec>::decode;
    let encode: EncodeFn =
        std::hint::black_box(<Utf16ByteCodec as Codec>::encode);

    assert_eq!(
        ByteOrder::LittleEndian,
        byte_order(new_fn(ByteOrder::LittleEndian))
    );
    assert_eq!(Charset::UTF_16BE, inherent_charset(codec));
    assert_eq!(Charset::UTF_16BE, trait_charset(&codec));
    assert_eq!(2, min_units);
    assert_eq!(Utf16::MAX_BYTES_PER_CHAR, max_units);
    assert_eq!(4, encode_len(&codec, &'😀'));

    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];
    assert_eq!(
        4,
        unsafe { encode(&mut codec, &'😀', &mut output, 0) }
            .expect("encode pair bytes")
    );
    let (decoded, consumed) =
        unsafe { decode(&mut codec, &output, 0) }.expect("decode pair bytes");
    assert_eq!(('😀', 4), (decoded, consumed.get()));
}
