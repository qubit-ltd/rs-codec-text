// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
/// Namespace for UTF-8 constants and byte classification helpers.
pub enum Utf8 {}

impl Utf8 {
    /// Maximum number of UTF-8 bytes needed for one Unicode scalar value.
    pub const MAX_UNITS_PER_CHAR: usize = 4;

    /// Maximum number of UTF-8 bytes needed for one Unicode scalar value.
    pub const MAX_BYTES_PER_CHAR: usize = Self::MAX_UNITS_PER_CHAR;

    /// Tests whether a byte encodes an ASCII scalar value by itself.
    ///
    /// # Parameters
    ///
    /// - `byte`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `byte` is in `0x00..=0x7F`.
    #[inline(always)]
    pub const fn is_single_byte(byte: u8) -> bool {
        byte <= 0x7f
    }

    /// Tests whether a byte can lead a multi-byte UTF-8 sequence.
    ///
    /// # Parameters
    ///
    /// - `byte`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `byte` is in `0xC2..=0xF4`.
    #[inline(always)]
    pub const fn is_leading_byte(byte: u8) -> bool {
        byte >= 0xc2 && byte <= 0xf4
    }

    /// Tests whether a byte is a UTF-8 continuation byte.
    ///
    /// # Parameters
    ///
    /// - `byte`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `byte` matches the `10xxxxxx` continuation pattern.
    #[inline(always)]
    pub const fn is_continuation_byte(byte: u8) -> bool {
        (byte & 0xc0) == 0x80
    }

    /// Returns the complete UTF-8 sequence length implied by a leading byte.
    ///
    /// # Parameters
    ///
    /// - `byte`: The first byte of the UTF-8 sequence.
    ///
    /// # Returns
    ///
    /// Returns `Some(1..=4)` for valid ASCII or leading bytes, and `None` for
    /// continuation bytes or invalid leading bytes.
    #[inline(always)]
    pub const fn byte_len_from_leading_byte(byte: u8) -> Option<usize> {
        if byte <= 0x7f {
            Some(1)
        } else if byte >= 0xc2 && byte <= 0xdf {
            Some(2)
        } else if byte >= 0xe0 && byte <= 0xef {
            Some(3)
        } else if byte >= 0xf0 && byte <= 0xf4 {
            Some(4)
        } else {
            None
        }
    }

    /// Returns the number of UTF-8 bytes needed for `ch`.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to size.
    ///
    /// # Returns
    ///
    /// Returns `1`, `2`, `3`, or `4`.
    #[inline(always)]
    pub const fn byte_len(ch: char) -> usize {
        let code_point = ch as u32;
        if code_point <= 0x7f {
            1
        } else if code_point <= 0x7ff {
            2
        } else if code_point <= 0xffff {
            3
        } else {
            4
        }
    }
}
