// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec_text::Charset;
use qubit_codec_text::CharsetRegistrationErrorKind;

#[test]
fn test_charset_registration_error_kind_preserves_conflicting_charset() {
    let candidate =
        Charset::new_static("x-kind-conflict", "Kind Conflict", &["utf8"]);
    let error = Charset::register(candidate)
        .expect_err("built-in aliases cannot be registered twice");

    assert_eq!(
        CharsetRegistrationErrorKind::ConflictingLabel {
            existing: Charset::UTF_8,
        },
        error.kind(),
    );
}
