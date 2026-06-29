// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::num::NonZeroUsize;

use crate::error::{CharsetCodecDecodeResult, map_charset_decode_failure};
use crate::{
    Charset, CharsetCodec, CharsetDecodeError, CharsetDecodeErrorKind, CharsetDecodeResult,
    CharsetEncodeError, CharsetEncodeResult, Unicode, Utf8,
};
use qubit_codec::Codec;
use qubit_io::UncheckedSlice;

/// UTF-8 byte-buffer charset codec.
///
/// # Examples
///
/// ```rust
/// use qubit_codec::Codec;
/// use qubit_codec_text::{
///     CharsetCodec,
///     Charset,
///     Utf8,
///     Utf8Codec,
/// };
///
/// let mut codec = Utf8Codec;
/// assert_eq!(Charset::UTF_8, codec.charset());
/// assert_eq!(
///     Utf8::MAX_UNITS_PER_CHAR,
///     <Utf8Codec as Codec>::MAX_UNITS_PER_VALUE.get(),
/// );
///
/// let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
/// let written = codec.encode_len(&'é').get();
/// unsafe {
///     codec.encode(&'é', &mut output, 0).expect("buffer fits");
/// }
/// let (value, consumed) = unsafe {
///     codec.decode(&output[..written], 0).expect("valid UTF-8")
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
    #[inline]
    pub const fn charset(self) -> Charset {
        Charset::UTF_8
    }
}

impl CharsetCodec for Utf8Codec {
    /// Returns UTF-8 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_8`].
    #[inline]
    fn charset(&self) -> Charset {
        Charset::UTF_8
    }
}

impl Codec for Utf8Codec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: NonZeroUsize = NonZeroUsize::MIN;
    const MAX_UNITS_PER_VALUE: NonZeroUsize = qubit_io::nz!(Utf8::MAX_UNITS_PER_CHAR);

    #[inline]
    fn encode_len(&self, ch: &char) -> NonZeroUsize {
        qubit_io::nz!(Utf8::byte_len(*ch))
    }

    #[inline]
    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> CharsetCodecDecodeResult<(char, NonZeroUsize)> {
        let (ch, consumed) =
            decode_prefix(input, input_index).map_err(map_charset_decode_failure)?;
        debug_assert!(consumed.get() <= input.len().saturating_sub(input_index));
        Ok((ch, consumed))
    }

    #[inline]
    unsafe fn encode(
        &mut self,
        ch: &char,
        output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<NonZeroUsize> {
        let written = encode_char(*ch, output, output_index);
        debug_assert_eq!(written, Utf8::byte_len(*ch));
        debug_assert!(written <= output.len().saturating_sub(output_index));
        Ok(qubit_io::nz!(written))
    }
}

/// Decodes the first UTF-8 character from a closed byte slice starting at
/// `index`.
///
/// The caller normally provides at least `Utf8::MAX_UNITS_PER_CHAR` readable
/// bytes from `index`. If fewer bytes are present, this function treats the
/// slice as closed at EOF: complete shorter UTF-8 sequences still decode, while
/// truncated sequences return [`CharsetDecodeErrorKind::IncompleteSequence`].
///
/// # Arguments
///
/// * `input` - UTF-8 byte slice to decode from.
/// * `index` - Start offset in `input`; must be `<= input.len()`.
///
/// # Returns
///
/// Returns the decoded character and the non-zero number of consumed bytes.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::MalformedSequence` when the first byte or
///   continuation bytes are invalid for UTF-8.
/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before the
///   complete UTF-8 sequence is available.
#[inline]
fn decode_prefix(input: &[u8], index: usize) -> CharsetDecodeResult<(char, NonZeroUsize)> {
    debug_assert!(index < input.len());
    // SAFETY: The caller guarantees that at least one byte is readable from
    // `index`.
    let first = unsafe { qubit_io::UncheckedSlice::read(input, index) };
    let length = match Utf8::byte_len_from_leading_byte(first) {
        Some(length) => length,
        None => {
            let kind = CharsetDecodeErrorKind::malformed(first as u32);
            return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index));
        }
    };
    if !UncheckedSlice::range_fits(input.len(), index, length) {
        validate_partial(input, index)?;
        let kind = CharsetDecodeErrorKind::IncompleteSequence {
            required: length,
            available: input.len() - index,
        };
        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index));
    }
    let code_point = match length {
        1 => first as u32,
        2 => decode_two(input, index)?,
        3 => decode_three(input, index)?,
        4 => decode_four(input, index)?,
        _ => unreachable!("UTF-8 sequence length is limited to four bytes"),
    };
    let ch = Unicode::to_char(code_point).expect("well-formed UTF-8 decodes to a Unicode scalar");
    Ok((ch, qubit_io::nz!(length)))
}

/// Encodes one Unicode scalar value into UTF-8 at `index` in `output`.
///
/// The function writes the byte sequence for `ch` and returns how many bytes
/// were written.
///
/// # Arguments
///
/// * `ch` - The character to encode.
/// * `output` - Destination buffer.
/// * `index` - Start offset in `output`; must satisfy `index <= output.len()`.
///
/// # Returns
///
/// `Ok(usize)` with the number of UTF-8 bytes written (`1..=4`).
#[inline]
fn encode_char(ch: char, output: &mut [u8], index: usize) -> usize {
    let length = Utf8::byte_len(ch);
    debug_assert!(
        UncheckedSlice::range_fits(output.len(), index, length),
        "index + length exceeds output length"
    );
    // SAFETY: The caller guarantees that `length` bytes are writable from
    // `index`; `encode_utf8` writes directly into that checked range.
    let target = unsafe { qubit_io::UncheckedSlice::subslice_mut(output, index, length) };
    ch.encode_utf8(target);
    length
}

