// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::{
    Charset,
    CharsetDecodeError,
    CharsetEncodeError,
};
use qubit_codec::Codec;

/// Charset metadata carried by a low-level `Codec<Value = char>`.
///
/// `CharsetCodec` stays at the same low-level layer as [`Codec`]. It identifies
/// the charset, while actual storage unit type, single-value encoding, and
/// decoding are provided by the inherited unsafe [`Codec`] contract. Checked
/// buffer management, stream boundary handling, replacement policy, and
/// unmappable-character policy live in [`crate::CharsetDecoder`] and
/// [`crate::CharsetEncoder`].
///
/// Built-in variable-width codecs report incomplete prefixes through
/// [`CharsetDecodeError`], so buffered decoders can call
/// [`Codec::decode`] as soon as at least
/// [`Codec::min_units_per_value`] units are available.
///
/// # Associated Types
///
/// Implementors must also implement the low-level [`Codec`] contract with
/// `Value = char`, plus [`CharsetDecodeError`] and [`CharsetEncodeError`] as
/// the concrete error types. The storage unit is inherited from
/// [`Codec::Unit`], keeping text wrappers bound to the same object without
/// duplicating associated types.
pub trait CharsetCodec:
    Codec<
        Value = char,
        DecodeError = CharsetDecodeError,
        EncodeError = CharsetEncodeError,
    >
{
    /// Returns the charset handled by this codec.
    ///
    /// # Returns
    ///
    /// Returns the codec's charset descriptor.
    #[must_use]
    fn charset(&self) -> Charset;
}
