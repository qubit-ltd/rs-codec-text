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
    DecodeStatus,
    TextDecodeResult,
    TextDecoder,
    Utf32,
};

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
///     .decode_prefix(&[0x00, 0x01, 0xf6, 0x00], 0)
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
    /// Byte order used when reading UTF-32 units.
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

    /// Decodes one UTF-32 scalar value from a byte-prefixed UTF-32 stream.
    ///
    /// # Arguments
    ///
    /// * `input` - Byte-prefixed UTF-32 buffer.
    /// * `index` - Start offset for parsing; must satisfy `index <= input.len()`.
    ///
    /// # Returns
    ///
    /// * `Ok(DecodeStatus::NeedMore { required, available })` when only part of a
    ///   UTF-32 unit is present.
    /// * `Ok(DecodeStatus::Complete { value, consumed })` when one character is decoded.
    ///
    /// # Errors
    ///
    /// * `TextDecodeError::malformed_sequence` when byte index is invalid.
    /// * `TextDecodeError::invalid_code_point` when bytes represent a non-scalar value.
    fn decode_prefix(&self, input: &[u8], index: usize) -> TextDecodeResult<DecodeStatus> {
        utf32::decode_bytes_prefix(input, index, self.byte_order)
    }
}
