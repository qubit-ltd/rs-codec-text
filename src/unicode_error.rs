/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use std::error::Error;
use std::fmt;

use crate::UnicodeErrorKind;

/// Result type used by low-level Unicode cursor and encoder operations.
pub type UnicodeResult<T> = Result<T, UnicodeError>;

/// Error reported by low-level Unicode operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnicodeError {
    kind: UnicodeErrorKind,
    index: usize,
}

impl UnicodeError {
    /// Creates a new Unicode error.
    ///
    /// # Parameters
    ///
    /// - `kind`: The kind of error that was detected.
    /// - `index`: The input or output index at which the error was detected.
    ///
    /// # Returns
    ///
    /// Returns a new [`UnicodeError`] containing `kind` and `index`.
    #[inline]
    #[must_use]
    pub const fn new(kind: UnicodeErrorKind, index: usize) -> Self {
        Self { kind, index }
    }

    /// Returns the error kind.
    ///
    /// # Returns
    ///
    /// Returns the stored [`UnicodeErrorKind`].
    #[inline]
    #[must_use]
    pub const fn kind(self) -> UnicodeErrorKind {
        self.kind
    }

    /// Returns the index at which the error was detected.
    ///
    /// # Returns
    ///
    /// Returns the input or output index associated with this error.
    #[inline]
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }
}

impl fmt::Display for UnicodeError {
    /// Formats this error for diagnostics.
    ///
    /// # Parameters
    ///
    /// - `formatter`: The formatter to write the diagnostic text into.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the diagnostic text is written successfully.
    ///
    /// # Errors
    ///
    /// Returns any formatting error reported by `formatter`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} at index {}", self.kind, self.index)
    }
}

impl Error for UnicodeError {}
