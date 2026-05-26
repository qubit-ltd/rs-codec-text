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
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeResult,
    DecodeStatus,
    Utf32,
};
use qubit_codec::Codec;

/// Combined byte-serialized UTF-32 codec.
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
///     Utf32,
///     Utf32ByteCodec,
/// };
///
/// let codec = Utf32ByteCodec::new(ByteOrder::BigEndian);
/// assert_eq!(Charset::UTF_32BE, codec.charset());
/// assert_eq!(Utf32::MAX_BYTES_PER_CHAR, codec.max_units_per_char());
///
/// let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];
/// let written = codec.encode_one('中', &mut output, 0).expect("buffer fits");
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '中',
///         consumed: written,
///     },
///     codec.decode_one(&output[..written], 0).expect("valid UTF-32BE"),
/// );
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Utf32ByteCodec {
    /// Byte order used by both encoder and decoder paths.
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

    /// Returns the fixed-endian UTF-32 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_32LE`] or [`Charset::UTF_32BE`] according to this
    /// codec's configured byte order.
    #[must_use]
    #[inline]
    pub const fn charset(self) -> Charset {
        Charset::from_utf32_byte_order(self.byte_order)
    }

    /// Returns the maximum number of serialized UTF-32 bytes for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf32::MAX_BYTES_PER_CHAR`].
    #[must_use]
    #[inline]
    pub const fn max_units_per_char(self) -> usize {
        Utf32::MAX_BYTES_PER_CHAR
    }
}

impl CharsetCodec for Utf32ByteCodec {
    type Unit = u8;
    /// Returns the fixed-endian UTF-32 charset for the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_32BE`] when configured with
    /// `ByteOrder::BigEndian`, otherwise [`Charset::UTF_32LE`].
    #[inline]
    fn charset(&self) -> Charset {
        Charset::from_utf32_byte_order(self.byte_order)
    }

    /// Returns the fixed size (4 bytes) for one serialized UTF-32 scalar value.
    ///
    /// # Returns
    ///
    /// Returns [`Utf32::MAX_BYTES_PER_CHAR`].
    #[inline]
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
    /// * `Ok(DecodeStatus::NeedMore { required, available })` when fewer than
    ///   four bytes remain.
    /// * `Ok(DecodeStatus::Complete { value, consumed })` when one character is decoded.
    ///
    /// # Errors
    ///
    /// * [`crate::CharsetDecodeErrorKind::InvalidInputIndex`] when `index` is
    ///   greater than `input.len()`.
    /// * [`crate::CharsetDecodeErrorKind::InvalidCodePoint`] when bytes decode
    ///   to an invalid scalar.
    fn decode_one(&self, input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        match utf32::decode_bytes_prefix(input, index, self.byte_order)? {
            DecodeStatus::Complete { .. } => {
                let (value, consumed) = unsafe { <Self as Codec<char, u8>>::decode_unchecked(self, input, index)? };
                Ok(DecodeStatus::Complete { value, consumed })
            }
            status @ DecodeStatus::NeedMore { .. } => Ok(status),
        }
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
    /// * [`crate::CharsetEncodeErrorKind::BufferTooSmall`] if fewer than 4 bytes
    ///   remain in `output`.
    fn encode_one(&self, ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        if index > output.len() {
            return utf32::encode_bytes_char(ch, output, self.byte_order, index);
        }
        let required = 4;
        let available = output.len() - index;
        if available < required {
            return utf32::encode_bytes_char(ch, output, self.byte_order, index);
        }
        unsafe { <Self as Codec<char, u8>>::encode_unchecked(self, ch, output, index) }
    }
}

unsafe impl Codec<char, u8> for Utf32ByteCodec {
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    #[inline]
    fn min_units_per_value(&self) -> usize {
        4
    }

    #[inline]
    fn max_units_per_value(&self) -> usize {
        Utf32::MAX_BYTES_PER_CHAR
    }

    #[inline]
    unsafe fn decode_unchecked(&self, input: &[u8], index: usize) -> CharsetDecodeResult<(char, usize)> {
        debug_assert!(index + Utf32::MAX_BYTES_PER_CHAR <= input.len());

        match utf32::decode_bytes_prefix(input, index, self.byte_order)? {
            DecodeStatus::Complete { value, consumed } => Ok((value, consumed)),
            DecodeStatus::NeedMore { .. } => {
                unreachable!("Codec::decode_unchecked requires a complete UTF-32 byte value")
            }
        }
    }

    #[inline]
    unsafe fn encode_unchecked(&self, ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        debug_assert!(index + Utf32::MAX_BYTES_PER_CHAR <= output.len());

        utf32::encode_bytes_char(ch, output, self.byte_order, index)
    }
}
