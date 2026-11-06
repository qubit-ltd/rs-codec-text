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
    /// Converts an ASCII decimal digit byte to its numeric value.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=9)` for `0..=9`; otherwise returns `None`.
    #[inline]
    pub const fn byte_to_digit(value: u8) -> Option<u8> {
        Ascii::byte_to_digit(value)
    }
    /// Converts an ASCII decimal digit character to its numeric value.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=9)` for `0..=9`; otherwise returns `None`.
    #[inline]
    pub const fn char_to_digit(value: char) -> Option<u8> {
        match Self::char_to_byte(value) {
            Some(value) => Self::byte_to_digit(value),
            None => None,
        }
    }
    /// Converts an ASCII decimal digit code point to its numeric value.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=9)` for `0..=9`; otherwise returns `None`.
    #[inline]
    pub const fn code_point_to_digit(value: u32) -> Option<u8> {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::byte_to_digit(value),
            None => None,
        }
    }
    /// Converts an ASCII hexadecimal digit byte to its numeric value.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=15)` for ASCII hexadecimal digits; otherwise returns
    /// `None`.
    #[inline]
    pub const fn byte_to_hex_digit(value: u8) -> Option<u8> {
        Ascii::byte_to_hex_digit(value)
    }
    /// Converts an ASCII hexadecimal digit character to its numeric value.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=15)` for ASCII hexadecimal digits; otherwise returns
    /// `None`.
    #[inline]
    pub const fn char_to_hex_digit(value: char) -> Option<u8> {
        match Self::char_to_byte(value) {
            Some(value) => Self::byte_to_hex_digit(value),
            None => None,
        }
    }
    /// Converts an ASCII hexadecimal digit code point to its numeric value.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=15)` for ASCII hexadecimal digits; otherwise returns
    /// `None`.
    #[inline]
    pub const fn code_point_to_hex_digit(value: u32) -> Option<u8> {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::byte_to_hex_digit(value),
            None => None,
        }
    }
}
