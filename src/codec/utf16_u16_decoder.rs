/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use super::inner::utf16;
use crate::{
    Charset,
    DecodeStatus,
    TextDecodeResult,
    TextDecoder,
    Utf16,
};

/// Decoder for UTF-16 `u16` code-unit buffers.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     DecodeStatus,
///     TextDecoder,
///     Utf16U16Decoder,
/// };
///
/// let decoder = Utf16U16Decoder;
/// let decoded = decoder.decode_prefix(&[0xd83d, 0xde00], 0).expect("valid pair");
///
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '😀',
///         consumed: 2,
///     },
///     decoded,
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf16U16Decoder;

impl TextDecoder<u16> for Utf16U16Decoder {
    /// Returns UTF-16 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_16`].
    fn charset(&self) -> Charset {
        Charset::UTF_16
    }

    /// Returns the maximum number of UTF-16 code units for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf16::MAX_UNITS_PER_CHAR`].
    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_UNITS_PER_CHAR
    }

    /// Decodes one UTF-16 scalar value from a `u16` prefix.
    ///
    /// # Arguments
    ///
    /// * `input` - UTF-16 code-unit slice.
    /// * `index` - Start offset for parsing; must satisfy `index <= input.len()`.
    ///
    /// # Returns
    ///
    /// * `Ok(DecodeStatus::NeedMore { required, available })` when only partial units
    ///   are available.
    /// * `Ok(DecodeStatus::Complete { value, consumed })` when one character is decoded.
    ///
    /// # Errors
    ///
    /// * `TextDecodeError::malformed_sequence` for invalid UTF-16 units.
    /// * `TextDecodeError::invalid_code_point` when scalar conversion fails.
    fn decode_prefix(&self, input: &[u16], index: usize) -> TextDecodeResult<DecodeStatus> {
        utf16::decode_units_prefix(input, index)
    }
}
