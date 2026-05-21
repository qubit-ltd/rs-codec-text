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
    TextEncodeResult,
    TextEncoder,
    Utf32,
};

use super::helpers;

/// Combined byte-serialized UTF-32 codec.
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
///     Charset,
///     Utf32,
///     Utf32ByteCodec,
/// };
///
/// let codec = Utf32ByteCodec::new(ByteOrder::BigEndian);
/// assert_eq!(Charset::UTF_32BE, codec.charset());
/// assert_eq!(Utf32::MAX_BYTES_PER_CHAR, codec.max_units_per_char());
///
/// let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];
/// let written = codec.encode_char('中', &mut output).expect("buffer fits");
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '中',
///         consumed: written,
///     },
///     codec.decode_prefix(&output[..written]).expect("valid UTF-32BE"),
/// );
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Utf32ByteCodec {
    byte_order: ByteOrder,
}

impl Utf32ByteCodec {
    /// Creates a byte-serialized UTF-32 codec.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used by the byte buffer.
    ///
    /// # Returns
    ///
    /// Returns a UTF-32 byte codec.
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

    /// Returns the fixed-endian UTF-32 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_32LE`] or [`Charset::UTF_32BE`] according to this
    /// codec's configured byte order.
    #[must_use]
    pub const fn charset(self) -> Charset {
        Charset::from_utf32_byte_order(self.byte_order)
    }

    /// Returns the maximum number of serialized UTF-32 bytes for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf32::MAX_BYTES_PER_CHAR`].
    #[must_use]
    pub const fn max_units_per_char(self) -> usize {
        Utf32::MAX_BYTES_PER_CHAR
    }
}

impl TextDecoder<u8> for Utf32ByteCodec {
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

impl TextEncoder<u8> for Utf32ByteCodec {
    fn charset(&self) -> Charset {
        Charset::from_utf32_byte_order(self.byte_order)
    }

    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_BYTES_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u8]) -> TextEncodeResult<usize> {
        helpers::encode_utf32_bytes_char(ch, output, self.byte_order)
    }
}
