/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use crate::charset::ascii_folding;

/// Namespace for ASCII character and code point helpers.
pub enum Ascii {}

impl Ascii {
    /// Maximum valid ASCII character.
    pub const MAX_CHAR: char = '\u{007f}';

    /// Maximum valid ASCII byte.
    pub const MAX_BYTE: u8 = Self::MAX_CHAR as u8;

    /// Maximum number of ASCII characters emitted by [`Self::fold`].
    pub const MAX_FOLDING_COUNT: usize = 4;

    /// ASCII NUL.
    pub const NULL_CHAR: char = '\0';

    /// ASCII NUL as a byte.
    pub const NULL_BYTE: u8 = Self::NULL_CHAR as u8;

    /// ASCII SOH.
    pub const START_OF_HEADER_CHAR: char = '\u{0001}';

    /// ASCII SOH as a byte.
    pub const START_OF_HEADER_BYTE: u8 = Self::START_OF_HEADER_CHAR as u8;

    /// ASCII STX.
    pub const START_OF_TEXT_CHAR: char = '\u{0002}';

    /// ASCII STX as a byte.
    pub const START_OF_TEXT_BYTE: u8 = Self::START_OF_TEXT_CHAR as u8;

    /// ASCII ETX.
    pub const END_OF_TEXT_CHAR: char = '\u{0003}';

    /// ASCII ETX as a byte.
    pub const END_OF_TEXT_BYTE: u8 = Self::END_OF_TEXT_CHAR as u8;

    /// ASCII EOT.
    pub const START_OF_TRANSMISSION_CHAR: char = '\u{0004}';

    /// ASCII EOT as a byte.
    pub const START_OF_TRANSMISSION_BYTE: u8 = Self::START_OF_TRANSMISSION_CHAR as u8;

    /// ASCII ENQ.
    pub const ENQUIRY_CHAR: char = '\u{0005}';

    /// ASCII ENQ as a byte.
    pub const ENQUIRY_BYTE: u8 = Self::ENQUIRY_CHAR as u8;

    /// ASCII ACK.
    pub const ACKNOWLEDGMENT_CHAR: char = '\u{0006}';

    /// ASCII ACK as a byte.
    pub const ACKNOWLEDGMENT_BYTE: u8 = Self::ACKNOWLEDGMENT_CHAR as u8;

    /// ASCII BEL.
    pub const BELL_CHAR: char = '\u{0007}';

    /// ASCII BEL as a byte.
    pub const BELL_BYTE: u8 = Self::BELL_CHAR as u8;

    /// ASCII BS.
    pub const BACKSPACE_CHAR: char = '\u{0008}';

    /// ASCII BS as a byte.
    pub const BACKSPACE_BYTE: u8 = Self::BACKSPACE_CHAR as u8;

    /// ASCII HT.
    pub const HORIZONTAL_TAB_CHAR: char = '\t';

    /// ASCII HT as a byte.
    pub const HORIZONTAL_TAB_BYTE: u8 = Self::HORIZONTAL_TAB_CHAR as u8;

    /// ASCII LF.
    pub const LINE_FEED_CHAR: char = '\n';

    /// ASCII LF as a byte.
    pub const LINE_FEED_BYTE: u8 = Self::LINE_FEED_CHAR as u8;

    /// ASCII VT.
    pub const VERTICAL_TAB_CHAR: char = '\u{000b}';

    /// ASCII VT as a byte.
    pub const VERTICAL_TAB_BYTE: u8 = Self::VERTICAL_TAB_CHAR as u8;

    /// ASCII FF.
    pub const FORM_FEED_CHAR: char = '\u{000c}';

    /// ASCII FF as a byte.
    pub const FORM_FEED_BYTE: u8 = Self::FORM_FEED_CHAR as u8;

    /// ASCII CR.
    pub const CARRIAGE_RETURN_CHAR: char = '\r';

    /// ASCII CR as a byte.
    pub const CARRIAGE_RETURN_BYTE: u8 = Self::CARRIAGE_RETURN_CHAR as u8;

    /// ASCII SO.
    pub const SHIFT_OUT_CHAR: char = '\u{000e}';

    /// ASCII SO as a byte.
    pub const SHIFT_OUT_BYTE: u8 = Self::SHIFT_OUT_CHAR as u8;

