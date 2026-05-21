/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use crate::{
    ParsingPosition,
    Unicode,
    UnicodeError,
    UnicodeErrorKind,
    UnicodeResult,
};

/// Namespace for low-level UTF-8 helpers.
///
/// Decoding rejects every byte sequence that is not well-formed under
/// [Unicode Standard, Table 3-7], including overlong encodings, surrogate
/// code points, and code points above `U+10FFFF`.
///
/// [Unicode Standard, Table 3-7]: https://www.unicode.org/versions/latest/core-spec/chapter-3/#G7404
pub enum Utf8 {}

impl Utf8 {
    /// Maximum number of UTF-8 code units needed for a scalar value.
    pub const MAX_CODE_UNIT_COUNT: usize = 4;

    /// Minimum leading UTF-8 byte.
    pub const MIN_LEADING: u8 = 0xc2;

    /// Maximum leading UTF-8 byte.
    pub const MAX_LEADING: u8 = 0xf4;

    /// Mask for trailing UTF-8 bytes.
    pub const TRAILING_MASK: u8 = 0xc0;

    /// Pattern for trailing UTF-8 bytes.
    pub const TRAILING_PATTERN: u8 = 0x80;

    /// Maximum code point encoded by one UTF-8 byte.
    pub const MAX_ONE_CODE_UNIT: u32 = 0x7f;

    /// Maximum code point encoded by two UTF-8 bytes.
    pub const MAX_TWO_CODE_UNIT: u32 = 0x7ff;

    /// Maximum code point encoded by three UTF-8 bytes.
    pub const MAX_THREE_CODE_UNIT: u32 = 0xffff;

    /// Maximum code point encoded by four UTF-8 bytes.
    pub const MAX_FOUR_CODE_UNIT: u32 = Unicode::UNICODE_MAX;

    /// Tests whether a byte encodes a scalar value by itself.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is an ASCII byte and therefore a complete UTF-8
    /// code point by itself.
    #[inline]
    #[must_use]
    pub const fn is_single(ch: u8) -> bool {
        ch <= Self::MAX_ONE_CODE_UNIT as u8
    }

    /// Tests whether a byte can be a leading UTF-8 byte.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in the valid UTF-8 leading byte range
    /// `0xC2..=0xF4`.
    #[inline]
    #[must_use]
    pub const fn is_leading(ch: u8) -> bool {
        ch >= Self::MIN_LEADING && ch <= Self::MAX_LEADING
    }

    /// Tests whether a byte is a trailing UTF-8 byte.
    ///
    /// # Parameters
    ///
    /// - `ch`: The byte to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` matches the UTF-8 continuation byte pattern
    /// `10xxxxxx`.
    #[inline]
    #[must_use]
    pub const fn is_trailing(ch: u8) -> bool {
        (ch & Self::TRAILING_MASK) == Self::TRAILING_PATTERN
    }

    /// Returns the number of trailing bytes required by a leading byte.
    ///
    /// # Parameters
    ///
    /// - `ch`: The candidate leading UTF-8 byte.
    ///
    /// # Returns
    ///
    /// Returns `Some(1)`, `Some(2)`, or `Some(3)` for a valid leading byte.
    /// Returns `None` if `ch` is not a valid UTF-8 leading byte.
    #[inline]
    #[must_use]
    pub const fn trailing_count(ch: u8) -> Option<usize> {
        if ch >= 0xc2 && ch <= 0xdf {
            Some(1)
        } else if ch >= 0xe0 && ch <= 0xef {
            Some(2)
        } else if ch >= 0xf0 && ch <= 0xf4 {
            Some(3)
        } else {
            None
        }
    }

