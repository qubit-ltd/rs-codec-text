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
    BufferedDecodeEngine,
    BufferedDecoder,
    CapacityError,
    DecodeErrorFactory,
    TranscodeProgress,
    Transcoder,
};

use crate::{
    CharsetDecodeError,
    CharsetDecodeErrorKind,
};

use super::{
    charset_codec::CharsetCodec,
    charset_decode_hooks::CharsetDecodeHooks,
    charset_decode_policy::CharsetDecodePolicy,
    malformed_action::MalformedAction,
};

/// Converts units of one charset into Unicode scalar values.
///
/// `CharsetDecoder` wraps a low-level [`CharsetCodec`] and applies the
/// configured [`MalformedAction`] whenever the codec reports malformed input.
/// The decoder asks the wrapped codec whether one value can be decoded from the
/// currently available units. If the codec reports a valid incomplete prefix,
/// the tail is left in the caller-provided input slice and
/// [`crate::TranscodeStatus::NeedInput`] is returned. Callers must handle
/// incomplete EOF tails before calling [`Transcoder::finish`].
///
/// # Type Parameters
///
/// - `C`: Low-level charset codec used to decode source storage units into one
///   Unicode scalar value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharsetDecoder<C>
where
    C: CharsetCodec,
{
    /// Common buffered decode engine.
    engine: BufferedDecodeEngine<C, CharsetDecodeHooks, C::Unit>,
    /// Public malformed-input policy metadata.
    policy: CharsetDecodePolicy,
}

impl<C> DecodeErrorFactory<C> for CharsetDecodeError
where
    C: CharsetCodec,
{
    /// Creates an input-index error using the charset from `codec`.
    #[inline(always)]
    fn invalid_input_index(codec: &C, index: usize, input_len: usize) -> Self {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len };
        Self::new(codec.charset(), kind, index)
    }
}

impl<C> CharsetDecoder<C>
where
    C: CharsetCodec,
{
    /// Creates a decoder with default replacement policy.
    ///
    /// # Parameters
    ///
    /// - `codec`: Low-level charset codec used to decode input units.
    ///
    /// # Returns
    ///
    /// Returns a decoder whose malformed action is [`MalformedAction::Replace`]
    /// and whose replacement character is `U+FFFD`.
    #[must_use]
    #[inline(always)]
    pub fn new(codec: C) -> Self {
        Self::with_policy(codec, CharsetDecodePolicy::default())
    }

    /// Creates a decoder with an explicit malformed-input policy.
    ///
    /// # Parameters
    ///
    /// - `codec`: Low-level charset codec used to decode input units.
    /// - `policy`: Malformed-input policy used by the decoder.
    ///
    /// # Returns
    ///
    /// Returns a decoder configured with `policy`.
    #[must_use]
    #[inline(always)]
    pub fn with_policy(codec: C, policy: CharsetDecodePolicy) -> Self {
        let hooks = CharsetDecodeHooks::from_policy(policy);
        Self {
            engine: BufferedDecodeEngine::new(codec, hooks),
            policy,
        }
    }

    /// Returns the configured malformed-input action.
    ///
    /// # Returns
    ///
    /// Returns the action used when source input is malformed.
    #[must_use]
    #[inline(always)]
    pub const fn malformed_action(&self) -> MalformedAction {
        self.policy.malformed_action()
    }

    /// Returns the configured replacement character.
    ///
    /// # Returns
    ///
    /// Returns the character emitted when [`MalformedAction::Replace`] is used.
    #[must_use]
    #[inline(always)]
    pub const fn replacement(&self) -> char {
        self.policy.replacement()
    }
}

impl<C> BufferedDecoder<C::Unit, char> for CharsetDecoder<C> where C: CharsetCodec {}

impl<C> Transcoder<C::Unit, char> for CharsetDecoder<C>
where
    C: CharsetCodec,
{
    type Error = CharsetDecodeError;

    /// Returns the maximum number of characters decoded from `input_len` units.
    #[inline(always)]
    fn max_output_len(&self, input_len: usize) -> Result<usize, CapacityError> {
        self.engine.max_output_len::<char>(input_len)
    }

    /// Returns the maximum number of characters emitted by finishing internal state.
    #[inline(always)]
    fn max_finish_output_len(&self) -> Result<usize, CapacityError> {
        Ok(self.engine.max_finish_output_len::<char>())
    }

    /// Clears hook-owned state while keeping decoder policy.
    #[inline(always)]
    fn reset(&mut self) {
        self.engine.reset::<char>();
    }

    /// Decodes source units into Unicode scalar values while applying malformed policy.
    #[inline(always)]
    fn transcode(
        &mut self,
        input: &[C::Unit],
        input_index: usize,
        output: &mut [char],
        output_index: usize,
    ) -> Result<TranscodeProgress, Self::Error> {
        self.engine.transcode::<char>(input, input_index, output, output_index)
    }

    /// Finishes decoder-owned final output after EOF.
    #[inline(always)]
    fn finish(&mut self, output: &mut [char], output_index: usize) -> Result<TranscodeProgress, Self::Error> {
        self.engine.finish::<char>(output, output_index)
    }
}