    /// ASCII SI.
    pub const SHIFT_IN_CHAR: char = '\u{000f}';

    /// ASCII SI as a byte.
    pub const SHIFT_IN_BYTE: u8 = Self::SHIFT_IN_CHAR as u8;

    /// ASCII DLE.
    pub const DATA_LINK_ESCAPE_CHAR: char = '\u{0010}';

    /// ASCII DLE as a byte.
    pub const DATA_LINK_ESCAPE_BYTE: u8 = Self::DATA_LINK_ESCAPE_CHAR as u8;

    /// ASCII DC1.
    pub const DEVICE_CONTROL_1_CHAR: char = '\u{0011}';

    /// ASCII DC1 as a byte.
    pub const DEVICE_CONTROL_1_BYTE: u8 = Self::DEVICE_CONTROL_1_CHAR as u8;

    /// ASCII DC2.
    pub const DEVICE_CONTROL_2_CHAR: char = '\u{0012}';

    /// ASCII DC2 as a byte.
    pub const DEVICE_CONTROL_2_BYTE: u8 = Self::DEVICE_CONTROL_2_CHAR as u8;

    /// ASCII DC3.
    pub const DEVICE_CONTROL_3_CHAR: char = '\u{0013}';

    /// ASCII DC3 as a byte.
    pub const DEVICE_CONTROL_3_BYTE: u8 = Self::DEVICE_CONTROL_3_CHAR as u8;

    /// ASCII DC4.
    pub const DEVICE_CONTROL_4_CHAR: char = '\u{0014}';

    /// ASCII DC4 as a byte.
    pub const DEVICE_CONTROL_4_BYTE: u8 = Self::DEVICE_CONTROL_4_CHAR as u8;

    /// ASCII NAK.
    pub const NEGATIVE_ACKNOWLEDGEMENT_CHAR: char = '\u{0015}';

    /// ASCII NAK as a byte.
    pub const NEGATIVE_ACKNOWLEDGEMENT_BYTE: u8 = Self::NEGATIVE_ACKNOWLEDGEMENT_CHAR as u8;

    /// ASCII SYN.
    pub const SYNCHRONOUS_IDLE_CHAR: char = '\u{0016}';

    /// ASCII SYN as a byte.
    pub const SYNCHRONOUS_IDLE_BYTE: u8 = Self::SYNCHRONOUS_IDLE_CHAR as u8;

    /// ASCII ETB.
    pub const END_OF_TRANS_BLOCK_CHAR: char = '\u{0017}';

    /// ASCII ETB as a byte.
    pub const END_OF_TRANS_BLOCK_BYTE: u8 = Self::END_OF_TRANS_BLOCK_CHAR as u8;

    /// ASCII CAN.
    pub const CANCEL_CHAR: char = '\u{0018}';

    /// ASCII CAN as a byte.
    pub const CANCEL_BYTE: u8 = Self::CANCEL_CHAR as u8;

    /// ASCII EM.
    pub const END_OF_MEDIUM_CHAR: char = '\u{0019}';

    /// ASCII EM as a byte.
    pub const END_OF_MEDIUM_BYTE: u8 = Self::END_OF_MEDIUM_CHAR as u8;

    /// ASCII SUB.
    pub const SUBSTITUTE_CHAR: char = '\u{001a}';

    /// ASCII SUB as a byte.
    pub const SUBSTITUTE_BYTE: u8 = Self::SUBSTITUTE_CHAR as u8;

    /// ASCII ESC.
    pub const ESCAPE_CHAR: char = '\u{001b}';

    /// ASCII ESC as a byte.
    pub const ESCAPE_BYTE: u8 = Self::ESCAPE_CHAR as u8;

    /// ASCII FS.
    pub const FILE_SEPARATOR_CHAR: char = '\u{001c}';

    /// ASCII FS as a byte.
    pub const FILE_SEPARATOR_BYTE: u8 = Self::FILE_SEPARATOR_CHAR as u8;

    /// ASCII GS.
    pub const GROUP_SEPARATOR_CHAR: char = '\u{001d}';

    /// ASCII GS as a byte.
    pub const GROUP_SEPARATOR_BYTE: u8 = Self::GROUP_SEPARATOR_CHAR as u8;

    /// ASCII RS.
    pub const RECORD_SEPARATOR_CHAR: char = '\u{001e}';

