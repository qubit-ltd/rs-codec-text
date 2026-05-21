/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0.
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use crate::{
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeResult,
    DecodeStatus,
    Unicode,
};

/// Single-byte ISO-8859-1 codec for bytes.
///
/// `Latin1Codec` converts between ISO-8859-1 bytes and Unicode scalar values.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Latin1Codec;

impl Latin1Codec {
    /// Returns the ISO-8859-1 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::ISO_8859_1`].
    #[must_use]
    #[inline]
    pub const fn charset(self) -> Charset {
        Charset::ISO_8859_1
    }

    /// Returns the maximum number of bytes needed for one Latin-1 character.
    ///
    /// # Returns
    ///
    /// Returns `1`.
    #[must_use]
    #[inline]
    pub const fn max_units_per_char(self) -> usize {
        1
    }
}

impl CharsetCodec<u8> for Latin1Codec {
    /// Returns the charset descriptor for this codec.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::ISO_8859_1`].
    #[inline]
    fn charset(&self) -> Charset {
        Charset::ISO_8859_1
    }

    /// Returns the maximum number of output bytes for one character.
    ///
    /// # Returns
    ///
    /// Returns `1`.
    #[inline]
    fn max_units_per_char(&self) -> usize {
        1
    }

    /// Decodes one ISO-8859-1 byte into a `char`.
    ///
    /// # Parameters
    ///
    /// - `input`: Complete input byte slice.
    /// - `index`: Absolute byte index at which decoding starts.
    ///
    /// # Returns
    ///
    /// `Ok(DecodeStatus::Complete { value, consumed: 1 })` always when input exists.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetDecodeError::malformed_sequence`] when `index` is out of
    /// range.
    #[inline]
    fn decode_one(&self, input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        if index > input.len() {
            return Err(CharsetDecodeError::malformed_sequence(
                Charset::ISO_8859_1,
                index,
            ));
        }

        if index == input.len() {
            return Ok(DecodeStatus::NeedMore {
                required: index + 1,
                available: 0,
            });
        }

        let value = input[index] as u32;
        Ok(DecodeStatus::Complete {
            value: Unicode::to_char(value).expect("valid Latin-1 byte decodes to Unicode scalar"),
            consumed: 1,
        })
    }

    /// Encodes one `char` into one ISO-8859-1 byte.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to encode.
    /// - `output`: Output byte slice.
    /// - `index`: Absolute output index where writing starts.
    ///
    /// # Returns
    ///
    /// `Ok(1)` when one byte is written.
    ///
    /// # Errors
    ///
    /// * `CharsetEncodeErrorKind::BufferTooSmall` if `index >= output.len()`.
    /// * `CharsetEncodeErrorKind::UnmappableCharacter` if `ch` > `U+00FF`.
    #[inline]
    fn encode_one(&self, ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        if index >= output.len() {
            return Err(CharsetEncodeError::buffer_too_small(
                Charset::ISO_8859_1,
                index,
            ));
        }

        let value = ch as u32;
        if value > Unicode::LATIN1_MAX {
            return Err(CharsetEncodeError::unmappable_character(
                Charset::ISO_8859_1,
                index,
                value,
            ));
        }

        // Since we validated `value`, cast is safe for 0..=0xFF.
        output[index] = value as u8;
        Ok(1)
    }
}
