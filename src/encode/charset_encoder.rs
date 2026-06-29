// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::{
    fmt,
    num::NonZeroUsize,
};

use qubit_codec::{
    CapacityError,
    TranscodeEncodeEngine,
    TranscodeEncodeEngineError,
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
pub struct CharsetEncoder<C>
where
    C: CharsetCodec,
{
    /// Common buffered encode engine.
    engine: TranscodeEncodeEngine<C, CharsetEncodeHooks<C::Unit>>,
    /// Public unmappable-input policy metadata.
    policy: CharsetEncodePolicy,
    /// Number of units used by replacement policy.
    replacement_units_len: Option<NonZeroUsize>,
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
        let (hooks, replacement_units_len) = Self::create_hooks(&codec, policy)
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
            replacement_units_len,
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
        let (hooks, replacement_units_len) =
            Self::create_hooks(&codec, policy)?;
        Ok(Self {
            engine: TranscodeEncodeEngine::new(codec, hooks),
            policy,
            replacement_units_len,
        })
    }

    /// Returns the configured unmappable-character action.
    ///
    /// # Returns
    ///
    /// Returns the action used when target encoding cannot represent a
    /// character.
    #[must_use]
    #[inline]
    pub const fn unmappable_action(&self) -> UnmappableAction {
        self.policy.unmappable_action()
    }

    /// Returns the configured replacement character.
    ///
    /// # Returns
    ///
    /// Returns the character encoded when [`UnmappableAction::Replace`] is
    /// used.
    #[must_use]
    #[inline]
    pub const fn replacement(&self) -> char {
        self.policy.replacement()
    }

    /// Returns the charset encoded by the wrapped codec.
    ///
    /// # Returns
    ///
    /// Returns the charset reported by the low-level codec.
    #[must_use]
    #[inline]
    pub fn charset(&self) -> crate::Charset {
        self.codec().charset()
    }

    /// Returns the wrapped low-level codec.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the codec owned by this encoder.
    #[must_use]
    #[inline]
    pub fn codec(&self) -> &C {
        self.engine.codec()
    }

    /// Returns the wrapped low-level codec mutably.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the codec owned by this encoder.
    #[must_use]
    #[inline]
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
    #[must_use]
    #[inline]
    pub fn into_codec(self) -> C {
        let (codec, _) = self.engine.into_parts();
        codec
    }

    /// Creates encode hooks for `policy`.
    pub(crate) fn create_hooks(
        codec: &C,
        policy: CharsetEncodePolicy,
    ) -> Result<
        (CharsetEncodeHooks<C::Unit>, Option<NonZeroUsize>),
        CharsetEncodeError,
    > {
        let hooks = CharsetEncodeHooks::new(
            policy.unmappable_action(),
            policy.replacement(),
        );
        if policy.unmappable_action() != UnmappableAction::Replace {
            return Ok((hooks, None));
        }
        let replacement_units_len =
            replacement_len(codec, policy.replacement())?;
        Ok((hooks, Some(replacement_units_len)))
    }
}

impl<C> Transcoder<char, C::Unit> for CharsetEncoder<C>
where
    C: CharsetCodec,
{
    type Error = CharsetEncodeError;

    /// Returns the maximum number of target units needed for `input_len`
    /// characters.
    #[inline]
    fn max_transcode_output_len(
        &self,
        input_len: usize,
    ) -> Result<usize, CapacityError> {
        self.engine.max_transcode_output_len(input_len)
    }

    /// Returns the maximum target units emitted by finishing internal state.
    #[inline]
    fn max_finish_output_len(&self) -> Result<usize, CapacityError> {
        self.engine.max_finish_output_len()
    }

    /// Returns the maximum target units emitted when resetting stream state.
    #[inline]
    fn max_reset_output_len(&self) -> Result<usize, CapacityError> {
        self.engine.max_reset_output_len()
    }

    /// Runs encoder reset while keeping encoder policy.
    #[inline]
    fn reset(
        &mut self,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<usize, TranscodeError<Self::Error>> {
        self.engine
            .reset(output, output_index)
            .map_err(|error| error.map_domain(map_encode_engine_error))
    }

    /// Encodes characters into the target charset while applying unmappable
    /// policy.
    #[inline]
    fn transcode(
        &mut self,
        input: &[char],
        input_index: usize,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, TranscodeError<Self::Error>> {
        self.engine
            .transcode(input, input_index, output, output_index)
            .map_err(|error| error.map_domain(map_encode_engine_error))
    }

    /// Finishes encoder-owned final output after EOF.
    #[inline]
    fn finish(
        &mut self,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<usize, TranscodeError<Self::Error>> {
        self.engine
            .finish(output, output_index)
            .map_err(|error| error.map_domain(map_encode_engine_error))
    }
}

impl<C> TranscodeEncoder<char, C::Unit> for CharsetEncoder<C> where
    C: CharsetCodec
{
}

impl<C> fmt::Debug for CharsetEncoder<C>
where
    C: CharsetCodec + fmt::Debug,
{
    /// Formats the encoder without exposing additional bounds for unit values.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharsetEncoder")
            .field("engine", &self.engine)
            .field("unmappable_action", &self.unmappable_action())
            .field("replacement", &self.replacement())
            .field(
                "replacement_units_len",
                &self.replacement_units_len.map(NonZeroUsize::get),
            )
            .finish()
    }
}

#[inline]
fn map_encode_engine_error(
    error: TranscodeEncodeEngineError<CharsetEncodeError, CharsetEncodeError>,
) -> CharsetEncodeError {
    match error {
        TranscodeEncodeEngineError::CodecEncode { source, .. }
        | TranscodeEncodeEngineError::CodecReset { source }
        | TranscodeEncodeEngineError::CodecFlush { source } => source,
        TranscodeEncodeEngineError::Hook(error) => error,
    }
}
