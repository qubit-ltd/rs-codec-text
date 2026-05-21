/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use super::inner::utf32;
use crate::{
    Charset,
    DecodeStatus,
    TextDecodeResult,
    TextDecoder,
    Utf32,
};

/// Decoder for UTF-32 `u32` code-unit buffers.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     DecodeStatus,
///     TextDecoder,
///     Utf32U32Decoder,
/// };
///
/// let decoder = Utf32U32Decoder;
/// let decoded = decoder
///     .decode_prefix(&['中' as u32], 0)
///     .expect("valid UTF-32");
///
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '中',
///         consumed: 1,
///     },
///     decoded,
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf32U32Decoder;

impl TextDecoder<u32> for Utf32U32Decoder {
    /// Returns UTF-32 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_32`].
    fn charset(&self) -> Charset {
        Charset::UTF_32
    }

    /// Returns the fixed size (1 unit) for one UTF-32 scalar value.
    ///
    /// # Returns
    ///
    /// Returns [`Utf32::MAX_UNITS_PER_CHAR`].
    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_UNITS_PER_CHAR
    }

    /// Decodes one UTF-32 scalar value from a `u32` prefix.
    ///
    /// # Arguments
    ///
    /// * `input` - UTF-32 unit slice.
    /// * `index` - Start offset for parsing; must satisfy `index <= input.len()`.
    ///
    /// # Returns
    ///
    /// * `Ok(DecodeStatus::NeedMore { required, available })` when no unit exists at `index`.
    /// * `Ok(DecodeStatus::Complete { value, consumed })` with `consumed == 1`.
    ///
    /// # Errors
    ///
    /// * `TextDecodeError::malformed_sequence` when `index` is out of bounds.
    /// * `TextDecodeError::invalid_code_point` when unit is not a valid scalar.
    fn decode_prefix(&self, input: &[u32], index: usize) -> TextDecodeResult<DecodeStatus> {
        utf32::decode_units_prefix(input, index)
    }
}
