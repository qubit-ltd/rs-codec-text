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
    Charset,
    CharsetDecodeResult,
    CharsetEncodeResult,
};

use super::decode_status::DecodeStatus;

/// Low-level charset algorithm for one storage-unit representation.
///
/// `CharsetCodec` performs only primitive per-character encoding and decoding.
/// Policy decisions such as replacement, ignoring malformed input, or reporting
/// unmappable characters are handled by [`crate::CharsetDecoder`] and
/// [`crate::CharsetEncoder`].
///
/// # Associated Types
///
/// - `Unit`: Storage unit used by the encoded representation, such as `u8` for
///   byte-oriented charsets, `u16` for UTF-16 code units, or `u32` for UTF-32
///   code units.
pub trait CharsetCodec {
    /// Storage unit used by the encoded representation.
    ///
    /// `u8` is used by byte-oriented codecs such as UTF-8 and Latin-1;
    /// `u16` is used by UTF-16 code-unit codecs; `u32` is used by UTF-32
    /// code-unit codecs.
    type Unit: Copy + Default;
    /// Returns the charset handled by this codec.
    ///
    /// # Returns
    ///
    /// Returns the codec's charset descriptor.
    #[must_use]
    fn charset(&self) -> Charset;

    /// Returns the maximum number of storage units needed for one character.
    ///
    /// # Returns
    ///
    /// Returns an upper bound for one encoded Unicode scalar value.
    #[must_use]
    fn max_units_per_char(&self) -> usize;

    /// Decodes one Unicode scalar value from `input` starting at `index`.
    ///
    /// # Parameters
    ///
    /// - `input`: Complete input unit slice.
    /// - `index`: Absolute input unit index where decoding starts.
    ///
    /// # Returns
    ///
    /// Returns [`DecodeStatus::Complete`] when one scalar value is available.
    /// Its `consumed` value must be greater than zero and must not exceed
    /// `input.len() - index`.
    ///
    /// Returns [`DecodeStatus::NeedMore`] when the current prefix is valid but
    /// incomplete. Its `required` value is the absolute input length required
    /// to complete the current value and must be greater than `input.len()`;
    /// its `available` value is `input.len() - index`.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CharsetDecodeError`] when `index` is outside `input`,
    /// or when the sequence at `index` is malformed or decodes to a non-scalar
    /// value.
    fn decode_one(&self, input: &[Self::Unit], index: usize) -> CharsetDecodeResult<DecodeStatus>;

    /// Encodes one Unicode scalar value into `output` starting at `index`.
    ///
    /// # Parameters
    ///
    /// - `ch`: Unicode scalar value to encode.
    /// - `output`: Complete output unit slice.
    /// - `index`: Absolute output unit index where writing starts.
    ///
    /// # Returns
    ///
    /// Returns the number of output units written.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CharsetEncodeError`] when `ch` cannot be represented by
    /// this charset or `output` does not have enough capacity from `index`.
    fn encode_one(&self, ch: char, output: &mut [Self::Unit], index: usize) -> CharsetEncodeResult<usize>;
}