    /// Returns the number of UTF-8 bytes needed for a scalar value.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The Unicode code point to size.
    ///
    /// # Returns
    ///
    /// Returns `Some(1..=4)` for a valid Unicode scalar value. Returns `None`
    /// if `code_point` is above `0x10FFFF` or is a UTF-16 surrogate value.
    #[inline]
    #[must_use]
    pub const fn code_unit_count(code_point: u32) -> Option<usize> {
        if code_point > Unicode::UNICODE_MAX || Unicode::is_surrogate(code_point as i32) {
            None
        } else if code_point <= Self::MAX_ONE_CODE_UNIT {
            Some(1)
        } else if code_point <= Self::MAX_TWO_CODE_UNIT {
            Some(2)
        } else if code_point <= Self::MAX_THREE_CODE_UNIT {
            Some(3)
        } else {
            Some(4)
        }
    }

    /// Moves a cursor from a trailing byte to the start of its UTF-8 code point.
    ///
    /// If the cursor is not currently on a trailing byte, the cursor is left
    /// unchanged and `Ok(0)` is returned.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor to inspect and possibly move.
    /// - `buffer`: The UTF-8 byte buffer.
    /// - `start_index`: The lower bound, inclusive, for backward scanning.
    ///
    /// # Returns
    ///
    /// Returns `Ok(count)` with the number of trailing bytes skipped when the
    /// start byte is found. Returns `Ok(0)` if no movement is needed.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Malformed` if bytes before the cursor cannot
    /// form a valid UTF-8 sequence. Returns `UnicodeErrorKind::Incomplete` if
    /// the scan reaches `start_index` before finding the leading byte.
    ///
    /// # Panics
    ///
    /// Panics if `start_index > pos.index()` or `pos.index() >= buffer.len()`.
    pub fn set_to_start(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        start_index: usize,
    ) -> UnicodeResult<usize> {
        let index = pos.index();
        assert!(start_index <= index && index < buffer.len());
        if !Self::is_trailing(buffer[index]) {
            return Ok(0);
        }
        for candidate in (start_index..index).rev() {
            let byte = buffer[candidate];
            if Self::is_leading(byte) {
                let expected = Self::trailing_count(byte).expect("leading byte has trailing count");
                let actual = index - candidate;
                if actual <= expected {
                    pos.set_index(candidate);
                    return Ok(actual);
                }
                return UnicodeError::fail(pos, candidate, UnicodeErrorKind::Malformed);
            }
            if !Self::is_trailing(byte) {
                return UnicodeError::fail(pos, candidate, UnicodeErrorKind::Malformed);
            }
            if index - candidate >= Self::MAX_CODE_UNIT_COUNT - 1 {
                return UnicodeError::fail(pos, candidate, UnicodeErrorKind::Malformed);
            }
        }
        UnicodeError::fail(pos, start_index, UnicodeErrorKind::Incomplete)
    }

    /// Moves a cursor from a leading byte to the terminal byte of its UTF-8 code point.
    ///
    /// If the cursor is not currently on a leading byte, the cursor is left
    /// unchanged and `Ok(0)` is returned.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor to inspect and possibly move.
    /// - `buffer`: The UTF-8 byte buffer.
    /// - `end_index`: The upper bound, exclusive, for forward scanning.
    ///
    /// # Returns
    ///
    /// Returns `Ok(count)` with the number of trailing bytes in the current code
    /// point. Returns `Ok(0)` if no movement is needed.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Incomplete` if the required trailing bytes
    /// would extend to or beyond `end_index`. Returns `UnicodeErrorKind::Malformed`
    /// if a required trailing byte is not a UTF-8 continuation byte.
    ///
    /// # Panics
    ///
    /// Panics if `end_index > buffer.len()` or `pos.index() > end_index`.
    pub fn set_to_terminal(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        let index = pos.index();
        assert!(end_index <= buffer.len() && index <= end_index);
        if index == end_index || !Self::is_leading(buffer[index]) {
            return Ok(0);
        }
        let trailing_count = Self::trailing_count(buffer[index]).expect("leading byte");
        let terminal_index = index + trailing_count;
        if terminal_index >= end_index {
            return UnicodeError::fail(pos, end_index, UnicodeErrorKind::Incomplete);
        }
        for (current, byte) in buffer
            .iter()
            .enumerate()
            .take(terminal_index + 1)
            .skip(index + 1)
        {
            if !Self::is_trailing(*byte) {
                return UnicodeError::fail(pos, current, UnicodeErrorKind::Malformed);
            }
        }
        pos.set_index(terminal_index);
        Ok(trailing_count)
    }

