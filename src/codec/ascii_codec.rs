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
    Ascii,
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeProbe,
    CharsetEncodeResult,
};
use qubit_codec::Codec;

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
    #[inline(always)]
    pub const fn charset(self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetCodec for AsciiCodec {
    /// Returns the charset descriptor for this codec.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::ASCII`].
    #[inline(always)]
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetEncodeProbe for AsciiCodec {
    /// Encodes one `char` into one ASCII byte.
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
    /// * `CharsetEncodeErrorKind::UnmappableCharacter` if `ch` is not ASCII.
    #[inline(always)]
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize> {
        if ch > Ascii::MAX_CHAR {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        Ok(1)
    }
}

unsafe impl Codec for AsciiCodec {
    type Value = char;
    type Unit = u8;
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
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, index));
        }
        if index == input.len() {
            let kind = CharsetDecodeErrorKind::IncompleteSequence {
                required: 1,
                available: 0,
            };
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, index));
        }

        let value = input[index];
        if value > Ascii::MAX_BYTE {
            let kind = CharsetDecodeErrorKind::MalformedSequence {
                value: Some(value as u32),
            };
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, index));
        }
        debug_assert!(index < input.len());
        Ok((value as char, core::num::NonZeroUsize::MIN))
    }

    #[inline(always)]
    unsafe fn encode_unchecked(&self, ch: &char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        debug_assert!(index < output.len());

        if *ch > Ascii::MAX_CHAR {
            let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: *ch as u32 };
            return Err(CharsetEncodeError::new(Charset::ASCII, kind, index));
        }
        output[index] = *ch as u8;
        Ok(1)
    }
}
