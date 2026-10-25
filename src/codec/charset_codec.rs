// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::Codec;

use crate::Charset;
use crate::CharsetDecodeError;
use crate::CharsetEncodeError;

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
/// - Before calling [`Codec::decode`], callers must provide a valid input
///   boundary and at least [`Codec::MIN_UNITS_PER_VALUE`] readable units from
///   `input_index`. Implementations must not read beyond the available units. A
///   valid prefix may still be incomplete even when the minimum readable width
///   is satisfied; return [`crate::CharsetDecodeErrorKind::IncompleteSequence`]
///   in that case.
/// - malformed and invalid-code-point errors may report a consumed-unit count
///   with [`CharsetDecodeError::with_consumed`]. The count must be non-zero and
///   should describe the smallest invalid sequence width known to the codec.
///   Charset decoder recovery uses this error's width to advance after a
///   malformed sequence. For a multi-unit invalid sequence, set the width with
///   [`CharsetDecodeError::with_consumed`] and then use
///   [`CharsetDecodeError::into_codec_failure`] so the error and decode failure
///   carry the same width. Avoid constructing conflicting widths manually.
///
///   ```
///   use core::num::NonZeroUsize;
///   use qubit_codec_text::Charset;
///   use qubit_codec_text::CharsetDecodeError;
///   use qubit_codec_text::CharsetDecodeErrorKind;
///
///   let width = NonZeroUsize::new(2).expect("non-zero malformed width");
///   let error = CharsetDecodeError::new(
///       Charset::UTF_16LE,
///       CharsetDecodeErrorKind::malformed(0xdc00),
///       0,
///   ).with_consumed(width);
///   let failure = error.into_codec_failure();
///   assert_eq!(Some(width), failure.consumed_units());
///   assert_eq!(Some(width), failure.invalid_source().unwrap().consumed());
///   ```
/// - [`Codec::can_encode_value`] must return `true` before callers use
///   [`Codec::encode_len`] or enter [`Codec::encode`] for a value. `encode_len`
///   must not exceed [`Codec::MAX_ENCODE_UNITS_PER_VALUE`] and must match the
///   number of units written by `encode` for the same value and codec state. A
///   zero length is valid for stateful codecs that deliberately buffer output.
/// - Implementations used with the default [`crate::CharsetEncoder`] or
///   [`crate::CharsetConverter`] policy must be able to encode `?`. Failure to
///   encode `?` is an implementation bug; those constructors intentionally
///   panic to expose it immediately.
/// - [`Codec::encode`] is called only after the caller has confirmed that
///   [`Codec::can_encode_value`] returns `true` and reserved either the exact
///   writable range reported by `encode_len` for the same value and codec
///   state, or at least [`Codec::MAX_ENCODE_UNITS_PER_VALUE`]. It must not
///   write outside the reserved range.
/// - codecs whose [`Codec::Unit`] is `u16` or `u32` operate on already-formed
///   host values, not serialized bytes. Use byte-oriented codecs when byte
///   order matters across files, processes, or network boundaries.
pub trait CharsetCodec:
    Codec<Value = char, DecodeError = CharsetDecodeError, EncodeError = CharsetEncodeError>
{
    /// Returns the charset handled by this codec.
    ///
    /// # Returns
    ///
    /// Returns the codec's charset descriptor.
    #[must_use]
    fn charset(&self) -> Charset;
}