    /// Advances the cursor over one UTF-8 code point.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor to advance.
    /// - `buffer`: The UTF-8 byte buffer.
    /// - `end_index`: The upper bound, exclusive, for decoding.
    ///
    /// # Returns
    ///
    /// Returns `Ok(count)` with the number of bytes skipped. Returns `Ok(0)` if
    /// the cursor is already at `end_index`.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::get_next`] when the next sequence is
    /// malformed or incomplete.
    ///
    /// # Panics
    ///
    /// Panics under the same conditions as [`Self::get_next`].
    #[inline]
    pub fn forward(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        let old_index = pos.index();
        match Self::get_next(pos, buffer, end_index)? {
            Some(_) => Ok(pos.index() - old_index),
            None => Ok(0),
        }
    }

    /// Moves the cursor backward over one UTF-8 code point.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor to move backward.
    /// - `buffer`: The UTF-8 byte buffer.
    /// - `start_index`: The lower bound, inclusive, for backward scanning.
    ///
    /// # Returns
    ///
    /// Returns `Ok(count)` with the number of bytes moved. Returns `Ok(0)` if
    /// the cursor is already at `start_index`.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Malformed` if the bytes before the cursor do
    /// not form a valid UTF-8 code point. Returns `UnicodeErrorKind::Incomplete`
    /// if the scan reaches `start_index` before a complete code point is found.
    ///
    /// # Panics
    ///
    /// Panics if `start_index > pos.index()` or `pos.index() > buffer.len()`.
    pub fn backward(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        start_index: usize,
    ) -> UnicodeResult<usize> {
        let index = pos.index();
        assert!(start_index <= index && index <= buffer.len());
        if index == start_index {
            return Ok(0);
        }
        let previous_index = index - 1;
        let previous = buffer[previous_index];
        if Self::is_single(previous) {
            pos.set_index(previous_index);
            return Ok(1);
        }
        if !Self::is_trailing(previous) {
            return UnicodeError::fail(pos, previous_index, UnicodeErrorKind::Malformed);
        }
        for candidate in (start_index..previous_index).rev() {
            let byte = buffer[candidate];
            if Self::is_leading(byte) {
                let expected = Self::trailing_count(byte).expect("leading byte") + 1;
                let actual = index - candidate;
                if actual == expected {
                    pos.set_index(candidate);
                    return Ok(actual);
                }
                return UnicodeError::fail(pos, candidate, UnicodeErrorKind::Malformed);
            }
            if !Self::is_trailing(byte) {
                return UnicodeError::fail(pos, candidate, UnicodeErrorKind::Malformed);
            }
            if index - candidate >= Self::MAX_CODE_UNIT_COUNT {
                return UnicodeError::fail(pos, candidate, UnicodeErrorKind::Malformed);
            }
        }
        UnicodeError::fail(pos, start_index, UnicodeErrorKind::Incomplete)
    }

