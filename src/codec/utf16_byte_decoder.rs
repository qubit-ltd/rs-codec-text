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
    DecodeStatus,
    TextDecodeResult,
    TextDecoder,
    Utf16,
};

/// Decoder for byte-serialized UTF-16 buffers.
///
/// The decoder uses the configured byte order for every UTF-16 code unit. It
/// does not detect or skip a BOM; callers that accept BOM-prefixed input should
/// call [`crate::UnicodeBom::detect`] first and then advance by the BOM length.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     ByteOrder,
///     DecodeStatus,
///     TextDecoder,
///     Utf16ByteDecoder,
/// };
///
/// let decoder = Utf16ByteDecoder::new(ByteOrder::LittleEndian);
/// let decoded = decoder
///     .decode_prefix(&[0x3d, 0xd8, 0x00, 0xde], 0)
///     .expect("valid UTF-16LE");
///
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '😀',
///         consumed: 4,
///     },
///     decoded,
/// );
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Utf16ByteDecoder {
    /// Byte order used when reading UTF-16 units.
    byte_order: ByteOrder,
}

impl Utf16ByteDecoder {
    /// Creates a byte-serialized UTF-16 decoder.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used by the input bytes.
    ///
    /// # Returns
    ///
    /// Returns a UTF-16 byte decoder.
    #[must_use]
    pub const fn new(byte_order: ByteOrder) -> Self {
        Self { byte_order }
    }

    /// Returns the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns the byte order used by this decoder.
    #[must_use]
    pub const fn byte_order(self) -> ByteOrder {
        self.byte_order
    }
}

impl TextDecoder<u8> for Utf16ByteDecoder {
    fn charset(&self) -> Charset {
        Charset::from_utf16_byte_order(self.byte_order)
    }

    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_BYTES_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u8], index: usize) -> TextDecodeResult<DecodeStatus> {
        utf16::decode_bytes_prefix(input, index, self.byte_order)
    }
}
