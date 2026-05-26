/***************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ***************************************************************************/
use qubit_io::{
    BigEndian,
    BinaryCodec,
    ByteOrder,
    LittleEndian,
};

use crate::{
    Charset,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
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
/// * `CharsetDecodeErrorKind::InvalidInputIndex` when `index` is greater than
///   `input.len()`.
/// * `CharsetDecodeErrorKind::InvalidCodePoint` when `input[index]` is not a
///   valid scalar.
pub(crate) fn decode_units_prefix(input: &[u32], index: usize) -> CharsetDecodeResult<DecodeStatus> {
    if index > input.len() {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
        return Err(CharsetDecodeError::new(Charset::UTF_32, kind, index));
    }
    if index == input.len() {
        return Ok(DecodeStatus::NeedMore {
            required: index + 1,
            available: 0,
        });
    }
    match Unicode::to_char(input[index]) {
        Some(ch) => Ok(DecodeStatus::Complete { value: ch, consumed: 1 }),
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
pub(crate) fn encode_units_char(ch: char, output: &mut [u32], index: usize) -> CharsetEncodeResult<usize> {
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
/// * `CharsetDecodeErrorKind::InvalidInputIndex` when `index` is greater than
///   `input.len()`.
/// * `CharsetDecodeErrorKind::InvalidCodePoint` when the decoded unit is not a
///   valid scalar.
pub(crate) fn decode_bytes_prefix(
    input: &[u8],
    index: usize,
    byte_order: ByteOrder,
) -> CharsetDecodeResult<DecodeStatus> {
    let charset = Charset::from_utf32_byte_order(byte_order);
    if index > input.len() {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
        return Err(CharsetDecodeError::new(charset, kind, index));
    }
    let available = input.len() - index;
    if available < 4 {
        return Ok(DecodeStatus::NeedMore {
            required: index.saturating_add(4),
            available,
        });
    }
    // SAFETY: The length check above guarantees that `index..index + 4` is in bounds.
    let unit = match byte_order {
        ByteOrder::BigEndian => unsafe { BinaryCodec::<u32, BigEndian>::read_unchecked(input, index) },
        ByteOrder::LittleEndian => unsafe { BinaryCodec::<u32, LittleEndian>::read_unchecked(input, index) },
    };
    match Unicode::to_char(unit) {
        Some(ch) => Ok(DecodeStatus::Complete { value: ch, consumed: 4 }),
        None => {
            let kind = CharsetDecodeErrorKind::InvalidCodePoint { value: unit };
            Err(CharsetDecodeError::new(charset, kind, index))
        }
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
/// * `CharsetEncodeErrorKind::BufferTooSmall` when output has fewer than four
///   bytes from `index`.
pub(crate) fn encode_bytes_char(
    ch: char,
    output: &mut [u8],
    byte_order: ByteOrder,
    index: usize,
) -> CharsetEncodeResult<usize> {
    let charset = Charset::from_utf32_byte_order(byte_order);
    if index > output.len() {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + 4,
            available: 0,
        };
        return Err(CharsetEncodeError::new(charset, kind, index));
    }
    let required = 4;
    let available = output.len() - index;
    if available < required {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + required,
            available,
        };
        return Err(CharsetEncodeError::new(charset, kind, index));
    }
    // SAFETY: The capacity check above guarantees that `index..index + 4` is in bounds.
    match byte_order {
        ByteOrder::BigEndian => unsafe { BinaryCodec::<u32, BigEndian>::write_unchecked(output, index, ch as u32) },
        ByteOrder::LittleEndian => unsafe {
            BinaryCodec::<u32, LittleEndian>::write_unchecked(output, index, ch as u32)
        },
    }
    Ok(4)
}
