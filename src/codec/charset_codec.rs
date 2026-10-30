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
/// # Malformed-input recovery
///
/// For an [`qubit_codec::DecodeFailure::Invalid`] returned directly by
/// [`Codec::decode`] for `input_index`, a known consumed width is measured
/// from that call's `input_index` in the codec's input units. The next
/// position after the malformed prefix is `input_index + consumed.get()`, not
/// `source.index() + consumed.get()`. The source index identifies where the
/// failure was detected; it can point to a valid following unit that must be
/// preserved.
///
/// [`crate::Utf8Codec`] and byte-oriented UTF-16 codecs count bytes;
/// [`crate::Utf16U16Codec`] counts `u16` units. `None` provides no recovery
/// width. This rule does not apply to incomplete failures or provide a resume
/// cursor for buffered transcode errors. Every subsequent unsafe decode must
/// still satisfy its input-boundary and minimum-readable-width requirements.
///
/// ```
/// use qubit_codec::Codec;
/// use qubit_codec::DecodeFailure;
///
/// use qubit_codec_text::Utf8Codec;
///
/// let mut codec = Utf8Codec;
/// let input = [b'X', 0xe1, 0x80, b'A'];
/// let input_index = 1;
/// // SAFETY: Three bytes are readable at input_index; UTF-8 requires at least one.
/// let failure = unsafe { codec.decode(&input, input_index) }
///     .expect_err("the UTF-8 prefix ends before ASCII A");
/// let DecodeFailure::Invalid { source, consumed, .. } = failure else {
///     panic!("expected an invalid UTF-8 sequence");
/// };
/// let width = consumed.expect("known malformed prefix width").get();
/// assert_eq!(3, source.index());
/// assert_eq!(2, width);
/// let next_index = input_index + width;
/// assert_eq!(3, next_index);
/// // SAFETY: The recovery position retains one readable ASCII byte.
/// let (ch, used) = unsafe { codec.decode(&input, next_index) }
///     .expect("the valid following character is preserved");
/// assert_eq!(('A', 1), (ch, used.get()));
/// ```
///
/// For `u16` input `[0x0058, 0xd800, 0x0041]` starting at 1, the detection
/// index is 2 and the width is 1, so decoding resumes at 2. The byte-oriented
/// UTF-16 equivalent starts at byte index 2, detects the error at byte index
/// 4, and reports width 2.
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
/// - With the default encoder or converter policy, the target codec must encode
///   at least one of U+FFFD or the question mark. Constructors prefer U+FFFD
///   and fall back to the question mark; they panic only when neither is
///   encodable. Explicit ignore or report policies do not require these
///   replacement characters.
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
