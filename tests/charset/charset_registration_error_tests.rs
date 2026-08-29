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
fn test_charset_registration_error_exposes_invalid_label_context() {
    let error = Charset::try_new("-_", "Invalid", &[]).expect_err("empty normalized labels must be rejected");

    assert_eq!("-_", error.label());
    assert_eq!(CharsetRegistrationErrorKind::InvalidLabel, error.kind());
    assert_eq!(None, error.existing());
    assert_eq!(Charset::new_static("-_", "Invalid", &[]), error.candidate());
    assert_eq!("charset label \"-_\" for Invalid is invalid", error.to_string());
}
