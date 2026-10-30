// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec_text::normalize_label_loose;
use qubit_codec_text::normalize_label_whatwg;

#[test]
fn test_label_normalizers_apply_their_respective_separator_rules() {
    assert_eq!("utf8", normalize_label_loose("  UTF-_8  "));
    assert_eq!("utf-_8", normalize_label_whatwg("  UTF-_8  "));
    assert_eq!("É", normalize_label_loose("É"));
}

#[test]
fn test_label_normalizers_preserve_non_ascii_without_validating_labels() {
    let input = " \tÉ-UTF_8\r\n";
    assert_eq!("Éutf8", normalize_label_loose(input));
    assert_eq!("É-utf_8", normalize_label_whatwg(input));
}