    /// ASCII RS as a byte.
    pub const RECORD_SEPARATOR_BYTE: u8 = Self::RECORD_SEPARATOR_CHAR as u8;

    /// ASCII US.
    pub const UNIT_SEPARATOR_CHAR: char = '\u{001f}';

    /// ASCII US as a byte.
    pub const UNIT_SEPARATOR_BYTE: u8 = Self::UNIT_SEPARATOR_CHAR as u8;

    /// ASCII DEL.
    pub const DELETE_CHAR: char = '\u{007f}';

    /// ASCII DEL as a byte.
    pub const DELETE_BYTE: u8 = Self::DELETE_CHAR as u8;

    /// ASCII space.
    pub const SPACE_CHAR: char = ' ';

    /// ASCII space as a byte.
    pub const SPACE_BYTE: u8 = Self::SPACE_CHAR as u8;

    /// ASCII exclamation mark.
    pub const EXCLAMATION_CHAR: char = '!';

    /// ASCII exclamation mark as a byte.
    pub const EXCLAMATION_BYTE: u8 = Self::EXCLAMATION_CHAR as u8;

    /// ASCII double quote.
    pub const DOUBLE_QUOTE_CHAR: char = '"';

    /// ASCII double quote as a byte.
    pub const DOUBLE_QUOTE_BYTE: u8 = Self::DOUBLE_QUOTE_CHAR as u8;

    /// ASCII number sign.
    pub const SHARP_CHAR: char = '#';

    /// ASCII number sign as a byte.
    pub const SHARP_BYTE: u8 = Self::SHARP_CHAR as u8;

    /// ASCII dollar sign.
    pub const DOLLAR_CHAR: char = '$';

    /// ASCII dollar sign as a byte.
    pub const DOLLAR_BYTE: u8 = Self::DOLLAR_CHAR as u8;

    /// ASCII percent sign.
    pub const PERCENT_CHAR: char = '%';

    /// ASCII percent sign as a byte.
    pub const PERCENT_BYTE: u8 = Self::PERCENT_CHAR as u8;

    /// ASCII ampersand.
    pub const AMPERSAND_CHAR: char = '&';

    /// ASCII ampersand as a byte.
    pub const AMPERSAND_BYTE: u8 = Self::AMPERSAND_CHAR as u8;

    /// ASCII tab.
    pub const TAB_CHAR: char = '\t';

    /// ASCII tab as a byte.
    pub const TAB_BYTE: u8 = Self::TAB_CHAR as u8;

    /// ASCII backslash.
    pub const BACKSLASH_CHAR: char = '\\';

    /// ASCII backslash as a byte.
    pub const BACKSLASH_BYTE: u8 = Self::BACKSLASH_CHAR as u8;

    /// ASCII single quote.
    pub const SINGLE_QUOTE_CHAR: char = '\'';

    /// ASCII single quote as a byte.
    pub const SINGLE_QUOTE_BYTE: u8 = Self::SINGLE_QUOTE_CHAR as u8;

    /// ASCII back quote.
    pub const BACK_QUOTE_CHAR: char = '`';

    /// ASCII back quote as a byte.
    pub const BACK_QUOTE_BYTE: u8 = Self::BACK_QUOTE_CHAR as u8;

    /// ASCII comma.
    pub const COMMA_CHAR: char = ',';

    /// ASCII comma as a byte.
    pub const COMMA_BYTE: u8 = Self::COMMA_CHAR as u8;

    /// ASCII period.
    pub const PERIOD_CHAR: char = '.';

    /// ASCII period as a byte.
    pub const PERIOD_BYTE: u8 = Self::PERIOD_CHAR as u8;

    /// Minimum printable ASCII character.
    pub const MIN_PRINTABLE_CHAR: char = ' ';

    /// Minimum printable ASCII character as a byte.
    pub const MIN_PRINTABLE_BYTE: u8 = Self::MIN_PRINTABLE_CHAR as u8;

    /// Maximum printable ASCII character.
    pub const MAX_PRINTABLE_CHAR: char = '~';

    /// Maximum printable ASCII character as a byte.
    pub const MAX_PRINTABLE_BYTE: u8 = Self::MAX_PRINTABLE_CHAR as u8;

    /// All printable ASCII characters.
    pub const PRINTABLE_CHARS: [char; 95] = [
        ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5',
        '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
        'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a',
        'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
        'x', 'y', 'z', '{', '|', '}', '~',
    ];

