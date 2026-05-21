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
    Utf16,
};

use super::helpers;

/// Encoder for UTF-16 `u16` code-unit buffers.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     TextEncoder,
///     Utf16,
///     Utf16U16Encoder,
/// };
///
/// let encoder = Utf16U16Encoder;
/// let mut output = [0_u16; Utf16::MAX_UNITS_PER_CHAR];
/// let written = encoder.encode_char('😀', &mut output).expect("buffer fits");
///
/// assert_eq!(2, written);
/// assert_eq!([0xd83d, 0xde00], output);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf16U16Encoder;

impl TextEncoder<u16> for Utf16U16Encoder {
    fn charset(&self) -> Charset {
        Charset::UTF_16
    }

    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_UNITS_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u16]) -> TextEncodeResult<usize> {
        helpers::encode_utf16_units_char(ch, output)
    }
}
