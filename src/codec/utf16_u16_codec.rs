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
    Charset,
    DecodeStatus,
    TextDecodeResult,
    TextDecoder,
    TextEncodeResult,
    TextEncoder,
    Utf16,
};

use super::helpers;

/// Combined UTF-16 `u16` code-unit codec.
///
/// `Utf16U16Codec` works with UTF-16 code units rather than serialized bytes.
/// Use [`crate::Utf16ByteCodec`] when the input or output is a byte stream with an
/// explicit byte order.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     DecodeStatus,
///     TextDecoder,
///     TextEncoder,
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
/// let written = codec.encode_char('😀', &mut output).expect("buffer fits");
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '😀',
///         consumed: written,
///     },
///     codec.decode_prefix(&output[..written]).expect("valid UTF-16"),
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
    pub const fn charset(self) -> Charset {
        Charset::UTF_16
    }

    /// Returns the maximum number of UTF-16 code units needed for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf16::MAX_UNITS_PER_CHAR`].
    #[must_use]
    pub const fn max_units_per_char(self) -> usize {
        Utf16::MAX_UNITS_PER_CHAR
    }
}

impl TextDecoder<u16> for Utf16U16Codec {
    fn charset(&self) -> Charset {
        Charset::UTF_16
    }

    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_UNITS_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u16]) -> TextDecodeResult<DecodeStatus<char>> {
        helpers::decode_utf16_units_prefix(input)
    }
}

impl TextEncoder<u16> for Utf16U16Codec {
    fn charset(&self) -> Charset {
        Charset::UTF_16
    }

    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_UNITS_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u16]) -> TextEncodeResult<usize> {
        helpers::encode_utf16_units_char(ch, output)
    }
}
