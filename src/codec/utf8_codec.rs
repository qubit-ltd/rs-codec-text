/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use super::inner::utf8;
use crate::{
    Charset,
    CharsetCodec,
    CharsetDecodeResult,
    CharsetEncodeResult,
    DecodeStatus,
    Utf8,
};

/// UTF-8 byte-buffer charset codec.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     CharsetCodec,
///     DecodeStatus,
///     Charset,
///     Utf8,
///     Utf8Codec,
/// };
///
/// let codec = Utf8Codec;
/// assert_eq!(Charset::UTF_8, codec.charset());
/// assert_eq!(Utf8::MAX_UNITS_PER_CHAR, codec.max_units_per_char());
///
/// let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
/// let written = codec.encode_one('é', &mut output, 0).expect("buffer fits");
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: 'é',
///         consumed: written,
///     },
///     codec.decode_one(&output[..written], 0).expect("valid UTF-8"),
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf8Codec;

impl Utf8Codec {
    /// Returns the UTF-8 encoding descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_8`].
    #[must_use]
    #[inline]
    pub const fn charset(self) -> Charset {
        Charset::UTF_8
    }

    /// Returns the maximum number of UTF-8 bytes needed for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf8::MAX_UNITS_PER_CHAR`].
    #[must_use]
    #[inline]
    pub const fn max_units_per_char(self) -> usize {
        Utf8::MAX_UNITS_PER_CHAR
    }
}

impl CharsetCodec for Utf8Codec {
    type Unit = u8;
    /// Returns UTF-8 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_8`].
    #[inline]
    fn charset(&self) -> Charset {
        Charset::UTF_8
    }

    /// Returns the maximum number of UTF-8 bytes for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf8::MAX_UNITS_PER_CHAR`].
    #[inline]
    fn max_units_per_char(&self) -> usize {
        Utf8::MAX_UNITS_PER_CHAR
    }

    /// Decodes one UTF-8 character from a byte prefix.
    ///
    /// # Arguments
    ///
    /// * `input` - UTF-8 byte slice.
    /// * `index` - Start offset for decoding; must satisfy `index <= input.len()`.
    ///
    /// # Returns
    ///
    /// * `Ok(DecodeStatus::NeedMore { required, available })` for partial input.
    /// * `Ok(DecodeStatus::Complete { value, consumed })` for a decoded scalar value.
    ///
    /// # Errors
    ///
    /// * [`crate::CharsetDecodeErrorKind::InvalidInputIndex`] when `index` is
    ///   greater than `input.len()`.
    /// * [`crate::CharsetDecodeErrorKind::MalformedSequence`] for invalid UTF-8
    ///   byte sequence.
    fn decode_one(&self, input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        utf8::decode_prefix(input, index)
    }

    /// Encodes one Unicode scalar value into UTF-8 bytes at `index`.
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
    /// `Ok(usize)` with encoded bytes (`1..=4`).
    ///
    /// # Errors
    ///
    /// * [`crate::CharsetEncodeErrorKind::BufferTooSmall`] if output has
    ///   insufficient bytes from `index`.
    fn encode_one(&self, ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        utf8::encode_char(ch, output, index)
    }
}
