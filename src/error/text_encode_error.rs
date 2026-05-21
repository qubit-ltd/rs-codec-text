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
    Charset,
    TextEncodeErrorKind,
};

/// Error reported by a text encoder.
///
/// The error always carries the target charset, error kind, and output or
/// input index associated with the failure. Errors tied to a raw code point or
/// character value expose that value through [`Self::value`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextEncodeError {
    charset: Charset,
    kind: TextEncodeErrorKind,
    index: usize,
    value: Option<u32>,
}

/// Result type returned by text encoders.
pub type TextEncodeResult<T> = Result<T, TextEncodeError>;

impl TextEncodeError {
    /// Creates an encoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `kind`: The failure category.
    /// - `index`: The output unit index or input code point index associated with the failure.
    ///
    /// # Returns
    ///
    /// Returns an encoding error carrying the supplied context.
    #[must_use]
    pub const fn new(charset: Charset, kind: TextEncodeErrorKind, index: usize) -> Self {
        Self {
            charset,
            kind,
            index,
            value: None,
        }
    }

    /// Creates an encoding error with an associated raw value.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `kind`: The failure category.
    /// - `index`: The output unit index or input code point index associated with the failure.
    /// - `value`: The raw code point or character value associated with the failure.
    ///
    /// # Returns
    ///
    /// Returns an encoding error carrying the supplied context and value.
    #[must_use]
    pub const fn with_value(
        charset: Charset,
        kind: TextEncodeErrorKind,
        index: usize,
        value: u32,
    ) -> Self {
        Self {
            charset,
            kind,
            index,
            value: Some(value),
        }
    }

    /// Creates an invalid-code-point encoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `index`: The input code point index associated with the failure.
    /// - `value`: The invalid raw code point value.
    ///
    /// # Returns
    ///
    /// Returns an encoding error with [`TextEncodeErrorKind::InvalidCodePoint`].
    #[must_use]
    pub const fn invalid_code_point(charset: Charset, index: usize, value: u32) -> Self {
        Self::with_value(charset, TextEncodeErrorKind::InvalidCodePoint, index, value)
    }

    /// Creates an unmappable-character encoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `index`: The input character index associated with the failure.
    /// - `value`: The unmappable raw character value.
    ///
    /// # Returns
    ///
    /// Returns an encoding error with [`TextEncodeErrorKind::UnmappableCharacter`].
    #[must_use]
    pub const fn unmappable_character(charset: Charset, index: usize, value: u32) -> Self {
        Self::with_value(
            charset,
            TextEncodeErrorKind::UnmappableCharacter,
            index,
            value,
        )
    }

    /// Creates a buffer-too-small encoding error.
    ///
    /// # Parameters
    ///
    /// - `charset`: The target charset.
    /// - `index`: The output unit index or available output length associated with the failure.
    ///
    /// # Returns
    ///
    /// Returns an encoding error with [`TextEncodeErrorKind::BufferTooSmall`].
    #[must_use]
    pub const fn buffer_too_small(charset: Charset, index: usize) -> Self {
        Self::new(charset, TextEncodeErrorKind::BufferTooSmall, index)
    }

    /// Returns the target charset.
    ///
    /// # Returns
    ///
    /// Returns the stored [`Charset`].
    #[must_use]
    pub const fn charset(self) -> Charset {
        self.charset
    }

    /// Returns the encoding error kind.
    ///
    /// # Returns
    ///
    /// Returns the stored [`TextEncodeErrorKind`].
    #[must_use]
    pub const fn kind(self) -> TextEncodeErrorKind {
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
            charset: self.charset,
            kind: self.kind,
            index: self.index + base,
            value: self.value,
        }
    }
}

impl fmt::Display for TextEncodeError {
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

impl Error for TextEncodeError {}
