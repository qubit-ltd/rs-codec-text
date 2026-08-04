// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec_text::UnmappableAction;

#[test]
fn test_unmappable_action_default_replaces() {
    assert_eq!(UnmappableAction::Replace, UnmappableAction::default());
}
