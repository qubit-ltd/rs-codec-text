// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::error::Error;
use core::fmt;
use core::num::NonZeroUsize;

use qubit_codec::DecodeFailure;
use qubit_codec::TranscodeDecodeError;
use qubit_codec::TranscodeFailure;

use crate::Charset;
use crate::CharsetDecodeErrorKind;

/// Error reported by a charset decoder.
///
/// The error always carries the charset, error kind, and operation index at
/// which the failure was detected. The index refers to source units for input
/// errors and to output `char` slots for output index or capacity errors.
/// Errors that decode a raw numeric value, such
/// as malformed input units or invalid UTF-32 scalar values, carry that value
/// through [`Self::kind`] and [`Self::value`]. Invalid-input errors may also
/// carry a consumed-unit count so buffered decoders can make progress without
/// an extra status wrapper. Buffer index and output-capacity failures are
/// represented by dedicated [`CharsetDecodeErrorKind`] variants so the public
/// decoder API can expose a single domain error type.
///
/// # Examples
///
/// ```
/// use qubit_codec_text::{Charset, CharsetDecodeError, CharsetDecodeErrorKind};
///
/// let error = CharsetDecodeError::new(
///     Charset::UTF_8,
///     CharsetDecodeErrorKind::malformed(0xff),
///     2,
/// );
/// assert_eq!(error.charset(), Charset::UTF_8);
/// assert_eq!(error.index(), 2);
/// assert_eq!(error.value(), Some(0xff));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub struct CharsetDecodeError {
    /// Charset being decoded when this error was detected.
    charset: Charset,
    /// Failure category describing the decoding error.
    kind: CharsetDecodeErrorKind,
    /// Source-unit or output-character index associated with the failure.
    index: usize,
    /// Units that may be consumed after invalid input.
    consumed: Option<NonZeroUsize>,
}

/// Result type returned by charset decoders.
///
/// # Type Parameters
///
/// - `T`: Successful value produced by a decoding operation.
pub type CharsetDecodeResult<T> = Result<T, CharsetDecodeError>;

/// Result type returned by [`qubit_codec::Codec`] charset decoders.
pub(crate) type CharsetCodecDecodeResult<T> = Result<T, DecodeFailure<CharsetDecodeError>>;

impl CharsetDecodeError {
    /// Maps a decoder transcode error into a charset decode error.
    ///
    /// Framework failures are mapped with `charset`; codec-domain errors retain
    /// their source error data. Mapping a domain error discards the outer
    /// phase, input index, and input-consumption metadata; inspect the original
    /// transcode error first when that context is needed. Source-owned index
    /// and consumption values may differ from the outer values. Framework
    /// failures may also lose context as described by
    /// [`Self::map_transcode_failure`]; inspect the original error first when
    /// its full diagnostic fields are needed.
    ///
    /// # Returns
    ///
    /// Returns the mapped charset decoding error.
    pub fn from_transcode_error(charset: Charset, error: TranscodeDecodeError<Self>) -> Self {
        match error {
            TranscodeDecodeError::Failure(failure) => Self::map_transcode_failure(charset, failure),
            TranscodeDecodeError::Domain(error) => error.into_source(),
        }
    }

    /// Maps a transcode-layer failure into a charset decode error.
    ///
    /// # Parameters
    ///
    /// - `charset`: Charset being decoded.
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
    /// Other failures, including `InvalidOutputRange`, become
    /// [`CharsetDecodeErrorKind::UnexpectedTranscodeFailure`] with index
    /// `usize::MAX`; their original indices and sizes are not retained.
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
        use TranscodeFailure::TranscodeAfterFinish;
        use TranscodeFailure::TranscodeBeforeReset;

