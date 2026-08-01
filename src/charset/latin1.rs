// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use super::Ascii;

/// Namespace for ISO-8859-1 / Latin-1 character and code point helpers.
pub enum Latin1 {}

impl Latin1 {
    /// Maximum valid Latin-1 character.
    pub const MAX_CHAR: char = '\u{00ff}';

    /// Maximum valid Latin-1 byte.
    pub const MAX_BYTE: u8 = 0xff;

    /// Maximum valid Latin-1 code point.
    pub const MAX_CODE_POINT: u32 = Self::MAX_BYTE as u32;

    /// Tests whether a byte is a Latin-1 byte.
    ///
    /// # Parameters
    ///
    /// - `_value`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for every byte value because ISO-8859-1 maps all
    /// `0x00..=0xFF` bytes.
    #[inline(always)]
    #[must_use]
    pub const fn is_latin1_byte(_value: u8) -> bool {
        true
    }

    /// Tests whether a character is a Latin-1 character.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is in `U+0000..=U+00FF`.
    #[inline]
    #[must_use]
    pub const fn is_latin1_char(value: char) -> bool {
        value <= Self::MAX_CHAR
    }

    /// Tests whether an integer value is a Latin-1 code point.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is in `0x00..=0xFF`.
    #[inline]
    #[must_use]
    pub const fn is_latin1_code_point(value: u32) -> bool {
        value <= Self::MAX_CODE_POINT
    }

    /// Tests whether a byte is Java-style Latin-1 whitespace.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
    #[inline(always)]
    #[must_use]
    pub const fn is_whitespace_byte(value: u8) -> bool {
        Ascii::is_whitespace_byte(value)
    }

    /// Tests whether a character is Java-style Latin-1 whitespace.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
    #[inline]
    #[must_use]
    pub const fn is_whitespace_char(value: char) -> bool {
        match Self::char_to_byte(value) {
            Some(value) => Self::is_whitespace_byte(value),
            None => false,
        }
    }

