// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::ByteOrder;

use crate::Unicode;
use crate::UnicodeBom;

/// Namespace for UTF-32 constants and code-unit classification helpers.
///
/// # Examples
///
/// ```
/// use qubit_codec_text::Utf32;
///
/// assert!(Utf32::is_valid_unit('🦀' as u32));
/// assert_eq!(Utf32::unit_len('🦀'), 1);
/// ```
pub enum Utf32 {}

impl Utf32 {
    /// Maximum number of UTF-32 code units needed for one Unicode scalar value.
    pub const MAX_UNITS_PER_CHAR: usize = 1;

    /// Maximum number of serialized UTF-32 bytes needed for one Unicode scalar
    /// value.
    pub const MAX_BYTES_PER_CHAR: usize = 4;

    /// Tests whether a UTF-32 unit is a valid Unicode scalar value.
    ///
    /// # Parameters
    ///
    /// - `unit`: The UTF-32 unit to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `unit` is a valid Unicode scalar value.
    #[must_use]
    #[inline]
    pub const fn is_valid_unit(unit: u32) -> bool {
        Unicode::is_scalar_value(unit)
    }

    /// Returns the UTF-32 unit count needed for a character.
    ///
    /// # Parameters
    ///
    /// - `_ch`: The character to size.
    ///
    /// # Returns
    ///
    /// Always returns `1`.
    #[must_use]
    #[inline]
    pub const fn unit_len(_ch: char) -> usize {
        1
    }

    /// Detects a UTF-32 BOM and returns its byte order.
    ///
    /// # Parameters
    ///
    /// - `bytes`: The byte buffer to inspect.
    ///
    /// # Returns
    ///
    /// Returns `Some(ByteOrder)` for UTF-32 BOM prefixes, or `None` otherwise.
    pub fn detect_bom(bytes: &[u8]) -> Option<ByteOrder> {
        match UnicodeBom::detect(bytes) {
            Some(UnicodeBom::Utf32BigEndian) => Some(ByteOrder::BigEndian),
            Some(UnicodeBom::Utf32LittleEndian) => Some(ByteOrder::LittleEndian),
            _ => None,
        }
    }
}
