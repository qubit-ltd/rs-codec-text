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
    ByteOrder,
    Charset,
    TextEncodeResult,
    TextEncoder,
    Utf32,
};

/// Encoder for byte-serialized UTF-32 buffers.
///
/// The encoder serializes UTF-32 units using the configured byte order. It does
/// not write a BOM automatically; callers that need one should prepend the bytes
/// from [`crate::UnicodeBom`].
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     ByteOrder,
///     TextEncoder,
///     Utf32,
///     Utf32ByteEncoder,
/// };
///
/// let encoder = Utf32ByteEncoder::new(ByteOrder::BigEndian);
/// let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];
/// let written = encoder.encode_char('😀', &mut output, 0).expect("buffer fits");
///
/// assert_eq!(4, written);
/// assert_eq!([0x00, 0x01, 0xf6, 0x00], output);
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Utf32ByteEncoder {
    /// Byte order used when serializing UTF-32 units.
    byte_order: ByteOrder,
}

impl Utf32ByteEncoder {
    /// Creates a byte-serialized UTF-32 encoder.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used to serialize UTF-32 units.
    ///
    /// # Returns
    ///
    /// Returns a UTF-32 byte encoder.
    #[must_use]
    pub const fn new(byte_order: ByteOrder) -> Self {
        Self { byte_order }
    }

    /// Returns the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns the byte order used by this encoder.
    #[must_use]
    pub const fn byte_order(self) -> ByteOrder {
        self.byte_order
    }
}

impl TextEncoder<u8> for Utf32ByteEncoder {
    /// Returns the fixed-endian UTF-32 charset for the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_32BE`] when configured with
    /// `ByteOrder::BigEndian`, otherwise [`Charset::UTF_32LE`].
    fn charset(&self) -> Charset {
        Charset::from_utf32_byte_order(self.byte_order)
    }

    /// Returns the fixed size (4 bytes) for one serialized UTF-32 scalar value.
    ///
    /// # Returns
    ///
    /// Returns [`Utf32::MAX_BYTES_PER_CHAR`].
    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_BYTES_PER_CHAR
    }

    /// Encodes one Unicode scalar value into UTF-32 bytes at `index`.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode scalar value to encode.
    /// * `output` - Destination byte buffer.
    /// * `index` - Start offset where 4 bytes are written; must satisfy
    ///   `index <= output.len()`.
    ///
    /// # Returns
    ///
    /// Always returns `Ok(4)` on success.
    ///
    /// # Errors
    ///
    /// * `TextEncodeError::buffer_too_small` if fewer than 4 bytes remain in `output`.
    fn encode_char(&self, ch: char, output: &mut [u8], index: usize) -> TextEncodeResult<usize> {
        utf32::encode_bytes_char(ch, output, self.byte_order, index)
    }
}
