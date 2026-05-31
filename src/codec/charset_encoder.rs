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

use qubit_codec::{
    BufferedEncodeEngine,
    BufferedEncoder,
    CapacityError,
    EncodeErrorFactory,
    TranscodeProgress,
    Transcoder,
};

use crate::{
    CharsetEncodeError,
    CharsetEncodeErrorKind,
};

use super::{
    charset_codec::CharsetCodec,
    charset_encode_hooks::{
        CharsetEncodeHooks,
        encode_replacement,
    },
    charset_encode_policy::CharsetEncodePolicy,
    charset_encode_probe::CharsetEncodeProbe,
    unmappable_action::UnmappableAction,
};

impl<C> EncodeErrorFactory<C> for CharsetEncodeError
where
    C: CharsetCodec,
{
    /// Creates an input-index error using the charset from `codec`.
    #[inline(always)]
    fn invalid_input_index(codec: &C, index: usize, input_len: usize) -> Self {
        let kind = CharsetEncodeErrorKind::InvalidInputIndex { input_len };
        Self::new(codec.charset(), kind, index)
    }
}

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
#[derive(Clone)]
pub struct CharsetEncoder<C>
where
    C: CharsetEncodeProbe,
{
    /// Common buffered encode engine.
    engine: BufferedEncodeEngine<C, CharsetEncodeHooks<C::Unit>>,
    /// Public unmappable-input policy metadata.
    policy: CharsetEncodePolicy,
    /// Number of cached units used by replacement policy.
    replacement_units_len: usize,
}

impl<C> CharsetEncoder<C>
where
    C: CharsetEncodeProbe,
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
    /// encoded by the codec, [`CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT`]
    /// is used.
    ///
    /// # Panics
    ///
    /// Panics when neither [`CharsetEncodePolicy::DEFAULT_REPLACEMENT`] nor
    /// [`CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT`] can be encoded by
    /// `codec`.
    /// Built-in codecs can always encode the fallback `?`; failure here means
    /// the supplied codec cannot encode a minimal ASCII replacement. For custom
    /// [`crate::CharsetCodec`] implementations, this indicates a broken codec
    /// invariant rather than recoverable input data.
    #[must_use]
    pub fn new(codec: C) -> Self {
        let policy = CharsetEncodePolicy::default();
        match Self::create_hooks(&codec, policy) {
            Ok((hooks, replacement_units_len)) => Self {
                engine: BufferedEncodeEngine::new(codec, hooks),
                policy,
                replacement_units_len,
            },
            Err(default_error) => {
                let fallback_policy = CharsetEncodePolicy::replace(CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT);
                match Self::create_hooks(&codec, fallback_policy) {
                    Ok((hooks, replacement_units_len)) => Self {
                        engine: BufferedEncodeEngine::new(codec, hooks),
                        policy: fallback_policy,
                        replacement_units_len,
                    },
                    Err(_) => panic!(
                        "cannot initialize CharsetEncoder for {:?}: neither {:?} nor {:?} is encodable ({default_error})",
                        codec.charset(),
                        CharsetEncodePolicy::DEFAULT_REPLACEMENT,
                        CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT,
                    ),
                }
            }
        }
    }

    /// Creates an encoder with an explicit unmappable-input policy.
    ///
    /// # Errors
    ///
    /// Returns an error when `policy` uses replacement and the replacement
    /// character cannot be encoded by `codec`.
    #[inline]
    pub fn with_policy(codec: C, policy: CharsetEncodePolicy) -> Result<Self, CharsetEncodeError> {
        let (hooks, replacement_units_len) = Self::create_hooks(&codec, policy)?;
        Ok(Self {
            engine: BufferedEncodeEngine::new(codec, hooks),
            policy,
            replacement_units_len,
        })
    }

    /// Returns the configured unmappable-character action.
    ///
    /// # Returns
    ///
    /// Returns the action used when target encoding cannot represent a character.
    #[must_use]
    #[inline(always)]
    pub const fn unmappable_action(&self) -> UnmappableAction {
        self.policy.unmappable_action()
    }

    /// Returns the configured replacement character.
    ///
    /// # Returns
    ///
    /// Returns the character encoded when [`UnmappableAction::Replace`] is used.
    #[must_use]
    #[inline(always)]
    pub const fn replacement(&self) -> char {
        self.policy.replacement()
    }

    /// Creates encode hooks for `policy`.
    #[inline]
    pub(super) fn create_hooks(
        codec: &C,
        policy: CharsetEncodePolicy,
    ) -> Result<(CharsetEncodeHooks<C::Unit>, usize), CharsetEncodeError> {
        let mut hooks = CharsetEncodeHooks::new(policy.unmappable_action(), policy.replacement());
        if policy.unmappable_action() != UnmappableAction::Replace {
            return Ok((hooks, 0));
        }
        let replacement_units = encode_replacement(codec, policy.replacement())?;
        let replacement_units_len = replacement_units.len();
        hooks.replacement_units = replacement_units;
        Ok((hooks, replacement_units_len))
    }
}

impl<C> Transcoder<char, C::Unit> for CharsetEncoder<C>
where
    C: CharsetEncodeProbe,
{
    type Error = CharsetEncodeError;

    /// Returns the maximum number of target units needed for `input_len` characters.
    #[inline(always)]
    fn max_output_len(&self, input_len: usize) -> Result<usize, CapacityError> {
        self.engine.max_output_len::<char, C::Unit>(input_len)
    }

    /// Returns the maximum target units emitted by finishing internal state.
    #[inline(always)]
    fn max_finish_output_len(&self) -> Result<usize, CapacityError> {
        Ok(self.engine.max_finish_output_len::<char, C::Unit>())
    }

    /// Clears hook-owned state while keeping encoder policy.
    #[inline(always)]
    fn reset(&mut self) {
        self.engine.reset::<char, C::Unit>();
    }

    /// Encodes characters into the target charset while applying unmappable policy.
    #[inline(always)]
    fn transcode(
        &mut self,
        input: &[char],
        input_index: usize,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, Self::Error> {
        self.engine
            .transcode::<char, C::Unit>(input, input_index, output, output_index)
    }

    /// Finishes encoder-owned final output after EOF.
    #[inline(always)]
    fn finish(&mut self, output: &mut [C::Unit], output_index: usize) -> Result<TranscodeProgress, Self::Error> {
        self.engine.finish::<char, C::Unit>(output, output_index)
    }
}

impl<C> BufferedEncoder<char, C::Unit> for CharsetEncoder<C> where C: CharsetEncodeProbe {}

impl<C> Eq for CharsetEncoder<C> where C: CharsetEncodeProbe + Eq {}

impl<C> PartialEq for CharsetEncoder<C>
where
    C: CharsetEncodeProbe + PartialEq,
{
    /// Compares encoder configuration without leaking cached-unit trait bounds.
    fn eq(&self, other: &Self) -> bool {
        self.engine == other.engine && self.policy == other.policy
    }
}

impl<C> fmt::Debug for CharsetEncoder<C>
where
    C: CharsetEncodeProbe + fmt::Debug,
{
    /// Formats the encoder without exposing additional bounds for cached units.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharsetEncoder")
            .field("engine", &self.engine)
            .field("unmappable_action", &self.unmappable_action())
            .field("replacement", &self.replacement())
            .field("replacement_units_len", &self.replacement_units_len)
            .finish()
    }
}