    /// Reads the next UTF-8 code point and advances the cursor.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor pointing at the next byte to decode.
    /// - `buffer`: The UTF-8 byte buffer.
    /// - `end_index`: The upper bound, exclusive, for decoding.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(ch))` and advances `pos` past the decoded character when
    /// a complete UTF-8 sequence is available. Returns `Ok(None)` if
    /// `pos.index() == end_index`.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Malformed` if the next bytes do not form a
    /// valid UTF-8 sequence. Returns `UnicodeErrorKind::Incomplete` if the next
    /// sequence is valid so far but reaches `end_index` before completion. The
    /// error is also recorded in `pos`.
    ///
    /// # Panics
    ///
    /// Panics if `end_index > buffer.len()` or `pos.index() > end_index`.
    pub fn get_next(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        end_index: usize,
    ) -> UnicodeResult<Option<char>> {
        let index = pos.index();
        assert!(end_index <= buffer.len() && index <= end_index);
        if index == end_index {
            return Ok(None);
        }
        let c0 = buffer[index];
        if Self::is_single(c0) {
            pos.set_index(index + 1);
            return Ok(Some(c0 as char));
        }
        if c0 < 0xc2 {
            return UnicodeError::fail(pos, index, UnicodeErrorKind::Malformed);
        }
        if c0 <= 0xdf {
            if end_index - index < 2 {
                return UnicodeError::fail(pos, end_index, UnicodeErrorKind::Incomplete);
            }
            let p1 = index + 1;
            let c1 = buffer[p1];
            if !Self::is_trailing(c1) {
                return UnicodeError::fail(pos, p1, UnicodeErrorKind::Malformed);
            }
            let code_point = (((c0 & 0x1f) as u32) << 6) | ((c1 & 0x3f) as u32);
            let ch = char::from_u32(code_point).expect("validated UTF-8 scalar value");
            pos.set_index(index + 2);
            return Ok(Some(ch));
        }
        if c0 <= 0xef {
            if end_index - index < 3 {
                return UnicodeError::fail(pos, end_index, UnicodeErrorKind::Incomplete);
            }
            let p1 = index + 1;
            let c1 = buffer[p1];
            if !Self::is_trailing(c1) || (c0 == 0xe0 && c1 < 0xa0) || (c0 == 0xed && c1 > 0x9f) {
                return UnicodeError::fail(pos, p1, UnicodeErrorKind::Malformed);
            }
            let p2 = index + 2;
            let c2 = buffer[p2];
            if !Self::is_trailing(c2) {
                return UnicodeError::fail(pos, p2, UnicodeErrorKind::Malformed);
            }
            let code_point =
                (((c0 & 0x0f) as u32) << 12) | (((c1 & 0x3f) as u32) << 6) | ((c2 & 0x3f) as u32);
            let ch = char::from_u32(code_point).expect("validated UTF-8 scalar value");
            pos.set_index(index + 3);
            return Ok(Some(ch));
        }
        if c0 <= 0xf4 {
            if end_index - index < 4 {
                return UnicodeError::fail(pos, end_index, UnicodeErrorKind::Incomplete);
            }
            let p1 = index + 1;
            let c1 = buffer[p1];
            if !Self::is_trailing(c1) || (c0 == 0xf0 && c1 < 0x90) || (c0 == 0xf4 && c1 > 0x8f) {
                return UnicodeError::fail(pos, p1, UnicodeErrorKind::Malformed);
            }
            let p2 = index + 2;
            let c2 = buffer[p2];
            if !Self::is_trailing(c2) {
                return UnicodeError::fail(pos, p2, UnicodeErrorKind::Malformed);
            }
            let p3 = index + 3;
            let c3 = buffer[p3];
            if !Self::is_trailing(c3) {
                return UnicodeError::fail(pos, p3, UnicodeErrorKind::Malformed);
            }
            let code_point = (((c0 & 0x07) as u32) << 18)
                | (((c1 & 0x3f) as u32) << 12)
                | (((c2 & 0x3f) as u32) << 6)
                | ((c3 & 0x3f) as u32);
            let ch = char::from_u32(code_point).expect("validated UTF-8 scalar value");
            pos.set_index(index + 4);
            return Ok(Some(ch));
        }
        UnicodeError::fail(pos, index, UnicodeErrorKind::Malformed)
    }

