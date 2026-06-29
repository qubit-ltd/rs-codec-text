// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use thiserror::Error;

/// Classifies failures detected while decoding encoded units into Unicode text.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[non_exhaustive]
pub enum CharsetDecodeErrorKind {
    /// The requested input unit index is outside the input buffer.
    #[error("The input unit index is outside the input buffer.")]
    InvalidInputIndex {
        /// Length of the input provided to the codec call.
        input_len: usize,
    },

    /// The requested output character index is outside the output buffer.
    #[error("The output character index is outside the output buffer.")]
    InvalidOutputIndex {
        /// Length of the output provided to the codec call.
        output_len: usize,
    },

    /// The supplied output buffer is too small for decoded characters.
    #[error(
        "The output buffer is too small (required {required} characters, available {available} characters)."
    )]
    BufferTooSmall {
        /// Total output characters required.
        required: usize,

        /// Total output characters currently available.
        available: usize,
    },

    /// Output length arithmetic overflowed.
    #[error("The output length arithmetic overflowed.")]
    OutputLengthOverflow,

    /// The input units do not form a well-formed encoded sequence.
    #[error("The encoded text sequence is malformed.")]
    MalformedSequence {
        /// Optional malformed raw value captured from the offending input
        /// unit.
        value: Option<u32>,
    },

    /// The closed input ended before a complete character was available.
    #[error(
        "The encoded text sequence is incomplete (required {required} units, available {available} units)."
    )]
    IncompleteSequence {
        /// Total units required to complete the current sequence.
        required: usize,

        /// Total units currently available from the start of the current
        /// sequence.
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
    /// Creates a malformed-sequence error with a captured raw value.
    ///
    /// # Parameters
    ///
    /// - `value`: Raw malformed unit or byte value observed by the decoder.
    ///
    /// # Returns
    ///
    /// Returns [`Self::MalformedSequence`] carrying `value`.
    #[must_use]
    #[inline]
    pub const fn malformed(value: u32) -> Self {
        Self::MalformedSequence { value: Some(value) }
    }

    /// Creates a malformed-sequence error without a captured raw value.
    ///
    /// # Returns
    ///
    /// Returns [`Self::MalformedSequence`] with no associated raw value.
    #[must_use]
    #[inline]
    pub const fn malformed_unknown() -> Self {
        Self::MalformedSequence { value: None }
    }

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
            Self::BufferTooSmall { required, .. } => Some(required),
            Self::InvalidInputIndex { .. }
            | Self::InvalidOutputIndex { .. }
            | Self::OutputLengthOverflow
            | Self::MalformedSequence { .. }
            | Self::InvalidCodePoint { .. } => None,
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
            Self::BufferTooSmall { available, .. } => Some(available),
            Self::InvalidInputIndex { .. }
            | Self::InvalidOutputIndex { .. }
            | Self::OutputLengthOverflow
            | Self::MalformedSequence { .. }
            | Self::InvalidCodePoint { .. } => None,
        }
    }

    /// Returns the raw malformed value associated with this decoding error, if
    /// any.
    ///
    /// # Returns
    ///
    /// - `Some(value)` for [`Self::MalformedSequence`] when a specific
    ///   malformed unit value is available.
    /// - `Some(value)` for [`Self::InvalidCodePoint`].
    /// - `None` for other variants.
    #[must_use]
    #[inline]
    pub const fn value(self) -> Option<u32> {
        match self {
            Self::MalformedSequence { value } => value,
            Self::InvalidCodePoint { value } => Some(value),
            Self::InvalidInputIndex { .. }
            | Self::InvalidOutputIndex { .. }
            | Self::BufferTooSmall { .. }
            | Self::OutputLengthOverflow
            | Self::IncompleteSequence { .. } => None,
        }
    }

    /// Returns whether this kind represents incomplete input.
    ///
    /// # Returns
    ///
    /// Returns `true` for [`Self::IncompleteSequence`].
    #[must_use]
    #[inline]
    pub const fn is_incomplete(self) -> bool {
        matches!(self, Self::IncompleteSequence { .. })
    }

    /// Returns incomplete-input details for this kind.
    ///
    /// # Returns
    ///
    /// Returns `Some((required, available))` for [`Self::IncompleteSequence`],
    /// or `None` for all other variants.
    #[must_use]
    #[inline]
    pub const fn incomplete(self) -> Option<(usize, usize)> {
        match self {
            Self::IncompleteSequence {
                required,
                available,
            } => Some((required, available)),
            Self::InvalidInputIndex { .. }
            | Self::InvalidOutputIndex { .. }
            | Self::BufferTooSmall { .. }
            | Self::OutputLengthOverflow
            | Self::MalformedSequence { .. }
            | Self::InvalidCodePoint { .. } => None,
        }
    }

    /// Returns whether this kind represents malformed encoded input.
    ///
    /// # Returns
    ///
    /// Returns `true` for malformed sequences and invalid decoded scalar
    /// values. Incomplete input is caller-owned tail data and is not
    /// treated as malformed input by buffered charset decoders.
    #[must_use]
    #[inline]
    pub const fn is_malformed_input(self) -> bool {
        matches!(
            self,
            Self::MalformedSequence { .. } | Self::InvalidCodePoint { .. }
        )
    }
}
