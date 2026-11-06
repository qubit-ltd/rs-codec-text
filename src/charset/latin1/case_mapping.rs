// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use super::super::Ascii;
use super::Latin1;

impl Latin1 {
    /// Converts a byte to uppercase using Latin-1 case rules.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns the uppercase Latin-1 equivalent when one exists; otherwise
    /// returns `value` unchanged.
    #[inline]
    #[must_use]
    pub const fn byte_to_uppercase(value: u8) -> u8 {
        if Ascii::is_lowercase_letter_byte(value)
            || (value >= 0xe0 && value <= 0xf6)
            || (value >= 0xf8 && value <= 0xfe)
        {
            value - 0x20
        } else {
            value
        }
    }
    /// Converts a character to uppercase using Latin-1 case rules.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to convert.
    ///
    /// # Returns
    ///
    /// Returns the uppercase Latin-1 equivalent when one exists; otherwise
    /// returns `value` unchanged.
    #[inline]
    #[must_use]
    pub const fn char_to_uppercase(value: char) -> char {
        match Self::char_to_byte(value) {
            Some(value) => Self::byte_to_char(Self::byte_to_uppercase(value)),
            None => value,
        }
    }
    /// Converts a raw code point to uppercase using Latin-1 case rules.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to convert.
    ///
    /// # Returns
    ///
    /// Returns the uppercase Latin-1 equivalent when one exists; otherwise
    /// returns `value` unchanged.
    #[inline]
    #[must_use]
    pub const fn code_point_to_uppercase(value: u32) -> u32 {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::byte_to_uppercase(value) as u32,
            None => value,
        }
    }
    /// Converts a byte to lowercase using Latin-1 case rules.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns the lowercase Latin-1 equivalent when one exists; otherwise
    /// returns `value` unchanged.
    #[inline]
    #[must_use]
    pub const fn byte_to_lowercase(value: u8) -> u8 {
        if Ascii::is_uppercase_letter_byte(value)
            || (value >= 0xc0 && value <= 0xd6)
            || (value >= 0xd8 && value <= 0xde)
        {
            value + 0x20
        } else {
            value
        }
    }
    /// Converts a character to lowercase using Latin-1 case rules.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to convert.
    ///
    /// # Returns
    ///
    /// Returns the lowercase Latin-1 equivalent when one exists; otherwise
    /// returns `value` unchanged.
    #[inline]
    #[must_use]
    pub const fn char_to_lowercase(value: char) -> char {
        match Self::char_to_byte(value) {
            Some(value) => Self::byte_to_char(Self::byte_to_lowercase(value)),
            None => value,
        }
    }
    /// Converts a raw code point to lowercase using Latin-1 case rules.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to convert.
    ///
    /// # Returns
    ///
    /// Returns the lowercase Latin-1 equivalent when one exists; otherwise
    /// returns `value` unchanged.
    #[inline]
    #[must_use]
    pub const fn code_point_to_lowercase(value: u32) -> u32 {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::byte_to_lowercase(value) as u32,
            None => value,
        }
    }
}
