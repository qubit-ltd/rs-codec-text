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
    DecodeStatus,
    TextDecoder,
    TextDecodingResult,
    TextEncoding,
    Utf16,
};

use super::helpers;

/// Decoder for UTF-16 `u16` code-unit buffers.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     DecodeStatus,
///     TextDecoder,
///     Utf16U16Decoder,
/// };
///
/// let decoder = Utf16U16Decoder;
/// let decoded = decoder.decode_prefix(&[0xd83d, 0xde00]).expect("valid pair");
///
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '😀',
///         consumed: 2,
///     },
///     decoded,
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf16U16Decoder;

impl TextDecoder<u16> for Utf16U16Decoder {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::UTF_16
    }

    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_UNITS_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u16]) -> TextDecodingResult<DecodeStatus<char>> {
        helpers::decode_utf16_units_prefix(input)
    }
}
