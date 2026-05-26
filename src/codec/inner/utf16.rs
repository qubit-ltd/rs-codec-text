/***************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ***************************************************************************/
use qubit_codec::ByteOrder;

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
    Utf16,
};

/// Decodes the first UTF-16 character from a `u16` prefix.
///
/// The function handles three cases:
/// 1. ASCII/non-surrogate units decode to a single `char`.
/// 2. High-surrogate pairs are combined with the following unit into one scalar value.
/// 3. Isolated low-surrogates are rejected as malformed.
///
/// # Arguments
///
/// * `input` - UTF-16 unit slice to decode from.
/// * `index` - Start offset in `input`; must be `<= input.len()`.
///
/// # Returns
///
/// * `Ok(DecodeStatus::NeedMore { required, available })` if more units are needed
///   (e.g. dangling high-surrogate).
/// * `Ok(DecodeStatus::Complete { value, consumed })` when one code point is decoded.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::InvalidInputIndex` when `index` is greater than
///   `input.len()`.
/// * `CharsetDecodeErrorKind::MalformedSequence` for invalid UTF-16 sequences
///   (invalid high/low surrogate pairing).
/// # Panics
///
/// This function does not panic for invalid UTF-16 input because invalid input
/// is surfaced as `CharsetDecodeError`.
pub(crate) fn decode_units_prefix(input: &[u16], index: usize) -> CharsetDecodeResult<DecodeStatus> {
    if index > input.len() {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
        return Err(CharsetDecodeError::new(Charset::UTF_16, kind, index));
    }
    if index == input.len() {
        return Ok(DecodeStatus::NeedMore {
            required: index + 1,
            available: 0,
        });
    }
    let first = input[index];
    if Utf16::is_high_surrogate(first) {
        if input.len() < index + 2 {
            return Ok(DecodeStatus::NeedMore {
                required: index + 2,
                available: input.len() - index,
            });
        }
        let second = input[index + 1];
        match Utf16::compose_pair(first, second).and_then(Unicode::to_char) {
            Some(ch) => Ok(DecodeStatus::Complete { value: ch, consumed: 2 }),
            None => {
                let kind = CharsetDecodeErrorKind::MalformedSequence {
                    value: Some(second as u32),
                };
                Err(CharsetDecodeError::new(Charset::UTF_16, kind, index + 1))
            }
        }
    } else if Utf16::is_low_surrogate(first) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(first as u32),
        };
        Err(CharsetDecodeError::new(Charset::UTF_16, kind, index))
    } else {
        let ch = char::from_u32(first as u32).expect("non-surrogate UTF-16 unit is a scalar value");
        Ok(DecodeStatus::Complete { value: ch, consumed: 1 })
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
pub(crate) fn encode_units_char(ch: char, output: &mut [u16], index: usize) -> CharsetEncodeResult<usize> {
    if index > output.len() {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + 1,
            available: 0,
        };
        return Err(CharsetEncodeError::new(Charset::UTF_16, kind, index));
    }
    let length = Utf16::unit_len(ch);
    let available = output.len() - index;
    if available < length {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + length,
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

/// Decodes the first UTF-16 character from a byte prefix.
///
/// The input bytes are interpreted with `byte_order`, then decoded using the same
/// surrogate rules as unit-based decoding.
///
/// # Arguments
///
/// * `input` - UTF-16 encoded byte slice.
/// * `index` - Start offset in `input` bytes; must be `<= input.len()`.
/// * `byte_order` - Byte order used to read UTF-16 units.
///
/// # Returns
///
/// * `Ok(DecodeStatus::NeedMore { required, available })` if a complete unit/pair is
///   not yet available.
/// * `Ok(DecodeStatus::Complete { value, consumed })` when one decoded character is
///   available. `consumed` is the number of bytes consumed.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::InvalidInputIndex` when `index` is greater than
///   `input.len()`.
/// * `CharsetDecodeErrorKind::MalformedSequence` for invalid UTF-16 byte
///   sequences or malformed surrogate usage.
pub(crate) fn decode_bytes_prefix(
    input: &[u8],
    index: usize,
    byte_order: ByteOrder,
) -> CharsetDecodeResult<DecodeStatus> {
    let charset = Charset::from_utf16_byte_order(byte_order);
    if index > input.len() {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
        return Err(CharsetDecodeError::new(charset, kind, index));
    }
    let available = input.len() - index;
    if available < 2 {
        return Ok(DecodeStatus::NeedMore {
            required: index.saturating_add(2),
            available,
        });
    }
    let first = read_ordered_u16(input, index, byte_order);
    if Utf16::is_high_surrogate(first) {
        if available < 4 {
            return Ok(DecodeStatus::NeedMore {
                required: index.saturating_add(4),
                available,
            });
        }
        let second = read_ordered_u16(input, index + 2, byte_order);
        match Utf16::compose_pair(first, second).and_then(Unicode::to_char) {
            Some(ch) => Ok(DecodeStatus::Complete { value: ch, consumed: 4 }),
            None => {
                let kind = CharsetDecodeErrorKind::MalformedSequence {
                    value: Some(second as u32),
                };
                Err(CharsetDecodeError::new(charset, kind, index + 2))
            }
        }
    } else if Utf16::is_low_surrogate(first) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(first as u32),
        };
        Err(CharsetDecodeError::new(charset, kind, index))
    } else {
        let ch = char::from_u32(first as u32).expect("non-surrogate UTF-16 unit is a scalar value");
        Ok(DecodeStatus::Complete { value: ch, consumed: 2 })
    }
}

