// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::{
    ByteOrder,
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeResult,
    Unicode,
    Utf32,
};
use qubit_codec::Codec;
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
/// use qubit_codec_text::{
///     ByteOrder,
///     CharsetCodec,
///     Codec,
///     Charset,
///     Utf32,
///     Utf32ByteCodec,
/// };
///
/// let mut codec = Utf32ByteCodec::new(ByteOrder::BigEndian);
/// assert_eq!(Charset::UTF_32BE, codec.charset());
/// assert_eq!(Utf32::MAX_BYTES_PER_CHAR, codec.max_units_per_value().get());
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
    #[inline(always)]
    pub const fn new(byte_order: ByteOrder) -> Self {
        Self { byte_order }
    }

    /// Returns the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns the byte order used by this codec.
    #[must_use]
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn charset(&self) -> Charset {
        Charset::from_utf32_byte_order(self.byte_order)
    }
}

unsafe impl Codec for Utf32ByteCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    #[inline(always)]
    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        qubit_io::nz!(4)
    }

    #[inline(always)]
    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        qubit_io::nz!(Utf32::MAX_BYTES_PER_CHAR)
    }

    #[inline(always)]
    unsafe fn decode(
        &mut self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let (ch, consumed) =
            decode_bytes_prefix(input, index, self.byte_order)?;
        debug_assert!(consumed.get() <= input.len().saturating_sub(index));
        Ok((ch, consumed))
    }

    #[inline(always)]
    unsafe fn encode(
        &mut self,
        ch: &char,
        output: &mut [u8],
        index: usize,
    ) -> CharsetEncodeResult<core::num::NonZeroUsize> {
        let written = encode_bytes_char(*ch, output, self.byte_order, index);
        debug_assert_eq!(written, Utf32::MAX_BYTES_PER_CHAR);
        debug_assert!(written <= output.len().saturating_sub(index));
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
///   valid scalar.
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
            Err(CharsetDecodeError::new(charset, kind, index).with_consumed(4))
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
fn encode_bytes_char(
    ch: char,
    output: &mut [u8],
    byte_order: ByteOrder,
    index: usize,
) -> usize {
    let required = 4;
    debug_assert!(UncheckedSlice::range_fits(output.len(), index, required));
    write_ordered_u32(output, index, ch as u32, byte_order);
    4
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
#[inline(always)]
fn read_ordered_u32(input: &[u8], index: usize, byte_order: ByteOrder) -> u32 {
    // SAFETY: The caller guarantees that four bytes are readable from `index`.
    let unit = unsafe { UncheckedSlice::read_ne_unaligned(input, index) };
    match byte_order {
        ByteOrder::BigEndian => u32::from_be(unit),
        ByteOrder::LittleEndian => u32::from_le(unit),
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
#[inline(always)]
fn write_ordered_u32(
    output: &mut [u8],
    index: usize,
    unit: u32,
    byte_order: ByteOrder,
) {
    let value = match byte_order {
        ByteOrder::BigEndian => unit.to_be(),
        ByteOrder::LittleEndian => unit.to_le(),
    };
    // SAFETY: The caller guarantees that four bytes are writable from `index`.
    unsafe { UncheckedSlice::write_ne_unaligned(output, index, value) };
}
