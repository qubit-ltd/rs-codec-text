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
    Charset,
    TextEncodeResult,
    TextEncoder,
    Utf32,
};

use super::helpers;

/// Encoder for UTF-32 `u32` code-unit buffers.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     TextEncoder,
///     Utf32,
///     Utf32U32Encoder,
/// };
///
/// let encoder = Utf32U32Encoder;
/// let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];
/// let written = encoder.encode_char('中', &mut output).expect("buffer fits");
///
/// assert_eq!(1, written);
/// assert_eq!('中' as u32, output[0]);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf32U32Encoder;

impl TextEncoder<u32> for Utf32U32Encoder {
    fn charset(&self) -> Charset {
        Charset::UTF_32
    }

    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_UNITS_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u32]) -> TextEncodeResult<usize> {
        helpers::encode_utf32_units_char(ch, output)
    }
}
