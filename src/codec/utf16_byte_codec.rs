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
    CharsetCodec,
    CharsetDecodeResult,
    CharsetEncodeResult,
    DecodeStatus,
    Utf16,
};

/// Combined byte-serialized UTF-16 codec.
///
/// The codec uses one configured byte order for both decoding and encoding. It
/// does not detect, consume, or emit a BOM automatically; callers should use
/// [`crate::UnicodeBom`] when a byte stream may carry an explicit BOM.
///
/// # Examples
///
/// ```rust
/// use qubit_codec_text::{
///     ByteOrder,
///     CharsetCodec,
///     DecodeStatus,
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
/// let written = codec.encode_one('😀', &mut output, 0).expect("buffer fits");
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '😀',
///         consumed: written,
///     },
///     codec.decode_one(&output[..written], 0).expect("valid UTF-16LE"),
/// );
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Utf16ByteCodec {
    /// Byte order used by both encoder and decoder paths.
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
    #[inline]
    pub const fn new(byte_order: ByteOrder) -> Self {
        Self { byte_order }
    }

    /// Returns the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns the byte order used by this codec.
    #[must_use]
    #[inline]
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
    #[inline]
    pub const fn charset(self) -> Charset {
        Charset::from_utf16_byte_order(self.byte_order)
    }

    /// Returns the maximum number of serialized UTF-16 bytes for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf16::MAX_BYTES_PER_CHAR`].
    #[must_use]
    #[inline]
    pub const fn max_units_per_char(self) -> usize {
        Utf16::MAX_BYTES_PER_CHAR
    }
}

impl CharsetCodec for Utf16ByteCodec {
    type Unit = u8;
    /// Returns the fixed-endian UTF-16 charset for the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_16BE`] when configured with
    /// `ByteOrder::BigEndian`, otherwise [`Charset::UTF_16LE`].
    #[inline]
    fn charset(&self) -> Charset {
        Charset::from_utf16_byte_order(self.byte_order)
    }

    /// Returns the maximum number of UTF-16 bytes for a single encoded character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf16::MAX_BYTES_PER_CHAR`].
    #[inline]
    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_BYTES_PER_CHAR
    }

    /// Decodes one UTF-16 scalar value from a byte-prefixed UTF-16 stream.
    ///
    /// # Arguments
    ///
    /// * `input` - Byte-prefixed UTF-16 buffer.
    /// * `index` - Start offset for parsing; must satisfy `index <= input.len()`.
    ///
    /// # Returns
    ///
    /// * `Ok(DecodeStatus::NeedMore { required, available })` when the current slice
    ///   has only a partial unit/pair.
    /// * `Ok(DecodeStatus::Complete { value, consumed })` when one Unicode scalar value
    ///   is decoded.
    ///
    /// # Errors
    ///
    /// * [`crate::CharsetDecodeErrorKind::InvalidInputIndex`] when `index` is
    ///   greater than `input.len()`.
    /// * [`crate::CharsetDecodeErrorKind::MalformedSequence`] when UTF-16
    ///   structure is malformed.
    fn decode_one(&self, input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        utf16::decode_bytes_prefix(input, index, self.byte_order)
    }

    /// Encodes one Unicode scalar value into UTF-16 bytes at `index`.
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
    /// `Ok(usize)` with the number of written bytes (`2` for BMP and `4` for supplementary).
    ///
    /// # Errors
    ///
    /// * [`crate::CharsetEncodeErrorKind::BufferTooSmall`] if output does not
    ///   have enough space.
    fn encode_one(&self, ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        utf16::encode_bytes_char(ch, output, self.byte_order, index)
    }
}
