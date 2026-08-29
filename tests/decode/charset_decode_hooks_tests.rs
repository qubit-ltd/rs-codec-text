// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::TranscodeDecodeError;
use qubit_codec_text::CharsetDecodeErrorKind;
use qubit_codec_text::CharsetDecodePolicy;
use qubit_codec_text::CharsetDecoder;
use qubit_codec_text::Utf8Codec;

#[test]
fn test_charset_decode_hooks_apply_the_configured_malformed_action() {
    let input = [b'A', 0x80, b'B'];

    let mut replace = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::replace('!'));
    let mut replace_output = ['\0'; 3];
    let written = replace
        .transcode_complete_into(&input, &mut replace_output)
        .expect("replacement policy should decode malformed input");
    assert_eq!(['A', '!', 'B'], replace_output[..written]);

    let mut ignore = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::ignore());
    let mut ignore_output = ['\0'; 3];
    let written = ignore
        .transcode_complete_into(&input, &mut ignore_output)
        .expect("ignore policy should skip malformed input");
    assert_eq!(['A', 'B'], ignore_output[..written]);

    let mut report = CharsetDecoder::with_policy(Utf8Codec, CharsetDecodePolicy::report());
    let mut report_output = ['\0'; 3];
    let error = report
        .transcode_complete_into(&input, &mut report_output)
        .expect_err("report policy should reject malformed input");
    let TranscodeDecodeError::Domain(error) = error else {
        panic!("report policy should return a charset-domain error");
    };
    assert!(matches!(
        error.into_source().kind(),
        CharsetDecodeErrorKind::MalformedSequence { .. }
    ));
    assert_eq!(1, error.into_source().index());
}
