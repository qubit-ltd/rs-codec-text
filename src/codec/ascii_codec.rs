// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0.
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::num::NonZeroUsize;

use crate::{
    Ascii, Charset, CharsetCodec, CharsetDecodeError, CharsetDecodeErrorKind, CharsetDecodeResult,
    CharsetEncodeError, CharsetEncodeResult,
};
use qubit_codec::Codec;
use qubit_codec::{read_unchecked, write_unchecked};

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
    #[inline(always)]
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
    #[inline(always)]
    fn charset(&self) -> Charset {
        Charset::ASCII
    }
}

unsafe impl Codec for AsciiCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    #[inline(always)]
    fn min_units_per_value(&self) -> NonZeroUsize {
        NonZeroUsize::MIN
    }

    #[inline(always)]
    fn max_units_per_value(&self) -> NonZeroUsize {
        NonZeroUsize::MIN
    }

    #[inline(always)]
    fn can_encode_value(&self, value: &char) -> bool {
        Ascii::is_ascii_char(*value)
    }

    #[inline(always)]
    unsafe fn decode(
        &mut self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, NonZeroUsize)> {
        debug_assert!(index < input.len());

        // SAFETY: The caller guarantees that `index` is readable.
        let value = unsafe { read_unchecked(input, index) };
        if !Ascii::is_ascii_byte(value) {
            let kind = CharsetDecodeErrorKind::MalformedSequence {
                value: Some(value as u32),
            };
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, index));
        }
        Ok((value as char, NonZeroUsize::MIN))
    }

    #[inline(always)]
    unsafe fn encode(
        &mut self,
        ch: &char,
        output: &mut [u8],
        index: usize,
    ) -> CharsetEncodeResult<NonZeroUsize> {
        debug_assert!(self.can_encode_value(ch));
        debug_assert!(index < output.len());

        // SAFETY: The caller guarantees that `ch` is encodable and `index` is
        // writable.
        unsafe {
            write_unchecked(output, index, *ch as u8);
        }
        Ok(NonZeroUsize::MIN)
    }
}
