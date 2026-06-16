// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
/// Namespace for Unicode constants and encoding-independent code point helpers.
pub enum Unicode {}

impl Unicode {
    /// Maximum valid Unicode code point.
    pub const MAX_CODE_POINT: u32 = 0x10ffff;

    /// Unicode replacement character.
    pub const REPLACEMENT_CHARACTER: char = '\u{fffd}';

    /// Unicode byte order mark character.
    pub const BOM: char = '\u{feff}';

    /// Maximum valid ASCII code point.
    pub const ASCII_MAX: u32 = 0x7f;

    /// Maximum valid Latin-1 code point.
    pub const LATIN1_MAX: u32 = 0xff;

    /// Minimum supplementary code point.
    pub const SUPPLEMENTARY_MIN: u32 = 0x10000;

    /// Minimum high-surrogate code unit value.
    pub const HIGH_SURROGATE_MIN: u32 = 0xd800;

    /// Maximum high-surrogate code unit value.
    pub const HIGH_SURROGATE_MAX: u32 = 0xdbff;

    /// Minimum low-surrogate code unit value.
    pub const LOW_SURROGATE_MIN: u32 = 0xdc00;

    /// Maximum low-surrogate code unit value.
    pub const LOW_SURROGATE_MAX: u32 = 0xdfff;

    /// Minimum surrogate code unit value.
    pub const SURROGATE_MIN: u32 = Self::HIGH_SURROGATE_MIN;

    /// Maximum surrogate code unit value.
    pub const SURROGATE_MAX: u32 = Self::LOW_SURROGATE_MAX;

    /// Tests whether `value` is in the Unicode code point range.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for values in `0x0000..=0x10FFFF`, including surrogate
    /// code points.
    #[inline(always)]
    pub const fn is_code_point(value: u32) -> bool {
        value <= Self::MAX_CODE_POINT
    }

    /// Tests whether `value` is a valid Unicode scalar value.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for Unicode code points excluding UTF-16 surrogate
    /// values.
    #[inline(always)]
    pub const fn is_scalar_value(value: u32) -> bool {
        Self::is_code_point(value) && !Self::is_surrogate(value)
    }

    /// Tests whether `value` is a UTF-16 surrogate code point.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point or code-unit value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is in `0xD800..=0xDFFF`.
    #[inline(always)]
    pub const fn is_surrogate(value: u32) -> bool {
        value >= Self::SURROGATE_MIN && value <= Self::SURROGATE_MAX
    }

    /// Tests whether `value` is a high surrogate.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point or code-unit value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is in `0xD800..=0xDBFF`.
    #[inline(always)]
    pub const fn is_high_surrogate(value: u32) -> bool {
        value >= Self::HIGH_SURROGATE_MIN && value <= Self::HIGH_SURROGATE_MAX
    }

    /// Tests whether `value` is a low surrogate.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point or code-unit value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is in `0xDC00..=0xDFFF`.
    #[inline(always)]
    pub const fn is_low_surrogate(value: u32) -> bool {
        value >= Self::LOW_SURROGATE_MIN && value <= Self::LOW_SURROGATE_MAX
    }

    /// Tests whether `value` is in the basic multilingual plane.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is in `0x0000..=0xFFFF`.
    #[inline(always)]
    pub const fn is_bmp(value: u32) -> bool {
        value < Self::SUPPLEMENTARY_MIN
    }

    /// Tests whether `value` is a supplementary Unicode code point.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is in `0x10000..=0x10FFFF`.
    #[inline(always)]
    pub const fn is_supplementary(value: u32) -> bool {
        value >= Self::SUPPLEMENTARY_MIN && value <= Self::MAX_CODE_POINT
    }

    /// Tests whether `value` is an ASCII code point.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is in `0x00..=0x7F`.
    #[inline(always)]
    pub const fn is_ascii(value: u32) -> bool {
        value <= Self::ASCII_MAX
    }

    /// Tests whether `value` is a Unicode noncharacter.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for Unicode noncharacters such as `U+FDD0..=U+FDEF` and
    /// code points ending in `FFFE` or `FFFF`.
    #[inline(always)]
    pub const fn is_noncharacter(value: u32) -> bool {
        Self::is_code_point(value)
            && ((value >= 0xfdd0 && value <= 0xfdef)
                || (value & 0xfffe) == 0xfffe)
    }

    /// Tests whether `value` is a Unicode C0, C1, or DEL control code point.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `U+0000..=U+001F`, `U+007F`, and `U+0080..=U+009F`.
    #[inline(always)]
    pub const fn is_control(value: u32) -> bool {
        value <= 0x1f || (value >= 0x7f && value <= 0x9f)
    }

    /// Returns the Unicode plane containing `value`.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to inspect.
    ///
    /// # Returns
    ///
    /// Returns `Some(plane)` for Unicode code points, or `None` for values
    /// above `U+10FFFF`.
    #[inline(always)]
    pub const fn plane(value: u32) -> Option<u32> {
        if Self::is_code_point(value) {
            Some(value >> 16)
        } else {
            None
        }
    }

    /// Converts a raw code point into a Rust `char`.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(char)` for valid Unicode scalar values and `None`
    /// otherwise.
    #[inline(always)]
    pub fn to_char(value: u32) -> Option<char> {
        char::from_u32(value)
    }
}
