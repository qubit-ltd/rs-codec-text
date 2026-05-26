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
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
    DecodeStatus,
    Unicode,
};
use qubit_codec::Codec;

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

impl CharsetCodec for Latin1Codec {
    type Unit = u8;
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
    /// Returns [`CharsetDecodeErrorKind::InvalidInputIndex`] when `index` is
    /// greater than `input.len()`.
    #[inline]
    fn decode_one(&self, input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        if index > input.len() {
            let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
            return Err(CharsetDecodeError::new(Charset::ISO_8859_1, kind, index));
        }

        if index == input.len() {
            return Ok(DecodeStatus::NeedMore {
                required: index + 1,
                available: 0,
            });
        }

        let (value, consumed) = unsafe { <Self as Codec<char, u8>>::decode_unchecked(self, input, index)? };
        Ok(DecodeStatus::Complete { value, consumed })
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
            let kind = CharsetEncodeErrorKind::BufferTooSmall {
                required: index + 1,
                available: 0,
            };
            return Err(CharsetEncodeError::new(Charset::ISO_8859_1, kind, index));
        }

        let value = ch as u32;
        if value > Unicode::LATIN1_MAX {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value };
            return Err(CharsetEncodeError::new(Charset::ISO_8859_1, kind, index));
        }

        unsafe { <Self as Codec<char, u8>>::encode_unchecked(self, ch, output, index) }
    }
}

unsafe impl Codec<char, u8> for Latin1Codec {
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    #[inline]
    fn min_units_per_value(&self) -> usize {
        1
    }

    #[inline]
    fn max_units_per_value(&self) -> usize {
        1
    }

    #[inline]
    unsafe fn decode_unchecked(&self, input: &[u8], index: usize) -> CharsetDecodeResult<(char, usize)> {
        debug_assert!(index < input.len());

        let value = input[index] as u32;
        Ok((
            Unicode::to_char(value).expect("valid Latin-1 byte decodes to Unicode scalar"),
            1,
        ))
    }

    #[inline]
    unsafe fn encode_unchecked(&self, ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        debug_assert!(index < output.len());

        let value = ch as u32;
        if value > Unicode::LATIN1_MAX {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value };
            return Err(CharsetEncodeError::new(Charset::ISO_8859_1, kind, index));
        }
        output[index] = value as u8;
        Ok(1)
    }
}
