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
    CharsetDecodeError,
    CharsetEncodeError,
};
use qubit_codec::Codec;

/// Charset metadata carried by a low-level `Codec<char, Unit>`.
///
/// `CharsetCodec` stays at the same low-level layer as [`Codec`]. It identifies
/// the charset and storage unit type, while actual single-value encoding and
/// decoding are provided by the inherited unsafe [`Codec`] methods. Checked
/// buffer management, stream boundary handling, replacement policy, and
/// unmappable-character policy live in [`crate::CharsetDecoder`] and
/// [`crate::CharsetEncoder`].
///
/// Built-in variable-width codecs report incomplete prefixes through
/// [`CharsetDecodeError`], so buffered decoders can call
/// [`Codec::decode_unchecked`] as soon as at least
/// [`Codec::min_units_per_value`] units are available.
///
/// # Associated Types
///
/// Implementors must also implement the low-level [`Codec<char, Self::Unit>`]
/// contract with [`CharsetDecodeError`] and [`CharsetEncodeError`] as the
/// concrete error types. This keeps the checked text wrapper and unchecked
/// single-scalar codec API bound to the same object.
pub trait CharsetCodec:
    Codec<char, Self::Unit, DecodeError = CharsetDecodeError, EncodeError = CharsetEncodeError>
{
    /// Storage unit used by the encoded representation.
    ///
    /// `u8` is used by byte-oriented codecs such as UTF-8 and Latin-1;
    /// `u16` is used by UTF-16 code-unit codecs; `u32` is used by UTF-32
    /// code-unit codecs.
    type Unit: Copy + Default + Eq + core::fmt::Debug;
    /// Returns the charset handled by this codec.
    ///
    /// # Returns
    ///
    /// Returns the codec's charset descriptor.
    #[must_use]
    fn charset(&self) -> Charset;
}
