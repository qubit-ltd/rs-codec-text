/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
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
    Utf16,
};
use qubit_codec::Codec;

/// Combined byte-serialized UTF-16 codec.
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
///     Utf16,
///     Utf16ByteCodec,
/// };
///
/// let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
/// assert_eq!(Charset::UTF_16LE, codec.charset());
/// assert_eq!(Utf16::MAX_BYTES_PER_CHAR, codec.max_units_per_value().get());
///
/// let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];
/// let written = codec.encode_len('😀', 0).expect("mappable");
/// unsafe {
///     codec.encode_unchecked(&'😀', &mut output, 0).expect("buffer fits");
/// }
/// let (value, consumed) = unsafe {
///     codec.decode_unchecked(&output[..written], 0).expect("valid UTF-16LE")
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

    /// Returns the fixed-endian UTF-16 charset descriptor.
    ///
    /// # Returns
    ///
    /// Returns [`Charset::UTF_16LE`] or [`Charset::UTF_16BE`] according to this
    /// codec's configured byte order.
    #[must_use]
    #[inline(always)]
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
    #[inline(always)]
    fn charset(&self) -> Charset {
        Charset::from_utf16_byte_order(self.byte_order)
    }
}

impl CharsetEncodeProbe for Utf16ByteCodec {
    /// Encodes one Unicode scalar value into UTF-16 bytes at `index`.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode scalar value to encode.
    /// * `index` - Input character index used for error context.
    ///
    /// # Returns
    ///
    /// `Ok(usize)` with the required bytes (`2` for BMP and `4` for supplementary).
    #[inline(always)]
    fn encode_len(&self, ch: char, _index: usize) -> CharsetEncodeResult<usize> {
        Ok(Utf16::unit_len(ch) * 2)
    }
}

unsafe impl Codec for Utf16ByteCodec {
    type Value = char;
    type Unit = u8;
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;

    #[inline(always)]
    fn min_units_per_value(&self) -> core::num::NonZeroUsize {
        // SAFETY: 2 is non-zero.
        unsafe { core::num::NonZeroUsize::new_unchecked(2) }
    }

    #[inline(always)]
    fn max_units_per_value(&self) -> core::num::NonZeroUsize {
        // SAFETY: UTF-16 byte encoding uses at least one two-byte unit.
        unsafe { core::num::NonZeroUsize::new_unchecked(Utf16::MAX_BYTES_PER_CHAR) }
    }

    #[inline(always)]
    unsafe fn decode_unchecked(
        &self,
        input: &[u8],
        index: usize,
    ) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
        let (ch, consumed) = decode_bytes_prefix(input, index, self.byte_order)?;
        debug_assert!(consumed.get() <= input.len() - index);
        Ok((ch, consumed))
    }

    #[inline(always)]
    unsafe fn encode_unchecked(&self, ch: &char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        let written = encode_bytes_char(*ch, output, self.byte_order, index)?;
        debug_assert_eq!(written, ch.len_utf16() * 2);
        debug_assert!(written <= output.len() - index);
        Ok(written)
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
/// * `CharsetDecodeErrorKind::InvalidInputIndex` when `index` is greater than
///   `input.len()`.
/// * `CharsetDecodeErrorKind::MalformedSequence` for invalid UTF-16 byte
///   sequences or malformed surrogate usage.
/// * `CharsetDecodeErrorKind::IncompleteSequence` when EOF appears before a
///   complete UTF-16 unit or surrogate pair is available.
#[inline]
fn decode_bytes_prefix(
    input: &[u8],
    index: usize,
    byte_order: ByteOrder,
) -> CharsetDecodeResult<(char, core::num::NonZeroUsize)> {
    let charset = Charset::from_utf16_byte_order(byte_order);
    if index > input.len() {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
        return Err(CharsetDecodeError::new(charset, kind, index));
    }
    let available = input.len() - index;
    if available < 2 {
        let kind = CharsetDecodeErrorKind::IncompleteSequence { required: 2, available };
        return Err(CharsetDecodeError::new(charset, kind, index));
    }
    let first = read_ordered_u16(input, index, byte_order);
    if Utf16::is_high_surrogate(first) {
        if available < 4 {
            let kind = CharsetDecodeErrorKind::IncompleteSequence { required: 4, available };
            return Err(CharsetDecodeError::new(charset, kind, index));
        }
        let second = read_ordered_u16(input, index + 2, byte_order);
        match Utf16::compose_pair(first, second).and_then(Unicode::to_char) {
            Some(ch) => {
                // SAFETY: 4 is non-zero.
                Ok((ch, unsafe { core::num::NonZeroUsize::new_unchecked(4) }))
            }
            None => {
                let kind = CharsetDecodeErrorKind::MalformedSequence {
                    value: Some(second as u32),
                };
                Err(CharsetDecodeError::new(charset, kind, index + 2).with_consumed(4))
            }
        }
    } else if Utf16::is_low_surrogate(first) {
        let kind = CharsetDecodeErrorKind::MalformedSequence {
            value: Some(first as u32),
        };
        Err(CharsetDecodeError::new(charset, kind, index).with_consumed(2))
    } else {
        let ch = char::from_u32(first as u32).expect("non-surrogate UTF-16 unit is a scalar value");
        // SAFETY: 2 is non-zero.
        Ok((ch, unsafe { core::num::NonZeroUsize::new_unchecked(2) }))
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
/// `Ok(usize)` with the number of bytes written (`2` for BMP, `4` for supplementary).
///
/// # Errors
///
/// * `CharsetEncodeErrorKind::BufferTooSmall` when output bytes from `index`
///   are insufficient.
#[inline]
fn encode_bytes_char(ch: char, output: &mut [u8], byte_order: ByteOrder, index: usize) -> CharsetEncodeResult<usize> {
    let charset = Charset::from_utf16_byte_order(byte_order);
    if index > output.len() {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + 2,
            available: 0,
        };
        return Err(CharsetEncodeError::new(charset, kind, index));
    }
    let required = Utf16::unit_len(ch) * 2;
    let available = output.len() - index;
    if available < required {
        let kind = CharsetEncodeErrorKind::BufferTooSmall {
            required: index + required,
            available,
        };
        return Err(CharsetEncodeError::new(charset, kind, index));
    }
    let code_point = ch as u32;
    if required == 2 {
        write_ordered_u16(output, index, code_point as u16, byte_order);
    } else {
        let high = Utf16::high_surrogate(code_point).expect("supplementary scalar has high surrogate");
        let low = Utf16::low_surrogate(code_point).expect("supplementary scalar has low surrogate");
        write_ordered_u16(output, index, high, byte_order);
        write_ordered_u16(output, index + 2, low, byte_order);
    }
    Ok(required)
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
#[inline(always)]
fn read_ordered_u16(input: &[u8], index: usize, byte_order: ByteOrder) -> u16 {
    let bytes = [input[index], input[index + 1]];
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
#[inline(always)]
fn write_ordered_u16(output: &mut [u8], index: usize, unit: u16, byte_order: ByteOrder) {
    let bytes = match byte_order {
        ByteOrder::BigEndian => unit.to_be_bytes(),
        ByteOrder::LittleEndian => unit.to_le_bytes(),
    };
    output[index..index + 2].copy_from_slice(&bytes);
}
