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
    CharsetDecodeError,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeProbe,
    CharsetEncodeResult,
    Utf8,
};
use qubit_codec::Codec;

/// UTF-8 byte-buffer charset codec.
///
/// # Examples
///
/// ```rust
/// use qubit_codec_text::{
///     CharsetCodec,
///     CharsetEncodeProbe,
///     Codec,
///     Charset,
///     Utf8,
///     Utf8Codec,
/// };
///
/// let codec = Utf8Codec;
/// assert_eq!(Charset::UTF_8, codec.charset());
/// assert_eq!(Utf8::MAX_UNITS_PER_CHAR, codec.max_units_per_value().get());
///
/// let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
/// let written = codec.encode_len('é', 0).expect("mappable");
/// unsafe {
///     codec.encode_unchecked(&'é', &mut output, 0).expect("buffer fits");
/// }
/// let (value, consumed) = unsafe {
///     codec.decode_unchecked(&output[..written], 0).expect("valid UTF-8")
/// };
/// assert_eq!(('é', written), (value, consumed.get()));
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
    #[inline(always)]
    pub const fn charset(self) -> Charset {
        Charset::UTF_8
    }
}

impl CharsetCodec for Utf8Codec {
    type Unit = u8;

    /// Returns UTF-8 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_8`].
    #[inline(always)]
    fn charset(&self) -> Charset {
        Charset::UTF_8
    }
}

impl CharsetEncodeProbe for Utf8Codec {
    /// Encodes one Unicode scalar value into UTF-8 bytes at `index`.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode scalar value to encode.
    /// * `index` - Input character index used for error context.
    ///
    /// # Returns
    ///
    /// `Ok(usize)` with required encoded bytes (`1..=4`).
    #[inline(always)]
    fn encode_len(&self, ch: char, _index: usize) -> CharsetEncodeResult<usize> {
        Ok(Utf8::byte_len(ch))
    }
}

unsafe impl Codec<char, u8> for Utf8Codec {
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    #[inline(always)]
    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    #[inline(always)]
    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        // SAFETY: UTF-8 encodes every scalar value as at least one byte.
        unsafe { core::num::NonZeroUsize::new_unchecked(Utf8::MAX_UNITS_PER_CHAR) }
    }

    #[inline]
    unsafe fn decode_unchecked(
        &self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let (ch, consumed) = utf8::decode_prefix(input, index)?;
        debug_assert!(consumed.get() <= input.len() - index);
        Ok((ch, consumed))
    }

    #[inline]
    unsafe fn encode_unchecked(&self, ch: &char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        let written = utf8::encode_char(*ch, output, index)?;
        debug_assert_eq!(written, Utf8::byte_len(*ch));
        debug_assert!(written <= output.len() - index);
        Ok(written)
    }
}
