// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::engine::TranscodeEncodeEngine;
use qubit_codec::{
    CapacityError,
    TranscodeEncoder,
    TranscodeError,
    TranscodeProgress,
    Transcoder,
};

use crate::{
    CharsetCodec,
    CharsetEncodeError,
    UnmappableAction,
};

use super::{
    charset_encode_hooks::{
        CharsetEncodeHooks,
        replacement_len,
    },
    charset_encode_policy::CharsetEncodePolicy,
};

/// Converts Unicode scalar values into units of one charset.
///
/// `CharsetEncoder` wraps a low-level [`crate::CharsetCodec`] and applies the
/// configured [`UnmappableAction`] whenever the codec reports that an input
/// character cannot be represented by the target charset.
///
/// # Type Parameters
///
/// - `C`: Low-level charset codec used to encode one character into target
///   storage units.
#[derive(Debug)]
pub struct CharsetEncoder<C>
where
    C: CharsetCodec,
{
    /// Common buffered encode engine.
    engine: TranscodeEncodeEngine<C, CharsetEncodeHooks>,
    /// Public unmappable-input policy metadata.
    policy: CharsetEncodePolicy,
}

impl<C> CharsetEncoder<C>
where
    C: CharsetCodec,
{
    /// Creates an encoder with default replacement policy.
    ///
    /// # Parameters
    ///
    /// - `codec`: Low-level charset codec used to encode output units.
    ///
    /// # Returns
    ///
    /// Returns an encoder whose unmappable action is
    /// [`UnmappableAction::Replace`] and whose replacement character is
    /// [`CharsetEncodePolicy::DEFAULT_REPLACEMENT`]. If the default cannot be
    /// encoded by the codec,
    /// [`CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT`] is used.
    ///
    /// # Panics
    ///
    /// Panics when neither [`CharsetEncodePolicy::DEFAULT_REPLACEMENT`] nor
    /// [`CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT`] can be encoded by
    /// `codec`. This panic is intentional by design: reaching this branch means
    /// the supplied codec implementation is wrong, because the API requires a
    /// default replacement fallback that the codec can encode. Built-in codecs
    /// can always encode the fallback `?`; custom [`crate::CharsetCodec`]
    /// implementations that cannot encode it must fail fast during
    /// construction rather than defer the invariant violation to user input.
    #[must_use]
    pub fn new(codec: C) -> Self {
        let policy = CharsetEncodePolicy::default_for(&codec).unwrap_or_else(|error| {
            // This panic is intentional. If default replacement selection gets
            // here, the codec cannot encode even the required fallback `?`.
            // That violates the codec invariant expected by this API, so
            // construction fails fast to expose the broken codec implementation.
            panic!(
                "cannot initialize CharsetEncoder for {:?}: neither {:?} nor {:?} is encodable ({error})",
                codec.charset(),
                CharsetEncodePolicy::DEFAULT_REPLACEMENT,
                CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT,
            )
        });
        let hooks = Self::create_hooks(&codec, policy)
            // A policy chosen by `default_for` must be encodable; failing here
            // can only mean the codec violates the replacement fallback
            // invariant. This panic is deliberate for the same fail-fast reason
            // as the default-policy panic above.
            .expect(
                "validated default encode policy should create hooks; \
                 failure means the codec violated its replacement invariant",
            );
        Self {
            engine: TranscodeEncodeEngine::new(codec, hooks),
            policy,
        }
    }

    /// Creates an encoder with an explicit unmappable-input policy.
    ///
    /// # Errors
    ///
    /// Returns an error when `policy` uses replacement and the replacement
    /// character cannot be encoded by `codec`.
    pub fn with_policy(
        codec: C,
        policy: CharsetEncodePolicy,
    ) -> Result<Self, CharsetEncodeError> {
        let hooks = Self::create_hooks(&codec, policy)?;
        Ok(Self {
            engine: TranscodeEncodeEngine::new(codec, hooks),
            policy,
        })
    }

    /// Returns the configured unmappable-character action.
    ///
    /// # Returns
    ///
    /// Returns the action used when target encoding cannot represent a
    /// character.
    #[inline(always)]
    #[must_use]
    pub const fn unmappable_action(&self) -> UnmappableAction {
        self.policy.unmappable_action()
    }

    /// Returns the configured replacement character.
    ///
    /// # Returns
    ///
    /// Returns the character encoded when [`UnmappableAction::Replace`] is
    /// used.
    #[inline(always)]
    #[must_use]
    pub const fn replacement(&self) -> char {
        self.policy.replacement()
    }

    /// Returns the charset encoded by the wrapped codec.
    ///
    /// # Returns
    ///
    /// Returns the charset reported by the low-level codec.
    #[inline(always)]
    #[must_use]
    pub fn charset(&self) -> crate::Charset {
        self.codec().charset()
    }

    /// Returns the wrapped low-level codec.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the codec owned by this encoder.
    #[inline(always)]
    #[must_use]
    pub fn codec(&self) -> &C {
        self.engine.codec()
    }

    /// Returns the wrapped low-level codec mutably.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the codec owned by this encoder.
    #[inline(always)]
    #[must_use]
    pub fn codec_mut(&mut self) -> &mut C {
        self.engine.codec_mut()
    }

    /// Consumes the encoder and returns its codec.
    ///
    /// Encoder policy and hook state are discarded.
    ///
    /// # Returns
    ///
    /// Returns the low-level codec owned by this encoder.
    #[inline(always)]
    #[must_use]
    pub fn into_codec(self) -> C {
        let (codec, _) = self.engine.into_parts();
        codec
    }

    /// Runs encoder reset.
    ///
    /// # Errors
    ///
    /// Returns [`TranscodeError`] when `output_index` is invalid, output
    /// capacity is insufficient, or encoder reset emits a charset-domain
    /// error.
    #[inline]
    pub fn reset(
        &mut self,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<usize, TranscodeError<CharsetEncodeError, char>> {
        self.engine.reset(output, output_index)
    }

    /// Encodes characters into target units.
    ///
    /// # Errors
    ///
    /// Returns [`TranscodeError`] when indices are invalid, output
    /// capacity is insufficient, a character is unmappable under the
    /// configured policy, or the codec reports another encode-domain error.
    #[inline]
    pub fn transcode(
        &mut self,
        input: &[char],
        input_index: usize,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, TranscodeError<CharsetEncodeError, char>>
    {
        self.engine
            .transcode(input, input_index, output, output_index)
    }

    /// Finishes encoder-owned final output after EOF.
    ///
    /// # Errors
    ///
    /// Returns [`TranscodeError`] when finalization output cannot be
    /// written or when the codec reports a final encode-domain error.
    #[inline]
    pub fn finish(
        &mut self,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<usize, TranscodeError<CharsetEncodeError, char>> {
        self.engine.finish(output, output_index)
    }

    /// Runs a complete `reset -> transcode -> finish` encode stream.
    ///
    /// # Errors
    ///
    /// Returns [`TranscodeError`] when the supplied output buffer is too
    /// small, a character cannot be represented under the configured policy,
    /// or the codec reports a charset-domain encode error.
    #[inline]
    pub fn transcode_complete_into(
        &mut self,
        input: &[char],
        output: &mut [C::Unit],
    ) -> Result<usize, TranscodeError<CharsetEncodeError, char>> {
        <Self as Transcoder>::transcode_complete_into(self, input, output)
    }

    /// Creates encode hooks for `policy`.
    pub(crate) fn create_hooks(
        codec: &C,
        policy: CharsetEncodePolicy,
    ) -> Result<CharsetEncodeHooks, CharsetEncodeError> {
        let hooks = CharsetEncodeHooks::new(
            policy.unmappable_action(),
            policy.replacement(),
        );
        if policy.unmappable_action() != UnmappableAction::Replace {
            return Ok(hooks);
        }
        replacement_len(codec, policy.replacement())?;
        Ok(hooks)
    }
}

impl<C> Transcoder for CharsetEncoder<C>
where
    C: CharsetCodec,
{
    type Input = char;
    type Output = C::Unit;
    type DomainError = CharsetEncodeError;
    type FailureValue = char;

    /// Returns the maximum number of target units needed for `input_len`
    /// characters.
    #[inline(always)]
    fn max_transcode_output_len(
        &self,
        input_len: usize,
    ) -> Result<usize, CapacityError> {
        self.engine.max_transcode_output_len(input_len)
    }

    /// Returns the maximum target units emitted by finishing internal state.
    #[inline(always)]
    fn max_finish_output_len(&self) -> Result<usize, CapacityError> {
        self.engine.max_finish_output_len()
    }

    /// Returns the maximum target units emitted when resetting stream state.
    #[inline(always)]
    fn max_reset_output_len(&self) -> Result<usize, CapacityError> {
        self.engine.max_reset_output_len()
    }

    /// Runs encoder reset while keeping encoder policy.
    #[inline(always)]
    fn reset(
        &mut self,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<usize, TranscodeError<Self::DomainError, Self::FailureValue>>
    {
        self.engine.reset(output, output_index)
    }

    /// Encodes characters into the target charset while applying unmappable
    /// policy.
    #[inline(always)]
    fn transcode(
        &mut self,
        input: &[char],
        input_index: usize,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<
        TranscodeProgress,
        TranscodeError<Self::DomainError, Self::FailureValue>,
    > {
        self.engine
            .transcode(input, input_index, output, output_index)
    }

    /// Finishes encoder-owned final output after EOF.
    #[inline(always)]
    fn finish(
        &mut self,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<usize, TranscodeError<Self::DomainError, Self::FailureValue>>
    {
        self.engine.finish(output, output_index)
    }
}

impl<C> TranscodeEncoder for CharsetEncoder<C>
where
    C: CharsetCodec,
{
    //  empty
}
