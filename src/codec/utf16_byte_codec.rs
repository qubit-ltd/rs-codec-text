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
    DecodeStatus,
    TextDecoder,
    TextDecodingResult,
    TextEncoder,
    TextEncoding,
    TextEncodingResult,
    Utf16,
};

use super::helpers;

/// Combined byte-serialized UTF-16 codec.
///
/// The codec uses one configured byte order for both decoding and encoding. It
/// does not detect, consume, or emit a BOM automatically; callers should use
/// [`crate::UnicodeBom`] when a byte stream may carry an explicit BOM.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     ByteOrder,
///     DecodeStatus,
///     TextDecoder,
///     TextEncoder,
///     TextEncoding,
///     Utf16,
///     Utf16ByteCodec,
/// };
///
/// let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
/// assert_eq!(TextEncoding::UTF_16, codec.encoding());
/// assert_eq!(Utf16::MAX_BYTES_PER_CHAR, codec.max_units_per_char());
///
/// let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];
/// let written = codec.encode_char('😀', &mut output).expect("buffer fits");
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '😀',
///         consumed: written,
///     },
///     codec.decode_prefix(&output[..written]).expect("valid UTF-16LE"),
/// );
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Utf16ByteCodec {
    byte_order: ByteOrder,
}

impl Utf16ByteCodec {
    /// Creates a byte-serialized UTF-16 codec.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used by the byte buffer.
    ///
    /// # Returns
    ///
    /// Returns a UTF-16 byte codec.
    #[must_use]
    pub const fn new(byte_order: ByteOrder) -> Self {
        Self { byte_order }
    }

    /// Returns the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns the byte order used by this codec.
    #[must_use]
    pub const fn byte_order(self) -> ByteOrder {
        self.byte_order
    }

    /// Returns the UTF-16 encoding descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`TextEncoding::UTF_16`].
    #[must_use]
    pub const fn encoding(self) -> TextEncoding {
        TextEncoding::UTF_16
    }

    /// Returns the maximum number of serialized UTF-16 bytes for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf16::MAX_BYTES_PER_CHAR`].
    #[must_use]
    pub const fn max_units_per_char(self) -> usize {
        Utf16::MAX_BYTES_PER_CHAR
    }
}

impl TextDecoder<u8> for Utf16ByteCodec {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::UTF_16
    }

    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_BYTES_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u8]) -> TextDecodingResult<DecodeStatus<char>> {
        helpers::decode_utf16_bytes_prefix(input, self.byte_order)
    }
}

impl TextEncoder<u8> for Utf16ByteCodec {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::UTF_16
    }

    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_BYTES_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u8]) -> TextEncodingResult<usize> {
        helpers::encode_utf16_bytes_char(ch, output, self.byte_order)
    }
}
