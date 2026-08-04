// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec_text::{
    CharsetEncodePolicy,
    UnmappableAction,
};

#[test]
fn test_charset_encode_policy_constructors_preserve_action_and_replacement() {
    assert_eq!(
        UnmappableAction::Replace,
        CharsetEncodePolicy::replace('!').unmappable_action()
    );
    assert_eq!('!', CharsetEncodePolicy::replace('!').replacement());
    assert_eq!(
        UnmappableAction::Ignore,
        CharsetEncodePolicy::ignore().unmappable_action()
    );
    assert_eq!(
        UnmappableAction::Report,
        CharsetEncodePolicy::report().unmappable_action()
    );
    assert_eq!(
        CharsetEncodePolicy::default(),
        CharsetEncodePolicy::replace('\u{fffd}')
    );
}
