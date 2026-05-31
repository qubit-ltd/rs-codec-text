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
    CharsetEncodeProbe,
    CharsetEncodeResult,
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
    #[inline(always)]
    pub const fn charset(self) -> Charset {
        Charset::ISO_8859_1
    }
}

impl CharsetCodec for Latin1Codec {
    type Unit = u8;

    /// Returns the charset descriptor for this codec.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::ISO_8859_1`].
    #[inline(always)]
    fn charset(&self) -> Charset {
        Charset::ISO_8859_1
    }
}

impl CharsetEncodeProbe for Latin1Codec {
    /// Encodes one `char` into one ISO-8859-1 byte.
    ///
    /// # Parameters
    ///
    /// - `ch`: The character to encode.
    /// - `index`: Input character index used for error context.
    ///
    /// # Returns
    ///
    /// `Ok(1)` when one byte is needed.
    ///
    /// # Errors
    ///
    /// * `CharsetEncodeErrorKind::UnmappableCharacter` if `ch` > `U+00FF`.
    #[inline(always)]
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        let value = ch as u32;
        if value > Unicode::LATIN1_MAX {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value };
            return Err(CharsetEncodeError::new(Charset::ISO_8859_1, kind, index));
        }

        Ok(1)
    }
}

unsafe impl Codec<char, u8> for Latin1Codec {
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    #[inline(always)]
    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    #[inline(always)]
    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    #[inline(always)]
    unsafe fn decode_unchecked(
        &self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        if index > input.len() {
            let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
            return Err(CharsetDecodeError::new(Charset::ISO_8859_1, kind, index));
        }
        if index == input.len() {
            let kind = CharsetDecodeErrorKind::IncompleteSequence {
                required: 1,
                available: 0,
            };
            return Err(CharsetDecodeError::new(Charset::ISO_8859_1, kind, index));
        }

        let value = input[index] as u32;
        debug_assert!(index < input.len());
        Ok((
            Unicode::to_char(value).expect("valid Latin-1 byte decodes to Unicode scalar"),
            core::num::NonZeroUsize::MIN,
        ))
    }

    #[inline(always)]
    unsafe fn encode_unchecked(&self, ch: &char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        debug_assert!(index < output.len());

        let value = *ch as u32;
        if value > Unicode::LATIN1_MAX {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value };
            return Err(CharsetEncodeError::new(Charset::ISO_8859_1, kind, index));
        }
        output[index] = value as u8;
        Ok(1)
    }
}
