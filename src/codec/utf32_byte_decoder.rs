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
    ByteOrder,
    Charset,
    DecodeStatus,
    TextDecodeResult,
    TextDecoder,
    Utf32,
};

use super::helpers;

/// Decoder for byte-serialized UTF-32 buffers.
///
/// The decoder uses the configured byte order for each UTF-32 unit. It does not
/// detect or skip a BOM; callers that accept BOM-prefixed input should call
/// [`crate::UnicodeBom::detect`] first and then advance by the BOM length.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     ByteOrder,
///     DecodeStatus,
///     TextDecoder,
///     Utf32ByteDecoder,
/// };
///
/// let decoder = Utf32ByteDecoder::new(ByteOrder::BigEndian);
/// let decoded = decoder
///     .decode_prefix(&[0x00, 0x01, 0xf6, 0x00])
///     .expect("valid UTF-32BE");
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
pub struct Utf32ByteDecoder {
    byte_order: ByteOrder,
}

impl Utf32ByteDecoder {
    /// Creates a byte-serialized UTF-32 decoder.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used by the input bytes.
    ///
    /// # Returns
    ///
    /// Returns a UTF-32 byte decoder.
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

impl TextDecoder<u8> for Utf32ByteDecoder {
    fn charset(&self) -> Charset {
        Charset::from_utf32_byte_order(self.byte_order)
    }

    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_BYTES_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u8]) -> TextDecodeResult<DecodeStatus<char>> {
        helpers::decode_utf32_bytes_prefix(input, self.byte_order)
    }
}
