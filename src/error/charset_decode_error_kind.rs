// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use thiserror::Error;

/// Classifies failures detected while decoding encoded units into Unicode text.
///
/// # Examples
///
/// ```
/// use qubit_codec_text::CharsetDecodeErrorKind;
///
/// let error = CharsetDecodeErrorKind::malformed(0xff);
/// assert!(error.is_malformed_input());
/// assert_eq!(error.value(), Some(0xff));
/// ```
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[non_exhaustive]
#[must_use]
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
    #[error("The output buffer is too small (required {required} characters, available {available} characters).")]
    BufferTooSmall {
        /// Total output characters required.
        required: usize,

        /// Total output characters currently available.
        available: usize,
    },

    /// Output length arithmetic overflowed.
    #[error("The output length arithmetic overflowed.")]
    OutputLengthOverflow,

    /// The transcode layer reported a framework failure that has no
    /// charset-specific decode representation.
    #[error("The transcode layer reported an unexpected framework failure.")]
    UnexpectedTranscodeFailure,

    /// Transcoding was attempted before the first successful reset.
    #[error("transcode called before reset")]
    TranscodeBeforeReset,

    /// Finalization was attempted before the first successful reset.
    #[error("finish called before reset")]
    FinishBeforeReset,

    /// Transcoding was attempted after finalization without another reset.
    #[error("transcode called after finish without an intervening reset")]
    TranscodeAfterFinish,

    /// Finalization was attempted twice without another reset.
    #[error("finish called twice without an intervening reset")]
    FinishAfterFinish,

    /// Lifecycle execution failed; a successful reset is required before reuse.
    #[error("transcoder lifecycle is poisoned; a successful reset is required")]
    LifecyclePoisoned,

    /// The input units do not form a well-formed encoded sequence.
    #[error("The encoded text sequence is malformed.")]
    MalformedSequence {
        /// Optional malformed raw value captured from the offending input
        /// unit.
        value: Option<u32>,
    },

    /// The closed input ended before a complete character was available.
    #[error("The encoded text sequence is incomplete (required {required} units, available {available} units).")]
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
    #[inline]
    pub const fn malformed(value: u32) -> Self {
        Self::MalformedSequence { value: Some(value) }
    }

    /// Creates a malformed-sequence error without a captured raw value.
    ///
    /// # Returns
    ///
    /// Returns [`Self::MalformedSequence`] with no associated raw value.
    #[inline]
    pub const fn malformed_unknown() -> Self {
        Self::MalformedSequence { value: None }
    }

    /// Returns the required count for this kind, if available.
    ///
    /// # Returns
    ///
    /// - source units for [`Self::IncompleteSequence`];
    /// - output `char` slots for [`Self::BufferTooSmall`];
    /// - `None` for all other variants.
    #[inline]
    pub const fn required(self) -> Option<usize> {
        match self {
            Self::IncompleteSequence { required, .. } => Some(required),
            Self::BufferTooSmall { required, .. } => Some(required),
            Self::InvalidInputIndex { .. }
            | Self::InvalidOutputIndex { .. }
            | Self::OutputLengthOverflow
            | Self::UnexpectedTranscodeFailure
            | Self::TranscodeBeforeReset
            | Self::FinishBeforeReset
            | Self::TranscodeAfterFinish
            | Self::FinishAfterFinish
            | Self::LifecyclePoisoned
            | Self::MalformedSequence { .. }
            | Self::InvalidCodePoint { .. } => None,
        }
    }

    /// Returns the available count for this kind, if available.
    ///
    /// # Returns
    ///
    /// - source units for [`Self::IncompleteSequence`];
    /// - output `char` slots for [`Self::BufferTooSmall`];
    /// - `None` for all other variants.
    #[inline]
    pub const fn available(self) -> Option<usize> {
        match self {
            Self::IncompleteSequence { available, .. } => Some(available),
            Self::BufferTooSmall { available, .. } => Some(available),
            Self::InvalidInputIndex { .. }
            | Self::InvalidOutputIndex { .. }
            | Self::OutputLengthOverflow
            | Self::UnexpectedTranscodeFailure
            | Self::TranscodeBeforeReset
            | Self::FinishBeforeReset
            | Self::TranscodeAfterFinish
            | Self::FinishAfterFinish
            | Self::LifecyclePoisoned
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
    #[inline]
    pub const fn value(self) -> Option<u32> {
        match self {
            Self::MalformedSequence { value } => value,
            Self::InvalidCodePoint { value } => Some(value),
            Self::InvalidInputIndex { .. }
            | Self::InvalidOutputIndex { .. }
            | Self::BufferTooSmall { .. }
            | Self::OutputLengthOverflow
            | Self::UnexpectedTranscodeFailure
            | Self::TranscodeBeforeReset
            | Self::FinishBeforeReset
            | Self::TranscodeAfterFinish
            | Self::FinishAfterFinish
            | Self::LifecyclePoisoned
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
    #[inline]
    pub const fn incomplete(self) -> Option<(usize, usize)> {
        match self {
            Self::IncompleteSequence { required, available } => Some((required, available)),
            Self::InvalidInputIndex { .. }
            | Self::InvalidOutputIndex { .. }
            | Self::BufferTooSmall { .. }
            | Self::OutputLengthOverflow
            | Self::UnexpectedTranscodeFailure
            | Self::TranscodeBeforeReset
            | Self::FinishBeforeReset
            | Self::TranscodeAfterFinish
            | Self::FinishAfterFinish
            | Self::LifecyclePoisoned
            | Self::MalformedSequence { .. }
            | Self::InvalidCodePoint { .. } => None,
        }
    }

    /// Returns whether this kind represents malformed encoded input.
    ///
    /// # Returns
    ///
    /// Returns `true` for malformed sequences and invalid decoded scalar
    /// values. Incomplete input in an open stream is a tail awaiting more
    /// data. At EOF, [`crate::CharsetDecoder::transcode_eof`] applies the
    /// configured [`crate::MalformedAction`] to that tail. This predicate
    /// remains `false` for [`Self::IncompleteSequence`]; it does not describe
    /// the EOF action.
    #[must_use]
    #[inline]
    pub const fn is_malformed_input(self) -> bool {
        matches!(self, Self::MalformedSequence { .. } | Self::InvalidCodePoint { .. })
    }
}
