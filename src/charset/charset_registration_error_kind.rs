// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use super::charset::Charset;

/// Categorizes why a charset cannot be registered.
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
