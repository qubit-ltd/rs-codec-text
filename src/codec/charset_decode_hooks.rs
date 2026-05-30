/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use qubit_codec::{
    BufferedDecodeHooks,
    DecodeAction,
    DecodeContext,
};

use crate::CharsetDecodeError;

use super::{
    charset_codec::CharsetCodec,
    malformed_action::MalformedAction,
};

/// Malformed-input policy hooks used by [`super::CharsetDecoder`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(super) struct CharsetDecodeHooks {
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
    pub(super) const fn new(malformed_action: MalformedAction, replacement: char) -> Self {
        Self {
            malformed_action,
            replacement,
        }
    }
}

impl<C> BufferedDecodeHooks<C, C::Unit, char> for CharsetDecodeHooks
where
    C: CharsetCodec,
{
    type Error = CharsetDecodeError;

    /// Returns the maximum number of characters decoded from `input_len` units.
    #[inline(always)]
    fn max_output_len(&self, _codec: &C, input_len: usize) -> Option<usize> {
        Some(input_len)
    }

    /// Handles a charset decode failure during `transcode`.
    #[inline]
    fn handle_decode_error(
        &mut self,
        _codec: &C,
        error: CharsetDecodeError,
        context: DecodeContext,
    ) -> Result<DecodeAction<char>, Self::Error> {
        if let Some((required, available)) = error.kind().incomplete() {
            debug_assert!(required > available, "incomplete error did not require more input");
            return Ok(DecodeAction::NeedInput {
                required_total: required,
            });
        }
        if error.kind().is_malformed_input() {
            let consumed = error.policy_consumed(context);
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
}