    /// Tests whether a raw code point is Java-style Latin-1 whitespace.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
    #[inline]
    #[must_use]
    pub const fn is_whitespace_code_point(value: u32) -> bool {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::is_whitespace_byte(value),
            None => false,
        }
    }

    /// Tests whether a byte is a Latin-1 letter.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is an ISO-8859-1 encoded letter.
    #[inline(always)]
    #[must_use]
    pub const fn is_letter_byte(value: u8) -> bool {
        Self::is_uppercase_letter_byte(value)
            || Self::is_lowercase_letter_byte(value)
            || value == 0xaa
            || value == 0xba
    }

    /// Tests whether a character is a Latin-1 letter.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is in the Latin-1 letter set.
    #[inline]
    #[must_use]
    pub const fn is_letter_char(value: char) -> bool {
        match Self::char_to_byte(value) {
            Some(value) => Self::is_letter_byte(value),
            None => false,
        }
    }

    /// Tests whether a raw code point is a Latin-1 letter.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is in the Latin-1 letter set.
    #[inline]
    #[must_use]
    pub const fn is_letter_code_point(value: u32) -> bool {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::is_letter_byte(value),
            None => false,
        }
    }

    /// Tests whether a byte is an uppercase Latin-1 letter.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `A..=Z`, `U+00C0..=U+00D6`, and
    /// `U+00D8..=U+00DE`.
    #[inline(always)]
    #[must_use]
    pub const fn is_uppercase_letter_byte(value: u8) -> bool {
        Ascii::is_uppercase_letter_byte(value)
            || (value >= 0xc0 && value <= 0xd6)
            || (value >= 0xd8 && value <= 0xde)
    }

    /// Tests whether a character is an uppercase Latin-1 letter.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is an uppercase Latin-1 letter.
    #[inline]
    #[must_use]
    pub const fn is_uppercase_letter_char(value: char) -> bool {
        match Self::char_to_byte(value) {
            Some(value) => Self::is_uppercase_letter_byte(value),
            None => false,
        }
    }

    /// Tests whether a raw code point is an uppercase Latin-1 letter.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is an uppercase Latin-1 letter.
    #[inline]
    #[must_use]
    pub const fn is_uppercase_letter_code_point(value: u32) -> bool {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::is_uppercase_letter_byte(value),
            None => false,
        }
    }

    /// Tests whether a byte is a lowercase Latin-1 letter.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `a..=z`, `U+00B5`, `U+00DF`,
    /// `U+00E0..=U+00F6`, and `U+00F8..=U+00FF`.
    #[inline(always)]
    #[must_use]
    pub const fn is_lowercase_letter_byte(value: u8) -> bool {
        Ascii::is_lowercase_letter_byte(value)
            || value == 0xb5
            || value == 0xdf
            || (value >= 0xe0 && value <= 0xf6)
            || value >= 0xf8
    }

    /// Tests whether a character is a lowercase Latin-1 letter.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is a lowercase Latin-1 letter.
    #[inline]
    #[must_use]
    pub const fn is_lowercase_letter_char(value: char) -> bool {
        match Self::char_to_byte(value) {
            Some(value) => Self::is_lowercase_letter_byte(value),
            None => false,
        }
    }

    /// Tests whether a raw code point is a lowercase Latin-1 letter.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is a lowercase Latin-1 letter.
    #[inline]
    #[must_use]
    pub const fn is_lowercase_letter_code_point(value: u32) -> bool {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::is_lowercase_letter_byte(value),
            None => false,
        }
    }

    /// Tests whether a byte is an ASCII decimal digit.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `0..=9`.
    #[inline(always)]
    #[must_use]
    pub const fn is_digit_byte(value: u8) -> bool {
        Ascii::is_digit_byte(value)
    }

    /// Tests whether a character is an ASCII decimal digit in Latin-1.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `0..=9`.
    #[inline]
    #[must_use]
    pub const fn is_digit_char(value: char) -> bool {
        match Self::char_to_byte(value) {
            Some(value) => Self::is_digit_byte(value),
            None => false,
        }
    }

    /// Tests whether a raw code point is an ASCII decimal digit in Latin-1.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `0..=9`.
    #[inline]
    #[must_use]
    pub const fn is_digit_code_point(value: u32) -> bool {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::is_digit_byte(value),
            None => false,
        }
    }

    /// Tests whether a byte is an ASCII hexadecimal digit.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `0..=9`, `A..=F`, or `a..=f`.
    #[inline(always)]
    #[must_use]
    pub const fn is_hex_digit_byte(value: u8) -> bool {
        Ascii::is_hex_digit_byte(value)
    }

    /// Tests whether a character is an ASCII hexadecimal digit in Latin-1.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `0..=9`, `A..=F`, or `a..=f`.
    #[inline]
    #[must_use]
    pub const fn is_hex_digit_char(value: char) -> bool {
        match Self::char_to_byte(value) {
            Some(value) => Self::is_hex_digit_byte(value),
            None => false,
        }
    }

    /// Tests whether a raw code point is an ASCII hexadecimal digit in Latin-1.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `0..=9`, `A..=F`, or `a..=f`.
    #[inline]
    #[must_use]
    pub const fn is_hex_digit_code_point(value: u32) -> bool {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::is_hex_digit_byte(value),
            None => false,
        }
    }

    /// Tests whether a byte is an ASCII octal digit.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `0..=7`.
    #[inline(always)]
    #[must_use]
    pub const fn is_octal_digit_byte(value: u8) -> bool {
        Ascii::is_octal_digit_byte(value)
    }

    /// Tests whether a character is an ASCII octal digit in Latin-1.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `0..=7`.
    #[inline]
    #[must_use]
    pub const fn is_octal_digit_char(value: char) -> bool {
        match Self::char_to_byte(value) {
            Some(value) => Self::is_octal_digit_byte(value),
            None => false,
        }
    }

    /// Tests whether a raw code point is an ASCII octal digit in Latin-1.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `0..=7`.
    #[inline]
    #[must_use]
    pub const fn is_octal_digit_code_point(value: u32) -> bool {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::is_octal_digit_byte(value),
            None => false,
        }
    }

    /// Tests whether a byte is a Latin-1 letter or ASCII decimal digit.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for Latin-1 letters and `0..=9`.
    #[inline(always)]
    #[must_use]
    pub const fn is_letter_or_digit_byte(value: u8) -> bool {
        Self::is_letter_byte(value) || Self::is_digit_byte(value)
    }

    /// Tests whether a character is a Latin-1 letter or ASCII decimal digit.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for Latin-1 letters and `0..=9`.
    #[inline]
    #[must_use]
    pub const fn is_letter_or_digit_char(value: char) -> bool {
        match Self::char_to_byte(value) {
            Some(value) => Self::is_letter_or_digit_byte(value),
            None => false,
        }
    }

    /// Tests whether a raw code point is a Latin-1 letter or ASCII digit.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for Latin-1 letters and `0..=9`.
    #[inline]
    #[must_use]
    pub const fn is_letter_or_digit_code_point(value: u32) -> bool {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::is_letter_or_digit_byte(value),
            None => false,
        }
    }

    /// Tests whether a byte is printable in Latin-1.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for bytes outside the C0, DEL, and C1 control ranges.
    #[inline(always)]
    #[must_use]
    pub const fn is_printable_byte(value: u8) -> bool {
        !Self::is_control_byte(value)
    }

    /// Tests whether a character is printable in Latin-1.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for Latin-1 characters outside control ranges.
    #[inline]
    #[must_use]
    pub const fn is_printable_char(value: char) -> bool {
        match Self::char_to_byte(value) {
            Some(value) => Self::is_printable_byte(value),
            None => false,
        }
    }

    /// Tests whether a raw code point is printable in Latin-1.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for Latin-1 values outside control ranges.
    #[inline]
    #[must_use]
    pub const fn is_printable_code_point(value: u32) -> bool {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::is_printable_byte(value),
            None => false,
        }
    }

    /// Tests whether a byte is a Latin-1 control character.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for C0 controls, DEL, or C1 controls.
    #[inline(always)]
    #[must_use]
    pub const fn is_control_byte(value: u8) -> bool {
        value < 0x20 || (value >= 0x7f && value <= 0x9f)
    }

    /// Tests whether a character is a Latin-1 control character.
    ///
    /// # Parameters
    ///
    /// - `value`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for C0 controls, DEL, or C1 controls.
    #[inline]
    #[must_use]
    pub const fn is_control_char(value: char) -> bool {
        match Self::char_to_byte(value) {
            Some(value) => Self::is_control_byte(value),
            None => false,
        }
    }

    /// Tests whether a raw code point is a Latin-1 control character.
    ///
    /// # Parameters
    ///
    /// - `value`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for C0 controls, DEL, or C1 controls.
    #[inline]
    #[must_use]
    pub const fn is_control_code_point(value: u32) -> bool {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::is_control_byte(value),
            None => false,
        }
    }

    /// Converts a Latin-1 byte to the corresponding Unicode character.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns the character with the same scalar value as `value`.
    #[inline(always)]
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
    #[inline(always)]
    #[must_use]
    pub const fn byte_to_code_point(value: u8) -> u32 {
        value as u32
    }

    /// Compares two bytes while ignoring Latin-1 case.
    ///
    /// # Parameters
    ///
    /// - `left`: The first byte to compare.
    /// - `right`: The second byte to compare.
    ///
    /// # Returns
    ///
    /// Returns `true` if both bytes match after Latin-1 lowercase conversion.
    #[inline(always)]
    #[must_use]
    pub const fn equals_ignore_case_byte(left: u8, right: u8) -> bool {
        if left == right {
            true
        } else {
            Self::byte_to_lowercase(left) == Self::byte_to_lowercase(right)
        }
    }

    /// Compares two characters while ignoring Latin-1 case.
    ///
    /// # Parameters
    ///
    /// - `left`: The first character to compare.
    /// - `right`: The second character to compare.
    ///
    /// # Returns
    ///
    /// Returns `true` if both Latin-1 characters match after lowercase
    /// conversion. Non-Latin-1 characters are compared unchanged.
    #[inline]
    #[must_use]
    pub const fn equals_ignore_case_char(left: char, right: char) -> bool {
        if left == right {
            true
        } else {
            Self::char_to_lowercase(left) == Self::char_to_lowercase(right)
        }
    }

    /// Compares two raw code points while ignoring Latin-1 case.
    ///
    /// # Parameters
    ///
    /// - `left`: The first raw code point value to compare.
    /// - `right`: The second raw code point value to compare.
    ///
    /// # Returns
    ///
    /// Returns `true` if both values match after Latin-1 lowercase conversion.
    #[inline]
    #[must_use]
    pub const fn equals_ignore_case_code_point(left: u32, right: u32) -> bool {
        if left == right {
            true
        } else {
            Self::code_point_to_lowercase(left) == Self::code_point_to_lowercase(right)
        }
    }

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
    #[inline(always)]
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
    #[inline(always)]
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
    #[must_use]
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
    #[must_use]
    pub const fn code_point_to_byte(value: u32) -> Option<u8> {
        if Self::is_latin1_code_point(value) {
            Some(value as u8)
        } else {
            None
        }
    }

    /// Converts an ASCII decimal digit byte to its numeric value.
    ///
    /// # Parameters
    ///
    /// - `value`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=9)` for `0..=9`; otherwise returns `None`.
    #[inline(always)]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[inline(always)]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub const fn code_point_to_hex_digit(value: u32) -> Option<u8> {
        match Self::code_point_to_byte(value) {
            Some(value) => Self::byte_to_hex_digit(value),
            None => None,
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
    #[must_use]
    pub const fn code_point_to_char(value: u32) -> Option<char> {
        if Self::is_latin1_code_point(value) {
            Some(Self::byte_to_char(value as u8))
        } else {
            None
        }
    }
}
