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
    Ascii, Charset, CharsetCodec, CharsetDecodeError, CharsetDecodeErrorKind, CharsetDecodeResult,
    CharsetEncodeError, CharsetEncodeErrorKind, CharsetEncodeResult, DecodeStatus,
};

/// Single-byte ASCII codec for bytes.
///
/// `AsciiCodec` converts between one-byte ASCII-encoded data and Unicode scalar
/// values.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct AsciiCodec;

impl AsciiCodec {
    /// Returns the ASCII charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::ASCII`].
    #[must_use]
    #[inline]
    pub const fn charset(self) -> Charset {
        Charset::ASCII
    }

    /// Returns the maximum number of bytes needed for one ASCII character.
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

impl CharsetCodec for AsciiCodec {
    type Unit = u8;
    /// Returns the charset descriptor for this codec.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::ASCII`].
    #[inline]
    fn charset(&self) -> Charset {
        Charset::ASCII
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

    /// Decodes one ASCII byte into a `char`.
    ///
    /// # Parameters
    ///
    /// - `input`: Complete input byte slice.
    /// - `index`: Absolute byte index at which decoding starts.
    ///
    /// # Returns
    ///
    /// `Ok(DecodeStatus::Complete { value, consumed: 1 })` for ASCII bytes.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetDecodeErrorKind::MalformedSequence`] when:
    /// - `index` is out of range, or
    /// - current byte is not in the ASCII range `0x00..=0x7F`.
    #[inline]
    fn decode_one(&self, input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        if index > input.len() {
            let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, index));
        }

        if index == input.len() {
            return Ok(DecodeStatus::NeedMore {
                required: index + 1,
                available: 0,
            });
        }

        let value = input[index];
        if value > Ascii::MAX_BYTE {
            let kind = CharsetDecodeErrorKind::MalformedSequence {
                value: Some(value as u32),
            };
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, index));
        }

        Ok(DecodeStatus::Complete {
            value: value as char,
            consumed: 1,
        })
    }

    /// Encodes one `char` into one ASCII byte.
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
    /// * `CharsetEncodeErrorKind::UnmappableCharacter` if `ch` is not ASCII.
    #[inline]
    fn encode_one(&self, ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        if index >= output.len() {
            let kind = CharsetEncodeErrorKind::BufferTooSmall {
                required: index + 1,
                available: 0,
            };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }

        if ch > Ascii::MAX_CHAR {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        // Since we validated `ch`, the cast is safe.
        output[index] = ch as u8;
        Ok(1)
    }
}
