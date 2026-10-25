// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::Codec;
use qubit_utils::UncheckedSlice;

use crate::Charset;
use crate::CharsetCodec;
use crate::CharsetDecodeError;
use crate::CharsetDecodeErrorKind;
use crate::CharsetDecodeResult;
use crate::CharsetEncodeError;
use crate::CharsetEncodeResult;
use crate::Unicode;
use crate::Utf32;
use crate::error::CharsetCodecDecodeResult;

/// Combined UTF-32 `u32` code-unit codec.
///
/// `Utf32U32Codec` works with raw UTF-32 scalar-value units rather than
/// serialized bytes. Use [`crate::Utf32ByteCodec`] for byte streams with an
/// explicit byte order.
///
/// # Invariant
///
/// `u32` units are interpreted as host `u32` values that already contain
/// UTF-32 scalar values. This codec does not define a serialized byte order.
/// Data stored in files, sent over a network, or exchanged across byte streams
/// must be serialized with [`crate::Utf32ByteCodec`] first.
///
/// # Examples
///
/// ```rust
/// use qubit_codec::Codec;
/// use qubit_codec_text::{
///     CharsetCodec,
///     Charset,
///     Utf32,
///     Utf32U32Codec,
/// };
///
/// let mut codec = Utf32U32Codec;
/// assert_eq!(Charset::UTF_32, codec.charset());
/// assert_eq!(
///     Utf32::MAX_UNITS_PER_CHAR,
///     <Utf32U32Codec as Codec>::MAX_ENCODE_UNITS_PER_VALUE,
/// );
///
/// let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];
/// let written = codec.encode_len(&'中');
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

impl Codec for Utf32U32Codec {
    type Value = char;
    type Unit = u32;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = 1;
    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;

    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;

    /// Decodes one UTF-32 scalar value from the unit at `input_index`.
    ///
    /// # Parameters
    ///
    /// - `input`: UTF-32 scalar-value units.
    /// - `input_index`: Index of the unit to decode.
    ///
    /// # Returns
    ///
    /// Returns the decoded character and the number of consumed units, which
    /// is always one.
    ///
    /// # Errors
    ///
    /// Returns a decode error when the unit is not a valid Unicode scalar
    /// value.
    ///
    /// # Safety
    ///
    /// `input_index` must be a valid boundary in `input`, with at least
    /// `MIN_UNITS_PER_VALUE` readable units from that index. For this codec,
    /// that is one `u32` unit.
    #[inline]
    unsafe fn decode(
        &mut self,
        input: &[u32],
        input_index: usize,
    ) -> CharsetCodecDecodeResult<(char, core::num::NonZeroUsize)> {
        // SAFETY: Codec::decode requires one readable unit.
        let (ch, consumed) =
            unsafe { decode_units_prefix(input, input_index) }.map_err(CharsetDecodeError::into_codec_failure)?;
        debug_assert!(consumed.get() <= input.len().saturating_sub(input_index));
        Ok((ch, consumed))
    }

    /// Encodes one character as a UTF-32 scalar-value unit.
    ///
    /// # Parameters
    ///
    /// - `ch`: Character to encode.
    /// - `output`: Destination UTF-32 unit buffer.
    /// - `output_index`: Index at which to write the encoded unit.
    ///
    /// # Returns
    ///
    /// Returns the number of written units, which is always one.
    ///
    /// # Safety
    ///
    /// `can_encode_value` must return `true` for `ch`, and `output` must have
    /// either one writable unit from `output_index` or the exact length
    /// returned by `encode_len` for this value and codec state. Both lengths
    /// are one unit for this codec.
    #[inline]
    unsafe fn encode(&mut self, ch: &char, output: &mut [u32], output_index: usize) -> CharsetEncodeResult<usize> {
        // SAFETY: Codec::encode guarantees encode_len(ch) writable units.
        let written = unsafe { encode_units_char(*ch, output, output_index) };
        debug_assert_eq!(written, Utf32::MAX_UNITS_PER_CHAR);
        debug_assert!(written <= output.len().saturating_sub(output_index));
        Ok(Utf32::MAX_UNITS_PER_CHAR)
    }
}

/// Decodes the first UTF-32 character from a `u32` unit buffer.
///
/// Each UTF-32 unit is interpreted as a Unicode scalar value directly.
///
/// # Parameters
///
/// * `input` - UTF-32 unit slice to decode from.
/// * `index` - Start offset in `input`; at least one unit must be readable.
///
/// # Returns
///
/// Returns the decoded character and `1` consumed unit.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::InvalidCodePoint` when `input[index]` is not a
///   valid scalar.
///
/// # Safety
///
/// `index` must be strictly smaller than `input.len()`.
#[inline]
unsafe fn decode_units_prefix(input: &[u32], index: usize) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
    debug_assert!(index < input.len());
    // SAFETY: The helper precondition establishes `index < input.len()`.
    let unit = unsafe { UncheckedSlice::read(input, index) };
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
/// # Parameters
///
/// * `ch` - The character to encode.
/// * `output` - Destination unit buffer.
/// * `index` - Start offset in `output`; one unit must be writable.
///
/// # Returns
///
/// Returns the number of written units, which is always one.
///
/// # Safety
///
/// `output` must have one writable unit at `index`.
#[inline]
unsafe fn encode_units_char(ch: char, output: &mut [u32], index: usize) -> usize {
    debug_assert!(index < output.len());
    // SAFETY: The helper precondition establishes that one unit is writable at
    // `index`.
    unsafe {
        UncheckedSlice::write(output, index, ch as u32);
    }
    1
}
