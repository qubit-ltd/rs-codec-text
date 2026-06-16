// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::{
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeResult,
    Unicode,
    Utf32,
};
use qubit_codec::{Codec, nz};

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
///     Codec,
///     Charset,
///     Utf32,
///     Utf32U32Codec,
/// };
///
/// let mut codec = Utf32U32Codec;
/// assert_eq!(Charset::UTF_32, codec.charset());
/// assert_eq!(Utf32::MAX_UNITS_PER_CHAR, codec.max_units_per_value().get());
///
/// let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];
/// let written = codec.encode_len(&'中').get();
/// unsafe {
///     codec.encode(&'中', &mut output, 0).expect("buffer fits");
/// }
/// let (value, consumed) = unsafe {
///     codec.decode(&output[..written], 0).expect("valid UTF-32")
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

unsafe impl Codec for Utf32U32Codec {
    type Value = char;
    type Unit = u32;
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
    unsafe fn decode(
        &mut self,
        input: &[u32],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let (ch, consumed) = decode_units_prefix(input, index)?;
        debug_assert!(consumed.get() <= input.len().saturating_sub(index));
        Ok((ch, consumed))
    }

    #[inline(always)]
    unsafe fn encode(
        &mut self,
        ch: &char,
        output: &mut [u32],
        index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        let written = encode_units_char(*ch, output, index);
        debug_assert_eq!(written, Utf32::MAX_UNITS_PER_CHAR);
        debug_assert!(written <= output.len().saturating_sub(index));
        Ok(nz!(Utf32::MAX_UNITS_PER_CHAR))
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
/// * `CharsetDecodeErrorKind::InvalidCodePoint` when `input[index]` is not a
///   valid scalar.
#[inline]
fn decode_units_prefix(
    input: &[u32],
    index: usize,
) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
    debug_assert!(index < input.len());
    // SAFETY: The caller guarantees that `index` is readable.
    let unit = unsafe { *input.as_ptr().add(index) };
    match Unicode::to_char(unit) {
        Some(ch) => Ok((ch, core::num::NonZeroUsize::MIN)),
        None => {
            let kind = CharsetDecodeErrorKind::InvalidCodePoint { value: unit };
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
#[inline]
fn encode_units_char(ch: char, output: &mut [u32], index: usize) -> usize {
    debug_assert!(index < output.len());
    // SAFETY: The caller guarantees that one unit is writable at `index`.
    unsafe {
        *output.as_mut_ptr().add(index) = ch as u32;
    }
    1
}
