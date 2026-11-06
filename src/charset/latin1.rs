// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
/// Namespace for ISO-8859-1 / Latin-1 character and code point helpers.
///
/// # Examples
///
/// ```
/// use qubit_codec_text::Latin1;
///
/// assert!(Latin1::is_latin1_byte(0xff));
/// assert!(Latin1::is_latin1_char('ÿ'));
/// assert_eq!(Latin1::byte_to_char(0xff), 'ÿ');
/// ```
pub enum Latin1 {}

impl Latin1 {
    /// Maximum valid Latin-1 character.
    pub const MAX_CHAR: char = '\u{00ff}';

    /// Maximum valid Latin-1 byte.
    pub const MAX_BYTE: u8 = 0xff;

    /// Maximum valid Latin-1 code point.
    pub const MAX_CODE_POINT: u32 = Self::MAX_BYTE as u32;
}

/// Implements Latin-1 uppercase and lowercase mappings.
mod case_mapping;
/// Implements Latin-1 classification and case-insensitive comparison.
mod classification;
/// Implements byte, character, and code point conversions.
mod conversion;
/// Implements ASCII digit conversions.
mod digit_conversion;
