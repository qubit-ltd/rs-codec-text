/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use super::coder_progress::CoderProgress;

/// Converts one sequence of code units into another sequence of code units.
///
/// The trait is intentionally independent from charset concepts. Implementors
/// use `input_index` and `output_index` as absolute positions in the supplied
/// slices. Returned progress counters are relative counts from those positions.
///
/// # Type Parameters
///
/// - `Input`: Input unit type accepted by this coder.
/// - `Output`: Output unit type produced by this coder.
pub trait Coder<Input, Output> {
    /// Error reported for semantic conversion failures.
    type Error;

    /// Returns an upper bound for output units produced from `input_len` units.
    ///
    /// # Parameters
    ///
    /// - `input_len`: Number of input units the caller plans to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some(bound)` when the coder can provide a finite upper bound.
    /// Returns `None` when the bound is not known.
    #[must_use]
    fn max_output_len(&self, input_len: usize) -> Option<usize>;

    /// Resets state retained between conversion calls.
    ///
    /// Stateless coders may keep the default no-op implementation.
    #[inline]
    fn reset(&mut self) {}

    /// Converts input units into output units.
    ///
    /// # Parameters
    ///
    /// - `input`: Complete input unit slice visible to the coder.
    /// - `input_index`: Absolute input unit index where conversion starts.
    /// - `output`: Complete output unit slice visible to the coder.
    /// - `output_index`: Absolute output unit index where writing starts.
    ///
    /// # Returns
    ///
    /// Returns progress describing how many units were consumed and produced and
    /// why conversion stopped.
    ///
    /// # Errors
    ///
    /// Returns `Self::Error` for semantic conversion failures that the coder's
    /// policy does not absorb.
    fn convert(
        &mut self,
        input: &[Input],
        input_index: usize,
        output: &mut [Output],
        output_index: usize,
    ) -> Result<CoderProgress, Self::Error>;

    /// Flushes any buffered output after input conversion is complete.
    ///
    /// # Parameters
    ///
    /// - `output`: Complete output unit slice visible to the coder.
    /// - `output_index`: Absolute output unit index where writing starts.
    ///
    /// # Returns
    ///
    /// Returns progress for units written during flushing. Stateless coders
    /// return a completed progress value with zero counters.
    ///
    /// # Errors
    ///
    /// Returns `Self::Error` if pending state cannot be flushed according to the
    /// coder's policy.
    #[inline]
    fn finish(
        &mut self,
        _output: &mut [Output],
        _output_index: usize,
    ) -> Result<CoderProgress, Self::Error> {
        Ok(CoderProgress::complete(0, 0))
    }
}