    /// Reads the previous UTF-8 code point and moves the cursor to its start.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor positioned after the code point to read.
    /// - `buffer`: The UTF-8 byte buffer.
    /// - `start_index`: The lower bound, inclusive, for backward decoding.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(ch))` and moves `pos` to the first byte of that
    /// character. Returns `Ok(None)` if `pos.index() == start_index`.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Malformed` or `UnicodeErrorKind::Incomplete`
    /// if the bytes before the cursor do not form a complete valid UTF-8 code
    /// point. The original cursor index is restored before returning an error,
    /// and the error is recorded in `pos`.
    ///
    /// # Panics
    ///
    /// Panics under the same conditions as [`Self::backward`].
    pub fn get_previous(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        start_index: usize,
    ) -> UnicodeResult<Option<char>> {
        let old_index = pos.index();
        let count = Self::backward(pos, buffer, start_index)?;
        if count == 0 {
            return Ok(None);
        }
        let new_index = pos.index();
        let mut read_pos = ParsingPosition::new(new_index);
        match Self::get_next(&mut read_pos, buffer, old_index) {
            Ok(Some(ch)) => {
                pos.set_index(new_index);
                Ok(Some(ch))
            }
            Ok(None) => unreachable!("backward moved over a code point"),
            Err(error) => {
                pos.reset(old_index);
                pos.set_error(error.index(), error.kind());
                Err(error)
            }
        }
    }

    /// Encodes a scalar value into a UTF-8 output buffer.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The Unicode scalar value to encode.
    /// - `index`: The starting index in `buffer` at which bytes are written.
    /// - `buffer`: The caller-provided output buffer.
    /// - `end_index`: The upper bound, exclusive, for writing.
    ///
    /// # Returns
    ///
    /// Returns `Ok(count)` with the number of bytes written.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Malformed` if `code_point` is not a valid
    /// Unicode scalar value. Returns `UnicodeErrorKind::BufferOverflow` if the
    /// encoded bytes would extend past `end_index`.
    ///
    /// # Panics
    ///
    /// Panics if `end_index > buffer.len()` or `index > end_index`.
    pub fn put(
        code_point: u32,
        index: usize,
        buffer: &mut [u8],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        assert!(end_index <= buffer.len() && index <= end_index);
        let count = match Self::code_unit_count(code_point) {
            Some(count) => count,
            None => {
                return Err(UnicodeError::new(UnicodeErrorKind::Malformed, index));
            }
        };
        if end_index - index < count {
            return Err(UnicodeError::new(
                UnicodeErrorKind::BufferOverflow,
                end_index,
            ));
        }
        if code_point <= Self::MAX_ONE_CODE_UNIT {
            buffer[index] = code_point as u8;
        } else if code_point <= Self::MAX_TWO_CODE_UNIT {
            buffer[index] = ((code_point >> 6) as u8) | 0xc0;
            buffer[index + 1] = ((code_point & 0x3f) as u8) | 0x80;
        } else if code_point <= Self::MAX_THREE_CODE_UNIT {
            buffer[index] = ((code_point >> 12) as u8) | 0xe0;
            buffer[index + 1] = (((code_point >> 6) & 0x3f) as u8) | 0x80;
            buffer[index + 2] = ((code_point & 0x3f) as u8) | 0x80;
        } else {
            buffer[index] = ((code_point >> 18) as u8) | 0xf0;
            buffer[index + 1] = (((code_point >> 12) & 0x3f) as u8) | 0x80;
            buffer[index + 2] = (((code_point >> 6) & 0x3f) as u8) | 0x80;
            buffer[index + 3] = ((code_point & 0x3f) as u8) | 0x80;
        }
        Ok(count)
    }

    /// Encodes a Rust character into a UTF-8 output buffer.
    ///
    /// # Parameters
    ///
    /// - `ch`: The Unicode scalar value to encode.
    /// - `index`: The starting index in `buffer` at which bytes are written.
    /// - `buffer`: The caller-provided output buffer.
    /// - `end_index`: The upper bound, exclusive, for writing.
    ///
    /// # Returns
    ///
    /// Returns `Ok(count)` with the number of UTF-8 bytes written.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::BufferOverflow` if the encoded bytes would
    /// extend past `end_index`.
    ///
    /// # Panics
    ///
    /// Panics under the same conditions as [`Self::put`].
    #[inline]
    pub fn put_char(
        ch: char,
        index: usize,
        buffer: &mut [u8],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        Self::put(ch as u32, index, buffer, end_index)
    }
}
