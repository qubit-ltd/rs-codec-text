/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use super::coder_status::CoderStatus;

/// Counts how much work a [`crate::Coder`] completed before returning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoderProgress {
    /// Stop reason reported by the coder.
    status: CoderStatus,
    /// Number of input units consumed from the requested input index.
    read: usize,
    /// Number of output units written from the requested output index.
    written: usize,
}

impl CoderProgress {
    /// Creates a progress value.
    ///
    /// # Parameters
    ///
    /// - `status`: The reason conversion stopped.
    /// - `read`: Number of input units consumed from the call's input index.
    /// - `written`: Number of output units written from the call's output index.
    ///
    /// # Returns
    ///
    /// Returns a progress value carrying the supplied counters.
    #[must_use]
    #[inline]
    pub const fn new(status: CoderStatus, read: usize, written: usize) -> Self {
        Self {
            status,
            read,
            written,
        }
    }

    /// Creates a completed progress value.
    ///
    /// # Parameters
    ///
    /// - `read`: Number of consumed input units.
    /// - `written`: Number of produced output units.
    ///
    /// # Returns
    ///
    /// Returns a progress value whose status is [`CoderStatus::Complete`].
    #[must_use]
    #[inline]
    pub const fn complete(read: usize, written: usize) -> Self {
        Self::new(CoderStatus::Complete, read, written)
    }

    /// Creates a need-input progress value.
    ///
    /// # Parameters
    ///
    /// - `read`: Number of consumed input units.
    /// - `written`: Number of produced output units.
    ///
    /// # Returns
    ///
    /// Returns a progress value whose status is [`CoderStatus::NeedInput`].
    #[must_use]
    #[inline]
    pub const fn need_input(read: usize, written: usize) -> Self {
        Self::new(CoderStatus::NeedInput, read, written)
    }

    /// Creates a need-output progress value.
    ///
    /// # Parameters
    ///
    /// - `read`: Number of consumed input units.
    /// - `written`: Number of produced output units.
    ///
    /// # Returns
    ///
    /// Returns a progress value whose status is [`CoderStatus::NeedOutput`].
    #[must_use]
    #[inline]
    pub const fn need_output(read: usize, written: usize) -> Self {
        Self::new(CoderStatus::NeedOutput, read, written)
    }

    /// Returns the status that stopped conversion.
    ///
    /// # Returns
    ///
    /// Returns the stored [`CoderStatus`].
    #[must_use]
    #[inline]
    pub const fn status(self) -> CoderStatus {
        self.status
    }

    /// Returns the number of input units consumed by the call.
    ///
    /// # Returns
    ///
    /// Returns a count relative to the input index passed to the conversion call.
    #[must_use]
    #[inline]
    pub const fn read(self) -> usize {
        self.read
    }

    /// Returns the number of output units written by the call.
    ///
    /// # Returns
    ///
    /// Returns a count relative to the output index passed to the conversion call.
    #[must_use]
    #[inline]
    pub const fn written(self) -> usize {
        self.written
    }
}
