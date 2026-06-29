// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::{
    error::Error,
    fmt,
    num::NonZeroUsize,
};

use qubit_codec::DecodeFailure;

use crate::{
    Charset,
    CharsetDecodeErrorKind,
};

/// Error reported by a charset decoder.
///
/// The error always carries the charset, error kind, and input unit index at
/// which the failure was detected. Errors that decode a raw numeric value, such
/// as malformed input units or invalid UTF-32 scalar values, carry that value
/// through [`Self::kind`] and [`Self::value`]. Invalid-input errors may also
/// carry a consumed-unit count so buffered decoders can make progress without
/// an extra status wrapper. Buffer index and output-capacity failures are
/// reported by [`qubit_codec::TranscodeError`] rather than this domain error
/// kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharsetDecodeError {
    /// Charset being decoded when this error was detected.
    charset: Charset,
    /// Failure category describing the decoding error.
    kind: CharsetDecodeErrorKind,
    /// Input unit index at which decoding failure occurred.
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
pub(crate) type CharsetCodecDecodeResult<T> =
    Result<T, DecodeFailure<CharsetDecodeError>>;

/// Maps a charset decode error into codec-level flow or domain failure.
#[inline]
pub(crate) fn map_charset_decode_failure(
    error: CharsetDecodeError,
) -> DecodeFailure<CharsetDecodeError> {
    if let Some(required) = error.required() {
        if let Some(required) = NonZeroUsize::new(required) {
            DecodeFailure::incomplete(required)
        } else {
            #[cfg(debug_assertions)]
            panic!(
                "incomplete charset decode errors must require non-zero units",
            );
            #[cfg(not(debug_assertions))]
            {
                DecodeFailure::invalid_without_consumed(error)
            }
        }
    } else if let Some(consumed) = error.consumed() {
        DecodeFailure::invalid(error, consumed)
    } else {
        DecodeFailure::invalid_without_consumed(error)
    }
}

impl CharsetDecodeError {
    /// Creates a decoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The charset being decoded.
    /// - `kind`: The failure category.
    /// - `index`: The input unit index where the failure was detected.
    ///
    /// # Returns
    ///
    /// Returns a decoding error carrying the supplied context.
    #[inline]
    pub const fn new(
        charset: Charset,
        kind: CharsetDecodeErrorKind,
        index: usize,
    ) -> Self {
        Self {
            charset,
            kind,
            index,
            consumed: match kind {
                CharsetDecodeErrorKind::MalformedSequence { .. }
                | CharsetDecodeErrorKind::InvalidCodePoint { .. } => {
                    Some(NonZeroUsize::MIN)
                }
                CharsetDecodeErrorKind::IncompleteSequence { .. } => None,
            },
        }
    }

    /// Returns a copy of this error with invalid-input consumption context.
    ///
    /// # Parameters
    ///
    /// - `consumed`: Number of units that may be consumed to make progress.
    ///
    /// # Returns
    ///
    /// Returns this error carrying the supplied consumption count.
    #[must_use]
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
    #[must_use]
    #[inline]
    pub fn into_codec_failure(self) -> DecodeFailure<Self> {
        map_charset_decode_failure(self)
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

    /// Returns the input unit index associated with this error.
    ///
    /// # Returns
    ///
    /// Returns the index at which the error was detected.
    #[inline]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Returns required input units for this decoding error, if reported.
    ///
    /// # Returns
    ///
    /// Returns `Some(required)` for
    /// [`CharsetDecodeErrorKind::IncompleteSequence`], otherwise `None`.
    #[inline]
    pub const fn required(self) -> Option<usize> {
        self.kind.required()
    }

    /// Returns available input units for this decoding error, if reported.
    ///
    /// # Returns
    ///
    /// Returns `Some(available)` for
    /// [`CharsetDecodeErrorKind::IncompleteSequence`], otherwise `None`.
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

    /// Returns units that may be consumed after this invalid-input error.
    ///
    /// # Returns
    ///
    /// Returns `Some(consumed)` for malformed and invalid-code-point input, or
    /// `None` for incomplete input.
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
