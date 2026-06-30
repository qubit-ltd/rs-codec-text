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
/// [`Codec::MIN_UNITS_PER_VALUE`] units are available.
///
/// # Associated Types
///
/// Implementors must also implement the low-level [`Codec`] contract with
/// `Value = char`, plus [`CharsetDecodeError`] and [`CharsetEncodeError`] as
/// the concrete error types. The storage unit is inherited from
/// [`Codec::Unit`], keeping text wrappers bound to the same object without
/// duplicating associated types.
///
/// # Unsafe Codec Invariants
///
/// Implementors inherit the unsafe [`Codec`] contract and must keep these
/// charset-specific invariants true:
///
/// - [`Codec::decode`] is called only with `index < input.len()`, and must not
///   read outside `input[index..]`. If the visible input is a valid prefix of a
///   longer sequence, return
///   [`crate::CharsetDecodeErrorKind::IncompleteSequence`] instead of consuming
///   it as malformed input.
/// - malformed and invalid-code-point errors may report a consumed-unit count
///   with [`CharsetDecodeError::with_consumed`]. The count must be non-zero and
///   should describe the smallest invalid sequence width known to the codec.
/// - [`Codec::can_encode_value`] must return `true` before callers use
///   [`Codec::encode_len`] or enter [`Codec::encode`] for a value. `encode_len`
///   must be non-zero, must not exceed [`Codec::MAX_UNITS_PER_VALUE`], and must
///   match the number of units written by `encode` for the same value and codec
///   state.
/// - [`Codec::encode`] is called only after the caller reserved the exact
///   writable range reported by `encode_len`, and must not write outside that
///   range.
/// - codecs whose [`Codec::Unit`] is `u16` or `u32` operate on already-formed
///   host values, not serialized bytes. Use byte-oriented codecs when byte
///   order matters across files, processes, or network boundaries.
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
