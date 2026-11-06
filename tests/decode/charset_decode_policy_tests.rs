// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec_text::CharsetDecodePolicy;
use qubit_codec_text::MalformedAction;

#[test]
fn test_charset_decode_policy_constructors_preserve_action_and_replacement() {
    assert_eq!(
        MalformedAction::Replace,
        CharsetDecodePolicy::replace('!').malformed_action(),
        "replace() must select the Replace action"
    );
    assert_eq!(
        '!',
        CharsetDecodePolicy::replace('!').replacement(),
        "replace() must preserve its replacement character"
    );
    assert_eq!(
        MalformedAction::Ignore,
        CharsetDecodePolicy::ignore().malformed_action(),
        "ignore() must select the Ignore action"
    );
    assert_eq!(
        MalformedAction::Report,
        CharsetDecodePolicy::report().malformed_action(),
        "report() must select the Report action"
    );
    assert_eq!(
        CharsetDecodePolicy::default(),
        CharsetDecodePolicy::replace('\u{fffd}'),
        "the default policy must replace malformed input with U+FFFD"
    );
}
