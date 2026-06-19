// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0.
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::num::NonZeroUsize;

use crate::{
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeResult,
    Latin1,
};
use qubit_codec::Codec;

/// Single-byte ISO-8859-1 codec for bytes.
///
/// `Latin1Codec` converts between ISO-8859-1 bytes and Unicode scalar values.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Latin1Codec;

impl Latin1Codec {
    /// Returns the ISO-8859-1 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::ISO_8859_1`].
    #[inline(always)]
    #[must_use]
    pub const fn charset(self) -> Charset {
        Charset::ISO_8859_1
    }
}

impl CharsetCodec for Latin1Codec {
    /// Returns the charset descriptor for this codec.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::ISO_8859_1`].
    #[inline(always)]
    fn charset(&self) -> Charset {
        Charset::ISO_8859_1
    }
}

unsafe impl Codec for Latin1Codec {
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
        Latin1::is_latin1_char(*value)
    }

    #[inline(always)]
    unsafe fn decode(
        &mut self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, NonZeroUsize)> {
        debug_assert!(index < input.len());
        // SAFETY: The caller guarantees that `index` is readable.
        let value = unsafe { qubit_io::UncheckedSlice::read(input, index) };
        Ok((Latin1::byte_to_char(value), NonZeroUsize::MIN))
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

        let value = Latin1::char_to_byte(*ch)
            .expect("encodable Latin-1 character maps to byte");
        // SAFETY: The caller guarantees that `ch` is encodable and `index` is
        // writable.
        unsafe {
            qubit_io::UncheckedSlice::write(output, index, value);
        }
        Ok(NonZeroUsize::MIN)
    }
}
