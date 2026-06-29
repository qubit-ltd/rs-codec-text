// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::error::{CharsetCodecDecodeResult, map_charset_decode_failure};
use crate::{
    Charset, CharsetCodec, CharsetDecodeError, CharsetDecodeErrorKind, CharsetDecodeResult,
    CharsetEncodeError, CharsetEncodeResult, Unicode, Utf16,
};
use core::num::NonZeroUsize;
use qubit_codec::{ByteOrder, Codec};
use qubit_io::UncheckedSlice;

/// Combined byte-serialized UTF-16 codec.
///
/// The codec uses one configured byte order for both decoding and encoding. It
/// does not detect, consume, or emit a BOM automatically; callers should use
/// [`crate::UnicodeBom`] when a byte stream may carry an explicit BOM.
///
/// # Examples
///
/// ```rust
/// use qubit_codec::{
///     ByteOrder,
///     Codec,
/// };
/// use qubit_codec_text::{
///     CharsetCodec,
///     Charset,
///     Utf16,
///     Utf16ByteCodec,
/// };
///
/// let mut codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
/// assert_eq!(Charset::UTF_16LE, codec.charset());
/// assert_eq!(
///     Utf16::MAX_BYTES_PER_CHAR,
///     <Utf16ByteCodec as Codec>::MAX_UNITS_PER_VALUE.get(),
/// );
///
/// let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];
/// let written = codec.encode_len(&'😀').get();
/// unsafe {
///     codec.encode(&'😀', &mut output, 0).expect("buffer fits");
/// }
/// let (value, consumed) = unsafe {
///     codec.decode(&output[..written], 0).expect("valid UTF-16LE")
/// };
/// assert_eq!(('😀', written), (value, consumed.get()));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Utf16ByteCodec {
    /// Byte order used by both encoder and decoder paths.
    byte_order: ByteOrder,
}

impl Utf16ByteCodec {
    /// Creates a byte-serialized UTF-16 codec.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used by the byte buffer.
    ///
    /// # Returns
    ///
    /// Returns a UTF-16 byte codec.
    #[inline]
    #[must_use]
    pub const fn new(byte_order: ByteOrder) -> Self {
        Self { byte_order }
    }

    /// Returns the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns the byte order used by this codec.
    #[inline]
    #[must_use]
    pub const fn byte_order(self) -> ByteOrder {
        self.byte_order
    }

    /// Returns the fixed-endian UTF-16 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_16LE`] or [`Charset::UTF_16BE`] according to this
    /// codec's configured byte order.
    #[inline]
    #[must_use]
    pub const fn charset(self) -> Charset {
        Charset::from_utf16_byte_order(self.byte_order)
    }
}

impl CharsetCodec for Utf16ByteCodec {
    /// Returns the fixed-endian UTF-16 charset for the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_16BE`] when configured with
    /// `ByteOrder::BigEndian`, otherwise [`Charset::UTF_16LE`].
    #[inline]
    fn charset(&self) -> Charset {
        Charset::from_utf16_byte_order(self.byte_order)
    }
}

impl Codec for Utf16ByteCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: NonZeroUsize = qubit_io::nz!(2);
    const MAX_UNITS_PER_VALUE: NonZeroUsize = qubit_io::nz!(Utf16::MAX_BYTES_PER_CHAR);

    #[inline]
    fn encode_len(&self, ch: &char) -> NonZeroUsize {
        qubit_io::nz!(Utf16::unit_len(*ch) * 2)
    }

    #[inline]
    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> CharsetCodecDecodeResult<(char, NonZeroUsize)> {
        let (ch, consumed) = decode_bytes_prefix(input, input_index, self.byte_order)
            .map_err(map_charset_decode_failure)?;
        debug_assert!(consumed.get() <= input.len().saturating_sub(input_index));
        Ok((ch, consumed))
    }

    #[inline]
    unsafe fn encode(
        &mut self,
        ch: &char,
        output: &mut [u8],
        output_index: usize,
    ) -> CharsetEncodeResult<NonZeroUsize> {
        let written = encode_bytes_char(*ch, output, self.byte_order, output_index);
        debug_assert_eq!(written, ch.len_utf16() * 2);
        debug_assert!(written <= output.len().saturating_sub(output_index));
        Ok(qubit_io::nz!(written))
    }
}

