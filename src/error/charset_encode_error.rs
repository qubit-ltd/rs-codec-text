// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::error::Error;
use core::fmt;

use qubit_codec::TranscodeEncodeError;
use qubit_codec::TranscodeFailure;

use crate::Charset;
use crate::CharsetEncodeErrorKind;

/// Error reported by a charset encoder.
///
/// The error always carries the target charset, error kind, and operation
/// index associated with the failure. Input positions depend on the producing
/// operation: [`CharsetEncoder`](crate::CharsetEncoder) counts `char` entries,
/// while unmappable-character reports from
/// [`CharsetConverter`](crate::CharsetConverter) count source codec units in
/// the input slice. Output index and capacity errors use target codec units.
/// Codec-domain errors retain the producing codec's index semantics. Mapping
/// does not convert these positions to character offsets or stream-wide
/// offsets. Mappings that cannot preserve a position use `usize::MAX` as
/// described by [`Self::map_transcode_failure`]; that sentinel can also mark a
/// discarded position. Preserve the original transcode error when its outer
/// context is needed. A retained pending value keeps its originating position;
/// do not treat a mapped index as a safe resume cursor.
/// Errors tied to a raw code point or character value expose that value
/// through [`Self::kind`] and [`Self::value`].
///
/// # Examples
///
/// ```
/// use qubit_codec_text::{Charset, CharsetEncodeError, CharsetEncodeErrorKind};
///
/// let error = CharsetEncodeError::new(
///     Charset::UTF_8,
///     CharsetEncodeErrorKind::UnmappableCharacter { value: '中' as u32 },
///     2,
/// );
/// assert_eq!(error.charset(), Charset::UTF_8);
/// assert_eq!(error.index(), 2);
/// assert_eq!(error.value(), Some('中' as u32));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub struct CharsetEncodeError {
    /// Target charset of the operation that produced this error.
    charset: Charset,
    /// Failure category describing why encoding could not proceed.
    kind: CharsetEncodeErrorKind,
    /// Operation-specific input or output position associated with the failure.
    index: usize,
}

/// Result type returned by charset encoders.
///
/// # Type Parameters
///
/// - `T`: Successful value produced by an encoding operation.
pub type CharsetEncodeResult<T> = Result<T, CharsetEncodeError>;

impl CharsetEncodeError {
    /// Maps an encoder transcode error into a charset encode error.
    ///
    /// Framework failures and unencodable values are mapped with `charset`;
    /// codec-domain errors retain their source error data. Mapping a domain
    /// error discards the outer phase, input index, and input-consumption
    /// metadata; inspect the original transcode error first when that context
    /// is needed. Source-owned index and consumption values may differ from the
    /// outer values. Framework failures may also lose context as described by
    /// [`Self::map_transcode_failure`]; inspect the original error first when
    /// its full diagnostic fields are needed.
    pub fn from_transcode_error(charset: Charset, error: TranscodeEncodeError<Self, char>) -> Self {
        match error {
            TranscodeEncodeError::Failure(failure) => Self::map_transcode_failure(charset, failure),
            TranscodeEncodeError::Unencodable { input_index, value } => {
                Self::map_unencodable(charset, input_index, value)
            }
            TranscodeEncodeError::Domain(error) => error.into_source(),
        }
    }

