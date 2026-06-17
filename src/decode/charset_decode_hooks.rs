// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::num::NonZeroUsize;

use qubit_codec::{CapacityError, DecodeAction, DecodeContext, TranscodeDecodeHooks};

use crate::{CharsetCodec, CharsetDecodeError, MalformedAction};

use super::charset_decode_policy::CharsetDecodePolicy;

/// Malformed-input policy hooks used by [`super::CharsetDecoder`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct CharsetDecodeHooks {
    /// Action used for malformed input units.
    pub(super) malformed_action: MalformedAction,
    /// Replacement character used by [`MalformedAction::Replace`].
    pub(super) replacement: char,
}

impl CharsetDecodeHooks {
    /// Creates charset decode hooks.
    ///
    /// # Parameters
    ///
    /// - `malformed_action`: Initial malformed-input policy.
    /// - `replacement`: Replacement character used by replace policy.
    ///
    /// # Returns
    ///
    /// Returns hooks carrying the supplied policy.
    #[must_use]
    #[inline(always)]
    pub(crate) const fn new(malformed_action: MalformedAction, replacement: char) -> Self {
        Self {
            malformed_action,
            replacement,
        }
    }

    /// Creates charset decode hooks from a public policy.
    #[must_use]
    #[inline(always)]
    pub(crate) const fn from_policy(policy: CharsetDecodePolicy) -> Self {
        Self::new(policy.malformed_action(), policy.replacement())
    }

    /// Returns a non-zero consumed-unit count bounded by visible input.
    ///
    /// # Parameters
    ///
    /// - `reported`: Units reported by the charset decode error.
    /// - `available`: Units visible at the malformed input boundary.
    ///
    /// # Returns
    ///
    /// Returns a non-zero consumed-unit count.
    #[must_use]
    #[inline]
    fn malformed_consumed(reported: Option<usize>, available: usize) -> NonZeroUsize {
        let consumed = reported.unwrap_or(1).min(available).max(1);
        NonZeroUsize::new(consumed).expect("malformed input consumption is non-zero")
    }
}

impl<C> TranscodeDecodeHooks<C> for CharsetDecodeHooks
where
    C: CharsetCodec,
{
    type Error = CharsetDecodeError;

    /// Returns the maximum number of characters decoded from `input_len` units.
    #[inline(always)]
    fn max_output_len(&self, _codec: &C, input_len: usize) -> Result<usize, CapacityError> {
        Ok(input_len)
    }

    /// Handles a charset decode failure during `transcode`.
    fn handle_decode_error(
        &mut self,
        _codec: &mut C,
        error: CharsetDecodeError,
        context: DecodeContext,
    ) -> Result<DecodeAction<char>, Self::Error> {
        if let Some((required, available)) = error.kind().incomplete() {
            debug_assert!(
                required > available,
                "incomplete error did not require more input"
            );
            return Ok(DecodeAction::NeedInput {
                required_total: required,
            });
        }
        if error.kind().is_malformed_input() {
            let consumed =
                CharsetDecodeHooks::malformed_consumed(error.consumed(), context.available());
            return match self.malformed_action {
                MalformedAction::Report => Err(error),
                MalformedAction::Ignore => Ok(DecodeAction::Skip { consumed }),
                MalformedAction::Replace => Ok(DecodeAction::Emit {
                    value: self.replacement,
                    consumed,
                }),
            };
        }
        Err(error)
    }

    /// Maps charset decode flush errors unchanged.
    #[inline(always)]
    fn map_decode_flush_error(&mut self, _codec: &mut C, error: CharsetDecodeError) -> Self::Error {
        error
    }
}
