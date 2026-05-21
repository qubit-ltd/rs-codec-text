/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use super::inner::utf8;
use crate::{
    Charset,
    TextEncodeResult,
    TextEncoder,
    Utf8,
};

/// Encoder for UTF-8 byte buffers.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     TextEncoder,
///     Utf8,
///     Utf8Encoder,
/// };
///
/// let encoder = Utf8Encoder;
/// let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];
/// let written = encoder.encode_char('😀', &mut output, 0).expect("buffer fits");
///
/// assert_eq!("😀".as_bytes(), &output[..written]);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf8Encoder;

impl TextEncoder<u8> for Utf8Encoder {
    fn charset(&self) -> Charset {
        Charset::UTF_8
    }

    fn max_units_per_char(&self) -> usize {
        Utf8::MAX_UNITS_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u8], index: usize) -> TextEncodeResult<usize> {
        utf8::encode_char(ch, output, index)
    }
}
