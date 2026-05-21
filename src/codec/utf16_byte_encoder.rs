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
    ByteOrder,
    Charset,
    TextEncodeResult,
    TextEncoder,
    Utf16,
};

/// Encoder for byte-serialized UTF-16 buffers.
///
/// The encoder serializes UTF-16 code units using the configured byte order. It
/// does not write a BOM automatically; callers that need one should prepend the
/// bytes from [`crate::UnicodeBom`].
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     ByteOrder,
///     TextEncoder,
///     Utf16,
///     Utf16ByteEncoder,
/// };
///
/// let encoder = Utf16ByteEncoder::new(ByteOrder::LittleEndian);
/// let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];
/// let written = encoder.encode_char('😀', &mut output, 0).expect("buffer fits");
///
/// assert_eq!(4, written);
/// assert_eq!([0x3d, 0xd8, 0x00, 0xde], output);
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Utf16ByteEncoder {
    /// Byte order used when serializing UTF-16 units.
    byte_order: ByteOrder,
}

impl Utf16ByteEncoder {
    /// Creates a byte-serialized UTF-16 encoder.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used to serialize UTF-16 units.
    ///
    /// # Returns
    ///
    /// Returns a UTF-16 byte encoder.
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

impl TextEncoder<u8> for Utf16ByteEncoder {
    fn charset(&self) -> Charset {
        Charset::from_utf16_byte_order(self.byte_order)
    }

    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_BYTES_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u8], index: usize) -> TextEncodeResult<usize> {
        utf16::encode_bytes_char(ch, output, self.byte_order, index)
    }
}
