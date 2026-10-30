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
/// Returns a string after trimming leading and trailing ASCII whitespace,
/// removing `-` and `_` separators, and lowercasing ASCII letters. Non-ASCII
/// characters are preserved. This function does not validate whether the
/// label is supported or valid, and its output is not guaranteed to contain
/// only ASCII.
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
/// Returns a string after trimming leading and trailing ASCII whitespace and
/// lowercasing ASCII letters. Non-ASCII characters, punctuation, and
/// separators are preserved. This is WHATWG-style preprocessing, not
/// validation against the Encoding Standard label table; its output is not
/// guaranteed to contain only ASCII. Unlike
/// [`normalize_label_loose`], this function keeps `-` and `_` intact.
#[must_use]
pub fn normalize_label_whatwg(label: &str) -> String {
    label.trim_ascii().to_ascii_lowercase()
}
