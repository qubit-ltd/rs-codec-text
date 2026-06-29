// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0.
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::num::NonZeroUsize;

use crate::error::{CharsetCodecDecodeResult, map_charset_decode_failure};
use crate::{
    Ascii, Charset, CharsetCodec, CharsetDecodeError, CharsetDecodeErrorKind, CharsetEncodeError,
    CharsetEncodeResult,
};
use qubit_codec::Codec;

/// Single-byte ASCII codec for bytes.
///
/// `AsciiCodec` converts between one-byte ASCII-encoded data and Unicode scalar
/// values.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct AsciiCodec;

impl AsciiCodec {
    /// Returns the ASCII charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::ASCII`].
    #[inline]
    #[must_use]
    pub const fn charset(self) -> Charset {
        Charset::ASCII
    }
}

impl CharsetCodec for AsciiCodec {
    /// Returns the charset descriptor for this codec.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::ASCII`].
    #[inline]
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

impl Codec for AsciiCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: NonZeroUsize = NonZeroUsize::MIN;
    const MAX_UNITS_PER_VALUE: NonZeroUsize = NonZeroUsize::MIN;

    #[inline]
    fn can_encode_value(&self, value: &char) -> bool {
        Ascii::is_ascii_char(*value)
    }

    #[inline]
    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> CharsetCodecDecodeResult<(char, NonZeroUsize)> {
        debug_assert!(input_index < input.len());

        // SAFETY: The caller guarantees that `input_index` is readable.
        let value = unsafe { qubit_io::UncheckedSlice::read(input, input_index) };
        if !Ascii::is_ascii_byte(value) {
            let kind = CharsetDecodeErrorKind::malformed(value as u32);
            return Err(map_charset_decode_failure(CharsetDecodeError::new(
                Charset::ASCII,
                kind,
                input_index,
            )));
        }
        Ok((value as char, NonZeroUsize::MIN))
    }

    #[inline]
    unsafe fn encode(
        &mut self,
        ch: &char,
        output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<NonZeroUsize> {
        debug_assert!(self.can_encode_value(ch));
        debug_assert!(output_index < output.len());

        // SAFETY: The caller guarantees that `ch` is encodable and
        // `output_index` is writable.
        unsafe {
            qubit_io::UncheckedSlice::write(output, output_index, *ch as u8);
        }
        Ok(NonZeroUsize::MIN)
    }
}
