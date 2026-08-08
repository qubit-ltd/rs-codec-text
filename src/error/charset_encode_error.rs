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
/// index associated with the failure. For buffer errors this is the
/// caller-supplied output index. Errors tied to a raw code point or character
/// value expose that value through [`Self::kind`] and [`Self::value`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharsetEncodeError {
    /// Target charset of the operation that produced this error.
    charset: Charset,
    /// Failure category describing why encoding could not proceed.
    kind: CharsetEncodeErrorKind,
    /// Output unit index or input code point index where failure occurred.
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
    /// codec-domain errors retain their original charset error.
    #[must_use]
    pub fn from_transcode_error(
        charset: Charset,
        error: TranscodeEncodeError<Self, char>,
    ) -> Self {
        match error {
            TranscodeEncodeError::Failure(failure) => {
                Self::map_transcode_failure(charset, failure)
            }
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
    /// Returns the charset-level representation of `error`. Buffer and index
    /// failures retain their original indices and sizes.
    /// Decode-only framework failures are reported as
    /// [`CharsetEncodeErrorKind::UnexpectedTranscodeFailure`] instead of
    /// being misreported as output-length overflow.
    #[must_use]
    pub fn map_transcode_failure(
        charset: Charset,
        error: TranscodeFailure,
    ) -> Self {
        use TranscodeFailure::IncompleteInput;
        use TranscodeFailure::InsufficientOutput;
        use TranscodeFailure::InvalidInputIndex;
        use TranscodeFailure::InvalidOutputIndex;
        use TranscodeFailure::OutputLengthOverflow;
        use TranscodeFailure::TrailingInput;

        match error {
            InvalidInputIndex { index, input_len } => Self::new(
                charset,
                CharsetEncodeErrorKind::InvalidInputIndex { input_len },
                index,
            ),
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
                CharsetEncodeErrorKind::BufferTooSmall {
                    required,
                    available,
                },
                output_index,
            ),
            OutputLengthOverflow => Self::new(
                charset,
                CharsetEncodeErrorKind::OutputLengthOverflow,
                usize::MAX,
            ),
            IncompleteInput {
                input_index,
                required,
                available,
            } => Self::new(
                charset,
                CharsetEncodeErrorKind::IncompleteInput {
                    required,
                    available,
                },
                input_index,
            ),
            TrailingInput { .. } => Self::new(
                charset,
                CharsetEncodeErrorKind::UnexpectedTranscodeFailure,
                usize::MAX,
            ),
            _ => Self::new(
                charset,
                CharsetEncodeErrorKind::UnexpectedTranscodeFailure,
                usize::MAX,
            ),
        }
    }

    /// Maps a transcode-layer unencodable value into a charset encode error.
    #[must_use]
    pub fn map_unencodable(
        charset: Charset,
        input_index: usize,
        value: Option<char>,
    ) -> Self {
        Self::new(
            charset,
            match value {
                Some(value) => CharsetEncodeErrorKind::UnmappableCharacter {
                    value: value as u32,
                },
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
    /// - `index`: The operation index associated with the failure.
    ///
    /// # Returns
    ///
    /// Returns an encoding error carrying the supplied context.
    #[inline]
    pub const fn new(
        charset: Charset,
        kind: CharsetEncodeErrorKind,
        index: usize,
    ) -> Self {
        Self {
            charset,
            kind,
            index,
        }
    }

    /// Returns required output units for this encoding error, if reported.
    ///
    /// # Returns
    ///
    /// Returns `Some(required)` for [`CharsetEncodeErrorKind::BufferTooSmall`]
    /// and [`CharsetEncodeErrorKind::IncompleteInput`], otherwise `None`.
    #[inline]
    pub const fn required(self) -> Option<usize> {
        self.kind.required()
    }

    /// Returns available output units for this encoding error, if reported.
    ///
    /// # Returns
    ///
    /// Returns `Some(available)` for
    /// [`CharsetEncodeErrorKind::BufferTooSmall`] and
    /// [`CharsetEncodeErrorKind::IncompleteInput`], otherwise `None`.
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
    /// Returns the stored index.
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