/// Decodes a two-byte UTF-8 sequence starting at `index`.
///
/// # Arguments
///
/// * `input` - Byte slice containing the sequence.
/// * `index` - Start offset of a two-byte leading byte.
///
/// # Returns
///
/// The decoded Unicode scalar value as a `u32` on success.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::MalformedSequence` when the second byte is not a
///   valid UTF-8 continuation byte.
#[inline]
fn decode_two(input: &[u8], index: usize) -> CharsetDecodeResult<u32> {
    let first = byte_at(input, index);
    let second = validate_second_byte(input, index)?;
    Ok((((first & 0x1f) as u32) << 6) | ((second & 0x3f) as u32))
}

/// Validates the bytes already present in an incomplete UTF-8 prefix.
///
/// This is used after the total sequence length is known, to catch malformed
/// continuation bytes before more data arrives.
///
/// # Arguments
///
/// * `input` - Prefix slice being decoded.
/// * `index` - Start offset of the current UTF-8 sequence.
///
/// # Returns
///
/// `Ok(())` if currently available bytes are structurally valid, otherwise a
/// decoding error describing the first malformed position.
#[inline]
fn validate_partial(input: &[u8], index: usize) -> CharsetDecodeResult<()> {
    if UncheckedSlice::range_fits(input.len(), index, 2) {
        validate_second_byte(input, index)?;
    }
    if UncheckedSlice::range_fits(input.len(), index, 3) {
        validate_continuation_byte(input, index, 2)?;
    }
    Ok(())
}

/// Checks whether `second` is legal for a UTF-8 leading byte `first`.
///
/// # Arguments
///
/// * `first` - UTF-8 leading byte.
/// * `second` - Byte to validate as the first continuation-like byte.
///
/// # Returns
///
/// `true` when the pair `(first, second)` is valid for UTF-8 sequence decoding,
/// otherwise `false`.
#[inline]
fn is_valid_second_byte(first: u8, second: u8) -> bool {
    match first {
        0xc2..=0xdf | 0xe1..=0xec | 0xee..=0xef | 0xf1..=0xf3 => Utf8::is_continuation_byte(second),
        0xe0 => (0xa0..=0xbf).contains(&second),
        0xed => (0x80..=0x9f).contains(&second),
        0xf0 => (0x90..=0xbf).contains(&second),
        0xf4 => (0x80..=0x8f).contains(&second),
        _ => false,
    }
}

/// Decodes a three-byte UTF-8 sequence starting at `index`.
///
/// # Arguments
///
/// * `input` - Byte slice containing the sequence.
/// * `index` - Start offset of a three-byte leading byte.
///
/// # Returns
///
/// The decoded Unicode scalar value as a `u32` on success.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::MalformedSequence` when the second or third byte
///   is invalid.
#[inline]
fn decode_three(input: &[u8], index: usize) -> CharsetDecodeResult<u32> {
    let first = byte_at(input, index);
    let second = validate_second_byte(input, index)?;
    let third = validate_continuation_byte(input, index, 2)?;
    Ok((((first & 0x0f) as u32) << 12) | (((second & 0x3f) as u32) << 6) | ((third & 0x3f) as u32))
}

/// Decodes a four-byte UTF-8 sequence starting at `index`.
///
/// # Arguments
///
/// * `input` - Byte slice containing the sequence.
/// * `index` - Start offset of a four-byte leading byte.
///
/// # Returns
///
/// The decoded Unicode scalar value as a `u32` on success.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::MalformedSequence` when any continuation byte is
///   invalid.
#[inline]
fn decode_four(input: &[u8], index: usize) -> CharsetDecodeResult<u32> {
    let first = byte_at(input, index);
    let second = validate_second_byte(input, index)?;
    let third = validate_continuation_byte(input, index, 2)?;
    let fourth = validate_continuation_byte(input, index, 3)?;
    Ok((((first & 0x07) as u32) << 18)
        | (((second & 0x3f) as u32) << 12)
        | (((third & 0x3f) as u32) << 6)
        | ((fourth & 0x3f) as u32))
}

/// Validates the first continuation-like byte after a UTF-8 leading byte.
#[inline]
fn validate_second_byte(input: &[u8], index: usize) -> CharsetDecodeResult<u8> {
    let first = byte_at(input, index);
    let second = byte_at(input, index + 1);
    if is_valid_second_byte(first, second) {
        Ok(second)
    } else {
        Err(malformed_byte_error(
            second,
            index.saturating_add(1),
            qubit_io::nz!(2),
        ))
    }
}

/// Validates a regular UTF-8 continuation byte at `offset` from `index`.
#[inline]
fn validate_continuation_byte(
    input: &[u8],
    index: usize,
    offset: usize,
) -> CharsetDecodeResult<u8> {
    let byte = byte_at(input, index + offset);
    if Utf8::is_continuation_byte(byte) {
        Ok(byte)
    } else {
        Err(malformed_byte_error(
            byte,
            index.saturating_add(offset),
            NonZeroUsize::new(offset + 1).expect("UTF-8 consumed width is non-zero"),
        ))
    }
}

/// Creates a malformed-byte error with consumed width metadata.
fn malformed_byte_error(byte: u8, index: usize, consumed: NonZeroUsize) -> CharsetDecodeError {
    let kind = CharsetDecodeErrorKind::malformed(byte as u32);
    CharsetDecodeError::new(Charset::UTF_8, kind, index).with_consumed(consumed)
}

/// Reads one byte from an already checked slice.
#[inline]
fn byte_at(input: &[u8], index: usize) -> u8 {
    debug_assert!(index < input.len());
    // SAFETY: Callers check sequence availability before reading the byte.
    unsafe { qubit_io::UncheckedSlice::read(input, index) }
}
