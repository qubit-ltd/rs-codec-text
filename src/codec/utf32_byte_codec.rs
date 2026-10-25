// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::ByteOrder;
use qubit_codec::Codec;
use qubit_utils::SliceRange;
use qubit_utils::UncheckedSlice;
use qubit_utils::nonzero;

use crate::Charset;
use crate::CharsetCodec;
use crate::CharsetDecodeError;
use crate::CharsetDecodeErrorKind;
use crate::CharsetDecodeResult;
use crate::CharsetEncodeError;
use crate::CharsetEncodeResult;
use crate::Unicode;
use crate::Utf32;
use crate::error::CharsetCodecDecodeResult;

/// Combined byte-serialized UTF-32 codec.
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
///     Utf32,
///     Utf32ByteCodec,
/// };
///
/// let mut codec = Utf32ByteCodec::new(ByteOrder::BigEndian);
/// assert_eq!(Charset::UTF_32BE, codec.charset());
/// assert_eq!(
///     Utf32::MAX_BYTES_PER_CHAR,
///     <Utf32ByteCodec as Codec>::MAX_ENCODE_UNITS_PER_VALUE,
/// );
///
/// let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];
/// let written = codec.encode_len(&'中');
/// unsafe {
///     codec.encode(&'中', &mut output, 0).expect("buffer fits");
/// }
/// let (value, consumed) = unsafe {
///     codec.decode(&output[..written], 0).expect("valid UTF-32BE")
/// };
/// assert_eq!(('中', written), (value, consumed.get()));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Utf32ByteCodec {
    /// Byte order used by both encoder and decoder paths.
    byte_order: ByteOrder,
}

impl Utf32ByteCodec {
    /// Creates a byte-serialized UTF-32 codec.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used by the byte buffer.
    ///
    /// # Returns
    ///
    /// Returns a UTF-32 byte codec.
    #[must_use]
    #[inline]
    pub const fn new(byte_order: ByteOrder) -> Self {
        Self { byte_order }
    }

    /// Returns the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns the byte order used by this codec.
    #[must_use]
    #[inline]
    pub const fn byte_order(self) -> ByteOrder {
        self.byte_order
    }

    /// Returns the fixed-endian UTF-32 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_32LE`] or [`Charset::UTF_32BE`] according to this
    /// codec's configured byte order.
    #[must_use]
    #[inline]
    pub const fn charset(self) -> Charset {
        Charset::from_utf32_byte_order(self.byte_order)
    }
}

impl CharsetCodec for Utf32ByteCodec {
    /// Returns the fixed-endian UTF-32 charset for the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_32BE`] for [`ByteOrder::BigEndian`] and
    /// [`Charset::UTF_32LE`] for [`ByteOrder::LittleEndian`].
    /// [`ByteOrder::NativeEndian`] selects the corresponding charset for the
    /// target platform's native byte order.
    #[inline]
    fn charset(&self) -> Charset {
        Charset::from_utf32_byte_order(self.byte_order)
    }
}

impl Codec for Utf32ByteCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    const MIN_UNITS_PER_VALUE: usize = 4;
    const MAX_ENCODE_UNITS_PER_VALUE: usize = Utf32::MAX_BYTES_PER_CHAR;

    const MAX_DECODE_UNITS_PER_VALUE: usize = Utf32::MAX_BYTES_PER_CHAR;

    #[inline]
    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> CharsetCodecDecodeResult<(char, core::num::NonZeroUsize)> {
        // SAFETY: Codec::decode requires four readable bytes.
        let (ch, consumed) = unsafe { decode_bytes_prefix(input, input_index, self.byte_order) }
            .map_err(CharsetDecodeError::into_codec_failure)?;
        debug_assert!(consumed.get() <= input.len().saturating_sub(input_index));
        Ok((ch, consumed))
    }

    #[inline]
    unsafe fn encode(&mut self, ch: &char, output: &mut [u8], output_index: usize) -> CharsetEncodeResult<usize> {
        // SAFETY: Codec::encode guarantees encode_len(ch) writable bytes.
        let written = unsafe { encode_bytes_char(*ch, output, self.byte_order, output_index) };
        debug_assert_eq!(written, Utf32::MAX_BYTES_PER_CHAR);
        debug_assert!(written <= output.len().saturating_sub(output_index));
        Ok(Utf32::MAX_BYTES_PER_CHAR)
    }
}

