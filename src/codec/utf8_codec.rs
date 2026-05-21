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
    Utf8,
};

use super::helpers;

/// Combined UTF-8 byte-buffer codec.
///
/// `Utf8Codec` implements both [`TextEncoder<u8>`] and [`TextDecoder<u8>`].
/// It also exposes inherent metadata methods so callers can use
/// `codec.charset()` without disambiguating between the encoder and decoder
/// traits.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     DecodeStatus,
///     TextDecoder,
///     TextEncoder,
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
/// let written = codec.encode_char('é', &mut output).expect("buffer fits");
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: 'é',
///         consumed: written,
///     },
///     codec.decode_prefix(&output[..written]).expect("valid UTF-8"),
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
    pub const fn charset(self) -> Charset {
        Charset::UTF_8
    }

    /// Returns the maximum number of UTF-8 bytes needed for one character.
    ///
    /// # Returns
    ///
    /// Returns [`Utf8::MAX_UNITS_PER_CHAR`].
    #[must_use]
    pub const fn max_units_per_char(self) -> usize {
        Utf8::MAX_UNITS_PER_CHAR
    }
}

impl TextDecoder<u8> for Utf8Codec {
    fn charset(&self) -> Charset {
        Charset::UTF_8
    }

    fn max_units_per_char(&self) -> usize {
        Utf8::MAX_UNITS_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u8]) -> TextDecodeResult<DecodeStatus<char>> {
        helpers::decode_utf8_prefix(input)
    }
}

impl TextEncoder<u8> for Utf8Codec {
    fn charset(&self) -> Charset {
        Charset::UTF_8
    }

    fn max_units_per_char(&self) -> usize {
        Utf8::MAX_UNITS_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u8]) -> TextEncodeResult<usize> {
        helpers::encode_utf8_char(ch, output)
    }
}
