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

/// Namespace for low-level UTF-16 helpers.
pub enum Utf16 {}

impl Utf16 {
    /// Maximum number of UTF-16 code units needed for a scalar value.
    pub const MAX_CODE_UNIT_COUNT: usize = 2;

    /// Tests whether a UTF-16 code unit encodes a scalar value by itself.
    ///
    /// # Parameters
    ///
    /// - `ch`: The UTF-16 code unit to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is not a surrogate code unit.
    #[inline]
    #[must_use]
    pub const fn is_single(ch: u16) -> bool {
        !Self::is_surrogate(ch)
    }

    /// Tests whether a UTF-16 code unit is a high surrogate.
    ///
    /// # Parameters
    ///
    /// - `ch`: The UTF-16 code unit to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0xD800..=0xDBFF`.
    #[inline]
    #[must_use]
    pub const fn is_leading(ch: u16) -> bool {
        Unicode::is_high_surrogate(ch as i32)
    }

    /// Tests whether a UTF-16 code unit is a low surrogate.
    ///
    /// # Parameters
    ///
    /// - `ch`: The UTF-16 code unit to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0xDC00..=0xDFFF`.
    #[inline]
    #[must_use]
    pub const fn is_trailing(ch: u16) -> bool {
        Unicode::is_low_surrogate(ch as i32)
    }

