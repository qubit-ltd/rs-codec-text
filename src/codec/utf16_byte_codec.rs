/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use super::inner::utf16;

use crate::{
    ByteOrder,
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeProbe,
    CharsetEncodeResult,
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
    type Unit = u8;

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

unsafe impl Codec<char, u8> for Utf16ByteCodec {
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
        let (ch, consumed) = utf16::decode_bytes_prefix(input, index, self.byte_order)?;
        debug_assert!(consumed.get() <= input.len() - index);
        Ok((ch, consumed))
    }

    #[inline(always)]
    unsafe fn encode_unchecked(&self, ch: &char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        let written = utf16::encode_bytes_char(*ch, output, self.byte_order, index)?;
        debug_assert_eq!(written, ch.len_utf16() * 2);
        debug_assert!(written <= output.len() - index);
        Ok(written)
    }
}
