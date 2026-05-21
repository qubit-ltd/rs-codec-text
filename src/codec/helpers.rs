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
    ByteOrder,
    DecodeStatus,
    TextDecodingError,
    TextDecodingResult,
    TextEncoding,
    TextEncodingError,
    TextEncodingResult,
    Unicode,
    Utf8,
    Utf16,
};

/// Decodes the first UTF-8 character from a byte prefix.
pub(super) fn decode_utf8_prefix(input: &[u8]) -> TextDecodingResult<DecodeStatus<char>> {
    if input.is_empty() {
        return Ok(DecodeStatus::NeedMore {
            required: 1,
            available: 0,
        });
    }
    let first = input[0];
    let length = match Utf8::byte_len_from_leading_byte(first) {
        Some(length) => length,
        None => {
            return Err(TextDecodingError::malformed_sequence(
                TextEncoding::UTF_8,
                0,
            ));
        }
    };
    if input.len() < length {
        validate_utf8_partial(input)?;
        return Ok(DecodeStatus::NeedMore {
            required: length,
            available: input.len(),
        });
    }
    let code_point = match length {
        1 => first as u32,
        2 => decode_utf8_two(input)?,
        3 => decode_utf8_three(input)?,
        4 => decode_utf8_four(input)?,
        _ => unreachable!("UTF-8 sequence length is limited to four bytes"),
    };
    match Unicode::to_char(code_point) {
        Some(ch) => Ok(DecodeStatus::Complete {
            value: ch,
            consumed: length,
        }),
        None => Err(TextDecodingError::invalid_code_point(
            TextEncoding::UTF_8,
            0,
            code_point,
        )),
    }
}

/// Encodes one character into UTF-8 bytes.
pub(super) fn encode_utf8_char(ch: char, output: &mut [u8]) -> TextEncodingResult<usize> {
    let length = Utf8::byte_len(ch);
    if output.len() < length {
        return Err(TextEncodingError::buffer_too_small(
            TextEncoding::UTF_8,
            output.len(),
        ));
    }
    let mut scratch = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
    let encoded = ch.encode_utf8(&mut scratch);
    output[..length].copy_from_slice(encoded.as_bytes());
    Ok(length)
}

/// Decodes the first UTF-16 character from a `u16` prefix.
pub(super) fn decode_utf16_units_prefix(input: &[u16]) -> TextDecodingResult<DecodeStatus<char>> {
    if input.is_empty() {
        return Ok(DecodeStatus::NeedMore {
            required: 1,
            available: 0,
        });
    }
    let first = input[0];
    if Utf16::is_high_surrogate(first) {
        if input.len() < 2 {
            return Ok(DecodeStatus::NeedMore {
                required: 2,
                available: input.len(),
            });
        }
        let second = input[1];
        match Utf16::compose_pair(first, second).and_then(Unicode::to_char) {
            Some(ch) => Ok(DecodeStatus::Complete {
                value: ch,
                consumed: 2,
            }),
            None => Err(TextDecodingError::malformed_sequence(
                TextEncoding::UTF_16,
                1,
            )),
        }
    } else if Utf16::is_low_surrogate(first) {
        Err(TextDecodingError::malformed_sequence(
            TextEncoding::UTF_16,
            0,
        ))
    } else {
        let ch = char::from_u32(first as u32).expect("non-surrogate UTF-16 unit is a scalar value");
        Ok(DecodeStatus::Complete {
            value: ch,
            consumed: 1,
        })
    }
}

/// Encodes one character into UTF-16 `u16` units.
pub(super) fn encode_utf16_units_char(ch: char, output: &mut [u16]) -> TextEncodingResult<usize> {
    let length = Utf16::unit_len(ch);
    if output.len() < length {
        return Err(TextEncodingError::buffer_too_small(
            TextEncoding::UTF_16,
            output.len(),
        ));
    }
    let code_point = ch as u32;
    if length == 1 {
        output[0] = code_point as u16;
    } else {
        output[0] =
            Utf16::high_surrogate(code_point).expect("supplementary scalar has high surrogate");
        output[1] =
            Utf16::low_surrogate(code_point).expect("supplementary scalar has low surrogate");
    }
    Ok(length)
}

/// Decodes the first UTF-16 character from a byte prefix.
pub(super) fn decode_utf16_bytes_prefix(
    input: &[u8],
    byte_order: ByteOrder,
) -> TextDecodingResult<DecodeStatus<char>> {
    if input.len() < 2 {
        return Ok(DecodeStatus::NeedMore {
            required: 2,
            available: input.len(),
        });
    }
    let first = byte_order.read_u16(input);
    if Utf16::is_high_surrogate(first) {
        if input.len() < 4 {
            return Ok(DecodeStatus::NeedMore {
                required: 4,
                available: input.len(),
            });
        }
        let second = byte_order.read_u16(&input[2..]);
        match Utf16::compose_pair(first, second).and_then(Unicode::to_char) {
            Some(ch) => Ok(DecodeStatus::Complete {
                value: ch,
                consumed: 4,
            }),
            None => Err(TextDecodingError::malformed_sequence(
                TextEncoding::UTF_16,
                2,
            )),
        }
    } else if Utf16::is_low_surrogate(first) {
        Err(TextDecodingError::malformed_sequence(
            TextEncoding::UTF_16,
            0,
        ))
    } else {
        let ch = char::from_u32(first as u32).expect("non-surrogate UTF-16 unit is a scalar value");
        Ok(DecodeStatus::Complete {
            value: ch,
            consumed: 2,
        })
    }
}