    /// Tests whether a UTF-16 code unit is any surrogate.
    ///
    /// # Parameters
    ///
    /// - `ch`: The UTF-16 code unit to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `ch` is in `0xD800..=0xDFFF`.
    #[inline]
    #[must_use]
    pub const fn is_surrogate(ch: u16) -> bool {
        Unicode::is_surrogate(ch as i32)
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
        Unicode::is_surrogate_pair(high, low)
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
    /// Returns `Some(code_point)` if the two code units form a valid surrogate
    /// pair. Returns `None` otherwise.
    #[inline]
    #[must_use]
    pub const fn compose(high: u16, low: u16) -> Option<u32> {
        Unicode::compose_surrogate_pair(high, low)
    }

    /// Decomposes a supplementary code point into its high surrogate.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The supplementary code point to decompose.
    ///
    /// # Returns
    ///
    /// Returns `Some(high_surrogate)` for a supplementary code point. Returns
    /// `None` for BMP and out-of-range values.
    #[inline]
    #[must_use]
    pub const fn decompose_high(code_point: u32) -> Option<u16> {
        Unicode::decompose_high_surrogate(code_point)
    }

    /// Decomposes a supplementary code point into its low surrogate.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The supplementary code point to decompose.
    ///
    /// # Returns
    ///
    /// Returns `Some(low_surrogate)` for a supplementary code point. Returns
    /// `None` for BMP and out-of-range values.
    #[inline]
    #[must_use]
    pub const fn decompose_low(code_point: u32) -> Option<u16> {
        Unicode::decompose_low_surrogate(code_point)
    }

    /// Returns the number of trailing UTF-16 code units for this leading unit.
    ///
    /// # Parameters
    ///
    /// - `ch`: The UTF-16 code unit to inspect.
    ///
    /// # Returns
    ///
    /// Returns `1` if `ch` is a high surrogate and therefore requires one low
    /// surrogate. Returns `0` for all other code units.
    #[inline]
    #[must_use]
    pub const fn trailing_count(ch: u16) -> usize {
        if Self::is_leading(ch) { 1 } else { 0 }
    }

    /// Returns the number of UTF-16 code units needed for a scalar value.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The code point to size.
    ///
    /// # Returns
    ///
    /// Returns `Some(1)` for BMP scalar values and `Some(2)` for supplementary
    /// scalar values. Returns `None` for surrogate values or values above
    /// `0x10FFFF`.
    #[inline]
    #[must_use]
    pub const fn code_unit_count(code_point: u32) -> Option<usize> {
        if code_point > Unicode::UNICODE_MAX || Unicode::is_surrogate(code_point as i32) {
            None
        } else if code_point >= Unicode::SUPPLEMENTARY_MIN {
            Some(2)
        } else {
            Some(1)
        }
    }

    /// Moves a cursor from a trailing unit to the start of its UTF-16 code point.
    ///
    /// If the cursor is not currently on a low surrogate, the cursor is left
    /// unchanged and `Ok(0)` is returned.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor to inspect and possibly move.
    /// - `buffer`: The UTF-16 code-unit buffer.
    /// - `start_index`: The lower bound, inclusive, for backward scanning.
    ///
    /// # Returns
    ///
    /// Returns `Ok(1)` if the cursor is moved from a low surrogate to its high
    /// surrogate. Returns `Ok(0)` if no movement is needed.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Incomplete` if the low surrogate appears at
    /// `start_index`. Returns `UnicodeErrorKind::Malformed` if the previous code
    /// unit is not a high surrogate.
    ///
    /// # Panics
    ///
    /// Panics if `start_index > pos.index()` or `pos.index() >= buffer.len()`.
    pub fn set_to_start(
        pos: &mut ParsingPosition,
        buffer: &[u16],
        start_index: usize,
    ) -> UnicodeResult<usize> {
        let index = pos.index();
        assert!(start_index <= index && index < buffer.len());
        if !Self::is_trailing(buffer[index]) {
            return Ok(0);
        }
        if index == start_index {
            return UnicodeError::fail(pos, index, UnicodeErrorKind::Incomplete);
        }
        let leading_index = index - 1;
        if Self::is_leading(buffer[leading_index]) {
            pos.set_index(leading_index);
            Ok(1)
        } else {
            UnicodeError::fail(pos, leading_index, UnicodeErrorKind::Malformed)
        }
    }

    /// Moves a cursor from a leading unit to the terminal unit of its UTF-16 code point.
    ///
    /// If the cursor is not currently on a high surrogate, the cursor is left
    /// unchanged and `Ok(0)` is returned.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor to inspect and possibly move.
    /// - `buffer`: The UTF-16 code-unit buffer.
    /// - `end_index`: The upper bound, exclusive, for forward scanning.
    ///
    /// # Returns
    ///
    /// Returns `Ok(1)` if the cursor is moved from a high surrogate to its low
    /// surrogate. Returns `Ok(0)` if no movement is needed.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Incomplete` if the required low surrogate
    /// would be at or beyond `end_index`. Returns `UnicodeErrorKind::Malformed`
    /// if the following code unit is not a low surrogate.
    ///
    /// # Panics
    ///
    /// Panics if `end_index > buffer.len()` or `pos.index() > end_index`.
    pub fn set_to_terminal(
        pos: &mut ParsingPosition,
        buffer: &[u16],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        let index = pos.index();
        assert!(end_index <= buffer.len() && index <= end_index);
        if index == end_index || !Self::is_leading(buffer[index]) {
            return Ok(0);
        }
        let trailing_index = index + 1;
        if trailing_index >= end_index {
            return UnicodeError::fail(pos, trailing_index, UnicodeErrorKind::Incomplete);
        }
        if Self::is_trailing(buffer[trailing_index]) {
            pos.set_index(trailing_index);
            Ok(1)
        } else {
            UnicodeError::fail(pos, trailing_index, UnicodeErrorKind::Malformed)
        }
    }

    /// Advances the cursor over one UTF-16 code point.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor to advance.
    /// - `buffer`: The UTF-16 code-unit buffer.
    /// - `end_index`: The upper bound, exclusive, for decoding.
    ///
    /// # Returns
    ///
    /// Returns `Ok(count)` with the number of code units skipped. Returns
    /// `Ok(0)` if the cursor is already at `end_index`.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::get_next`] when the next code-unit
    /// sequence is malformed or incomplete.
    ///
    /// # Panics
    ///
    /// Panics under the same conditions as [`Self::get_next`].
    #[inline]
    pub fn forward(
        pos: &mut ParsingPosition,
        buffer: &[u16],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        let old_index = pos.index();
        match Self::get_next(pos, buffer, end_index)? {
            Some(_) => Ok(pos.index() - old_index),
            None => Ok(0),
        }
    }

    /// Moves the cursor backward over one UTF-16 code point.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor to move backward.
    /// - `buffer`: The UTF-16 code-unit buffer.
    /// - `start_index`: The lower bound, inclusive, for backward scanning.
    ///
    /// # Returns
    ///
    /// Returns `Ok(count)` with the number of code units moved. Returns `Ok(0)`
    /// if the cursor is already at `start_index`.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Malformed` if the code units before the cursor
    /// do not form a valid UTF-16 code point. Returns
    /// `UnicodeErrorKind::Incomplete` if a low surrogate has no preceding high
    /// surrogate within the scan range.
    ///
    /// # Panics
    ///
    /// Panics if `start_index > pos.index()` or `pos.index() > buffer.len()`.
    pub fn backward(
        pos: &mut ParsingPosition,
        buffer: &[u16],
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
            Ok(1)
        } else if !Self::is_trailing(previous) {
            UnicodeError::fail(pos, previous_index, UnicodeErrorKind::Malformed)
        } else if previous_index == start_index {
            UnicodeError::fail(pos, previous_index, UnicodeErrorKind::Incomplete)
        } else {
            let leading_index = previous_index - 1;
            if Self::is_leading(buffer[leading_index]) {
                pos.set_index(leading_index);
                Ok(2)
            } else {
                UnicodeError::fail(pos, leading_index, UnicodeErrorKind::Malformed)
            }
        }
    }

