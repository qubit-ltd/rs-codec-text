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

use crate::Ascii;
use crate::Charset;
use crate::CharsetCodec;
use crate::CharsetDecodeError;
use crate::CharsetDecodeErrorKind;
use crate::CharsetEncodeError;
use crate::CharsetEncodeResult;
use crate::error::CharsetCodecDecodeResult;

/// Single-byte ASCII codec for bytes.
///
/// `AsciiCodec` converts between one-byte ASCII-encoded data and Unicode scalar
/// values.
///
/// # Examples
///
/// ```rust
/// use qubit_codec::Codec;
/// use qubit_codec_text::AsciiCodec;
///
/// let mut codec = AsciiCodec;
/// let mut output = [0_u8; 1];
/// unsafe {
///     codec.encode(&'A', &mut output, 0).expect("ASCII buffer fits");
/// }
/// let (value, consumed) = unsafe {
///     codec.decode(&output, 0).expect("valid ASCII")
/// };
/// assert_eq!(('A', 1), (value, consumed.get()));
/// ```
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
    /// Unicode scalar value represented by each ASCII byte.
    type Value = char;
    /// Eight-bit storage unit used by the ASCII representation.
    type Unit = u8;
    /// Charset error produced when an input byte is outside ASCII.
    type DecodeError = CharsetDecodeError;
    /// Charset error produced when a character cannot be represented in ASCII.
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = 1;
    const MAX_ENCODE_UNITS_PER_VALUE: usize = 1;

    const MAX_DECODE_UNITS_PER_VALUE: usize = 1;

    /// Reports whether `value` is an ASCII character representable in one byte.
    ///
    /// # Parameters
    ///
    /// - `value`: Character whose ASCII encodability is checked.
    #[inline]
    fn can_encode_value(&self, value: &char) -> bool {
        Ascii::is_ascii_char(*value)
    }

    /// Decodes one ASCII byte and reports malformed bytes with their input
    /// index.
    ///
    /// # Parameters
    ///
    /// - `input`: Byte buffer containing the encoded character.
    /// - `input_index`: Index of the byte to decode.
    ///
    /// # Returns
    ///
    /// Returns the decoded character and its non-zero one-byte width.
    ///
    /// # Errors
    ///
    /// Returns a charset decode failure when the selected byte is not ASCII.
    ///
    /// # Safety
    ///
    /// `input_index` must identify a readable byte in `input`.
    #[inline]
    unsafe fn decode(&mut self, input: &[u8], input_index: usize) -> CharsetCodecDecodeResult<(char, NonZeroUsize)> {
        debug_assert!(input_index < input.len());

        // SAFETY: The caller guarantees that `input_index` is readable.
        let value = unsafe { UncheckedSlice::read(input, input_index) };
        if !Ascii::is_ascii_byte(value) {
            let kind = CharsetDecodeErrorKind::malformed(value as u32);
            return Err(CharsetDecodeError::new(Charset::ASCII, kind, input_index).into_codec_failure());
        }
        Ok((value as char, NonZeroUsize::MIN))
    }

    /// Writes one ASCII character as a byte at the requested output index.
    ///
    /// # Parameters
    ///
    /// - `ch`: ASCII character to encode.
    /// - `output`: Writable byte buffer receiving the encoded character.
    /// - `output_index`: Index where the encoded byte is written.
    ///
    /// # Returns
    ///
    /// Returns the number of bytes written, which is always one.
    ///
    /// # Errors
    ///
    /// This implementation cannot fail after its preconditions are met.
    ///
    /// # Safety
    ///
    /// `ch` must be encodable by this codec, and `output_index` must identify a
    /// writable byte in `output`.
    #[inline]
    unsafe fn encode(&mut self, ch: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
        debug_assert!(self.can_encode_value(ch));
        debug_assert!(output_index < output.len());

        // SAFETY: The caller guarantees that `ch` is encodable and
        // `output_index` is writable.
        unsafe {
            UncheckedSlice::write(output, output_index, *ch as u8);
        }
        Ok(1)
    }
}
