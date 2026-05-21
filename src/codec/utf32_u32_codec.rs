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
    DecodeStatus,
    TextDecodeResult,
    TextDecoder,
    TextEncodeResult,
    TextEncoder,
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
/// use qubit_text_codec::{
///     DecodeStatus,
///     TextDecoder,
///     TextEncoder,
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
/// let written = codec.encode_char('中', &mut output, 0).expect("buffer fits");
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '中',
///         consumed: written,
///     },
///     codec.decode_prefix(&output[..written], 0).expect("valid UTF-32"),
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
    pub const fn charset(self) -> Charset {
        Charset::UTF_32
    }

    /// Returns the maximum number of UTF-32 code units needed for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf32::MAX_UNITS_PER_CHAR`].
    #[must_use]
    pub const fn max_units_per_char(self) -> usize {
        Utf32::MAX_UNITS_PER_CHAR
    }
}

impl TextDecoder<u32> for Utf32U32Codec {
    fn charset(&self) -> Charset {
        Charset::UTF_32
    }

    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_UNITS_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u32], index: usize) -> TextDecodeResult<DecodeStatus> {
        utf32::decode_units_prefix(input, index)
    }
}

impl TextEncoder<u32> for Utf32U32Codec {
    fn charset(&self) -> Charset {
        Charset::UTF_32
    }

    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_UNITS_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u32], index: usize) -> TextEncodeResult<usize> {
        utf32::encode_units_char(ch, output, index)
    }
}
