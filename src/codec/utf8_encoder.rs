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
    TextEncodeResult,
    TextEncoder,
    Utf8,
};

/// Encoder for UTF-8 byte buffers.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     TextEncoder,
///     Utf8,
///     Utf8Encoder,
/// };
///
/// let encoder = Utf8Encoder;
/// let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
/// let written = encoder.encode_char('😀', &mut output, 0).expect("buffer fits");
///
/// assert_eq!("😀".as_bytes(), &output[..written]);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf8Encoder;

impl TextEncoder<u8> for Utf8Encoder {
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

    /// Encodes one Unicode scalar value into UTF-8 bytes at `index`.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode scalar value to encode.
    /// * `output` - Destination byte buffer.
    /// * `index` - Start offset where bytes are written; must satisfy
    ///   `index <= output.len()`.
    ///
    /// # Returns
    ///
    /// `Ok(usize)` with encoded bytes (`1..=4`).
    ///
    /// # Errors
    ///
    /// * `TextEncodeError::buffer_too_small` if destination is too small.
    fn encode_char(&self, ch: char, output: &mut [u8], index: usize) -> TextEncodeResult<usize> {
        utf8::encode_char(ch, output, index)
    }
}
