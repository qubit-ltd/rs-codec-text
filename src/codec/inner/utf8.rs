/***************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ***************************************************************************/
use crate::{
    Charset,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
    Unicode,
    Utf8,
};

/// Decodes the first UTF-8 character from a closed byte slice starting at `index`.
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
/// Returns the decoded character and the number of consumed bytes.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::InvalidInputIndex` when `index` is greater than
///   `input.len()`.
/// * `CharsetDecodeErrorKind::MalformedSequence` when the first byte or
///   continuation bytes are invalid for UTF-8.
/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before the
///   complete UTF-8 sequence is available.
pub(crate) fn decode_prefix(input: &[u8], index: usize) -> CharsetDecodeResult<(char, usize)> {
    if index > input.len() {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
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
    if input.len() < index + length {
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
    Ok((ch, length))
}

/// Encodes one Unicode scalar value into UTF-8 at `index` in `output`.
///
/// The function writes the byte sequence for `ch` and returns how many bytes were
/// written.
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
pub(crate) fn encode_char(ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
    if index > output.len() {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + 1,
            available: 0,
        };
        return Err(CharsetEncodeError::new(Charset::UTF_8, kind, index));
    }
    let length = Utf8::byte_len(ch);
    let available = output.len() - index;
    if available < length {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + length,
            available,
        };
        return Err(CharsetEncodeError::new(Charset::UTF_8, kind, index));
    }
    let mut scratch = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
    let encoded = ch.encode_utf8(&mut scratch);
    output[index..index + length].copy_from_slice(encoded.as_bytes());
    Ok(length)
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
pub(crate) fn decode_two(input: &[u8], index: usize) -> CharsetDecodeResult<u32> {
    let second = input[index + 1];
    if !Utf8::is_continuation_byte(second) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(second as u32),
        };
        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index + 1).with_consumed(2));
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
pub(crate) fn validate_partial(input: &[u8], index: usize) -> CharsetDecodeResult<()> {
    if input.len() >= index + 2 && !is_valid_second_byte(input[index], input[index + 1]) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(input[index + 1] as u32),
        };
        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index + 1).with_consumed(2));
    }
    if input.len() >= index + 3 && !Utf8::is_continuation_byte(input[index + 2]) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(input[index + 2] as u32),
        };
        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index + 2).with_consumed(3));
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
pub(crate) fn is_valid_second_byte(first: u8, second: u8) -> bool {
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
pub(crate) fn decode_three(input: &[u8], index: usize) -> CharsetDecodeResult<u32> {
    let first = input[index];
    let second = input[index + 1];
    let third = input[index + 2];
    if !is_valid_second_byte(first, second) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(second as u32),
        };
        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index + 1).with_consumed(2));
    }
    if !Utf8::is_continuation_byte(third) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(third as u32),
        };
        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index + 2).with_consumed(3));
    }
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
pub(crate) fn decode_four(input: &[u8], index: usize) -> CharsetDecodeResult<u32> {
    let first = input[index];
    let second = input[index + 1];
    let third = input[index + 2];
    let fourth = input[index + 3];
    if !is_valid_second_byte(first, second) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(second as u32),
        };
        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index + 1).with_consumed(2));
    }
    if !Utf8::is_continuation_byte(third) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(third as u32),
        };
        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index + 2).with_consumed(3));
    }
    if !Utf8::is_continuation_byte(fourth) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(fourth as u32),
        };
        return Err(CharsetDecodeError::new(Charset::UTF_8, kind, index + 3).with_consumed(4));
    }
    Ok((((first & 0x07) as u32) << 18)
        | (((second & 0x3f) as u32) << 12)
        | (((third & 0x3f) as u32) << 6)
        | ((fourth & 0x3f) as u32))
}
