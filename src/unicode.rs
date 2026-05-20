/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
/// Namespace for raw Unicode code point helpers.
pub enum Unicode {}

impl Unicode {
    /// Maximum valid ASCII code point.
    pub const ASCII_MAX: u32 = 0x7f;

    /// Maximum valid Latin-1 code point.
    pub const LATIN1_MAX: u32 = 0xff;

    /// Maximum valid Unicode code point.
    pub const UNICODE_MAX: u32 = 0x10ffff;

    /// Minimum supplementary code point.
    pub const SUPPLEMENTARY_MIN: u32 = 0x10000;

    /// Minimum high-surrogate code unit value.
    pub const HIGH_SURROGATE_MIN: u16 = 0xd800;

    /// Maximum high-surrogate code unit value.
    pub const HIGH_SURROGATE_MAX: u16 = 0xdbff;

    /// Minimum low-surrogate code unit value.
    pub const LOW_SURROGATE_MIN: u16 = 0xdc00;

    /// Maximum low-surrogate code unit value.
    pub const LOW_SURROGATE_MAX: u16 = 0xdfff;

    /// Minimum surrogate code unit value.
    pub const SURROGATE_MIN: u16 = Self::HIGH_SURROGATE_MIN;

    /// Maximum surrogate code unit value.
    pub const SURROGATE_MAX: u16 = Self::LOW_SURROGATE_MAX;

    /// Number of bits shifted when composing or decomposing surrogate pairs.
    pub const HIGH_SURROGATE_SHIFT: u32 = 10;

    /// Mask used to decompose the low surrogate payload.
    pub const SURROGATE_DECOMPOSE_MASK: u32 = (1 << Self::HIGH_SURROGATE_SHIFT) - 1;

    /// Number of bits shifted to obtain a Unicode plane.
    pub const PLANE_SHIFT: u32 = 16;

    /// Tests whether a raw value is a valid ASCII code point.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `code_point` is in `0x00..=0x7F`.
    #[inline]
    #[must_use]
    pub const fn is_valid_ascii(code_point: i32) -> bool {
        code_point >= 0 && (code_point as u32) <= Self::ASCII_MAX
    }

    /// Tests whether a raw value is a valid Latin-1 code point.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `code_point` is in `0x00..=0xFF`.
    #[inline]
    #[must_use]
    pub const fn is_valid_latin1(code_point: i32) -> bool {
        code_point >= 0 && (code_point as u32) <= Self::LATIN1_MAX
    }

    /// Tests whether a raw value is in the Unicode code point range.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `code_point` is in `0x0000..=0x10FFFF`.
    /// This range check does not exclude UTF-16 surrogate code points.
    #[inline]
    #[must_use]
    pub const fn is_valid_unicode(code_point: i32) -> bool {
        code_point >= 0 && (code_point as u32) <= Self::UNICODE_MAX
    }

    /// Tests whether a raw value is in the basic multilingual plane.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `code_point` is in `0x0000..=0xFFFF`.
    #[inline]
    #[must_use]
    pub const fn is_bmp(code_point: i32) -> bool {
        code_point >= 0 && (code_point as u32) < Self::SUPPLEMENTARY_MIN
    }

    /// Tests whether a value is a supplementary Unicode code point.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `code_point` is in `0x10000..=0x10FFFF`.
    #[inline]
    #[must_use]
    pub const fn is_supplementary(code_point: u32) -> bool {
        code_point >= Self::SUPPLEMENTARY_MIN && code_point <= Self::UNICODE_MAX
    }

    /// Tests whether a raw value is a UTF-16 high surrogate.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The raw code point or code-unit value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `code_point` is in `0xD800..=0xDBFF`.
    #[inline]
    #[must_use]
    pub const fn is_high_surrogate(code_point: i32) -> bool {
        code_point >= Self::HIGH_SURROGATE_MIN as i32
            && code_point <= Self::HIGH_SURROGATE_MAX as i32
    }

    /// Tests whether a raw value is a UTF-16 low surrogate.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The raw code point or code-unit value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `code_point` is in `0xDC00..=0xDFFF`.
    #[inline]
    #[must_use]
    pub const fn is_low_surrogate(code_point: i32) -> bool {
        code_point >= Self::LOW_SURROGATE_MIN as i32 && code_point <= Self::LOW_SURROGATE_MAX as i32
    }

    /// Tests whether a raw value is any UTF-16 surrogate.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The raw code point or code-unit value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `code_point` is in `0xD800..=0xDFFF`.
    #[inline]
    #[must_use]
    pub const fn is_surrogate(code_point: i32) -> bool {
        code_point >= Self::SURROGATE_MIN as i32 && code_point <= Self::SURROGATE_MAX as i32
    }

