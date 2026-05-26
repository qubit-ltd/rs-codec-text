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
    CharsetDecodeErrorKind,
};

/// Non-error status reported by the safe [`crate::CharsetCodec`] wrapper.
///
/// Values are reported for a [`crate::CharsetCodec::decode_one`] call over a
/// complete input slice and an absolute start index. `Complete` advances by a
/// positive number of units from that start index. `NeedMore` reports an
/// absolute required input length and the units currently available from the
/// same start index.
///
/// This type is intentionally part of the text-specific safe wrapper layer. It
/// is not returned by the lower-level [`qubit_codec::Codec`] trait, whose
/// unchecked decode contract is reserved for callers that already know a
/// complete value is readable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum DecodeStatus {
    /// A complete value was decoded from the prefix.
    Complete {
        /// The decoded Unicode scalar value.
        value: char,

        /// The number of input units consumed.
        ///
        /// This value must be greater than zero and must not exceed the units
        /// available from the decode start index.
        consumed: usize,
    },

    /// The current prefix is well-formed so far but incomplete.
    NeedMore {
        /// The absolute input length required to complete the current value.
        ///
        /// For a `decode_one(input, index)` call, this value must be greater
        /// than `input.len()` because `NeedMore` is only valid when the current
        /// slice is incomplete.
        required: usize,

        /// The number of input units currently available from the start index.
        ///
        /// For a `decode_one(input, index)` call, this is `input.len() - index`.
        available: usize,
    },
}

impl DecodeStatus {
    /// Converts an incomplete decode status into a closed-input decoding error.
    ///
    /// This helper is intended for stream or file readers that reach EOF after
    /// a codec reported [`DecodeStatus::NeedMore`]. The `required` field in
    /// [`DecodeStatus::NeedMore`] is an absolute input length for the current
    /// `decode_one(input, index)` call, while
    /// [`CharsetDecodeErrorKind::IncompleteSequence`] reports how many units
    /// the current sequence requires from `index`.
    ///
    /// # Parameters
    ///
    /// - `charset`: Charset being decoded.
    /// - `index`: Absolute input index where the incomplete sequence starts.
    ///
    /// # Returns
    ///
    /// Returns `Some(CharsetDecodeError)` for [`DecodeStatus::NeedMore`], or
    /// `None` when the status is [`DecodeStatus::Complete`].
    #[must_use]
    pub const fn incomplete_error(self, charset: Charset, index: usize) -> Option<CharsetDecodeError> {
        match self {
            Self::NeedMore { required, available } => {
                let kind = CharsetDecodeErrorKind::IncompleteSequence {
                    required: required.saturating_sub(index),
                    available,
                };
                Some(CharsetDecodeError::new(charset, kind, index))
            }
            Self::Complete { .. } => None,
        }
    }
}