    /// Maps a transcode-layer failure into a charset encode error.
    ///
    /// # Parameters
    ///
    /// - `charset`: Target charset being encoded.
    /// - `error`: Framework failure reported by the transcode layer.
    ///
    /// # Returns
    ///
    /// Returns the charset-level representation of `error`.
    /// `InvalidInputIndex`, `InvalidOutputIndex`, `InsufficientOutput`, and
    /// `IncompleteInput` retain their original index and associated sizes.
    /// Lifecycle failures retain their specific kind, and
    /// `OutputLengthOverflow` retains its overflow kind; these mappings use
    /// `usize::MAX` as the index.
    /// Other failures, including `InvalidOutputRange` and `TrailingInput`,
    /// become [`CharsetEncodeErrorKind::UnexpectedTranscodeFailure`] with
    /// index `usize::MAX`; their original context is not retained.
    /// Inspect or preserve the original transcode error before mapping when
    /// those details are needed. The sentinel can therefore also indicate a
    /// discarded position, rather than an originally absent position.
    /// Directly constructed errors are not restricted to that sentinel.
    pub fn map_transcode_failure(charset: Charset, error: TranscodeFailure) -> Self {
        use TranscodeFailure::FinishAfterFinish;
        use TranscodeFailure::FinishBeforeReset;
        use TranscodeFailure::IncompleteInput;
        use TranscodeFailure::InsufficientOutput;
        use TranscodeFailure::InvalidInputIndex;
        use TranscodeFailure::InvalidOutputIndex;
        use TranscodeFailure::LifecyclePoisoned;
        use TranscodeFailure::OutputLengthOverflow;
        use TranscodeFailure::TrailingInput;
        use TranscodeFailure::TranscodeAfterFinish;
        use TranscodeFailure::TranscodeBeforeReset;

        match error {
            InvalidInputIndex { index, input_len } => {
                Self::new(charset, CharsetEncodeErrorKind::InvalidInputIndex { input_len }, index)
            }
            InvalidOutputIndex { index, output_len } => Self::new(
                charset,
                CharsetEncodeErrorKind::InvalidOutputIndex { output_len },
                index,
            ),
            InsufficientOutput {
                output_index,
                required,
                available,
            } => Self::new(
                charset,
                CharsetEncodeErrorKind::BufferTooSmall { required, available },
                output_index,
            ),
            OutputLengthOverflow => Self::new(charset, CharsetEncodeErrorKind::OutputLengthOverflow, usize::MAX),
            IncompleteInput {
                input_index,
                required,
                available,
            } => Self::new(
                charset,
                CharsetEncodeErrorKind::IncompleteInput { required, available },
                input_index,
            ),
            TrailingInput { .. } => Self::new(charset, CharsetEncodeErrorKind::UnexpectedTranscodeFailure, usize::MAX),
            TranscodeBeforeReset => Self::new(charset, CharsetEncodeErrorKind::TranscodeBeforeReset, usize::MAX),
            FinishBeforeReset => Self::new(charset, CharsetEncodeErrorKind::FinishBeforeReset, usize::MAX),
            TranscodeAfterFinish => Self::new(charset, CharsetEncodeErrorKind::TranscodeAfterFinish, usize::MAX),
            FinishAfterFinish => Self::new(charset, CharsetEncodeErrorKind::FinishAfterFinish, usize::MAX),
            LifecyclePoisoned => Self::new(charset, CharsetEncodeErrorKind::LifecyclePoisoned, usize::MAX),
            _ => Self::new(charset, CharsetEncodeErrorKind::UnexpectedTranscodeFailure, usize::MAX),
        }
    }

    /// Maps a transcode-layer unencodable value into a charset encode error.
    pub fn map_unencodable(charset: Charset, input_index: usize, value: Option<char>) -> Self {
        Self::new(
            charset,
            match value {
                Some(value) => CharsetEncodeErrorKind::UnmappableCharacter { value: value as u32 },
                None => CharsetEncodeErrorKind::UnencodableValue,
            },
            input_index,
        )
    }

    /// Creates an encoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `kind`: The failure category.
    /// - `index`: The operation-specific input or output position associated
    ///   with `kind`. This constructor stores the supplied position without
    ///   converting its units.
    ///
    /// # Returns
    ///
    /// Returns an encoding error carrying the supplied context.
    #[inline]
    pub const fn new(charset: Charset, kind: CharsetEncodeErrorKind, index: usize) -> Self {
        Self { charset, kind, index }
    }

