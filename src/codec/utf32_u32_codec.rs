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
    Charset,
    CharsetCodec,
    CharsetDecodeResult,
    CharsetEncodeResult,
    DecodeStatus,
    Utf32,
};

/// Combined UTF-32 `u32` code-unit codec.
///
/// `Utf32U32Codec` works with raw UTF-32 scalar-value units rather than
/// serialized bytes. Use [`crate::Utf32ByteCodec`] for byte streams with an
/// explicit byte order.
///
/// # Examples
///
/// ```rust
/// use qubit_codec_text::{
///     CharsetCodec,
///     DecodeStatus,
///     Charset,
///     Utf32,
///     Utf32U32Codec,
/// };
///
/// let codec = Utf32U32Codec;
/// assert_eq!(Charset::UTF_32, codec.charset());
/// assert_eq!(Utf32::MAX_UNITS_PER_CHAR, codec.max_units_per_char());
///
/// let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];
/// let written = codec.encode_one('中', &mut output, 0).expect("buffer fits");
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '中',
///         consumed: written,
///     },
///     codec.decode_one(&output[..written], 0).expect("valid UTF-32"),
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf32U32Codec;

impl Utf32U32Codec {
    /// Returns the UTF-32 encoding descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_32`].
    #[must_use]
    #[inline]
    pub const fn charset(self) -> Charset {
        Charset::UTF_32
    }

    /// Returns the maximum number of UTF-32 code units needed for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf32::MAX_UNITS_PER_CHAR`].
    #[must_use]
    #[inline]
    pub const fn max_units_per_char(self) -> usize {
        Utf32::MAX_UNITS_PER_CHAR
    }
}

impl CharsetCodec for Utf32U32Codec {
    type Unit = u32;
    /// Returns UTF-32 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_32`].
    #[inline]
    fn charset(&self) -> Charset {
        Charset::UTF_32
    }

    /// Returns the fixed size (1 unit) for one UTF-32 scalar value.
    ///
    /// # Returns
    ///
    /// Returns [`Utf32::MAX_UNITS_PER_CHAR`].
    #[inline]
    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_UNITS_PER_CHAR
    }

    /// Decodes one UTF-32 scalar value from a `u32` prefix.
    ///
    /// # Arguments
    ///
    /// * `input` - UTF-32 unit slice.
    /// * `index` - Start offset for parsing; must satisfy `index <= input.len()`.
    ///
    /// # Returns
    ///
    /// * `Ok(DecodeStatus::NeedMore { required, available })` when no unit is available.
    /// * `Ok(DecodeStatus::Complete { value, consumed })` with `consumed == 1`.
    ///
    /// # Errors
    ///
    /// * [`crate::CharsetDecodeErrorKind::InvalidInputIndex`] when `index` is
    ///   greater than `input.len()`.
    /// * [`crate::CharsetDecodeErrorKind::InvalidCodePoint`] when unit is not a
    ///   valid scalar.
    fn decode_one(&self, input: &[u32], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        utf32::decode_units_prefix(input, index)
    }

    /// Encodes one Unicode scalar value into a `u32` unit at `index`.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode scalar value to encode.
    /// * `output` - Destination `u32` buffer.
    /// * `index` - Start offset where one unit is written; must satisfy
    ///   `index < output.len()`.
    ///
    /// # Returns
    ///
    /// Always returns `Ok(1)` on success.
    ///
    /// # Errors
    ///
    /// * [`crate::CharsetEncodeErrorKind::BufferTooSmall`] if `output` has no
    ///   room at `index`.
    fn encode_one(&self, ch: char, output: &mut [u32], index: usize) -> CharsetEncodeResult<usize> {
        utf32::encode_units_char(ch, output, index)
    }
}
