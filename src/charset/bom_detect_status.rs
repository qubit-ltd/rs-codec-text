// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::UnicodeBom;

/// Result of checking whether the available stream prefix identifies a BOM.
///
/// `Pending` means more bytes may change the result, `Match` carries the
/// selected BOM, and `None` means no supported BOM can match the prefix.
/// Callers should retain the bytes while detection is pending.
///
/// # Examples
///
/// ```rust
/// use qubit_codec_text::{BomDetectStatus, UnicodeBom};
///
/// let status = UnicodeBom::detect_progress(&[0xef], false);
/// assert_eq!(BomDetectStatus::Pending, status);
///
/// let status = UnicodeBom::detect_progress(&[0xef, 0xbb, 0xbf], true);
/// assert_eq!(BomDetectStatus::Match(UnicodeBom::Utf8), status);
/// ```
#[must_use]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BomDetectStatus {
    /// The supplied bytes are still a possible BOM prefix.
    Pending,

    /// A supported Unicode BOM was detected.
    Match(UnicodeBom),

    /// No supported Unicode BOM can match the supplied bytes.
    None,
}