    /// All printable ASCII characters as bytes.
    pub const PRINTABLE_BYTES: [u8; 95] =
        *b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

    /// All ASCII letter characters.
    pub const LETTER_CHARS: [char; 52] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
        'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    /// All ASCII letter characters as bytes.
    pub const LETTER_BYTES: [u8; 52] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    /// All ASCII letter and digit characters.
    pub const LETTER_DIGIT_CHARS: [char; 62] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
        'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    /// All ASCII letter and digit characters as bytes.
    pub const LETTER_DIGIT_BYTES: [u8; 62] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    /// ASCII digit characters.
    pub const DIGIT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    /// ASCII digit bytes.
    pub const DIGIT_BYTES: [u8; 10] = *b"0123456789";

    /// Lowercase ASCII hexadecimal digit characters.
    pub const LOWERCASE_HEX_DIGIT_CHARS: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    /// Lowercase ASCII hexadecimal digit bytes.
    pub const LOWERCASE_HEX_DIGIT_BYTES: [u8; 16] = *b"0123456789abcdef";

    /// Uppercase ASCII hexadecimal digit characters.
    pub const UPPERCASE_HEX_DIGIT_CHARS: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    /// Uppercase ASCII hexadecimal digit bytes.
    pub const UPPERCASE_HEX_DIGIT_BYTES: [u8; 16] = *b"0123456789ABCDEF";

    const CASE_DIFFERENCE: u32 = ('a' as u32) - ('A' as u32);

    /// Tests whether a byte is an ASCII byte.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in the ASCII range `0x00..=0x7F`.
    #[inline(always)]
    #[must_use]
    pub const fn is_ascii_byte(ch: u8) -> bool {
        ch <= Self::MAX_BYTE
    }

    /// Tests whether a character is an ASCII character.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in the ASCII range `U+0000..=U+007F`.
    #[inline(always)]
    #[must_use]
    pub const fn is_ascii_char(ch: char) -> bool {
        ch <= Self::MAX_CHAR
    }

    /// Tests whether an integer value is an ASCII code point.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in the ASCII range `0x00..=0x7F`.
    /// Negative values and values above `0x7F` return `false`.
    #[inline(always)]
    #[must_use]
    pub const fn is_ascii_code_point(ch: u32) -> bool {
        ch <= Self::MAX_CHAR as u32
    }

    /// Tests whether a byte is Java-style ASCII whitespace.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
    #[inline(always)]
    #[must_use]
    pub const fn is_whitespace_byte(ch: u8) -> bool {
        ch == b'\t' || ch == b'\n' || ch == b'\x0c' || ch == b'\r' || ch == b' '
    }

    /// Tests whether a character is Java-style ASCII whitespace.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
    #[inline(always)]
    #[must_use]
    pub const fn is_whitespace_char(ch: char) -> bool {
        ch == '\t' || ch == '\n' || ch == '\u{000c}' || ch == '\r' || ch == ' '
    }

    /// Tests whether a raw code point is Java-style ASCII whitespace.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for tab, line feed, form feed, carriage return, or space.
    /// Other values, including negative values, return `false`.
    #[inline(always)]
    #[must_use]
    pub const fn is_whitespace_code_point(ch: u32) -> bool {
        ch == '\t' as u32 || ch == '\n' as u32 || ch == '\u{000c}' as u32 || ch == '\r' as u32 || ch == ' ' as u32
    }

    /// Tests whether a byte is an ASCII letter.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `A..=Z` or `a..=z`.
    #[inline(always)]
    #[must_use]
    pub const fn is_letter_byte(ch: u8) -> bool {
        Self::is_uppercase_letter_byte(ch) || Self::is_lowercase_letter_byte(ch)
    }

    /// Tests whether a character is an ASCII letter.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `A..=Z` or `a..=z`.
    #[inline(always)]
    #[must_use]
    pub const fn is_letter_char(ch: char) -> bool {
        Self::is_uppercase_letter_char(ch) || Self::is_lowercase_letter_char(ch)
    }

    /// Tests whether a raw code point is an ASCII letter.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `A..=Z` or `a..=z`.
    #[inline(always)]
    #[must_use]
    pub const fn is_letter_code_point(ch: u32) -> bool {
        Self::is_uppercase_letter_code_point(ch) || Self::is_lowercase_letter_code_point(ch)
    }

