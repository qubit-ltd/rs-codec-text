// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::num::NonZeroUsize;

use qubit_codec::Codec;
use qubit_utils::UncheckedSlice;

use crate::Charset;
use crate::CharsetCodec;
use crate::CharsetDecodeError;
use crate::CharsetEncodeError;
use crate::CharsetEncodeResult;
use crate::Latin1;
use crate::error::CharsetCodecDecodeResult;

/// Single-byte ISO-8859-1 codec for bytes.
///
/// `Latin1Codec` converts between ISO-8859-1 bytes and Unicode scalar values.
///
/// # Examples
///
/// ```rust
/// use qubit_codec::Codec;
/// use qubit_codec_text::Latin1Codec;
///
/// let mut codec = Latin1Codec;
/// let mut output = [0_u8; 1];
/// let written = unsafe {
///     codec.encode(&'é', &mut output, 0).expect("Latin-1 buffer fits")
/// };
/// let (value, consumed) = unsafe {
///     codec.decode(&output[..written], 0).expect("valid Latin-1")
/// };
/// assert_eq!(('é', 1), (value, consumed.get()));
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Latin1Codec;

impl Latin1Codec {
    /// Returns the ISO-8859-1 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::ISO_8859_1`].
    #[inline]
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
    #[inline]
    fn charset(&self) -> Charset {
        Charset::ISO_8859_1
    }
}

impl Codec for Latin1Codec {
    /// Unicode scalar values represented by ISO-8859-1 bytes.
    type Value = char;
    /// The single-byte storage unit used by ISO-8859-1.
    type Unit = u8;
    /// Decode errors use the crate's shared charset error type.
    type DecodeError = CharsetDecodeError;
    /// Encode errors use the crate's shared charset error type.
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = 1;
    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;

    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;

    /// Reports whether the scalar value is representable as one Latin-1 byte.
    ///
    /// # Parameters
    ///
    /// - `value`: Unicode scalar value to check.
    ///
    /// # Returns
    ///
    /// Returns `true` when the value is in U+0000 through U+00FF.
    #[inline]
    fn can_encode_value(&self, value: &char) -> bool {
        Latin1::is_latin1_char(*value)
    }

    /// Decodes the byte at `input_index` as its corresponding Unicode scalar.
    ///
    /// Latin-1 maps every possible byte to a scalar, so decoding succeeds when
    /// the requested input unit is available.
    ///
    /// # Parameters
    ///
    /// - `input`: Source bytes containing the value.
    /// - `input_index`: Index of the byte to decode.
    ///
    /// # Returns
    ///
    /// Returns the corresponding scalar and a consumed width of one byte.
    ///
    /// # Safety
    ///
    /// `input_index` must identify a readable byte in `input`.
    #[inline]
    unsafe fn decode(&mut self, input: &[u8], input_index: usize) -> CharsetCodecDecodeResult<(char, NonZeroUsize)> {
        debug_assert!(input_index < input.len());
        // SAFETY: The caller guarantees that `input_index` is readable.
        let value = unsafe { UncheckedSlice::read(input, input_index) };
        Ok((Latin1::byte_to_char(value), NonZeroUsize::MIN))
    }

    /// Encodes a representable scalar as one Latin-1 byte at `output_index`.
    ///
    /// # Parameters
    ///
    /// - `ch`: Unicode scalar value to encode.
    /// - `output`: Destination byte buffer.
    /// - `output_index`: Index where the encoded byte is written.
    ///
    /// # Returns
    ///
    /// Returns the number of bytes written, which is always one.
    ///
    /// # Safety
    ///
    /// `ch` must be encodable by this codec, and `output_index` must identify
    /// a writable byte in `output`.
    #[inline]
    unsafe fn encode(&mut self, ch: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
        debug_assert!(self.can_encode_value(ch));
        debug_assert!(output_index < output.len());

        let value = Latin1::char_to_byte(*ch).expect("encodable Latin-1 character maps to byte");
        // SAFETY: The caller guarantees that `ch` is encodable and
        // `output_index` is writable.
        unsafe {
            UncheckedSlice::write(output, output_index, value);
        }
        Ok(1)
    }
}
