/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use super::inner::utf8;
use crate::{
    Charset,
    DecodeStatus,
    TextDecodeResult,
    TextDecoder,
    Utf8,
};

/// Decoder for UTF-8 byte buffers.
///
/// The decoder reads one Unicode scalar value from the beginning of a byte
/// slice. Incomplete but still-valid prefixes return [`DecodeStatus::NeedMore`];
/// malformed prefixes return [`crate::TextDecodeError`].
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     DecodeStatus,
///     TextDecoder,
///     Utf8Decoder,
/// };
///
/// let decoder = Utf8Decoder;
/// let decoded = decoder.decode_prefix("中".as_bytes(), 0).expect("valid UTF-8");
///
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '中',
///         consumed: 3,
///     },
///     decoded,
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf8Decoder;

impl TextDecoder<u8> for Utf8Decoder {
    /// Returns UTF-8 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_8`].
    fn charset(&self) -> Charset {
        Charset::UTF_8
    }

    /// Returns the maximum number of UTF-8 bytes for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf8::MAX_UNITS_PER_CHAR`].
    fn max_units_per_char(&self) -> usize {
        Utf8::MAX_UNITS_PER_CHAR
    }

    /// Decodes one UTF-8 character from a byte prefix.
    ///
    /// # Arguments
    ///
    /// * `input` - UTF-8 byte slice.
    /// * `index` - Start offset for decoding; must satisfy `index <= input.len()`.
    ///
    /// # Returns
    ///
    /// * `Ok(DecodeStatus::NeedMore { required, available })` when bytes are incomplete.
    /// * `Ok(DecodeStatus::Complete { value, consumed })` when one character is decoded.
    ///
    /// # Errors
    ///
    /// * `TextDecodeError::malformed_sequence` for malformed UTF-8.
    /// * `TextDecodeError::invalid_code_point` for non-scalar code points.
    fn decode_prefix(&self, input: &[u8], index: usize) -> TextDecodeResult<DecodeStatus> {
        utf8::decode_prefix(input, index)
    }
}
