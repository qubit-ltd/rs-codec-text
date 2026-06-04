/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use super::inner::utf16;
use crate::{
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeProbe,
    CharsetEncodeResult,
    Utf16,
};
use qubit_codec::Codec;

/// Combined UTF-16 `u16` code-unit codec.
///
/// `Utf16U16Codec` works with UTF-16 code units rather than serialized bytes.
/// Use [`crate::Utf16ByteCodec`] when the input or output is a byte stream with an
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
///     Utf16,
///     Utf16U16Codec,
/// };
///
/// let codec = Utf16U16Codec;
/// assert_eq!(Charset::UTF_16, codec.charset());
/// assert_eq!(Utf16::MAX_UNITS_PER_CHAR, codec.max_units_per_value().get());
///
/// let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];
/// let written = codec.encode_len('😀', 0).expect("mappable");
/// unsafe {
///     codec.encode_unchecked(&'😀', &mut output, 0).expect("buffer fits");
/// }
/// let (value, consumed) = unsafe {
///     codec.decode_unchecked(&output[..written], 0).expect("valid UTF-16")
/// };
/// assert_eq!(('😀', written), (value, consumed.get()));
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
    #[inline]
    pub const fn charset(self) -> Charset {
        Charset::UTF_16
    }
}

impl CharsetCodec for Utf16U16Codec {
    /// Returns UTF-16 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_16`].
    #[inline]
    fn charset(&self) -> Charset {
        Charset::UTF_16
    }
}

impl CharsetEncodeProbe for Utf16U16Codec {
    /// Encodes one Unicode scalar value into UTF-16 code units at `index`.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode scalar value to encode.
    /// * `index` - Input character index used for error context.
    ///
    /// # Returns
    ///
    /// `Ok(usize)` with the required UTF-16 units (`1` or `2`).
    #[inline]
    fn encode_len(&self, ch: char, _index: usize) -> CharsetEncodeResult<usize> {
        Ok(Utf16::unit_len(ch))
    }
}

unsafe impl Codec for Utf16U16Codec {
    type Value = char;
    type Unit = u16;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    #[inline]
    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    #[inline]
    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        // SAFETY: UTF-16 encodes every scalar value as at least one unit.
        unsafe { core::num::NonZeroUsize::new_unchecked(Utf16::MAX_UNITS_PER_CHAR) }
    }

    #[inline]
    unsafe fn decode_unchecked(
        &self,
        input: &[u16],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let (ch, consumed) = utf16::decode_units_prefix(input, index)?;
        debug_assert!(consumed.get() <= input.len() - index);
        Ok((ch, consumed))
    }

    #[inline(always)]
    unsafe fn encode_unchecked(&self, ch: &char, output: &mut [u16], index: usize) -> CharsetEncodeResult<usize> {
        let written = utf16::encode_units_char(*ch, output, index)?;
        debug_assert_eq!(written, ch.len_utf16());
        debug_assert!(written <= output.len() - index);
        Ok(written)
    }
}
