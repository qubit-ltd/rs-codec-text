// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use super::charset::Charset;

/// Categorizes why a charset descriptor cannot be registered.
///
/// Invalid labels normalize to an empty lookup key. Conflicting labels are
/// already owned by a built-in or registered charset.
///
/// # Examples
///
/// ```rust
/// use qubit_codec_text::Charset;
/// use qubit_codec_text::CharsetRegistrationErrorKind;
///
/// let error = Charset::try_new("", "Custom", &[])
///     .expect_err("an empty identifier cannot be registered");
/// assert_eq!(error.kind(), CharsetRegistrationErrorKind::InvalidLabel);
/// ```
#[must_use]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CharsetRegistrationErrorKind {
    /// A descriptor label normalizes to an empty lookup key.
    InvalidLabel,

    /// A descriptor label conflicts with a built-in or registered charset.
    ConflictingLabel {
        /// Charset already owning the conflicting label.
        existing: Charset,
    },
}