        match error {
            InvalidInputIndex { index, input_len } => {
                Self::new(charset, CharsetDecodeErrorKind::InvalidInputIndex { input_len }, index)
            }
            InvalidOutputIndex { index, output_len } => Self::new(
                charset,
                CharsetDecodeErrorKind::InvalidOutputIndex { output_len },
                index,
            ),
            InsufficientOutput {
                output_index,
                required,
                available,
            } => Self::new(
                charset,
                CharsetDecodeErrorKind::BufferTooSmall { required, available },
                output_index,
            ),
            OutputLengthOverflow => Self::new(charset, CharsetDecodeErrorKind::OutputLengthOverflow, usize::MAX),
            TranscodeBeforeReset => Self::new(charset, CharsetDecodeErrorKind::TranscodeBeforeReset, usize::MAX),
            FinishBeforeReset => Self::new(charset, CharsetDecodeErrorKind::FinishBeforeReset, usize::MAX),
            TranscodeAfterFinish => Self::new(charset, CharsetDecodeErrorKind::TranscodeAfterFinish, usize::MAX),
            FinishAfterFinish => Self::new(charset, CharsetDecodeErrorKind::FinishAfterFinish, usize::MAX),
            LifecyclePoisoned => Self::new(charset, CharsetDecodeErrorKind::LifecyclePoisoned, usize::MAX),
            IncompleteInput {
                input_index,
                required,
                available,
            } => Self::new(
                charset,
                CharsetDecodeErrorKind::IncompleteSequence { required, available },
                input_index,
            ),
            _ => Self::new(charset, CharsetDecodeErrorKind::UnexpectedTranscodeFailure, usize::MAX),
        }
    }

    /// Creates a decoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The charset being decoded.
    /// - `kind`: The failure category.
    /// - `index`: The operation index associated with `kind`: a source-unit
    ///   position for input errors, or an output-character position for output
    ///   index and capacity errors.
    ///
    /// Malformed input and invalid code-point errors default to a consumption
    /// count of one. All other kinds default to no consumption metadata.
    ///
    /// # Returns
    ///
    /// Returns a decoding error carrying the supplied context.
    #[inline]
    pub const fn new(charset: Charset, kind: CharsetDecodeErrorKind, index: usize) -> Self {
        Self {
            charset,
            kind,
            index,
            consumed: match kind {
                CharsetDecodeErrorKind::MalformedSequence { .. } | CharsetDecodeErrorKind::InvalidCodePoint { .. } => {
                    Some(NonZeroUsize::MIN)
                }
                CharsetDecodeErrorKind::InvalidInputIndex { .. }
                | CharsetDecodeErrorKind::InvalidOutputIndex { .. }
                | CharsetDecodeErrorKind::BufferTooSmall { .. }
                | CharsetDecodeErrorKind::OutputLengthOverflow
                | CharsetDecodeErrorKind::UnexpectedTranscodeFailure
                | CharsetDecodeErrorKind::TranscodeBeforeReset
                | CharsetDecodeErrorKind::FinishBeforeReset
                | CharsetDecodeErrorKind::TranscodeAfterFinish
                | CharsetDecodeErrorKind::FinishAfterFinish
                | CharsetDecodeErrorKind::LifecyclePoisoned
                | CharsetDecodeErrorKind::IncompleteSequence { .. } => None,
            },
        }
    }

    /// Returns a copy of this error with an explicit consumption count.
    ///
    /// # Parameters
    ///
    /// - `consumed`: Number of units supplied by the caller as consumption
    ///   metadata.
    ///
    /// This overrides the default metadata for any error kind. The caller is
    /// responsible for ensuring that the count is valid for the input and its
    /// recovery protocol. This metadata does not make input safe to skip for
    /// lifecycle or other non-input errors.
    ///
    /// # Returns
    ///
    /// Returns this error carrying the supplied consumption count.
    #[inline]
    pub const fn with_consumed(self, consumed: NonZeroUsize) -> Self {
        Self {
            charset: self.charset,
            kind: self.kind,
            index: self.index,
            consumed: Some(consumed),
        }
    }

    /// Converts this error into the failure type returned by
    /// [`qubit_codec::Codec::decode`].
    ///
    /// Incomplete input takes precedence over consumption metadata and remains
    /// an incomplete failure even when an explicit count was supplied. Other
    /// kinds use the stored count to select an invalid failure, or become an
    /// invalid-unknown failure when no count is present.
    ///
    /// # Panics
    ///
    /// Panics when the kind is `IncompleteSequence { required: 0, .. }` and
    /// debug assertions are enabled. Without debug assertions, that invalid
    /// incomplete error is converted to `invalid_unknown`.
    #[inline]
    pub fn into_codec_failure(self) -> DecodeFailure<Self> {
        if let Some((required, _)) = self.kind.incomplete() {
            if let Some(required) = NonZeroUsize::new(required) {
                DecodeFailure::incomplete_with_source(self, required)
            } else {
                #[cfg(debug_assertions)]
                panic!("incomplete charset decode errors must require non-zero units",);
                #[cfg(not(debug_assertions))]
                {
                    DecodeFailure::invalid_unknown(self)
                }
            }
        } else if let Some(consumed) = self.consumed() {
            DecodeFailure::invalid(self, consumed)
        } else {
            DecodeFailure::invalid_unknown(self)
        }
    }

    /// Returns the charset being decoded.
    ///
    /// # Returns
    ///
    /// Returns the stored [`Charset`].
    #[inline]
    pub const fn charset(self) -> Charset {
        self.charset
    }

    /// Returns the decoding error kind.
    ///
    /// # Returns
    ///
    /// Returns the stored [`CharsetDecodeErrorKind`].
    #[inline]
    pub const fn kind(self) -> CharsetDecodeErrorKind {
        self.kind
    }

    /// Returns the operation index associated with this error.
    ///
    /// # Returns
    ///
    /// Returns a source-unit position for input errors or an output-character
    /// position for output index and capacity errors. Interpret it together
    /// with [`Self::kind`].
    #[inline]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Returns the required count for this decoding error, if reported.
    ///
    /// # Returns
    ///
    /// Returns source units for [`CharsetDecodeErrorKind::IncompleteSequence`]
    /// or output `char` slots for [`CharsetDecodeErrorKind::BufferTooSmall`];
    /// returns `None` for other kinds.
    #[inline]
    pub const fn required(self) -> Option<usize> {
        self.kind.required()
    }

    /// Returns the available count for this decoding error, if reported.
    ///
    /// # Returns
    ///
    /// Returns available source units for
    /// [`CharsetDecodeErrorKind::IncompleteSequence`] or available output
    /// `char` slots for [`CharsetDecodeErrorKind::BufferTooSmall`]; returns
    /// `None` for other kinds.
    #[inline]
    pub const fn available(self) -> Option<usize> {
        self.kind.available()
    }

    /// Returns the raw value associated with this error.
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` when the error kind carries a raw unit or code
    /// point value, or `None` for kinds without an associated value.
    #[inline]
    pub const fn value(self) -> Option<u32> {
        self.kind.value()
    }

    /// Returns the stored consumption metadata, if any.
    ///
    /// # Returns
    ///
    /// Returns the stored consumption metadata. A newly constructed malformed
    /// or invalid-code-point error defaults to `Some(1)`; other newly
    /// constructed kinds default to `None`. An explicit value supplied through
    /// [`Self::with_consumed`] is returned for any kind.
    #[inline]
    pub const fn consumed(self) -> Option<NonZeroUsize> {
        self.consumed
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
            consumed: self.consumed,
        }
    }
}

impl fmt::Display for CharsetDecodeError {
    /// Formats this decoding error.
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
                "{} decoding error at index {} for value 0x{:x}: {}",
                self.charset, self.index, value, self.kind,
            )
        } else {
            write!(
                formatter,
                "{} decoding error at index {}: {}",
                self.charset, self.index, self.kind,
            )
        }
    }
}

impl Error for CharsetDecodeError {}
