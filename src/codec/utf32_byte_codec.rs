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
    CharsetEncodeErrorKind,
    CharsetEncodeProbe,
    CharsetEncodeResult,
    Unicode,
    Utf32,
};
use qubit_codec::Codec;

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
///     CharsetEncodeProbe,
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
/// let written = codec.encode_len('中', 0).expect("mappable");
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

impl CharsetEncodeProbe for Utf32ByteCodec {
    /// Encodes one Unicode scalar value into UTF-32 bytes at `index`.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode scalar value to encode.
    /// * `index` - Input character index used for error context.
    ///
    /// # Returns
    ///
    /// Always returns `Ok(4)`.
    #[inline(always)]
    fn encode_len(
        &self,
        _ch: char,
        _index: usize,
    ) -> CharsetEncodeResult<usize> {
        Ok(Utf32::MAX_BYTES_PER_CHAR)
    }
}

unsafe impl Codec for Utf32ByteCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;
    type DecodeState = ();
    type EncodeState = ();

    #[inline(always)]
    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        // SAFETY: 4 is non-zero.
        unsafe { core::num::NonZeroUsize::new_unchecked(4) }
    }

    #[inline(always)]
    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        // SAFETY: UTF-32 byte encoding always uses four bytes.
        unsafe {
            core::num::NonZeroUsize::new_unchecked(Utf32::MAX_BYTES_PER_CHAR)
        }
    }

    #[inline(always)]
    unsafe fn decode(
        &mut self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let (ch, consumed) =
            decode_bytes_prefix(input, index, self.byte_order)?;
        debug_assert!(consumed.get() <= input.len() - index);
        Ok((ch, consumed))
    }

    #[inline(always)]
    unsafe fn encode(
        &mut self,
        ch: &char,
        output: &mut [u8],
        index: usize,
    ) -> CharsetEncodeResult<usize> {
        let written = encode_bytes_char(*ch, output, self.byte_order, index)?;
        debug_assert_eq!(written, Utf32::MAX_BYTES_PER_CHAR);
        debug_assert!(written <= output.len() - index);
        Ok(written)
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
/// * `CharsetDecodeErrorKind::InvalidInputIndex` when `index` is greater than
///   `input.len()`.
/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before four
///   bytes are available.
/// * `CharsetDecodeErrorKind::InvalidCodePoint` when the decoded unit is not a
///   valid scalar.
#[inline]
fn decode_bytes_prefix(
    input: &[u8],
    index: usize,
    byte_order: ByteOrder,
) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
    let charset = Charset::from_utf32_byte_order(byte_order);
    if index > input.len() {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex {
            input_len: input.len(),
        };
        return Err(CharsetDecodeError::new(charset, kind, index));
    }
    let available = input.len() - index;
    if available < 4 {
        let kind = CharsetDecodeErrorKind::IncompleteSequence {
            required: 4,
            available,
        };
        return Err(CharsetDecodeError::new(charset, kind, index));
    }
    let unit = read_ordered_u32(input, index, byte_order);
    match Unicode::to_char(unit) {
        Some(ch) => {
            // SAFETY: 4 is non-zero.
            Ok((ch, unsafe { core::num::NonZeroUsize::new_unchecked(4) }))
        }
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
///
/// # Errors
///
/// * `CharsetEncodeErrorKind::BufferTooSmall` when output has fewer than four
///   bytes from `index`.
#[inline]
fn encode_bytes_char(
    ch: char,
    output: &mut [u8],
    byte_order: ByteOrder,
    index: usize,
) -> CharsetEncodeResult<usize> {
    let charset = Charset::from_utf32_byte_order(byte_order);
    if index > output.len() {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: required_index(index, 4),
            available: 0,
        };
        return Err(CharsetEncodeError::new(charset, kind, index));
    }
    let required = 4;
    let available = output.len() - index;
    if available < required {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: required_index(index, required),
            available,
        };
        return Err(CharsetEncodeError::new(charset, kind, index));
    }
    write_ordered_u32(output, index, ch as u32, byte_order);
    Ok(4)
}

#[inline(always)]
const fn required_index(index: usize, required_units: usize) -> usize {
    match index.checked_add(required_units) {
        Some(required) => required,
        None => usize::MAX,
    }
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
    let bytes = [
        input[index],
        input[index + 1],
        input[index + 2],
        input[index + 3],
    ];
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
#[inline(always)]
fn write_ordered_u32(
    output: &mut [u8],
    index: usize,
    unit: u32,
    byte_order: ByteOrder,
) {
    let bytes = match byte_order {
        ByteOrder::BigEndian => unit.to_be_bytes(),
        ByteOrder::LittleEndian => unit.to_le_bytes(),
    };
    output[index..index + 4].copy_from_slice(&bytes);
}
