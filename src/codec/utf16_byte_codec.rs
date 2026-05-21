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
///     Charset,
///     Utf16,
///     Utf16ByteCodec,
/// };
///
/// let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
/// assert_eq!(Charset::UTF_16LE, codec.charset());
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

    /// Returns the fixed-endian UTF-16 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_16LE`] or [`Charset::UTF_16BE`] according to this
    /// codec's configured byte order.
    #[must_use]
    pub const fn charset(self) -> Charset {
        Charset::from_utf16_byte_order(self.byte_order)
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
    fn charset(&self) -> Charset {
        Charset::from_utf16_byte_order(self.byte_order)
    }

    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_BYTES_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u8]) -> TextDecodeResult<DecodeStatus<char>> {
        helpers::decode_utf16_bytes_prefix(input, self.byte_order)
    }
}

impl TextEncoder<u8> for Utf16ByteCodec {
    fn charset(&self) -> Charset {
        Charset::from_utf16_byte_order(self.byte_order)
    }

    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_BYTES_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u8]) -> TextEncodeResult<usize> {
        helpers::encode_utf16_bytes_char(ch, output, self.byte_order)
    }
}