/// Decodes the first UTF-32 character from a closed byte buffer.
///
/// The input bytes are interpreted according to `byte_order`.
///
/// # Arguments
///
/// * `input` - UTF-32 encoded byte slice.
/// * `index` - Start byte offset; at least four bytes must be readable.
/// * `byte_order` - Byte order used to read a `u32` unit.
///
/// # Returns
///
/// Returns the decoded character and a non-zero count of `4` consumed bytes.
///
/// # Errors
///
/// * `CharsetDecodeErrorKind::InvalidCodePoint` when the decoded unit is not a
///   valid scalar. UTF-32 stores a numeric scalar value directly, so surrogate
///   code points and values above `0x10FFFF` are reported as invalid code
///   points instead of malformed multi-unit structure.
///
/// # Safety
///
/// `input` must have four readable bytes starting at `index`.
#[inline]
unsafe fn decode_bytes_prefix(
    input: &[u8],
    index: usize,
    byte_order: ByteOrder,
) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
    let charset = Charset::from_utf32_byte_order(byte_order);
    debug_assert!(SliceRange::range_fits(input.len(), index, 4));
    // SAFETY: The four-byte minimum range was established by Codec::decode.
    let unit = unsafe { read_ordered_u32(input, index, byte_order) };
    match Unicode::to_char(unit) {
        Some(ch) => Ok((ch, nonzero(4))),
        None => {
            let kind = CharsetDecodeErrorKind::InvalidCodePoint { value: unit };
            Err(CharsetDecodeError::new(charset, kind, index).with_consumed(nonzero(4)))
        }
    }
}

/// Encodes one character into byte-serialized UTF-32 at `index` in `output`.
///
/// # Arguments
///
/// * `ch` - The character to encode.
/// * `output` - Destination byte buffer.
/// * `byte_order` - Byte order used to write the 4-byte representation.
/// * `index` - Start byte offset; four bytes must be writable.
///
/// # Returns
///
/// Returns the number of bytes written, always four for UTF-32.
///
/// # Safety
///
/// `output` must have four writable bytes starting at `index`.
#[inline]
unsafe fn encode_bytes_char(ch: char, output: &mut [u8], byte_order: ByteOrder, index: usize) -> usize {
    let required = Utf32::MAX_BYTES_PER_CHAR;
    debug_assert!(SliceRange::range_fits(output.len(), index, required));
    // SAFETY: The required four-byte range includes the serialized unit.
    unsafe { write_ordered_u32(output, index, ch as u32, byte_order) };
    required
}

/// Reads one endian-aware `u32` value from an already checked byte slice.
///
/// # Parameters
///
/// - `input`: Source byte slice.
/// - `index`: Start byte offset. The caller must guarantee four bytes are
///   available from this offset.
/// - `byte_order`: Byte order used to interpret the four bytes.
///
/// # Returns
///
/// Returns the decoded UTF-32 unit.
///
/// # Safety
///
/// `input` must have four readable bytes starting at `index`.
#[inline]
unsafe fn read_ordered_u32(input: &[u8], index: usize, byte_order: ByteOrder) -> u32 {
    // SAFETY: The required four-byte range includes its first byte.
    let first = unsafe { UncheckedSlice::read(input, index) };
    // SAFETY: The required four-byte range includes index + 1 without overflow.
    let second = unsafe { UncheckedSlice::read(input, index + 1) };
    // SAFETY: The required four-byte range includes index + 2 without overflow.
    let third = unsafe { UncheckedSlice::read(input, index + 2) };
    // SAFETY: The required four-byte range includes index + 3 without overflow.
    let fourth = unsafe { UncheckedSlice::read(input, index + 3) };
    let bytes = [first, second, third, fourth];
    match byte_order {
        ByteOrder::BigEndian => u32::from_be_bytes(bytes),
        ByteOrder::LittleEndian => u32::from_le_bytes(bytes),
        ByteOrder::NativeEndian => u32::from_ne_bytes(bytes),
    }
}

/// Writes one endian-aware `u32` value into an already checked byte slice.
///
/// # Parameters
///
/// - `output`: Destination byte slice.
/// - `index`: Start byte offset. The caller must guarantee four bytes are
///   writable from this offset.
/// - `unit`: UTF-32 unit to write.
/// - `byte_order`: Byte order used to serialize the unit.
///
/// # Safety
///
/// `output` must have four writable bytes starting at `index`.
#[inline]
unsafe fn write_ordered_u32(output: &mut [u8], index: usize, unit: u32, byte_order: ByteOrder) {
    let bytes = match byte_order {
        ByteOrder::BigEndian => unit.to_be_bytes(),
        ByteOrder::LittleEndian => unit.to_le_bytes(),
        ByteOrder::NativeEndian => unit.to_ne_bytes(),
    };
    // SAFETY: The required four-byte range includes its first byte.
    unsafe { UncheckedSlice::write(output, index, bytes[0]) };
    // SAFETY: The required four-byte range includes index + 1 without overflow.
    unsafe { UncheckedSlice::write(output, index + 1, bytes[1]) };
    // SAFETY: The required four-byte range includes index + 2 without overflow.
    unsafe { UncheckedSlice::write(output, index + 2, bytes[2]) };
    // SAFETY: The required four-byte range includes index + 3 without overflow.
    unsafe { UncheckedSlice::write(output, index + 3, bytes[3]) };
}
