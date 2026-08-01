// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec_text::{CharsetDecodePolicy, MalformedAction};

#[test]
fn test_charset_decode_policy_constructors_preserve_action_and_replacement() {
    assert_eq!(
        MalformedAction::Replace,
        CharsetDecodePolicy::replace('!').malformed_action()
    );
    assert_eq!('!', CharsetDecodePolicy::replace('!').replacement());
    assert_eq!(
        MalformedAction::Ignore,
        CharsetDecodePolicy::ignore().malformed_action()
    );
    assert_eq!(
        MalformedAction::Report,
        CharsetDecodePolicy::report().malformed_action()
    );
    assert_eq!(
        CharsetDecodePolicy::default(),
        CharsetDecodePolicy::replace('\u{fffd}')
    );
}
