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
    TextEncodeResult,
    TextEncoder,
    Utf16,
};

/// Encoder for UTF-16 `u16` code-unit buffers.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     TextEncoder,
///     Utf16,
///     Utf16U16Encoder,
/// };
///
/// let encoder = Utf16U16Encoder;
/// let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];
/// let written = encoder.encode_char('😀', &mut output, 0).expect("buffer fits");
///
/// assert_eq!(2, written);
/// assert_eq!([0xd83d, 0xde00], output);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf16U16Encoder;

impl TextEncoder<u16> for Utf16U16Encoder {
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

    /// Encodes one Unicode scalar value into UTF-16 code units at `index`.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode scalar value to encode.
    /// * `output` - Destination `u16` buffer.
    /// * `index` - Start offset where units are written; must satisfy
    ///   `index <= output.len()`.
    ///
    /// # Returns
    ///
    /// `Ok(usize)` with the number of written UTF-16 units (`1` or `2`).
    ///
    /// # Errors
    ///
    /// * `TextEncodeError::buffer_too_small` when output capacity is insufficient.
    fn encode_char(&self, ch: char, output: &mut [u16], index: usize) -> TextEncodeResult<usize> {
        utf16::encode_units_char(ch, output, index)
    }
}
