/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
pub use crate::codec::decode_status::DecodeStatus;

use crate::{
    TextDecodingError,
    TextDecodingResult,
    TextEncoding,
};

/// Decodes encoded buffer units into Unicode scalar values.
///
/// `T` is the storage unit used by the caller-provided buffer. For example,
/// UTF-8 and byte-serialized UTF-16 use `u8`, while UTF-16 code-unit decoding
/// uses `u16`.
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
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: 'A',
///         consumed: 1,
///     },
///     decoder.decode_prefix(b"A").expect("valid UTF-8"),
/// );
/// ```
pub trait TextDecoder<T> {
    /// Returns the encoding handled by this decoder.
    ///
    /// # Returns
    ///
    /// Returns the decoder's text encoding.
    #[must_use]
    fn encoding(&self) -> TextEncoding;

    /// Returns the maximum number of input units needed for one Unicode scalar value.
    ///
    /// # Returns
    ///
    /// Returns the maximum unit count for this decoder's buffer representation.
    #[must_use]
    fn max_units_per_char(&self) -> usize;

    /// Decodes the first Unicode scalar value from `input`.
    ///
    /// # Parameters
    ///
    /// - `input`: Encoded units beginning at the character boundary to decode.
    ///
    /// # Returns
    ///
    /// Returns [`DecodeStatus::Complete`] when a character is available. Returns
    /// [`DecodeStatus::NeedMore`] when the prefix is valid so far but too short.
    ///
    /// # Errors
    ///
    /// Returns [`TextDecodingError`] when the prefix is malformed or decodes to an
    /// invalid Unicode scalar value.
    fn decode_prefix(&self, input: &[T]) -> TextDecodingResult<DecodeStatus<char>>;

    /// Decodes the next character from `input`, advancing `index` on success.
    ///
    /// # Parameters
    ///
    /// - `input`: The complete closed input buffer.
    /// - `index`: The current unit index, advanced by the consumed unit count.
    ///
    /// # Returns
    ///
    /// Returns `Some(ch)` when a character is decoded, or `None` if `index` is at
    /// the end of `input`.
    ///
    /// # Errors
    ///
    /// Returns a decoding error when `index` is out of bounds, the next sequence is
    /// malformed, or the closed input ends in the middle of a character.
    fn decode_next(&self, input: &[T], index: &mut usize) -> TextDecodingResult<Option<char>> {
        if *index > input.len() {
            return Err(TextDecodingError::malformed_sequence(
                self.encoding(),
                *index,
            ));
        }
        if *index == input.len() {
            return Ok(None);
        }
        match self
            .decode_prefix(&input[*index..])
            .map_err(|error| error.offset_by(*index))?
        {
            DecodeStatus::Complete { value, consumed } => {
                *index += consumed;
                Ok(Some(value))
            }
            DecodeStatus::NeedMore { available, .. } => Err(
                TextDecodingError::incomplete_sequence(self.encoding(), *index + available),
            ),
        }
    }
}
