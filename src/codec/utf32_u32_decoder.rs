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
    Utf32,
};

use super::helpers;

/// Decoder for UTF-32 `u32` code-unit buffers.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     DecodeStatus,
///     TextDecoder,
///     Utf32U32Decoder,
/// };
///
/// let decoder = Utf32U32Decoder;
/// let decoded = decoder.decode_prefix(&['中' as u32]).expect("valid UTF-32");
///
/// assert_eq!(
///     DecodeStatus::Complete {
///         value: '中',
///         consumed: 1,
///     },
///     decoded,
/// );
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf32U32Decoder;

impl TextDecoder<u32> for Utf32U32Decoder {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::UTF_32
    }

    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_UNITS_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u32]) -> TextDecodingResult<DecodeStatus<char>> {
        helpers::decode_utf32_units_prefix(input)
    }
}