    /// Reads the next UTF-16 code point and advances the cursor.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor pointing at the next code unit to decode.
    /// - `buffer`: The UTF-16 code-unit buffer.
    /// - `end_index`: The upper bound, exclusive, for decoding.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(ch))` and advances `pos` past the decoded character when
    /// a complete UTF-16 sequence is available. Returns `Ok(None)` if
    /// `pos.index() == end_index`.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Malformed` if the next code-unit sequence is
    /// not valid UTF-16. Returns `UnicodeErrorKind::Incomplete` if a high
    /// surrogate reaches `end_index` without its low surrogate. The error is
    /// also recorded in `pos`.
    ///
    /// # Panics
    ///
    /// Panics if `end_index > buffer.len()` or `pos.index() > end_index`.
    pub fn get_next(
        pos: &mut ParsingPosition,
        buffer: &[u16],
        end_index: usize,
    ) -> UnicodeResult<Option<char>> {
        let index = pos.index();
        assert!(end_index <= buffer.len() && index <= end_index);
        if index == end_index {
            return Ok(None);
        }
        let unit = buffer[index];
        if Self::is_single(unit) {
            let ch = char::from_u32(unit as u32)
                .expect("non-surrogate UTF-16 code unit must be a scalar value");
            pos.set_index(index + 1);
            Ok(Some(ch))
        } else if !Self::is_leading(unit) {
            UnicodeError::fail(pos, index, UnicodeErrorKind::Malformed)
        } else {
            let trailing_index = index + 1;
            if trailing_index >= end_index {
                return UnicodeError::fail(pos, trailing_index, UnicodeErrorKind::Incomplete);
            }
            let trailing = buffer[trailing_index];
            if let Some(code_point) = Self::compose(unit, trailing) {
                let ch = char::from_u32(code_point)
                    .expect("composed surrogate pair must be a scalar value");
                pos.set_index(trailing_index + 1);
                Ok(Some(ch))
            } else {
                UnicodeError::fail(pos, trailing_index, UnicodeErrorKind::Malformed)
            }
        }
    }

