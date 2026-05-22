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
    ByteOrder,
    Charset,
    CharsetDecodeError,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeResult,
    DecodeStatus,
    Unicode,
};

/// Decodes the first UTF-32 character from a `u32` prefix.
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
/// * `Ok(DecodeStatus::NeedMore { required, available })` if input ends exactly at `index`.
/// * `Ok(DecodeStatus::Complete { value, consumed })` when one character is decoded.
///   `consumed` is always `1`.
///
/// # Errors
///
/// * `CharsetDecodeError::malformed_sequence` when `index` is out of bounds.
/// * `CharsetDecodeError::invalid_code_point` when `input[index]` is not a valid scalar.
pub(crate) fn decode_units_prefix(
    input: &[u32],
    index: usize,
) -> CharsetDecodeResult<DecodeStatus> {
    if index > input.len() {
        return Err(CharsetDecodeError::malformed_sequence(
            Charset::UTF_32,
            index,
        ));
    }
    if index == input.len() {
        return Ok(DecodeStatus::NeedMore {
            required: index + 1,
            available: 0,
        });
    }
    match Unicode::to_char(input[index]) {
        Some(ch) => Ok(DecodeStatus::Complete {
            value: ch,
            consumed: 1,
        }),
        None => Err(CharsetDecodeError::invalid_code_point(
            Charset::UTF_32,
            index,
            input[index],
        )),
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
/// * `CharsetEncodeError::buffer_too_small` when no unit can be written at `index`.
pub(crate) fn encode_units_char(
    ch: char,
    output: &mut [u32],
    index: usize,
) -> CharsetEncodeResult<usize> {
    if index >= output.len() {
        return Err(CharsetEncodeError::buffer_too_small(Charset::UTF_32, index));
    }
    output[index] = ch as u32;
    Ok(1)
}

/// Decodes the first UTF-32 character from a byte prefix.
///
/// The input bytes are interpreted according to `byte_order`.
///
/// # Arguments
///
/// * `input` - UTF-32 encoded byte slice.
/// * `index` - Start byte offset; must be `<= input.len()`.
/// * `byte_order` - Byte order used to read a `u32` unit.
///
/// # Returns
///
/// * `Ok(DecodeStatus::NeedMore { required, available })` if fewer than four bytes remain.
/// * `Ok(DecodeStatus::Complete { value, consumed })` when one character is decoded.
///   `consumed` is always `4`.
///
/// # Errors
///
/// * `CharsetDecodeError::malformed_sequence` when `index` is out of bounds.
/// * `CharsetDecodeError::invalid_code_point` when the decoded unit is not a valid scalar.
pub(crate) fn decode_bytes_prefix(
    input: &[u8],
    index: usize,
    byte_order: ByteOrder,
) -> CharsetDecodeResult<DecodeStatus> {
    let charset = Charset::from_utf32_byte_order(byte_order);
    if index > input.len() {
        return Err(CharsetDecodeError::malformed_sequence(charset, index));
    }
    let available = input.len() - index;
    if available < 4 {
        return Ok(DecodeStatus::NeedMore {
            required: index.saturating_add(4),
            available,
        });
    }
    // SAFETY: The length check above guarantees that `index..index + 4` is in bounds.
    let unit = unsafe { byte_order.read_u32_at_unchecked(input, index) };
    match Unicode::to_char(unit) {
        Some(ch) => Ok(DecodeStatus::Complete {
            value: ch,
            consumed: 4,
        }),
        None => Err(CharsetDecodeError::invalid_code_point(charset, index, unit)),
    }
}

/// Encodes one character into byte-serialized UTF-32 at `index` in `output`.
///
/// # Arguments
///
/// * `ch` - The character to encode.
/// * `output` - Destination byte buffer.
/// * `byte_order` - Byte order used to write the 4-byte representation.
/// * `index` - Start byte offset; must satisfy `index <= output.len()`.
///
/// # Returns
///
/// `Ok(4)` on success, because UTF-32 always occupies exactly four bytes.
///
/// # Errors
///
/// * `CharsetEncodeError::buffer_too_small` when output has fewer than four bytes from `index`.
pub(crate) fn encode_bytes_char(
    ch: char,
    output: &mut [u8],
    byte_order: ByteOrder,
    index: usize,
) -> CharsetEncodeResult<usize> {
    let charset = Charset::from_utf32_byte_order(byte_order);
    if index > output.len() {
        return Err(CharsetEncodeError::buffer_too_small(charset, index));
    }
    if output.len() - index < 4 {
        return Err(CharsetEncodeError::buffer_too_small(charset, output.len()));
    }
    // SAFETY: The capacity check above guarantees that `index..index + 4` is in bounds.
    unsafe { byte_order.write_u32_at_unchecked(output, index, ch as u32) };
    Ok(4)
}
