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
    TextEncodeResult,
    TextEncoder,
    Utf32,
};

/// Encoder for UTF-32 `u32` code-unit buffers.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     TextEncoder,
///     Utf32,
///     Utf32U32Encoder,
/// };
///
/// let encoder = Utf32U32Encoder;
/// let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];
/// let written = encoder.encode_char('中', &mut output, 0).expect("buffer fits");
///
/// assert_eq!(1, written);
/// assert_eq!('中' as u32, output[0]);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf32U32Encoder;

impl TextEncoder<u32> for Utf32U32Encoder {
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

    /// Encodes one Unicode scalar value into a `u32` unit at `index`.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode scalar value to encode.
    /// * `output` - Destination `u32` buffer.
    /// * `index` - Start offset where one unit is written; must satisfy
    ///   `index < output.len()`.
    ///
    /// # Returns
    ///
    /// Always returns `Ok(1)` on success.
    ///
    /// # Errors
    ///
    /// * `TextEncodeError::buffer_too_small` when `output` has no room at `index`.
    fn encode_char(&self, ch: char, output: &mut [u32], index: usize) -> TextEncodeResult<usize> {
        utf32::encode_units_char(ch, output, index)
    }
}
