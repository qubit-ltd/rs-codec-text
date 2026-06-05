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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn encode_len(&self, ch: char, _index: usize) -> CharsetEncodeResult<usize> {
        Ok(Utf16::unit_len(ch))
    }
}

unsafe impl Codec for Utf16U16Codec {
    type Value = char;
    type Unit = u16;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    #[inline(always)]
    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    #[inline(always)]
    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        // SAFETY: UTF-16 encodes every scalar value as at least one unit.
        unsafe { core::num::NonZeroUsize::new_unchecked(Utf16::MAX_UNITS_PER_CHAR) }
    }

    #[inline(always)]
    unsafe fn decode_unchecked(
        &self,
        input: &[u16],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let (ch, consumed) = decode_units_prefix(input, index)?;
        debug_assert!(consumed.get() <= input.len() - index);
        Ok((ch, consumed))
    }

    #[inline(always)]
    unsafe fn encode_unchecked(&self, ch: &char, output: &mut [u16], index: usize) -> CharsetEncodeResult<usize> {
        let written = encode_units_char(*ch, output, index)?;
        debug_assert_eq!(written, ch.len_utf16());
        debug_assert!(written <= output.len() - index);
        Ok(written)
    }
}

/// Decodes the first UTF-16 character from a closed `u16` buffer.
///
/// The function handles three cases:
/// 1. ASCII/non-surrogate units decode to a single `char`.
/// 2. High-surrogate pairs are combined with the following unit into one scalar value.
/// 3. Isolated low-surrogates are rejected as malformed.
///
/// # Arguments
///
/// * `input` - UTF-16 unit slice to decode from. Normal streaming callers
///   provide at least [`Utf16::MAX_UNITS_PER_CHAR`] readable units unless EOF
///   has been reached.
/// * `index` - Start offset in `input`; must be `<= input.len()`.
///
/// # Returns
///
/// Returns the decoded character and the non-zero number of consumed UTF-16
/// units.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::InvalidInputIndex` when `index` is greater than
///   `input.len()`.
/// * `CharsetDecodeErrorKind::MalformedSequence` for invalid UTF-16 sequences
///   (invalid high/low surrogate pairing).
/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before a
///   complete surrogate pair is available.
///
/// # Panics
///
/// This function does not panic for invalid UTF-16 input because invalid input
/// is surfaced as `CharsetDecodeError`.
#[inline]
fn decode_units_prefix(input: &[u16], index: usize) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
    if index > input.len() {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
        return Err(CharsetDecodeError::new(Charset::UTF_16, kind, index));
    }
    if index == input.len() {
        let kind = CharsetDecodeErrorKind::IncompleteSequence {
            required: 1,
            available: 0,
        };
        return Err(CharsetDecodeError::new(Charset::UTF_16, kind, index));
    }
    let first = input[index];
    if Utf16::is_high_surrogate(first) {
        if !has_units(input.len(), index, 2) {
            let kind = CharsetDecodeErrorKind::IncompleteSequence {
                required: 2,
                available: input.len() - index,
            };
            return Err(CharsetDecodeError::new(Charset::UTF_16, kind, index));
        }
        let second = input[index + 1];
        match Utf16::compose_pair(first, second).and_then(Unicode::to_char) {
            Some(ch) => {
                // SAFETY: 2 is non-zero.
                Ok((ch, unsafe { core::num::NonZeroUsize::new_unchecked(2) }))
            }
            None => {
                let kind = CharsetDecodeErrorKind::MalformedSequence {
                    value: Some(second as u32),
                };
                Err(CharsetDecodeError::new(Charset::UTF_16, kind, required_index(index, 1)).with_consumed(2))
            }
        }
    } else if Utf16::is_low_surrogate(first) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(first as u32),
        };
        Err(CharsetDecodeError::new(Charset::UTF_16, kind, index))
    } else {
        let ch = char::from_u32(first as u32).expect("non-surrogate UTF-16 unit is a scalar value");
        Ok((ch, core::num::NonZeroUsize::MIN))
    }
}

/// Encodes one character into UTF-16 `u16` units at `index` in `output`.
///
/// The helper returns how many UTF-16 units are written:
/// one for BMP scalars, two for supplementary scalars.
///
/// # Arguments
///
/// * `ch` - The character to encode.
/// * `output` - Destination unit buffer.
/// * `index` - Start offset in `output`; must be `<= output.len()`.
///
/// # Returns
///
/// `Ok(usize)` with the number of written UTF-16 units (`1` or `2`).
///
/// # Errors
///
/// * `CharsetEncodeErrorKind::BufferTooSmall` when insufficient room exists
///   from `index`.
#[inline]
fn encode_units_char(ch: char, output: &mut [u16], index: usize) -> CharsetEncodeResult<usize> {
    if index > output.len() {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: required_index(index, 1),
            available: 0,
        };
        return Err(CharsetEncodeError::new(Charset::UTF_16, kind, index));
    }
    let length = Utf16::unit_len(ch);
    let available = output.len() - index;
    if available < length {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: required_index(index, length),
            available,
        };
        return Err(CharsetEncodeError::new(Charset::UTF_16, kind, index));
    }
    let code_point = ch as u32;
    if length == 1 {
        output[index] = code_point as u16;
    } else {
        output[index] = Utf16::high_surrogate(code_point).expect("supplementary scalar has high surrogate");
        output[index + 1] = Utf16::low_surrogate(code_point).expect("supplementary scalar has low surrogate");
    }
    Ok(length)
}

#[inline(always)]
const fn has_units(len: usize, index: usize, required_units: usize) -> bool {
    match index.checked_add(required_units) {
        Some(end) => len >= end,
        None => false,
    }
}

#[inline(always)]
const fn required_index(index: usize, required_units: usize) -> usize {
    match index.checked_add(required_units) {
        Some(required) => required,
        None => usize::MAX,
    }
}
