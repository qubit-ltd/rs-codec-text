// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use super::Latin1;

impl Latin1 {
    /// Converts a Latin-1 byte to the corresponding Unicode character.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns the character with the same scalar value as `value`.
    #[inline]
    #[must_use]
    pub const fn byte_to_char(value: u8) -> char {
        value as char
    }
    /// Converts a Latin-1 byte to the corresponding code point.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns the code point with the same numeric value as `value`.
    #[inline]
    #[must_use]
    pub const fn byte_to_code_point(value: u8) -> u32 {
        value as u32
    }
    /// Converts a character to a Latin-1 byte.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(byte)` when `value` is in `U+0000..=U+00FF`; otherwise
    /// returns `None`.
    #[inline]
    pub const fn char_to_byte(value: char) -> Option<u8> {
        if Self::is_latin1_char(value) {
            Some(value as u8)
        } else {
            None
        }
    }
    /// Converts a raw code point to a Latin-1 byte.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(byte)` when `value` is in `0x00..=0xFF`; otherwise
    /// returns `None`.
    #[inline]
    pub const fn code_point_to_byte(value: u32) -> Option<u8> {
        if Self::is_latin1_code_point(value) {
            Some(value as u8)
        } else {
            None
        }
    }
    /// Converts a raw code point to a Latin-1 character.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(char)` when `value` is in `0x00..=0xFF`; otherwise
    /// returns `None`.
    #[inline]
    pub const fn code_point_to_char(value: u32) -> Option<char> {
        if Self::is_latin1_code_point(value) {
            Some(Self::byte_to_char(value as u8))
        } else {
            None
        }
    }
}
