// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::TranscodeEncodeError;
use qubit_codec_text::AsciiCodec;
use qubit_codec_text::CharsetEncodeErrorKind;
use qubit_codec_text::CharsetEncodePolicy;
use qubit_codec_text::CharsetEncoder;

#[test]
fn test_charset_encode_hooks_apply_the_configured_unmappable_action() {
    let input = ['A', '中', 'B'];

    let mut replace = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::replace('?'))
        .expect("replacement character should be encodable");
    let mut replace_output = [0_u8; 3];
    let written = replace
        .transcode_complete_into(&input, &mut replace_output)
        .expect("replacement policy should encode unmappable input");
    assert_eq!(b"A?B", &replace_output[..written]);

    let mut ignore = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::ignore())
        .expect("ignore policy should be constructible");
    let mut ignore_output = [0_u8; 3];
    let written = ignore
        .transcode_complete_into(&input, &mut ignore_output)
        .expect("ignore policy should skip unmappable input");
    assert_eq!(b"AB", &ignore_output[..written]);

    let mut report = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::report())
        .expect("report policy should be constructible");
    let mut report_output = [0_u8; 3];
    let error = report
        .transcode_complete_into(&input, &mut report_output)
        .expect_err("report policy should reject unmappable input");
    let TranscodeEncodeError::Domain(error) = error else {
        panic!("report policy should return a charset-domain error");
    };
    assert!(matches!(
        error.into_source().kind(),
        CharsetEncodeErrorKind::UnmappableCharacter { .. }
    ));
    assert_eq!(1, error.into_source().index());
}
