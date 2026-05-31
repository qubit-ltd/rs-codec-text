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
    Unicode,
};

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
pub(crate) fn decode_units_prefix(input: &[u32], index: usize) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
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
#[inline(always)]
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

/// Decodes the first UTF-32 character from a closed byte buffer.
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
/// Returns the decoded character and a non-zero count of `4` consumed bytes.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::InvalidInputIndex` when `index` is greater than
///   `input.len()`.
/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before four
///   bytes are available.
/// * `CharsetDecodeErrorKind::InvalidCodePoint` when the decoded unit is not a
///   valid scalar.
#[inline]
pub(crate) fn decode_bytes_prefix(
    input: &[u8],
    index: usize,
    byte_order: ByteOrder,
) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
    let charset = Charset::from_utf32_byte_order(byte_order);
    if index > input.len() {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
        return Err(CharsetDecodeError::new(charset, kind, index));
    }
    let available = input.len() - index;
    if available < 4 {
        let kind = CharsetDecodeErrorKind::IncompleteSequence { required: 4, available };
        return Err(CharsetDecodeError::new(charset, kind, index));
    }
    let unit = read_ordered_u32(input, index, byte_order);
    match Unicode::to_char(unit) {
        Some(ch) => {
            // SAFETY: 4 is non-zero.
            Ok((ch, unsafe { core::num::NonZeroUsize::new_unchecked(4) }))
        }
        None => {
            let kind = CharsetDecodeErrorKind::InvalidCodePoint { value: unit };
            Err(CharsetDecodeError::new(charset, kind, index).with_consumed(4))
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
#[inline]
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
    write_ordered_u32(output, index, ch as u32, byte_order);
    Ok(4)
}

/// Reads one endian-aware `u32` value from an already checked byte slice.
///
/// # Parameters
///
/// - `input`: Source byte slice.
/// - `index`: Start byte offset. The caller must guarantee four bytes are
///   available from this offset.
/// - `byte_order`: Byte order used to interpret the four bytes.
///
/// # Returns
///
/// Returns the decoded UTF-32 unit.
#[inline(always)]
fn read_ordered_u32(input: &[u8], index: usize, byte_order: ByteOrder) -> u32 {
    let bytes = [input[index], input[index + 1], input[index + 2], input[index + 3]];
    match byte_order {
        ByteOrder::BigEndian => u32::from_be_bytes(bytes),
        ByteOrder::LittleEndian => u32::from_le_bytes(bytes),
    }
}

/// Writes one endian-aware `u32` value into an already checked byte slice.
///
/// # Parameters
///
/// - `output`: Destination byte slice.
/// - `index`: Start byte offset. The caller must guarantee four bytes are
///   writable from this offset.
/// - `unit`: UTF-32 unit to write.
/// - `byte_order`: Byte order used to serialize the unit.
#[inline(always)]
fn write_ordered_u32(output: &mut [u8], index: usize, unit: u32, byte_order: ByteOrder) {
    let bytes = match byte_order {
        ByteOrder::BigEndian => unit.to_be_bytes(),
        ByteOrder::LittleEndian => unit.to_le_bytes(),
    };
    output[index..index + 4].copy_from_slice(&bytes);
}