/// Encodes one character into byte-serialized UTF-16.
pub(super) fn encode_utf16_bytes_char(
    ch: char,
    output: &mut [u8],
    byte_order: ByteOrder,
) -> TextEncodingResult<usize> {
    let required = Utf16::unit_len(ch) * 2;
    if output.len() < required {
        return Err(TextEncodingError::buffer_too_small(
            TextEncoding::UTF_16,
            output.len(),
        ));
    }
    let mut units = [0_u16; Utf16::MAX_UNITS_PER_CHAR];
    let unit_count = encode_utf16_units_char(ch, &mut units)?;
    for (index, unit) in units.iter().take(unit_count).enumerate() {
        let bytes = byte_order.u16_bytes(*unit);
        let offset = index * 2;
        output[offset..offset + 2].copy_from_slice(&bytes);
    }
    Ok(required)
}

/// Decodes the first UTF-32 character from a `u32` prefix.
pub(super) fn decode_utf32_units_prefix(input: &[u32]) -> TextDecodingResult<DecodeStatus<char>> {
    if input.is_empty() {
        return Ok(DecodeStatus::NeedMore {
            required: 1,
            available: 0,
        });
    }
    match Unicode::to_char(input[0]) {
        Some(ch) => Ok(DecodeStatus::Complete {
            value: ch,
            consumed: 1,
        }),
        None => Err(TextDecodingError::invalid_code_point(
            TextEncoding::UTF_32,
            0,
            input[0],
        )),
    }
}

/// Encodes one character into a UTF-32 `u32` unit.
pub(super) fn encode_utf32_units_char(ch: char, output: &mut [u32]) -> TextEncodingResult<usize> {
    if output.is_empty() {
        return Err(TextEncodingError::buffer_too_small(TextEncoding::UTF_32, 0));
    }
    output[0] = ch as u32;
    Ok(1)
}

/// Decodes the first UTF-32 character from a byte prefix.
pub(super) fn decode_utf32_bytes_prefix(
    input: &[u8],
    byte_order: ByteOrder,
) -> TextDecodingResult<DecodeStatus<char>> {
    if input.len() < 4 {
        return Ok(DecodeStatus::NeedMore {
            required: 4,
            available: input.len(),
        });
    }
    let unit = byte_order.read_u32(input);
    match Unicode::to_char(unit) {
        Some(ch) => Ok(DecodeStatus::Complete {
            value: ch,
            consumed: 4,
        }),
        None => Err(TextDecodingError::invalid_code_point(
            TextEncoding::UTF_32,
            0,
            unit,
        )),
    }
}

/// Encodes one character into byte-serialized UTF-32.
pub(super) fn encode_utf32_bytes_char(
    ch: char,
    output: &mut [u8],
    byte_order: ByteOrder,
) -> TextEncodingResult<usize> {
    if output.len() < 4 {
        return Err(TextEncodingError::buffer_too_small(
            TextEncoding::UTF_32,
            output.len(),
        ));
    }
    output[..4].copy_from_slice(&byte_order.u32_bytes(ch as u32));
    Ok(4)
}

/// Decodes a two-byte UTF-8 sequence.
fn decode_utf8_two(input: &[u8]) -> TextDecodingResult<u32> {
    let second = input[1];
    if !Utf8::is_continuation_byte(second) {
        return Err(TextDecodingError::malformed_sequence(
            TextEncoding::UTF_8,
            1,
        ));
    }
    Ok((((input[0] & 0x1f) as u32) << 6) | ((second & 0x3f) as u32))
}

/// Validates the bytes already present in an incomplete UTF-8 sequence.
fn validate_utf8_partial(input: &[u8]) -> TextDecodingResult<()> {
    if input.len() >= 2 && !is_valid_utf8_second_byte(input[0], input[1]) {
        return Err(TextDecodingError::malformed_sequence(
            TextEncoding::UTF_8,
            1,
        ));
    }
    if input.len() >= 3 && !Utf8::is_continuation_byte(input[2]) {
        return Err(TextDecodingError::malformed_sequence(
            TextEncoding::UTF_8,
            2,
        ));
    }
    Ok(())
}

/// Tests whether the second byte is valid for the supplied UTF-8 leading byte.
fn is_valid_utf8_second_byte(first: u8, second: u8) -> bool {
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

/// Decodes a three-byte UTF-8 sequence.
fn decode_utf8_three(input: &[u8]) -> TextDecodingResult<u32> {
    let first = input[0];
    let second = input[1];
    let third = input[2];
    if !is_valid_utf8_second_byte(first, second) {
        return Err(TextDecodingError::malformed_sequence(
            TextEncoding::UTF_8,
            1,
        ));
    }
    if !Utf8::is_continuation_byte(third) {
        return Err(TextDecodingError::malformed_sequence(
            TextEncoding::UTF_8,
            2,
        ));
    }
    Ok((((first & 0x0f) as u32) << 12) | (((second & 0x3f) as u32) << 6) | ((third & 0x3f) as u32))
}

/// Decodes a four-byte UTF-8 sequence.
fn decode_utf8_four(input: &[u8]) -> TextDecodingResult<u32> {
    let first = input[0];
    let second = input[1];
    let third = input[2];
    let fourth = input[3];
    if !is_valid_utf8_second_byte(first, second) {
        return Err(TextDecodingError::malformed_sequence(
            TextEncoding::UTF_8,
            1,
        ));
    }
    if !Utf8::is_continuation_byte(third) {
        return Err(TextDecodingError::malformed_sequence(
            TextEncoding::UTF_8,
            2,
        ));
    }
    if !Utf8::is_continuation_byte(fourth) {
        return Err(TextDecodingError::malformed_sequence(
            TextEncoding::UTF_8,
            3,
        ));
    }
    Ok((((first & 0x07) as u32) << 18)
        | (((second & 0x3f) as u32) << 12)
        | (((third & 0x3f) as u32) << 6)
        | ((fourth & 0x3f) as u32))
}
