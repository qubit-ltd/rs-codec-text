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
    CharsetEncodeErrorKind,
    CharsetEncodeProbe,
    CharsetEncodeResult,
    Unicode,
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
/// let mut codec = Utf8Codec;
/// assert_eq!(Charset::UTF_8, codec.charset());
/// assert_eq!(Utf8::MAX_UNITS_PER_CHAR, codec.max_units_per_value().get());
///
/// let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
/// let written = codec.encode_len('é', 0).expect("mappable");
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
    #[inline(always)]
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
    fn encode_len(
        &self,
        ch: char,
        _index: usize,
    ) -> CharsetEncodeResult<usize> {
        Ok(Utf8::byte_len(ch))
    }
}

unsafe impl Codec for Utf8Codec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;
    type DecodeState = ();
    type EncodeState = ();

    #[inline(always)]
    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::MIN
    }

    #[inline(always)]
    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        // SAFETY: UTF-8 encodes every scalar value as at least one byte.
        unsafe {
            core::num::NonZeroUsize::new_unchecked(Utf8::MAX_UNITS_PER_CHAR)
        }
    }

    #[inline(always)]
    unsafe fn decode(
        &mut self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let (ch, consumed) = decode_prefix(input, index)?;
        debug_assert!(consumed.get() <= input.len() - index);
        Ok((ch, consumed))
    }

    #[inline(always)]
    unsafe fn encode(
        &mut self,
        ch: &char,
        output: &mut [u8],
        index: usize,
    ) -> CharsetEncodeResult<usize> {
        let written = encode_char(*ch, output, index)?;
        debug_assert_eq!(written, Utf8::byte_len(*ch));
        debug_assert!(written <= output.len() - index);
        Ok(written)
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
/// * `CharsetDecodeErrorKind::InvalidInputIndex` when `index` is greater than
///   `input.len()`.
/// * `CharsetDecodeErrorKind::MalformedSequence` when the first byte or
///   continuation bytes are invalid for UTF-8.
/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before the
///   complete UTF-8 sequence is available.
#[inline]
fn decode_prefix(
    input: &[u8],
    index: usize,
) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
    if index > input.len() {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex {
            input_len: input.len(),
        };
        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index));
    }
    if index == input.len() {
        let kind = CharsetDecodeErrorKind::IncompleteSequence {
            required: 1,
            available: 0,
        };
        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index));
    }
    let first = input[index];
    let length = match Utf8::byte_len_from_leading_byte(first) {
        Some(length) => length,
        None => {
            let kind = CharsetDecodeErrorKind::MalformedSequence {
                value: Some(first as u32),
            };
            return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index));
        }
    };
    if !has_units(input.len(), index, length) {
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
    let ch = Unicode::to_char(code_point)
        .expect("well-formed UTF-8 decodes to a Unicode scalar");
    Ok((
        ch,
        core::num::NonZeroUsize::new(length)
            .expect("well-formed UTF-8 sequence has non-zero length"),
    ))
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
///
/// # Errors
///
/// * `CharsetEncodeErrorKind::BufferTooSmall` if the destination does not have
///   enough space starting from `index`.
#[inline]
fn encode_char(
    ch: char,
    output: &mut [u8],
    index: usize,
) -> CharsetEncodeResult<usize> {
    if index > output.len() {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: required_index(index, 1),
            available: 0,
        };
        return Err(CharsetEncodeError::new(Charset::UTF_8, kind, index));
    }
    let length = Utf8::byte_len(ch);
    let available = output.len() - index;
    if available < length {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: required_index(index, length),
            available,
        };
        return Err(CharsetEncodeError::new(Charset::UTF_8, kind, index));
    }
    let mut scratch = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
    let encoded = ch.encode_utf8(&mut scratch);
    output[index..index + length].copy_from_slice(encoded.as_bytes());
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
    let second = input[index + 1];
    if !Utf8::is_continuation_byte(second) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(second as u32),
        };
        return Err(CharsetDecodeError::new(
            Charset::UTF_8,
            kind,
            required_index(index, 1),
        )
        .with_consumed(2));
    }
    Ok((((input[index] & 0x1f) as u32) << 6) | ((second & 0x3f) as u32))
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
    if has_units(input.len(), index, 2)
        && !is_valid_second_byte(input[index], input[index + 1])
    {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(input[index + 1] as u32),
        };
        return Err(CharsetDecodeError::new(
            Charset::UTF_8,
            kind,
            required_index(index, 1),
        )
        .with_consumed(2));
    }
    if has_units(input.len(), index, 3)
        && !Utf8::is_continuation_byte(input[index + 2])
    {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(input[index + 2] as u32),
        };
        return Err(CharsetDecodeError::new(
            Charset::UTF_8,
            kind,
            required_index(index, 2),
        )
        .with_consumed(3));
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
#[inline(always)]
fn is_valid_second_byte(first: u8, second: u8) -> bool {
    match first {
        0xc2..=0xdf => Utf8::is_continuation_byte(second),
        0xe0 => (0xa0..=0xbf).contains(&second),
        0xed => (0x80..=0x9f).contains(&second),
        0xe1..=0xec | 0xee..=0xef => Utf8::is_continuation_byte(second),
        0xf0 => (0x90..=0xbf).contains(&second),
        0xf1..=0xf3 => Utf8::is_continuation_byte(second),
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
    let first = input[index];
    let second = input[index + 1];
    let third = input[index + 2];
    if !is_valid_second_byte(first, second) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(second as u32),
        };
        return Err(CharsetDecodeError::new(
            Charset::UTF_8,
            kind,
            required_index(index, 1),
        )
        .with_consumed(2));
    }
    if !Utf8::is_continuation_byte(third) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(third as u32),
        };
        return Err(CharsetDecodeError::new(
            Charset::UTF_8,
            kind,
            required_index(index, 2),
        )
        .with_consumed(3));
    }
    Ok((((first & 0x0f) as u32) << 12)
        | (((second & 0x3f) as u32) << 6)
        | ((third & 0x3f) as u32))
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
    let first = input[index];
    let second = input[index + 1];
    let third = input[index + 2];
    let fourth = input[index + 3];
    if !is_valid_second_byte(first, second) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(second as u32),
        };
        return Err(CharsetDecodeError::new(
            Charset::UTF_8,
            kind,
            required_index(index, 1),
        )
        .with_consumed(2));
    }
    if !Utf8::is_continuation_byte(third) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(third as u32),
        };
        return Err(CharsetDecodeError::new(
            Charset::UTF_8,
            kind,
            required_index(index, 2),
        )
        .with_consumed(3));
    }
    if !Utf8::is_continuation_byte(fourth) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(fourth as u32),
        };
        return Err(CharsetDecodeError::new(
            Charset::UTF_8,
            kind,
            required_index(index, 3),
        )
        .with_consumed(4));
    }
    Ok((((first & 0x07) as u32) << 18)
        | (((second & 0x3f) as u32) << 12)
        | (((third & 0x3f) as u32) << 6)
        | ((fourth & 0x3f) as u32))
}