    /// Returns the required count for this encoding error, if reported.
    ///
    /// # Returns
    ///
    /// Returns target codec units for
    /// [`CharsetEncodeErrorKind::BufferTooSmall`] or input units for
    /// [`CharsetEncodeErrorKind::IncompleteInput`]; returns `None` for
    /// other kinds.
    #[inline]
    pub const fn required(self) -> Option<usize> {
        self.kind.required()
    }

    /// Returns the available count for this encoding error, if reported.
    ///
    /// # Returns
    ///
    /// Returns available target codec units for
    /// [`CharsetEncodeErrorKind::BufferTooSmall`] or available input units for
    /// [`CharsetEncodeErrorKind::IncompleteInput`]; returns `None` for other
    /// kinds.
    #[inline]
    pub const fn available(self) -> Option<usize> {
        self.kind.available()
    }

    /// Returns output length for this encoding error, if reported.
    ///
    /// # Returns
    ///
    /// Returns `Some(output_len)` for
    /// [`CharsetEncodeErrorKind::InvalidOutputIndex`], otherwise `None`.
    #[inline]
    pub const fn output_len(self) -> Option<usize> {
        self.kind.output_len()
    }

    /// Returns the target charset.
    ///
    /// # Returns
    ///
    /// Returns the stored [`Charset`].
    #[inline]
    pub const fn charset(self) -> Charset {
        self.charset
    }

    /// Returns the encoding error kind.
    ///
    /// # Returns
    ///
    /// Returns the stored [`CharsetEncodeErrorKind`].
    #[inline]
    pub const fn kind(self) -> CharsetEncodeErrorKind {
        self.kind
    }

    /// Returns the operation index associated with this error.
    ///
    /// # Returns
    ///
    /// Input positions depend on the producing operation:
    /// [`CharsetEncoder`](crate::CharsetEncoder) counts `char` entries, while
    /// unmappable-character reports from
    /// [`CharsetConverter`](crate::CharsetConverter) count source codec units
    /// in the input slice. Output index and capacity errors use target codec
    /// units. Codec-domain errors retain the producing codec's index
    /// semantics. Mapping does not convert these positions to character
    /// offsets or stream-wide offsets. Positions unavailable after mapping use
    /// `usize::MAX` as described by [`Self::map_transcode_failure`], which can
    /// also indicate a discarded position. Interpret the index together with
    /// [`Self::kind`].
    #[inline]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Returns the raw value associated with this error.
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` when the error kind carries a raw code point or
    /// character value, or `None` for kinds without an associated value.
    #[inline]
    pub const fn value(self) -> Option<u32> {
        self.kind.value()
    }

    /// Offsets this error by a base unit index.
    ///
    /// # Parameters
    ///
    /// - `base`: The base index to add to the stored index.
    ///
    /// # Returns
    ///
    /// Returns a copy of this error with its index shifted by `base`.
    ///
    /// If the shifted index cannot be represented, it is saturated to
    /// [`usize::MAX`].
    #[inline]
    pub const fn offset_by(self, base: usize) -> Self {
        Self {
            charset: self.charset,
            kind: self.kind,
            index: match self.index.checked_add(base) {
                Some(index) => index,
                None => usize::MAX,
            },
        }
    }
}

impl fmt::Display for CharsetEncodeError {
    /// Formats this encoding error.
    ///
    /// # Parameters
    ///
    /// - `formatter`: The formatter receiving the diagnostic message.
    ///
    /// # Errors
    ///
    /// Returns any formatting error reported by `formatter`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.kind.value() {
            write!(
                formatter,
                "{} encoding error at index {} for value 0x{:x}: {}",
                self.charset, self.index, value, self.kind,
            )
        } else {
            write!(
                formatter,
                "{} encoding error at index {}: {}",
                self.charset, self.index, self.kind,
            )
        }
    }
}

impl Error for CharsetEncodeError {}
