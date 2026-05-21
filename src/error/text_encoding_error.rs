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
    TextEncoding,
    TextEncodingErrorKind,
};

/// Error reported by a text encoder.
///
/// The error always carries the target encoding, error kind, and output or
/// input index associated with the failure. Errors tied to a raw code point or
/// character value expose that value through [`Self::value`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextEncodingError {
    encoding: TextEncoding,
    kind: TextEncodingErrorKind,
    index: usize,
    value: Option<u32>,
}

/// Result type returned by text encoders.
pub type TextEncodingResult<T> = Result<T, TextEncodingError>;

impl TextEncodingError {
    /// Creates an encoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The target encoding.
    /// - `kind`: The failure category.
    /// - `index`: The output unit index or input code point index associated with the failure.
    ///
    /// # Returns
    ///
    /// Returns an encoding error carrying the supplied context.
    #[must_use]
    pub const fn new(encoding: TextEncoding, kind: TextEncodingErrorKind, index: usize) -> Self {
        Self {
            encoding,
            kind,
            index,
            value: None,
        }
    }

    /// Creates an encoding error with an associated raw value.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The target encoding.
    /// - `kind`: The failure category.
    /// - `index`: The output unit index or input code point index associated with the failure.
    /// - `value`: The raw code point or character value associated with the failure.
    ///
    /// # Returns
    ///
    /// Returns an encoding error carrying the supplied context and value.
    #[must_use]
    pub const fn with_value(
        encoding: TextEncoding,
        kind: TextEncodingErrorKind,
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

    /// Creates an invalid-code-point encoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The target encoding.
    /// - `index`: The input code point index associated with the failure.
    /// - `value`: The invalid raw code point value.
    ///
    /// # Returns
    ///
    /// Returns an encoding error with [`TextEncodingErrorKind::InvalidCodePoint`].
    #[must_use]
    pub const fn invalid_code_point(encoding: TextEncoding, index: usize, value: u32) -> Self {
        Self::with_value(
            encoding,
            TextEncodingErrorKind::InvalidCodePoint,
            index,
            value,
        )
    }

    /// Creates an unmappable-character encoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The target encoding.
    /// - `index`: The input character index associated with the failure.
    /// - `value`: The unmappable raw character value.
    ///
    /// # Returns
    ///
    /// Returns an encoding error with [`TextEncodingErrorKind::UnmappableCharacter`].
    #[must_use]
    pub const fn unmappable_character(encoding: TextEncoding, index: usize, value: u32) -> Self {
        Self::with_value(
            encoding,
            TextEncodingErrorKind::UnmappableCharacter,
            index,
            value,
        )
    }

    /// Creates a buffer-too-small encoding error.
    ///
    /// # Parameters
    ///
    /// - `encoding`: The target encoding.
    /// - `index`: The output unit index or available output length associated with the failure.
    ///
    /// # Returns
    ///
    /// Returns an encoding error with [`TextEncodingErrorKind::BufferTooSmall`].
    #[must_use]
    pub const fn buffer_too_small(encoding: TextEncoding, index: usize) -> Self {
        Self::new(encoding, TextEncodingErrorKind::BufferTooSmall, index)
    }

    /// Returns the target encoding.
    ///
    /// # Returns
    ///
    /// Returns the stored [`TextEncoding`].
    #[must_use]
    pub const fn encoding(self) -> TextEncoding {
        self.encoding
    }

    /// Returns the encoding error kind.
    ///
    /// # Returns
    ///
    /// Returns the stored [`TextEncodingErrorKind`].
    #[must_use]
    pub const fn kind(self) -> TextEncodingErrorKind {
        self.kind
    }

    /// Returns the output unit index or input code point index associated with this error.
    ///
    /// # Returns
    ///
    /// Returns the stored index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Returns the raw value associated with this error.
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` when the encoder captured a raw value that caused
    /// the error, or `None` when the error is only tied to an output index.
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

impl fmt::Display for TextEncodingError {
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
        if let Some(value) = self.value {
            write!(
                formatter,
                "{} encoding error at index {} for value 0x{:x}: {}",
                self.encoding, self.index, value, self.kind,
            )
        } else {
            write!(
                formatter,
                "{} encoding error at index {}: {}",
                self.encoding, self.index, self.kind,
            )
        }
    }
}

impl Error for TextEncodingError {}
