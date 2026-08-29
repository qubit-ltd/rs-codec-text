// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::CapacityError;
use qubit_codec::TranscodeDecodeError;
use qubit_codec::TranscodeDecodeErrorOf;
use qubit_codec::engine::DecodeContext;
use qubit_codec::engine::DecodeIncompleteAction;
use qubit_codec::engine::DecodeInvalidAction;
use qubit_codec::engine::TranscodeDecodeHooks;

use crate::CharsetCodec;
use crate::CharsetDecodeError;
use crate::MalformedAction;

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
    #[inline]
    pub(crate) const fn new(malformed_action: MalformedAction, replacement: char) -> Self {
        Self {
            malformed_action,
            replacement,
        }
    }
}

impl<C> TranscodeDecodeHooks<C> for CharsetDecodeHooks
where
    C: CharsetCodec,
{
    /// Returns the maximum number of characters decoded from `input_len` units.
    #[inline]
    fn max_transcode_output_len(&self, _codec: &C, input_len: usize) -> Result<usize, CapacityError> {
        Ok(input_len)
    }

    /// Handles a charset decode failure during `transcode`.
    fn handle_invalid_decode(
        &mut self,
        _codec: &mut C,
        error: &CharsetDecodeError,
        _consumed: Option<core::num::NonZeroUsize>,
        context: DecodeContext,
    ) -> Result<DecodeInvalidAction<char>, TranscodeDecodeErrorOf<C>> {
        if error.kind().is_malformed_input() {
            let consumed = error.consumed().expect("malformed decode errors carry consumed width");
            return match self.malformed_action {
                MalformedAction::Report => Err(TranscodeDecodeError::domain_main(*error, context.input_index())),
                MalformedAction::Ignore => Ok(DecodeInvalidAction::Skip { consumed }),
                MalformedAction::Replace => Ok(DecodeInvalidAction::Emit {
                    value: self.replacement,
                    consumed,
                }),
            };
        }
        Err(TranscodeDecodeError::domain_main(*error, context.input_index()))
    }

    /// Selects the malformed-input action for a source tail left incomplete at
    /// end of input.
    ///
    /// Incomplete EOF tails use the same policy as malformed source input.
    /// The engine preserves the optional codec error when reporting is chosen.
    fn handle_incomplete_decode(
        &mut self,
        _codec: &mut C,
        _source: Option<&CharsetDecodeError>,
        _required_total: core::num::NonZeroUsize,
        _context: DecodeContext,
    ) -> Result<DecodeIncompleteAction<char>, TranscodeDecodeErrorOf<C>> {
        match self.malformed_action {
            MalformedAction::Report => Ok(DecodeIncompleteAction::Reject),
            MalformedAction::Ignore => Ok(DecodeIncompleteAction::Skip),
            MalformedAction::Replace => Ok(DecodeIncompleteAction::Emit {
                value: self.replacement,
            }),
        }
    }
}