/// Decodes the first UTF-16 character from a closed byte buffer.
///
/// The input bytes are interpreted with `byte_order`, then decoded using UTF-16
/// surrogate rules.
///
/// # Arguments
///
/// * `input` - UTF-16 encoded byte slice. Callers using closed-prefix decoding
///   provide at least [`Utf16::MAX_BYTES_PER_CHAR`] readable bytes unless EOF
///   has been reached.
/// * `index` - Start offset in `input` bytes; must be `<= input.len()`.
/// * `byte_order` - Byte order used to read UTF-16 units.
///
/// # Returns
///
/// Returns the decoded character and the non-zero number of consumed bytes.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::MalformedSequence` for invalid UTF-16 byte
///   sequences or malformed surrogate usage. Surrogates are valid UTF-16 code
///   units only inside well-formed pairs, so malformed pair structure is
///   reported as a sequence error rather than as an independently decoded
///   scalar value.
/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before a
///   complete UTF-16 unit or surrogate pair is available.
#[inline]
fn decode_bytes_prefix(
    input: &[u8],
    index: usize,
    byte_order: ByteOrder,
) -> CharsetDecodeResult<(char, NonZeroUsize)> {
    let charset = Charset::from_utf16_byte_order(byte_order);
    debug_assert!(UncheckedSlice::range_fits(input.len(), index, 2));
    let available = input.len() - index;
    let first = read_ordered_u16(input, index, byte_order);
    if Utf16::is_high_surrogate(first) {
        if available < 4 {
            let kind = CharsetDecodeErrorKind::IncompleteSequence {
                required: 4,
                available,
            };
            return Err(CharsetDecodeError::new(charset, kind, index));
        }
        let second = read_ordered_u16(input, index + 2, byte_order);
        match Utf16::compose_pair(first, second).and_then(Unicode::to_char) {
            Some(ch) => Ok((ch, qubit_io::nz!(4))),
            None => {
                let kind = CharsetDecodeErrorKind::malformed(second as u32);
                Err(
                    CharsetDecodeError::new(charset, kind, index.saturating_add(2))
                        .with_consumed(qubit_io::nz!(4)),
                )
            }
        }
    } else if Utf16::is_low_surrogate(first) {
        let kind = CharsetDecodeErrorKind::malformed(first as u32);
        Err(CharsetDecodeError::new(charset, kind, index).with_consumed(qubit_io::nz!(2)))
    } else {
        let ch = char::from_u32(first as u32).expect("non-surrogate UTF-16 unit is a scalar value");
        Ok((ch, qubit_io::nz!(2)))
    }
}

/// Encodes one character into byte-serialized UTF-16 at `index` in `output`.
///
/// # Arguments
///
/// * `ch` - The character to encode.
/// * `output` - Byte destination.
/// * `byte_order` - Byte order for writing UTF-16 units.
/// * `index` - Start offset in `output` bytes; must be `<= output.len()`.
///
/// # Returns
///
/// `Ok(usize)` with the number of bytes written (`2` for BMP, `4` for
/// supplementary).
#[inline]
fn encode_bytes_char(ch: char, output: &mut [u8], byte_order: ByteOrder, index: usize) -> usize {
    let required = Utf16::unit_len(ch) * 2;
    debug_assert!(UncheckedSlice::range_fits(output.len(), index, required));
    let code_point = ch as u32;
    if required == 2 {
        write_ordered_u16(output, index, code_point as u16, byte_order);
    } else {
        let high =
            Utf16::high_surrogate(code_point).expect("supplementary scalar has high surrogate");
        let low = Utf16::low_surrogate(code_point).expect("supplementary scalar has low surrogate");
        write_ordered_u16(output, index, high, byte_order);
        write_ordered_u16(output, index + 2, low, byte_order);
    }
    required
}

/// Reads one endian-aware `u16` value from an already checked byte slice.
///
/// # Parameters
///
/// - `input`: Source byte slice.
/// - `index`: Start byte offset. The caller must guarantee two bytes are
///   available from this offset.
/// - `byte_order`: Byte order used to interpret the two bytes.
///
/// # Returns
///
/// Returns the decoded UTF-16 unit.
#[inline]
fn read_ordered_u16(input: &[u8], index: usize, byte_order: ByteOrder) -> u16 {
    // SAFETY: The caller guarantees that two bytes are readable from `index`.
    let bytes = unsafe {
        [
            UncheckedSlice::read(input, index),
            UncheckedSlice::read(input, index + 1),
        ]
    };
    match byte_order {
        ByteOrder::BigEndian => u16::from_be_bytes(bytes),
        ByteOrder::LittleEndian => u16::from_le_bytes(bytes),
    }
}

/// Writes one endian-aware `u16` value into an already checked byte slice.
///
/// # Parameters
///
/// - `output`: Destination byte slice.
/// - `index`: Start byte offset. The caller must guarantee two bytes are
///   writable from this offset.
/// - `unit`: UTF-16 unit to write.
/// - `byte_order`: Byte order used to serialize the unit.
#[inline]
fn write_ordered_u16(output: &mut [u8], index: usize, unit: u16, byte_order: ByteOrder) {
    let bytes = match byte_order {
        ByteOrder::BigEndian => unit.to_be_bytes(),
        ByteOrder::LittleEndian => unit.to_le_bytes(),
    };
    // SAFETY: The caller guarantees that two bytes are writable from `index`.
    unsafe {
        UncheckedSlice::write(output, index, bytes[0]);
        UncheckedSlice::write(output, index + 1, bytes[1]);
    }
}
