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
    Charset,
    CharsetCodec,
    CharsetDecodeResult,
    CharsetEncodeResult,
    DecodeStatus,
    Utf16,
};

/// Combined UTF-16 `u16` code-unit codec.
///
/// `Utf16U16Codec` works with UTF-16 code units rather than serialized bytes.
/// Use [`crate::Utf16ByteCodec`] when the input or output is a byte stream with an
/// explicit byte order.
///
/// # Examples
///
/// ```rust
/// use qubit_codec_text::{
///     CharsetCodec,
///     DecodeStatus,
///     Charset,
///     Utf16,
///     Utf16U16Codec,
/// };
///
/// let codec = Utf16U16Codec;
/// assert_eq!(Charset::UTF_16, codec.charset());
/// assert_eq!(Utf16::MAX_UNITS_PER_CHAR, codec.max_units_per_char());
///
/// let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];
/// let written = codec.encode_one('😀', &mut output, 0).expect("buffer fits");
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '😀',
///         consumed: written,
///     },
///     codec.decode_one(&output[..written], 0).expect("valid UTF-16"),
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf16U16Codec;

impl Utf16U16Codec {
    /// Returns the UTF-16 encoding descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_16`].
    #[must_use]
    #[inline]
    pub const fn charset(self) -> Charset {
        Charset::UTF_16
    }

    /// Returns the maximum number of UTF-16 code units needed for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf16::MAX_UNITS_PER_CHAR`].
    #[must_use]
    #[inline]
    pub const fn max_units_per_char(self) -> usize {
        Utf16::MAX_UNITS_PER_CHAR
    }
}

impl CharsetCodec for Utf16U16Codec {
    type Unit = u16;
    /// Returns UTF-16 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_16`].
    #[inline]
    fn charset(&self) -> Charset {
        Charset::UTF_16
    }

    /// Returns the maximum number of UTF-16 code units for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf16::MAX_UNITS_PER_CHAR`].
    #[inline]
    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_UNITS_PER_CHAR
    }

    /// Decodes one UTF-16 scalar value from a `u16` prefix.
    ///
    /// # Arguments
    ///
    /// * `input` - UTF-16 code-unit slice.
    /// * `index` - Start offset for parsing; must satisfy `index <= input.len()`.
    ///
    /// # Returns
    ///
    /// * `Ok(DecodeStatus::NeedMore { required, available })` when only a partial unit/pair
    ///   is present.
    /// * `Ok(DecodeStatus::Complete { value, consumed })` when one character is decoded.
    ///
    /// # Errors
    ///
    /// * [`crate::CharsetDecodeErrorKind::InvalidInputIndex`] when `index` is
    ///   greater than `input.len()`.
    /// * [`crate::CharsetDecodeErrorKind::MalformedSequence`] for invalid
    ///   surrogate combinations.
    /// * [`crate::CharsetDecodeErrorKind::InvalidCodePoint`] when resulting
    ///   scalar is invalid.
    fn decode_one(&self, input: &[u16], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        utf16::decode_units_prefix(input, index)
    }

    /// Encodes one Unicode scalar value into UTF-16 code units at `index`.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode scalar value to encode.
    /// * `output` - Destination `u16` buffer.
    /// * `index` - Start offset where units are written; must satisfy
    ///   `index <= output.len()`.
    ///
    /// # Returns
    ///
    /// `Ok(usize)` with the number of written UTF-16 units (`1` or `2`).
    ///
    /// # Errors
    ///
    /// * [`crate::CharsetEncodeErrorKind::BufferTooSmall`] if destination is
    ///   insufficient.
    fn encode_one(&self, ch: char, output: &mut [u16], index: usize) -> CharsetEncodeResult<usize> {
        utf16::encode_units_char(ch, output, index)
    }
}