/// Encodes one character into byte-serialized UTF-16 at `index` in `output`.
///
/// The function first encodes into temporary UTF-16 units, then writes them using the
/// provided byte order.
///
/// # Arguments
///
/// * `ch` - The character to encode.
/// * `output` - Byte destination.
/// * `byte_order` - Byte order for writing UTF-16 units.
/// * `index` - Start offset in `output` bytes; must be `<= output.len()`.
///
/// # Returns
///
/// `Ok(usize)` with the number of bytes written (`2` for BMP, `4` for supplementary).
///
/// # Errors
///
/// * `CharsetEncodeErrorKind::BufferTooSmall` when output bytes from `index`
///   are insufficient.
pub(crate) fn encode_bytes_char(
    ch: char,
    output: &mut [u8],
    byte_order: ByteOrder,
    index: usize,
) -> CharsetEncodeResult<usize> {
    let charset = Charset::from_utf16_byte_order(byte_order);
    if index > output.len() {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + 2,
            available: 0,
        };
        return Err(CharsetEncodeError::new(charset, kind, index));
    }
    let required = Utf16::unit_len(ch) * 2;
    let available = output.len() - index;
    if available < required {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + required,
            available,
        };
        return Err(CharsetEncodeError::new(charset, kind, index));
    }
    let mut units = [0_u16; Utf16::MAX_UNITS_PER_CHAR];
    let unit_count = encode_units_char(ch, &mut units, 0)?;
    for (unit_index, unit) in units.iter().take(unit_count).enumerate() {
        let offset = index + unit_index * 2;
        write_ordered_u16(output, offset, *unit, byte_order);
    }
    Ok(required)
}

/// Reads one endian-aware `u16` value from an already checked byte slice.
///
/// # Parameters
///
/// - `input`: Source byte slice.
/// - `index`: Start byte offset. The caller must guarantee two bytes are
///   available from this offset.
/// - `byte_order`: Byte order used to interpret the two bytes.
///
/// # Returns
///
/// Returns the decoded UTF-16 unit.
fn read_ordered_u16(input: &[u8], index: usize, byte_order: ByteOrder) -> u16 {
    let bytes = [input[index], input[index + 1]];
    match byte_order {
        ByteOrder::BigEndian => u16::from_be_bytes(bytes),
        ByteOrder::LittleEndian => u16::from_le_bytes(bytes),
    }
}

/// Writes one endian-aware `u16` value into an already checked byte slice.
///
/// # Parameters
///
/// - `output`: Destination byte slice.
/// - `index`: Start byte offset. The caller must guarantee two bytes are
///   writable from this offset.
/// - `unit`: UTF-16 unit to write.
/// - `byte_order`: Byte order used to serialize the unit.
fn write_ordered_u16(output: &mut [u8], index: usize, unit: u16, byte_order: ByteOrder) {
    let bytes = match byte_order {
        ByteOrder::BigEndian => unit.to_be_bytes(),
        ByteOrder::LittleEndian => unit.to_le_bytes(),
    };
    output[index..index + 2].copy_from_slice(&bytes);
}