    /// Tests whether two UTF-16 code units form a surrogate pair.
    ///
    /// # Parameters
    ///
    /// - `high`: The candidate high surrogate.
    /// - `low`: The candidate low surrogate.
    ///
    /// # Returns
    ///
    /// Returns `true` if `high` is a high surrogate and `low` is a low surrogate.
    #[inline]
    #[must_use]
    pub const fn is_surrogate_pair(high: u16, low: u16) -> bool {
        Self::is_high_surrogate(high as i32) && Self::is_low_surrogate(low as i32)
    }

    /// Composes a UTF-16 surrogate pair into a Unicode code point.
    ///
    /// # Parameters
    ///
    /// - `high`: The high surrogate code unit.
    /// - `low`: The low surrogate code unit.
    ///
    /// # Returns
    ///
    /// Returns `Some(code_point)` if `high` and `low` form a valid surrogate
    /// pair. Returns `None` if either code unit is not in the required surrogate
    /// range.
    #[inline]
    #[must_use]
    pub const fn compose_surrogate_pair(high: u16, low: u16) -> Option<u32> {
        if Self::is_surrogate_pair(high, low) {
            let high_payload = (high as u32) - (Self::HIGH_SURROGATE_MIN as u32);
            let low_payload = (low as u32) - (Self::LOW_SURROGATE_MIN as u32);
            Some(
                (high_payload << Self::HIGH_SURROGATE_SHIFT)
                    + low_payload
                    + Self::SUPPLEMENTARY_MIN,
            )
        } else {
            None
        }
    }

    /// Decomposes a supplementary code point into its high surrogate.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The supplementary code point to decompose.
    ///
    /// # Returns
    ///
    /// Returns `Some(high_surrogate)` if `code_point` is in
    /// `0x10000..=0x10FFFF`. Returns `None` for BMP and out-of-range values.
    #[inline]
    #[must_use]
    pub const fn decompose_high_surrogate(code_point: u32) -> Option<u16> {
        if Self::is_supplementary(code_point) {
            Some(
                (((code_point - Self::SUPPLEMENTARY_MIN) >> Self::HIGH_SURROGATE_SHIFT)
                    + Self::HIGH_SURROGATE_MIN as u32) as u16,
            )
        } else {
            None
        }
    }

    /// Decomposes a supplementary code point into its low surrogate.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The supplementary code point to decompose.
    ///
    /// # Returns
    ///
    /// Returns `Some(low_surrogate)` if `code_point` is in
    /// `0x10000..=0x10FFFF`. Returns `None` for BMP and out-of-range values.
    #[inline]
    #[must_use]
    pub const fn decompose_low_surrogate(code_point: u32) -> Option<u16> {
        if Self::is_supplementary(code_point) {
            Some(
                (((code_point - Self::SUPPLEMENTARY_MIN) & Self::SURROGATE_DECOMPOSE_MASK)
                    + Self::LOW_SURROGATE_MIN as u32) as u16,
            )
        } else {
            None
        }
    }

    /// Returns the Unicode plane containing a code point.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The code point whose plane is requested.
    ///
    /// # Returns
    ///
    /// Returns `Some(plane)` for values in `0x0000..=0x10FFFF`. Returns `None`
    /// for values above the Unicode maximum.
    #[inline]
    #[must_use]
    pub const fn plane(code_point: u32) -> Option<u32> {
        if code_point <= Self::UNICODE_MAX {
            Some(code_point >> Self::PLANE_SHIFT)
        } else {
            None
        }
    }

    /// Escapes a code point as Java-style Unicode escape text.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The code point to escape.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing `\uXXXX` for BMP values and
    /// `\uXXXXX` or longer uppercase hexadecimal escape text for supplementary
    /// values. Returns `None` if `code_point` is above `0x10FFFF`.
    #[must_use]
    pub fn escape_java(code_point: u32) -> Option<String> {
        if code_point > Self::UNICODE_MAX {
            None
        } else if code_point > 0xffff {
            Some(format!("\\u{code_point:X}"))
        } else {
            Some(format!("\\u{code_point:04X}"))
        }
    }

    /// Escapes a code point as a Rust-style Unicode escape text.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The code point to escape.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing `\u{...}` for valid Unicode scalar
    /// values. Returns `None` if `code_point` is above `0x10FFFF`.
    #[must_use]
    pub fn escape_rust(code_point: u32) -> Option<String> {
        if code_point > Self::UNICODE_MAX {
            None
        } else {
            Some(format!("\\u{{{code_point:X}}}"))
        }
    }
}
