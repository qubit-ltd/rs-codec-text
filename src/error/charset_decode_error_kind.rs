/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use thiserror::Error;

/// Classifies failures detected while decoding encoded units into Unicode text.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CharsetDecodeErrorKind {
    /// The input units do not form a well-formed encoded sequence.
    #[error("The encoded text sequence is malformed.")]
    MalformedSequence {
        /// Optional malformed raw value captured from the offending input unit.
        value: Option<u32>,
    },

    /// The requested input unit index is outside the input buffer.
    #[error("The input unit index is outside the input buffer.")]
    InvalidInputIndex {
        /// Length of the input provided to the codec call.
        input_len: usize,
    },

    /// The closed input ended before a complete character was available.
    #[error("The encoded text sequence is incomplete (required {required} units, available {available} units).")]
    IncompleteSequence {
        /// Total units required to complete the current sequence.
        required: usize,

        /// Total units currently available from the start of the current sequence.
        available: usize,
    },

    /// The decoded numeric value is not a valid Unicode scalar value.
    #[error("The decoded code point 0x{value:x} is not a valid Unicode scalar value.")]
    InvalidCodePoint {
        /// Raw decoded code-point value.
        value: u32,
    },
}

impl CharsetDecodeErrorKind {
    /// Returns the number of required input units for this kind, if available.
    ///
    /// # Returns
    ///
    /// - `Some(required)` for [`Self::IncompleteSequence`];
    /// - `None` for all other variants.
    #[must_use]
    #[inline]
    pub const fn required(self) -> Option<usize> {
        match self {
            Self::IncompleteSequence { required, .. } => Some(required),
            Self::MalformedSequence { .. } | Self::InvalidInputIndex { .. } | Self::InvalidCodePoint { .. } => None,
        }
    }

    /// Returns the number of currently available input units for this kind, if
    /// available.
    ///
    /// # Returns
    ///
    /// - `Some(available)` for [`Self::IncompleteSequence`];
    /// - `None` for all other variants.
    #[must_use]
    #[inline]
    pub const fn available(self) -> Option<usize> {
        match self {
            Self::IncompleteSequence { available, .. } => Some(available),
            Self::MalformedSequence { .. } | Self::InvalidInputIndex { .. } | Self::InvalidCodePoint { .. } => None,
        }
    }

    /// Returns the raw malformed value associated with this decoding error, if any.
    ///
    /// # Returns
    ///
    /// - `Some(value)` for [`Self::MalformedSequence`] when a specific malformed
    ///   unit value is available.
    /// - `Some(value)` for [`Self::InvalidCodePoint`].
    /// - `None` for other variants.
    #[must_use]
    #[inline]
    pub const fn value(self) -> Option<u32> {
        match self {
            Self::MalformedSequence { value } => value,
            Self::InvalidCodePoint { value } => Some(value),
            Self::IncompleteSequence { .. } | Self::InvalidInputIndex { .. } => None,
        }
    }

    /// Returns the input length when this error comes from an invalid input index.
    ///
    /// # Returns
    ///
    /// - `Some(input_len)` for [`Self::InvalidInputIndex`];
    /// - `None` for other variants.
    #[must_use]
    #[inline]
    pub const fn input_len(self) -> Option<usize> {
        match self {
            Self::InvalidInputIndex { input_len } => Some(input_len),
            Self::MalformedSequence { .. } | Self::IncompleteSequence { .. } | Self::InvalidCodePoint { .. } => None,
        }
    }
}