    /// Reads the previous UTF-16 code point and moves the cursor to its start.
    ///
    /// # Parameters
    ///
    /// - `pos`: The cursor positioned after the code point to read.
    /// - `buffer`: The UTF-16 code-unit buffer.
    /// - `start_index`: The lower bound, inclusive, for backward decoding.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(ch))` and moves `pos` to the first code unit of that
    /// character. Returns `Ok(None)` if `pos.index() == start_index`.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Malformed` or `UnicodeErrorKind::Incomplete`
    /// if the code units before the cursor do not form a complete valid UTF-16
    /// code point. The error is also recorded in `pos`.
    ///
    /// # Panics
    ///
    /// Panics under the same conditions as [`Self::backward`].
    pub fn get_previous(
        pos: &mut ParsingPosition,
        buffer: &[u16],
        start_index: usize,
    ) -> UnicodeResult<Option<char>> {
        let old_index = pos.index();
        let count = Self::backward(pos, buffer, start_index)?;
        if count == 0 {
            return Ok(None);
        }
        let new_index = pos.index();
        let ch = if count == 1 {
            char::from_u32(buffer[new_index] as u32)
                .expect("non-surrogate UTF-16 code unit must be a scalar value")
        } else {
            let code_point = Self::compose(buffer[new_index], buffer[old_index - 1])
                .expect("backward validated the surrogate pair");
            char::from_u32(code_point).expect("composed surrogate pair must be a scalar value")
        };
        pos.set_index(new_index);
        Ok(Some(ch))
    }

    /// Encodes a scalar value into a UTF-16 output buffer.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The Unicode scalar value to encode.
    /// - `index`: The starting index in `buffer` at which code units are
    ///   written.
    /// - `buffer`: The caller-provided output buffer.
    /// - `end_index`: The upper bound, exclusive, for writing.
    ///
    /// # Returns
    ///
    /// Returns `Ok(count)` with the number of UTF-16 code units written.
    ///
    /// # Errors
    ///
    /// Returns `UnicodeErrorKind::Malformed` if `code_point` is not a valid
    /// Unicode scalar value. Returns `UnicodeErrorKind::BufferOverflow` if the
    /// encoded code units would extend past `end_index`.
    ///
    /// # Panics
    ///
    /// Panics if `end_index > buffer.len()` or `index > end_index`.
    pub fn put(
        code_point: u32,
        index: usize,
        buffer: &mut [u16],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        assert!(end_index <= buffer.len() && index <= end_index);
        let count = match Self::code_unit_count(code_point) {
            Some(count) => count,
            None => {
                return Err(UnicodeError::new(UnicodeErrorKind::Malformed, index));
            }
        };
        if index + count > end_index {
            return Err(UnicodeError::new(
                UnicodeErrorKind::BufferOverflow,
                end_index,
            ));
        }
        if count == 1 {
            buffer[index] = code_point as u16;
        } else {
            buffer[index] =
                Self::decompose_high(code_point).expect("supplementary code point has high");
            buffer[index + 1] =
                Self::decompose_low(code_point).expect("supplementary code point has low");
        }
        Ok(count)
    }

    /// Escapes a scalar value as Java/JavaScript UTF-16 `\uXXXX` escape text.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The Unicode scalar value to escape.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing one `\uXXXX` escape for BMP scalar
    /// values or two surrogate `\uXXXX` escapes for supplementary scalar values.
    /// Returns `None` if `code_point` is a surrogate value or is above
    /// `0x10FFFF`.
    #[must_use]
    pub fn escape_java(code_point: u32) -> Option<String> {
        let count = Self::code_unit_count(code_point)?;
        if count == 1 {
            Some(format!("\\u{code_point:04X}"))
        } else {
            let high = Self::decompose_high(code_point)?;
            let low = Self::decompose_low(code_point)?;
            Some(format!("\\u{high:04X}\\u{low:04X}"))
        }
    }

    /// Escapes a scalar value as Rust-style `\u{...}` escape text.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The Unicode scalar value to escape.
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing `\u{...}` for valid Unicode scalar
    /// values. Returns `None` if `code_point` is a surrogate value or is above
    /// `0x10FFFF`.
    #[must_use]
    pub fn escape_rust(code_point: u32) -> Option<String> {
        Self::code_unit_count(code_point)?;
        Some(format!("\\u{{{code_point:X}}}"))
    }
}
