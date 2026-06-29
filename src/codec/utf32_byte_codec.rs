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
    CharsetEncodeError, CharsetEncodeResult, Unicode, Utf32,
};
use qubit_codec::{ByteOrder, Codec};
use qubit_io::UncheckedSlice;

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
///     <Utf32ByteCodec as Codec>::MAX_UNITS_PER_VALUE.get(),
/// );
///
/// let mut output = [0_u8; Utf32::MAX_BYTES_PER_CHAR];
/// let written = codec.encode_len(&'中').get();
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
    /// Returns [`Charset::UTF_32BE`] when configured with
    /// `ByteOrder::BigEndian`, otherwise [`Charset::UTF_32LE`].
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

    const MIN_UNITS_PER_VALUE: core::num::NonZeroUsize = qubit_io::nz!(4);
    const MAX_UNITS_PER_VALUE: core::num::NonZeroUsize = qubit_io::nz!(Utf32::MAX_BYTES_PER_CHAR);

    #[inline]
    unsafe fn decode(
        &mut self,
        input: &[u8],
        input_index: usize,
    ) -> CharsetCodecDecodeResult<(char, core::num::NonZeroUsize)> {
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
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        let written = encode_bytes_char(*ch, output, self.byte_order, output_index);
        debug_assert_eq!(written, Utf32::MAX_BYTES_PER_CHAR);
        debug_assert!(written <= output.len().saturating_sub(output_index));
        Ok(qubit_io::nz!(Utf32::MAX_BYTES_PER_CHAR))
    }
}

/// Decodes the first UTF-32 character from a closed byte buffer.
///
/// The input bytes are interpreted according to `byte_order`.
///
/// # Arguments
///
/// * `input` - UTF-32 encoded byte slice.
/// * `index` - Start byte offset; must be `<= input.len()`.
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
#[inline]
fn decode_bytes_prefix(
    input: &[u8],
    index: usize,
    byte_order: ByteOrder,
) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
    let charset = Charset::from_utf32_byte_order(byte_order);
    debug_assert!(UncheckedSlice::range_fits(input.len(), index, 4));
    let unit = read_ordered_u32(input, index, byte_order);
    match Unicode::to_char(unit) {
        Some(ch) => Ok((ch, qubit_io::nz!(4))),
        None => {
            let kind = CharsetDecodeErrorKind::InvalidCodePoint { value: unit };
            Err(CharsetDecodeError::new(charset, kind, index).with_consumed(qubit_io::nz!(4)))
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
/// * `index` - Start byte offset; must satisfy `index <= output.len()`.
///
/// # Returns
///
/// `Ok(4)` on success, because UTF-32 always occupies exactly four bytes.
#[inline]
fn encode_bytes_char(ch: char, output: &mut [u8], byte_order: ByteOrder, index: usize) -> usize {
    let required = Utf32::MAX_BYTES_PER_CHAR;
    debug_assert!(UncheckedSlice::range_fits(output.len(), index, required));
    write_ordered_u32(output, index, ch as u32, byte_order);
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
#[inline]
fn read_ordered_u32(input: &[u8], index: usize, byte_order: ByteOrder) -> u32 {
    // SAFETY: The caller guarantees that four bytes are readable from `index`.
    let bytes = unsafe {
        [
            UncheckedSlice::read(input, index),
            UncheckedSlice::read(input, index + 1),
            UncheckedSlice::read(input, index + 2),
            UncheckedSlice::read(input, index + 3),
        ]
    };
    match byte_order {
        ByteOrder::BigEndian => u32::from_be_bytes(bytes),
        ByteOrder::LittleEndian => u32::from_le_bytes(bytes),
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
#[inline]
fn write_ordered_u32(output: &mut [u8], index: usize, unit: u32, byte_order: ByteOrder) {
    let bytes = match byte_order {
        ByteOrder::BigEndian => unit.to_be_bytes(),
        ByteOrder::LittleEndian => unit.to_le_bytes(),
    };
    // SAFETY: The caller guarantees that four bytes are writable from `index`.
    unsafe {
        UncheckedSlice::write(output, index, bytes[0]);
        UncheckedSlice::write(output, index + 1, bytes[1]);
        UncheckedSlice::write(output, index + 2, bytes[2]);
        UncheckedSlice::write(output, index + 3, bytes[3]);
    }
}
