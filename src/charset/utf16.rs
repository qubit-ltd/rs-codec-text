// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::{
    ByteOrder,
    Unicode,
    UnicodeBom,
};

/// Namespace for UTF-16 constants and code-unit classification helpers.
pub enum Utf16 {}

impl Utf16 {
    /// Maximum number of UTF-16 code units needed for one Unicode scalar value.
    pub const MAX_UNITS_PER_CHAR: usize = 2;

    /// Maximum number of serialized UTF-16 bytes needed for one Unicode scalar
    /// value.
    pub const MAX_BYTES_PER_CHAR: usize = 4;

    /// Tests whether a UTF-16 unit is a high surrogate.
    ///
    /// # Parameters
    ///
    /// - `unit`: The UTF-16 unit to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `unit` is in `0xD800..=0xDBFF`.
    #[inline(always)]
    pub const fn is_high_surrogate(unit: u16) -> bool {
        (unit as u32) >= Unicode::HIGH_SURROGATE_MIN
            && (unit as u32) <= Unicode::HIGH_SURROGATE_MAX
    }

    /// Tests whether a UTF-16 unit is a low surrogate.
    ///
    /// # Parameters
    ///
    /// - `unit`: The UTF-16 unit to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `unit` is in `0xDC00..=0xDFFF`.
    #[inline(always)]
    pub const fn is_low_surrogate(unit: u16) -> bool {
        (unit as u32) >= Unicode::LOW_SURROGATE_MIN
            && (unit as u32) <= Unicode::LOW_SURROGATE_MAX
    }

    /// Tests whether a UTF-16 unit is any surrogate.
    ///
    /// # Parameters
    ///
    /// - `unit`: The UTF-16 unit to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `unit` is in `0xD800..=0xDFFF`.
    #[inline(always)]
    pub const fn is_surrogate(unit: u16) -> bool {
        (unit as u32) >= Unicode::SURROGATE_MIN
            && (unit as u32) <= Unicode::SURROGATE_MAX
    }

    /// Tests whether a UTF-16 unit encodes a scalar value by itself.
    ///
    /// # Parameters
    ///
    /// - `unit`: The UTF-16 unit to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for non-surrogate units.
    #[inline(always)]
    pub const fn is_single_unit(unit: u16) -> bool {
        !Self::is_surrogate(unit)
    }

    /// Tests whether two UTF-16 units form a surrogate pair.
    ///
    /// # Parameters
    ///
    /// - `high`: The candidate high surrogate.
    /// - `low`: The candidate low surrogate.
    ///
    /// # Returns
    ///
    /// Returns `true` if `high` is a high surrogate and `low` is a low
    /// surrogate.
    #[inline(always)]
    pub const fn is_surrogate_pair(high: u16, low: u16) -> bool {
        Self::is_high_surrogate(high) && Self::is_low_surrogate(low)
    }

    /// Returns the UTF-16 unit count needed for `ch`.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to size.
    ///
    /// # Returns
    ///
    /// Returns `1` for BMP scalar values and `2` for supplementary scalar
    /// values.
    #[inline(always)]
    pub const fn unit_len(ch: char) -> usize {
        if (ch as u32) >= Unicode::SUPPLEMENTARY_MIN {
            2
        } else {
            1
        }
    }

    /// Returns the UTF-16 unit count needed for a raw code point.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The raw code point to size.
    ///
    /// # Returns
    ///
    /// Returns `Some(1)` or `Some(2)` for scalar values and `None` otherwise.
    #[inline(always)]
    pub const fn unit_len_code_point(code_point: u32) -> Option<usize> {
        if !Unicode::is_scalar_value(code_point) {
            None
        } else if code_point >= Unicode::SUPPLEMENTARY_MIN {
            Some(2)
        } else {
            Some(1)
        }
    }

    /// Composes a surrogate pair into a Unicode code point.
    ///
    /// # Parameters
    ///
    /// - `high`: The high surrogate.
    /// - `low`: The low surrogate.
    ///
    /// # Returns
    ///
    /// Returns `Some(code_point)` when the pair is valid, or `None` otherwise.
    #[inline(always)]
    pub const fn compose_pair(high: u16, low: u16) -> Option<u32> {
        if Self::is_surrogate_pair(high, low) {
            let high_payload = (high as u32) - Unicode::HIGH_SURROGATE_MIN;
            let low_payload = (low as u32) - Unicode::LOW_SURROGATE_MIN;
            Some(
                (high_payload << 10) + low_payload + Unicode::SUPPLEMENTARY_MIN,
            )
        } else {
            None
        }
    }

    /// Returns the high surrogate for a supplementary code point.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The supplementary code point.
    ///
    /// # Returns
    ///
    /// Returns `Some(high_surrogate)` for supplementary code points and `None`
    /// otherwise.
    #[inline(always)]
    pub const fn high_surrogate(code_point: u32) -> Option<u16> {
        if Unicode::is_supplementary(code_point) {
            Some(
                (((code_point - Unicode::SUPPLEMENTARY_MIN) >> 10)
                    + Unicode::HIGH_SURROGATE_MIN) as u16,
            )
        } else {
            None
        }
    }

    /// Returns the low surrogate for a supplementary code point.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The supplementary code point.
    ///
    /// # Returns
    ///
    /// Returns `Some(low_surrogate)` for supplementary code points and `None`
    /// otherwise.
    #[inline(always)]
    pub const fn low_surrogate(code_point: u32) -> Option<u16> {
        if Unicode::is_supplementary(code_point) {
            Some(
                (((code_point - Unicode::SUPPLEMENTARY_MIN) & 0x3ff)
                    + Unicode::LOW_SURROGATE_MIN) as u16,
            )
        } else {
            None
        }
    }

    /// Detects a UTF-16 BOM and returns its byte order.
    ///
    /// # Parameters
    ///
    /// - `bytes`: The byte buffer to inspect.
    ///
    /// # Returns
    ///
    /// Returns `Some(ByteOrder)` for UTF-16 BOM prefixes, or `None` otherwise.
    #[inline]
    pub fn detect_bom(bytes: &[u8]) -> Option<ByteOrder> {
        match UnicodeBom::detect(bytes) {
            Some(UnicodeBom::Utf16BigEndian) => Some(ByteOrder::BigEndian),
            Some(UnicodeBom::Utf16LittleEndian) => {
                Some(ByteOrder::LittleEndian)
            }
            _ => None,
        }
    }
}
