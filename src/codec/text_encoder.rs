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
    Charset,
    TextEncodeError,
    TextEncodeResult,
    Unicode,
};

/// Encodes Unicode scalar values into caller-provided output units.
///
/// `T` is the storage unit used by the output buffer. For example, UTF-8 and
/// byte-serialized UTF-16 use `u8`, while UTF-16 code-unit encoding uses `u16`.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     TextEncoder,
///     Utf8Encoder,
/// };
///
/// let encoder = Utf8Encoder;
/// let mut output = [0_u8; 4];
/// let written = encoder.encode_char('A', &mut output).expect("buffer fits");
///
/// assert_eq!(1, written);
/// assert_eq!(b"A", &output[..written]);
/// ```
pub trait TextEncoder<T> {
    /// Returns the charset handled by this encoder.
    ///
    /// # Returns
    ///
    /// Returns the encoder's charset.
    #[must_use]
    fn charset(&self) -> Charset;

    /// Returns the maximum number of output units needed for one Unicode scalar value.
    ///
    /// # Returns
    ///
    /// Returns the maximum unit count for this encoder's buffer representation.
    #[must_use]
    fn max_units_per_char(&self) -> usize;

    /// Encodes one Unicode scalar value into `output`.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to encode.
    /// - `output`: The output unit buffer receiving the encoded representation.
    ///
    /// # Returns
    ///
    /// Returns the number of output units written.
    ///
    /// # Errors
    ///
    /// Returns [`crate::TextEncodeErrorKind::BufferTooSmall`] when `output`
    /// cannot hold the encoded character.
    fn encode_char(&self, ch: char, output: &mut [T]) -> TextEncodeResult<usize>;

    /// Encodes one raw Unicode code point into `output`.
    ///
    /// # Parameters
    ///
    /// - `code_point`: The raw code point to encode.
    /// - `output`: The output unit buffer receiving the encoded representation.
    ///
    /// # Returns
    ///
    /// Returns the number of output units written.
    ///
    /// # Errors
    ///
    /// Returns [`crate::TextEncodeErrorKind::InvalidCodePoint`] when
    /// `code_point` is not a Unicode scalar value. Returns any error reported by
    /// [`Self::encode_char`] for valid scalar values.
    fn encode_code_point(&self, code_point: u32, output: &mut [T]) -> TextEncodeResult<usize> {
        match Unicode::to_char(code_point) {
            Some(ch) => self.encode_char(ch, output),
            None => Err(TextEncodeError::invalid_code_point(
                self.charset(),
                0,
                code_point,
            )),
        }
    }
}