    /// Tests whether a byte is an uppercase ASCII letter.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `A..=Z`.
    #[inline(always)]
    #[must_use]
    pub const fn is_uppercase_letter_byte(ch: u8) -> bool {
        ch >= b'A' && ch <= b'Z'
    }

    /// Tests whether a character is an uppercase ASCII letter.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `A..=Z`.
    #[inline(always)]
    #[must_use]
    pub const fn is_uppercase_letter_char(ch: char) -> bool {
        ch >= 'A' && ch <= 'Z'
    }

    /// Tests whether a raw code point is an uppercase ASCII letter.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `A..=Z`.
    #[inline(always)]
    #[must_use]
    pub const fn is_uppercase_letter_code_point(ch: u32) -> bool {
        ch >= 'A' as u32 && ch <= 'Z' as u32
    }

    /// Tests whether a byte is a lowercase ASCII letter.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `a..=z`.
    #[inline(always)]
    #[must_use]
    pub const fn is_lowercase_letter_byte(ch: u8) -> bool {
        ch >= b'a' && ch <= b'z'
    }

    /// Tests whether a character is a lowercase ASCII letter.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `a..=z`.
    #[inline(always)]
    #[must_use]
    pub const fn is_lowercase_letter_char(ch: char) -> bool {
        ch >= 'a' && ch <= 'z'
    }

    /// Tests whether a raw code point is a lowercase ASCII letter.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `a..=z`.
    #[inline(always)]
    #[must_use]
    pub const fn is_lowercase_letter_code_point(ch: u32) -> bool {
        ch >= 'a' as u32 && ch <= 'z' as u32
    }

    /// Tests whether a byte is an ASCII decimal digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0..=9`.
    #[inline(always)]
    #[must_use]
    pub const fn is_digit_byte(ch: u8) -> bool {
        ch >= b'0' && ch <= b'9'
    }

