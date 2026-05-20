/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use crate::UnicodeErrorKind;

/// Mutable cursor used by low-level UTF-8 and UTF-16 operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsingPosition {
    index: usize,
    error_index: Option<usize>,
    error_kind: Option<UnicodeErrorKind>,
}

impl ParsingPosition {
    /// Creates a cursor at the specified code-unit index.
    ///
    /// # Parameters
    ///
    /// - `index`: The initial cursor index.
    ///
    /// # Returns
    ///
    /// Returns a cursor with no recorded error.
    #[inline]
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self {
            index,
            error_index: None,
            error_kind: None,
        }
    }

    /// Returns the current code-unit index.
    ///
    /// # Returns
    ///
    /// Returns the current cursor index.
    #[inline]
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Sets the current code-unit index and clears any previous error.
    ///
    /// # Parameters
    ///
    /// - `index`: The new cursor index.
    #[inline]
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
        self.clear_error();
    }

    /// Increases the current code-unit index by one and clears any previous error.
    #[inline]
    pub fn increase(&mut self) {
        self.increase_by(1);
    }

    /// Increases the current code-unit index by the specified amount.
    ///
    /// # Parameters
    ///
    /// - `amount`: The number of code units to add to the current index.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if adding `amount` overflows `usize`.
    #[inline]
    pub fn increase_by(&mut self, amount: usize) {
        self.index += amount;
        self.clear_error();
    }

    /// Decreases the current code-unit index by one and clears any previous error.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the current index is zero.
    #[inline]
    pub fn decrease(&mut self) {
        self.decrease_by(1);
    }

    /// Decreases the current code-unit index by the specified amount.
    ///
    /// # Parameters
    ///
    /// - `amount`: The number of code units to subtract from the current index.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if `amount` is greater than the current index.
    #[inline]
    pub fn decrease_by(&mut self, amount: usize) {
        self.index -= amount;
        self.clear_error();
    }

    /// Returns the index at which the last error was detected.
    ///
    /// # Returns
    ///
    /// Returns `Some(index)` when an error is recorded. Returns `None` when the
    /// cursor is currently successful.
    #[inline]
    #[must_use]
    pub const fn error_index(&self) -> Option<usize> {
        self.error_index
    }

    /// Returns the kind of the last error.
    ///
    /// # Returns
    ///
    /// Returns `Some(kind)` when an error is recorded. Returns `None` when the
    /// cursor is currently successful.
    #[inline]
    #[must_use]
    pub const fn error_kind(&self) -> Option<UnicodeErrorKind> {
        self.error_kind
    }

    /// Returns `true` when the cursor does not currently hold an error.
    ///
    /// # Returns
    ///
    /// Returns `true` if no error kind is recorded.
    #[inline]
    #[must_use]
    pub const fn success(&self) -> bool {
        self.error_kind.is_none()
    }

    /// Returns `true` when the cursor currently holds an error.
    ///
    /// # Returns
    ///
    /// Returns `true` if an error kind is recorded.
    #[inline]
    #[must_use]
    pub const fn fail(&self) -> bool {
        self.error_kind.is_some()
    }

    /// Clears the current error state without changing the current index.
    #[inline]
    pub fn clear_error(&mut self) {
        self.error_index = None;
        self.error_kind = None;
    }

    /// Resets the cursor to the specified index and clears any previous error.
    ///
    /// # Parameters
    ///
    /// - `index`: The new cursor index.
    #[inline]
    pub fn reset(&mut self, index: usize) {
        self.index = index;
        self.clear_error();
    }

    /// Records an error at the specified index without moving the cursor.
    ///
    /// # Parameters
    ///
    /// - `index`: The input or output index at which the error was detected.
    /// - `kind`: The kind of Unicode error to record.
    #[inline]
    pub(crate) fn set_error(&mut self, index: usize, kind: UnicodeErrorKind) {
        self.error_index = Some(index);
        self.error_kind = Some(kind);
    }
}

impl Default for ParsingPosition {
    /// Creates a cursor at index zero.
    ///
    /// # Returns
    ///
    /// Returns a cursor with index zero and no recorded error.
    #[inline]
    fn default() -> Self {
        Self::new(0)
    }
}
