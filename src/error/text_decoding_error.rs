/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use core::fmt;
use std::error::Error;

use crate::{
    TextDecodingErrorKind,
    TextEncoding,
};

/// Error reported by a text decoder.
///
/// The error always carries the encoding, error kind, and input unit index at
/// which the failure was detected. Errors that decode a raw numeric value, such
/// as invalid UTF-32 units, also carry that value through [`Self::value`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextDecodingError {
    encoding: TextEncoding,
    kind: TextDecodingErrorKind,
    index: usize,
    value: Option<u32>,
}

/// Result type returned by text decoders.
pub type TextDecodingResult<T> = Result<T, TextDecodingError>;

impl TextDecodingError {
    /// Creates a decoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The encoding being decoded.
    /// - `kind`: The failure category.
    /// - `index`: The input unit index where the failure was detected.
    ///
    /// # Returns
    ///
    /// Returns a decoding error carrying the supplied context.
    #[must_use]
    pub const fn new(encoding: TextEncoding, kind: TextDecodingErrorKind, index: usize) -> Self {
        Self {
            encoding,
            kind,
            index,
            value: None,
        }
    }

    /// Creates a decoding error with an associated raw value.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The encoding being decoded.
    /// - `kind`: The failure category.
    /// - `index`: The input unit index where the failure was detected.
    /// - `value`: The raw value associated with the failure.
    ///
    /// # Returns
    ///
    /// Returns a decoding error carrying the supplied context and value.
    #[must_use]
    pub const fn with_value(
        encoding: TextEncoding,
        kind: TextDecodingErrorKind,
        index: usize,
        value: u32,
    ) -> Self {
        Self {
            encoding,
            kind,
            index,
            value: Some(value),
        }
    }

    /// Creates a malformed-sequence decoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The encoding being decoded.
    /// - `index`: The input unit index where the malformed sequence was detected.
    ///
    /// # Returns
    ///
    /// Returns a decoding error with [`TextDecodingErrorKind::MalformedSequence`].
    #[must_use]
    pub const fn malformed_sequence(encoding: TextEncoding, index: usize) -> Self {
        Self::new(encoding, TextDecodingErrorKind::MalformedSequence, index)
    }

    /// Creates an incomplete-sequence decoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The encoding being decoded.
    /// - `index`: The input unit index where more input was required.
    ///
    /// # Returns
    ///
    /// Returns a decoding error with [`TextDecodingErrorKind::IncompleteSequence`].
    #[must_use]
    pub const fn incomplete_sequence(encoding: TextEncoding, index: usize) -> Self {
        Self::new(encoding, TextDecodingErrorKind::IncompleteSequence, index)
    }

    /// Creates an invalid-code-point decoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The encoding being decoded.
    /// - `index`: The input unit index associated with the invalid code point.
    /// - `value`: The invalid raw code point value.
    ///
    /// # Returns
    ///
    /// Returns a decoding error with [`TextDecodingErrorKind::InvalidCodePoint`].
    #[must_use]
    pub const fn invalid_code_point(encoding: TextEncoding, index: usize, value: u32) -> Self {
        Self::with_value(
            encoding,
            TextDecodingErrorKind::InvalidCodePoint,
            index,
            value,
        )
    }

    /// Returns the encoding being decoded.
    ///
    /// # Returns
    ///
    /// Returns the stored [`TextEncoding`].
    #[must_use]
    pub const fn encoding(self) -> TextEncoding {
        self.encoding
    }

    /// Returns the decoding error kind.
    ///
    /// # Returns
    ///
    /// Returns the stored [`TextDecodingErrorKind`].
    #[must_use]
    pub const fn kind(self) -> TextDecodingErrorKind {
        self.kind
    }

    /// Returns the input unit index associated with this error.
    ///
    /// # Returns
    ///
    /// Returns the index at which the error was detected.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Returns the raw value associated with this error.
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` when the decoder captured a raw value that caused
    /// the error, or `None` when the error is only tied to an input unit index.
    #[must_use]
    pub const fn value(self) -> Option<u32> {
        self.value
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
    #[must_use]
    pub const fn offset_by(self, base: usize) -> Self {
        Self {
            encoding: self.encoding,
            kind: self.kind,
            index: self.index + base,
            value: self.value,
        }
    }
}

impl fmt::Display for TextDecodingError {
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
        if let Some(value) = self.value {
            write!(
                formatter,
                "{} decoding error at index {} for value 0x{:x}: {}",
                self.encoding, self.index, value, self.kind,
            )
        } else {
            write!(
                formatter,
                "{} decoding error at index {}: {}",
                self.encoding, self.index, self.kind,
            )
        }
    }
}

impl Error for TextDecodingError {}
