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
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeProbe,
    CharsetEncodeResult,
    Unicode,
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
    #[inline]
    pub const fn charset(self) -> Charset {
        Charset::UTF_32
    }
}

impl CharsetCodec for Utf32U32Codec {
    /// Returns UTF-32 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_32`].
    #[inline]
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
    #[inline]
    fn encode_len(&self, _ch: char, _index: usize) -> CharsetEncodeResult<usize> {
        Ok(Utf32::MAX_UNITS_PER_CHAR)
    }
}

unsafe impl Codec for Utf32U32Codec {
    type Value = char;
    type Unit = u32;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    #[inline]
    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    #[inline]
    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    #[inline]
    unsafe fn decode_unchecked(
        &self,
        input: &[u32],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let (ch, consumed) = decode_units_prefix(input, index)?;
        debug_assert!(consumed.get() <= input.len() - index);
        Ok((ch, consumed))
    }

    unsafe fn encode_unchecked(&self, ch: &char, output: &mut [u32], index: usize) -> CharsetEncodeResult<usize> {
        let written = encode_units_char(*ch, output, index)?;
        debug_assert_eq!(written, Utf32::MAX_UNITS_PER_CHAR);
        debug_assert!(written <= output.len() - index);
        Ok(written)
    }
}

/// Decodes the first UTF-32 character from a closed `u32` buffer.
///
/// Each UTF-32 unit is interpreted as a Unicode scalar value directly.
///
/// # Arguments
///
/// * `input` - UTF-32 unit slice to decode from.
/// * `index` - Start offset in `input`; must be `<= input.len()`.
///
/// # Returns
///
/// Returns the decoded character and `1` consumed unit.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::InvalidInputIndex` when `index` is greater than
///   `input.len()`.
/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before one
///   UTF-32 unit is available.
/// * `CharsetDecodeErrorKind::InvalidCodePoint` when `input[index]` is not a
///   valid scalar.
#[inline]
fn decode_units_prefix(input: &[u32], index: usize) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
    if index > input.len() {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
        return Err(CharsetDecodeError::new(Charset::UTF_32, kind, index));
    }
    if index == input.len() {
        let kind = CharsetDecodeErrorKind::IncompleteSequence {
            required: 1,
            available: 0,
        };
        return Err(CharsetDecodeError::new(Charset::UTF_32, kind, index));
    }
    match Unicode::to_char(input[index]) {
        Some(ch) => Ok((ch, core::num::NonZeroUsize::MIN)),
        None => {
            let kind = CharsetDecodeErrorKind::InvalidCodePoint { value: input[index] };
            Err(CharsetDecodeError::new(Charset::UTF_32, kind, index))
        }
    }
}

/// Encodes one character into a UTF-32 `u32` unit at `index` in `output`.
///
/// # Arguments
///
/// * `ch` - The character to encode.
/// * `output` - Destination unit buffer.
/// * `index` - Start offset in `output`; must satisfy `index < output.len()`.
///
/// # Returns
///
/// Always `Ok(1)` to indicate one unit was written.
///
/// # Errors
///
/// * `CharsetEncodeErrorKind::BufferTooSmall` when no unit can be written at
///   `index`.
#[inline]
fn encode_units_char(ch: char, output: &mut [u32], index: usize) -> CharsetEncodeResult<usize> {
    if index >= output.len() {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + 1,
            available: 0,
        };
        return Err(CharsetEncodeError::new(Charset::UTF_32, kind, index));
    }
    output[index] = ch as u32;
    Ok(1)
}