    /// Tests whether a character is an ASCII decimal digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0..=9`.
    #[inline(always)]
    #[must_use]
    pub const fn is_digit_char(ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    /// Tests whether a raw code point is an ASCII decimal digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0..=9`.
    #[inline(always)]
    #[must_use]
    pub const fn is_digit_code_point(ch: u32) -> bool {
        ch >= '0' as u32 && ch <= '9' as u32
    }

    /// Tests whether a byte is an ASCII hexadecimal digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0..=9`, `A..=F`, or `a..=f`.
    #[inline(always)]
    #[must_use]
    pub const fn is_hex_digit_byte(ch: u8) -> bool {
        Self::is_digit_byte(ch) || (ch >= b'a' && ch <= b'f') || (ch >= b'A' && ch <= b'F')
    }

    /// Tests whether a character is an ASCII hexadecimal digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0..=9`, `A..=F`, or `a..=f`.
    #[inline(always)]
    #[must_use]
    pub const fn is_hex_digit_char(ch: char) -> bool {
        Self::is_digit_char(ch) || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F')
    }

    /// Tests whether a raw code point is an ASCII hexadecimal digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0..=9`, `A..=F`, or `a..=f`.
    #[inline(always)]
    #[must_use]
    pub const fn is_hex_digit_code_point(ch: u32) -> bool {
        Self::is_digit_code_point(ch)
            || (ch >= 'a' as u32 && ch <= 'f' as u32)
            || (ch >= 'A' as u32 && ch <= 'F' as u32)
    }

    /// Tests whether a byte is an ASCII octal digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0..=7`.
    #[inline(always)]
    #[must_use]
    pub const fn is_octal_digit_byte(ch: u8) -> bool {
        ch >= b'0' && ch <= b'7'
    }

    /// Tests whether a character is an ASCII octal digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0..=7`.
    #[inline(always)]
    #[must_use]
    pub const fn is_octal_digit_char(ch: char) -> bool {
        ch >= '0' && ch <= '7'
    }

    /// Tests whether a raw code point is an ASCII octal digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0..=7`.
    #[inline(always)]
    #[must_use]
    pub const fn is_octal_digit_code_point(ch: u32) -> bool {
        ch >= '0' as u32 && ch <= '7' as u32
    }

    /// Tests whether a byte is an ASCII letter or digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is an ASCII letter or decimal digit.
    #[inline(always)]
    #[must_use]
    pub const fn is_letter_or_digit_byte(ch: u8) -> bool {
        Self::is_letter_byte(ch) || Self::is_digit_byte(ch)
    }

    /// Tests whether a character is an ASCII letter or digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is an ASCII letter or decimal digit.
    #[inline(always)]
    #[must_use]
    pub const fn is_letter_or_digit_char(ch: char) -> bool {
        Self::is_letter_char(ch) || Self::is_digit_char(ch)
    }

    /// Tests whether a raw code point is an ASCII letter or digit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is an ASCII letter or decimal digit.
    #[inline(always)]
    #[must_use]
    pub const fn is_letter_or_digit_code_point(ch: u32) -> bool {
        Self::is_letter_code_point(ch) || Self::is_digit_code_point(ch)
    }

    /// Tests whether a byte is a printable ASCII character.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in the printable ASCII range `0x20..=0x7E`.
    #[inline(always)]
    #[must_use]
    pub const fn is_printable_byte(ch: u8) -> bool {
        ch >= Self::MIN_PRINTABLE_BYTE && ch <= Self::MAX_PRINTABLE_BYTE
    }

    /// Tests whether a character is a printable ASCII character.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in the printable ASCII range `U+0020..=U+007E`.
    #[inline(always)]
    #[must_use]
    pub const fn is_printable_char(ch: char) -> bool {
        ch >= Self::MIN_PRINTABLE_CHAR && ch <= Self::MAX_PRINTABLE_CHAR
    }

    /// Tests whether a raw code point is a printable ASCII character.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in the printable ASCII range `0x20..=0x7E`.
    #[inline(always)]
    #[must_use]
    pub const fn is_printable_code_point(ch: u32) -> bool {
        ch >= Self::MIN_PRINTABLE_CHAR as u32 && ch <= Self::MAX_PRINTABLE_CHAR as u32
    }

    /// Tests whether a byte is an ASCII control character.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `0x00..=0x1F` or `0x7F`.
    #[inline(always)]
    #[must_use]
    pub const fn is_control_byte(ch: u8) -> bool {
        ch < Self::MIN_PRINTABLE_BYTE || ch == Self::DELETE_BYTE
    }

    /// Tests whether a character is an ASCII control character.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `U+0000..=U+001F` or `U+007F`.
    #[inline(always)]
    #[must_use]
    pub const fn is_control_char(ch: char) -> bool {
        (ch < Self::MIN_PRINTABLE_CHAR) || ch == Self::DELETE_CHAR
    }

    /// Tests whether a raw code point is an ASCII control character.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to test.
    ///
    /// # Returns
    ///
    /// Returns `true` for `0x00..=0x1F` or `0x7F`.
    #[inline(always)]
    #[must_use]
    pub const fn is_control_code_point(ch: u32) -> bool {
        (ch < Self::MIN_PRINTABLE_CHAR as u32) || ch == Self::DELETE_CHAR as u32
    }

    /// Compares two bytes while ignoring ASCII case.
    ///
    /// # Parameters
    ///
    /// - `ch1`: The first byte to compare.
    /// - `ch2`: The second byte to compare.
    ///
    /// # Returns
    ///
    /// Returns `true` if the bytes are equal after converting ASCII uppercase
    /// letters to lowercase. Non-ASCII bytes are compared unchanged.
    #[inline(always)]
    #[must_use]
    pub const fn equals_ignore_case_byte(ch1: u8, ch2: u8) -> bool {
        if ch1 == ch2 {
            true
        } else {
            Self::byte_to_lowercase(ch1) == Self::byte_to_lowercase(ch2)
        }
    }

    /// Compares two characters while ignoring ASCII case.
    ///
    /// # Parameters
    ///
    /// - `ch1`: The first character to compare.
    /// - `ch2`: The second character to compare.
    ///
    /// # Returns
    ///
    /// Returns `true` if the characters are equal after converting ASCII
    /// uppercase letters to lowercase. Non-ASCII characters are compared
    /// unchanged.
    #[inline(always)]
    #[must_use]
    pub const fn equals_ignore_case_char(ch1: char, ch2: char) -> bool {
        if ch1 == ch2 {
            true
        } else {
            Self::char_to_lowercase(ch1) == Self::char_to_lowercase(ch2)
        }
    }

    /// Compares two raw code points while ignoring ASCII case.
    ///
    /// # Parameters
    ///
    /// - `ch1`: The first raw code point value to compare.
    /// - `ch2`: The second raw code point value to compare.
    ///
    /// # Returns
    ///
    /// Returns `true` if the values are equal after converting ASCII uppercase
    /// letters to lowercase. Values outside ASCII are compared unchanged.
    #[inline(always)]
    #[must_use]
    pub const fn equals_ignore_case_code_point(ch1: u32, ch2: u32) -> bool {
        if ch1 == ch2 {
            true
        } else {
            Self::code_point_to_lowercase(ch1) == Self::code_point_to_lowercase(ch2)
        }
    }

    /// Converts a byte to uppercase using ASCII case rules.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns the uppercase ASCII equivalent for `a..=z`; all other bytes are
    /// returned unchanged.
    #[inline(always)]
    #[must_use]
    pub const fn byte_to_uppercase(ch: u8) -> u8 {
        if ch >= b'a' && ch <= b'z' {
            ch - (Self::CASE_DIFFERENCE as u8)
        } else {
            ch
        }
    }

    /// Converts a character to uppercase using ASCII case rules.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to convert.
    ///
    /// # Returns
    ///
    /// Returns the uppercase ASCII equivalent for `a..=z`; all other characters
    /// are returned unchanged.
    #[inline(always)]
    #[must_use]
    pub const fn char_to_uppercase(ch: char) -> char {
        if ch >= 'a' && ch <= 'z' {
            ((ch as u8) - (Self::CASE_DIFFERENCE as u8)) as char
        } else {
            ch
        }
    }

    /// Converts a raw code point to uppercase using ASCII case rules.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to convert.
    ///
    /// # Returns
    ///
    /// Returns the uppercase ASCII equivalent for `a..=z`; all other values are
    /// returned unchanged.
    #[inline(always)]
    #[must_use]
    pub const fn code_point_to_uppercase(ch: u32) -> u32 {
        if ch >= 'a' as u32 && ch <= 'z' as u32 {
            ch - Self::CASE_DIFFERENCE
        } else {
            ch
        }
    }

    /// Converts a byte to lowercase using ASCII case rules.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns the lowercase ASCII equivalent for `A..=Z`; all other bytes are
    /// returned unchanged.
    #[inline(always)]
    #[must_use]
    pub const fn byte_to_lowercase(ch: u8) -> u8 {
        if ch >= b'A' && ch <= b'Z' {
            ch + (Self::CASE_DIFFERENCE as u8)
        } else {
            ch
        }
    }

    /// Converts a character to lowercase using ASCII case rules.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to convert.
    ///
    /// # Returns
    ///
    /// Returns the lowercase ASCII equivalent for `A..=Z`; all other characters
    /// are returned unchanged.
    #[inline(always)]
    #[must_use]
    pub const fn char_to_lowercase(ch: char) -> char {
        if ch >= 'A' && ch <= 'Z' {
            ((ch as u8) + (Self::CASE_DIFFERENCE as u8)) as char
        } else {
            ch
        }
    }

    /// Converts a raw code point to lowercase using ASCII case rules.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to convert.
    ///
    /// # Returns
    ///
    /// Returns the lowercase ASCII equivalent for `A..=Z`; all other values are
    /// returned unchanged.
    #[inline(always)]
    #[must_use]
    pub const fn code_point_to_lowercase(ch: u32) -> u32 {
        if ch >= 'A' as u32 && ch <= 'Z' as u32 {
            ch + Self::CASE_DIFFERENCE
        } else {
            ch
        }
    }

    /// Converts an ASCII decimal digit byte into its numeric value.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=9)` for `0..=9`; returns `None` otherwise.
    #[inline(always)]
    #[must_use]
    pub const fn byte_to_digit(ch: u8) -> Option<u8> {
        if Self::is_digit_byte(ch) { Some(ch - b'0') } else { None }
    }

    /// Converts an ASCII decimal digit character into its numeric value.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=9)` for `0..=9`; returns `None` otherwise.
    #[inline(always)]
    #[must_use]
    pub const fn char_to_digit(ch: char) -> Option<u8> {
        if Self::is_digit_char(ch) {
            Some((ch as u8) - b'0')
        } else {
            None
        }
    }

    /// Converts an ASCII decimal digit code point into its numeric value.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=9)` for `0..=9`; returns `None` otherwise.
    #[inline(always)]
    #[must_use]
    pub const fn code_point_to_digit(ch: u32) -> Option<u8> {
        if Self::is_digit_code_point(ch) {
            Some((ch - '0' as u32) as u8)
        } else {
            None
        }
    }

    /// Converts an ASCII hexadecimal digit byte into its numeric value.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=15)` for `0..=9`, `A..=F`, or `a..=f`; returns `None`
    /// otherwise.
    #[inline(always)]
    #[must_use]
    pub const fn byte_to_hex_digit(ch: u8) -> Option<u8> {
        if ch >= b'0' && ch <= b'9' {
            Some(ch - b'0')
        } else if ch >= b'A' && ch <= b'F' {
            Some(ch - (b'A' - 10))
        } else if ch >= b'a' && ch <= b'f' {
            Some(ch - (b'a' - 10))
        } else {
            None
        }
    }

    /// Converts an ASCII hexadecimal digit character into its numeric value.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=15)` for `0..=9`, `A..=F`, or `a..=f`; returns `None`
    /// otherwise.
    #[inline(always)]
    #[must_use]
    pub const fn char_to_hex_digit(ch: char) -> Option<u8> {
        if ch >= '0' && ch <= '9' {
            Some((ch as u8) - b'0')
        } else if ch >= 'A' && ch <= 'F' {
            Some((ch as u8) - (b'A' - 10))
        } else if ch >= 'a' && ch <= 'f' {
            Some((ch as u8) - (b'a' - 10))
        } else {
            None
        }
    }

    /// Converts an ASCII hexadecimal digit code point into its numeric value.
    ///
    /// # Parameters
    ///
    /// - `ch`: The raw code point value to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(0..=15)` for `0..=9`, `A..=F`, or `a..=f`; returns `None`
    /// otherwise.
    #[inline(always)]
    #[must_use]
    pub const fn code_point_to_hex_digit(ch: u32) -> Option<u8> {
        if ch >= '0' as u32 && ch <= '9' as u32 {
            Some((ch - '0' as u32) as u8)
        } else if ch >= 'A' as u32 && ch <= 'F' as u32 {
            Some((ch - ('A' as u32 - 10)) as u8)
        } else if ch >= 'a' as u32 && ch <= 'f' as u32 {
            Some((ch - ('a' as u32 - 10)) as u8)
        } else {
            None
        }
    }

    /// Folds a Unicode character to its ASCII replacement.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to fold.
    /// - `result`: The caller-provided output buffer that receives the folded
    ///   characters.
    /// - `offset`: The starting index in `result` at which folded characters are
    ///   written.
    ///
    /// # Returns
    ///
    /// Returns the number of characters written to `result` starting at
    /// `offset`. ASCII characters and unmapped non-ASCII characters write one
    /// character. Mapped characters write up to [`Self::MAX_FOLDING_COUNT`] ASCII
    /// characters.
    ///
    /// # Panics
    ///
    /// Panics if `result` has fewer than [`Self::MAX_FOLDING_COUNT`] writable slots
    /// after `offset`.
    pub fn fold(ch: char, result: &mut [char], offset: usize) -> usize {
        assert!(
            result.len().saturating_sub(offset) >= Self::MAX_FOLDING_COUNT,
            "ASCII folding output needs at least MAX_FOLDING_COUNT slots"
        );
        if ch.is_ascii() {
            result[offset] = ch;
            return 1;
        }
        match ascii_folding::fold_replacement(ch) {
            Some(replacement) => {
                for (index, replacement_char) in replacement.chars().enumerate() {
                    result[offset + index] = replacement_char;
                }
                replacement.len()
            }
            None => {
                result[offset] = ch;
                1
            }
        }
    }

    /// Folds a Unicode character into an owned string.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to fold.
    ///
    /// # Returns
    ///
    /// Returns the folded ASCII replacement as a `String`. If `ch` has no
    /// folding mapping, the returned string contains `ch` unchanged.
    #[must_use]
    pub fn fold_to_string(ch: char) -> String {
        let mut buffer = ['\0'; Self::MAX_FOLDING_COUNT];
        let count = Self::fold(ch, &mut buffer, 0);
        buffer[..count].iter().collect()
    }
}
