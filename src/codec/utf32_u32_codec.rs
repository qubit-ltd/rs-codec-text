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
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeProbe,
    CharsetEncodeResult,
    Utf32,
};
use qubit_codec::Codec;

/// Combined UTF-32 `u32` code-unit codec.
///
/// `Utf32U32Codec` works with raw UTF-32 scalar-value units rather than
/// serialized bytes. Use [`crate::Utf32ByteCodec`] for byte streams with an
/// explicit byte order.
///
/// # Examples
///
/// ```rust
/// use qubit_codec_text::{
///     CharsetCodec,
///     CharsetEncodeProbe,
///     Codec,
///     Charset,
///     Utf32,
///     Utf32U32Codec,
/// };
///
/// let codec = Utf32U32Codec;
/// assert_eq!(Charset::UTF_32, codec.charset());
/// assert_eq!(Utf32::MAX_UNITS_PER_CHAR, codec.max_units_per_value().get());
///
/// let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];
/// let written = codec.encode_len('中', 0).expect("mappable");
/// unsafe {
///     codec.encode_unchecked(&'中', &mut output, 0).expect("buffer fits");
/// }
/// let (value, consumed) = unsafe {
///     codec.decode_unchecked(&output[..written], 0).expect("valid UTF-32")
/// };
/// assert_eq!(('中', written), (value, consumed.get()));
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
    #[inline(always)]
    pub const fn charset(self) -> Charset {
        Charset::UTF_32
    }
}

impl CharsetCodec for Utf32U32Codec {
    type Unit = u32;

    /// Returns UTF-32 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_32`].
    #[inline(always)]
    fn charset(&self) -> Charset {
        Charset::UTF_32
    }
}

impl CharsetEncodeProbe for Utf32U32Codec {
    /// Encodes one Unicode scalar value into a `u32` unit at `index`.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode scalar value to encode.
    /// * `index` - Input character index used for error context.
    ///
    /// # Returns
    ///
    /// Always returns `Ok(1)`.
    #[inline(always)]
    fn encode_len(&self, _ch: char, _index: usize) -> CharsetEncodeResult<usize> {
        Ok(Utf32::MAX_UNITS_PER_CHAR)
    }
}

unsafe impl Codec<char, u32> for Utf32U32Codec {
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
        input: &[u32],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let (ch, consumed) = utf32::decode_units_prefix(input, index)?;
        debug_assert!(consumed.get() <= input.len() - index);
        Ok((ch, consumed))
    }

    #[inline(always)]
    unsafe fn encode_unchecked(&self, ch: &char, output: &mut [u32], index: usize) -> CharsetEncodeResult<usize> {
        let written = utf32::encode_units_char(*ch, output, index)?;
        debug_assert_eq!(written, Utf32::MAX_UNITS_PER_CHAR);
        debug_assert!(written <= output.len() - index);
        Ok(written)
    }
}
