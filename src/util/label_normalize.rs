// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Charset label normalization helpers.

/// Normalizes a charset label for this crate's permissive label matching.
///
/// # Parameters
///
/// - `label`: Label supplied by a caller.
///
/// # Returns
///
/// Returns a lowercase ASCII string after trimming ASCII whitespace and
/// removing `-` and `_` separators. Non-ASCII characters are preserved except
/// for ASCII case conversion.
#[must_use]
pub fn normalize_label_loose(label: &str) -> String {
    label
        .trim_ascii()
        .chars()
        .filter_map(|ch| match ch {
            '-' | '_' => None,
            _ => Some(ch.to_ascii_lowercase()),
        })
        .collect()
}

/// Normalizes a charset label using WHATWG-style preprocessing.
///
/// # Parameters
///
/// - `label`: Label supplied by a caller.
///
/// # Returns
///
/// Returns a lowercase ASCII string after trimming ASCII whitespace. Unlike
/// [`normalize_label_loose`], this function keeps punctuation and separators
/// intact, matching the preprocessing step used before consulting the WHATWG
/// Encoding Standard label table.
#[must_use]
pub fn normalize_label_whatwg(label: &str) -> String {
    label.trim_ascii().to_ascii_lowercase()
}
