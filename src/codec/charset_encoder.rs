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
    EncodeErrorFactory,
};

use crate::{
    BufferedEncoder,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
    TranscodeProgress,
    Transcoder,
};

use super::{
    charset_codec::CharsetCodec,
    charset_encode_hooks::CharsetEncodeHooks,
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
    /// Common buffered encoding engine.
    engine: BufferedEncodeEngine<C, CharsetEncodeHooks<C::Unit>>,
}

impl<C> CharsetEncoder<C>
where
    C: CharsetEncodeProbe,
{
    /// Default replacement character used when unmappable input is replaced.
    pub const DEFAULT_REPLACEMENT: char = '\u{fffd}';

    /// Fallback replacement used when the default replacement is unmappable.
    pub const DEFAULT_FALLBACK_REPLACEMENT: char = '?';

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
    /// [`CharsetEncoder::DEFAULT_REPLACEMENT`]. If the default cannot be encoded
    /// by the codec, [`CharsetEncoder::DEFAULT_FALLBACK_REPLACEMENT`] is used.
    ///
    /// # Panics
    ///
    /// Panics when neither [`Self::DEFAULT_REPLACEMENT`] nor
    /// [`Self::DEFAULT_FALLBACK_REPLACEMENT`] can be encoded by `codec`.
    /// Built-in codecs can always encode the fallback `?`; failure here means
    /// the supplied codec cannot encode a minimal ASCII replacement. For custom
    /// [`crate::CharsetCodec`] implementations, this indicates a broken codec
    /// invariant rather than recoverable input data.
    #[must_use]
    pub fn new(codec: C) -> Self {
        let hooks = CharsetEncodeHooks::new(UnmappableAction::Replace, Self::DEFAULT_REPLACEMENT);
        let mut encoder = Self {
            engine: BufferedEncodeEngine::new(codec, hooks),
        };
        match encoder.encode_replacement(Self::DEFAULT_REPLACEMENT) {
            Ok(replacement_units) => {
                let hooks = encoder.engine.hooks_mut();
                hooks.replacement = Self::DEFAULT_REPLACEMENT;
                hooks.replacement_units = replacement_units;
                encoder
            }
            Err(default_error) => match encoder.encode_replacement(Self::DEFAULT_FALLBACK_REPLACEMENT) {
                Ok(replacement_units) => {
                    let hooks = encoder.engine.hooks_mut();
                    hooks.replacement = Self::DEFAULT_FALLBACK_REPLACEMENT;
                    hooks.replacement_units = replacement_units;
                    encoder
                }
                Err(_) => panic!(
                    "cannot initialize CharsetEncoder for {:?}: neither {:?} nor {:?} is encodable ({default_error})",
                    encoder.codec().charset(),
                    Self::DEFAULT_REPLACEMENT,
                    Self::DEFAULT_FALLBACK_REPLACEMENT,
                ),
            },
        }
    }

    /// Creates an encoder with the provided replacement character.
    ///
    /// The replacement character is checked once on construction. If the codec
    /// cannot encode it, this returns an error immediately.
    ///
    /// # Parameters
    ///
    /// - `replacement`: Replacement character for unmappable input.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` when the character is encodable by the codec.
    /// - `Err(Self::Error)` when the replacement is unsupported.
    #[inline]
    pub fn with_replacement(mut self, replacement: char) -> Result<Self, CharsetEncodeError> {
        self.set_replacement(replacement)?;
        Ok(self)
    }

    /// Returns the wrapped low-level codec.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the configured codec.
    #[must_use]
    #[inline(always)]
    pub const fn codec(&self) -> &C {
        self.engine.codec()
    }

    /// Returns a mutable reference to the wrapped codec.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the configured codec.
    #[must_use]
    #[inline(always)]
    pub fn codec_mut(&mut self) -> &mut C {
        self.engine.codec_mut()
    }

    /// Returns the configured unmappable-character action.
    ///
    /// # Returns
    ///
    /// Returns the action used when target encoding cannot represent a character.
    #[must_use]
    #[inline(always)]
    pub const fn unmappable_action(&self) -> UnmappableAction {
        self.engine.hooks().unmappable_action
    }

    /// Sets the unmappable-character action.
    ///
    /// # Parameters
    ///
    /// - `action`: New policy for unmappable input characters.
    #[inline(always)]
    pub fn set_unmappable_action(&mut self, action: UnmappableAction) {
        self.engine.hooks_mut().unmappable_action = action;
    }

    /// Returns the configured replacement character.
    ///
    /// # Returns
    ///
    /// Returns the character encoded when [`UnmappableAction::Replace`] is used.
    #[must_use]
    #[inline(always)]
    pub const fn replacement(&self) -> char {
        self.engine.hooks().replacement
    }

    /// Sets the replacement character.
    ///
    /// # Parameters
    ///
    /// - `replacement`: New replacement character used by replace policy.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the codec cannot encode the given replacement.
    #[inline]
    pub fn set_replacement(&mut self, replacement: char) -> Result<(), CharsetEncodeError> {
        let replacement_units = self.encode_replacement(replacement)?;
        let hooks = self.engine.hooks_mut();
        hooks.replacement = replacement;
        hooks.replacement_units = replacement_units;
        Ok(())
    }

    /// Encodes a replacement character into a temporary buffer and returns the
    /// encoded unit sequence.
    ///
    /// # Parameters
    ///
    /// - `ch`: Replacement character to validate and encode.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<C::Unit>)` when the character is encodable.
    /// - `Err(CharsetEncodeError)` with codec-specific context when encoding fails.
    ///
    /// # Errors
    ///
    /// Returns an error when the target charset cannot encode the character.
    #[inline]
    fn encode_replacement(&self, ch: char) -> CharsetEncodeResult<Vec<C::Unit>> {
        let required = self.codec().encode_len(ch, 0)?;
        let mut output = vec![C::Unit::default(); required];
        // SAFETY: CharsetEncodeProbe reports the exact output width accepted by
        // charset codec implementations.
        let written = unsafe { self.codec().encode_unchecked(&ch, output.as_mut_slice(), 0) }?;
        debug_assert!(written <= required);
        output.truncate(written);
        Ok(output)
    }
}

impl<C> Transcoder<char, C::Unit> for CharsetEncoder<C>
where
    C: CharsetEncodeProbe,
{
    type Error = CharsetEncodeError;

    /// Returns the maximum number of target units needed for `input_len` characters.
    #[inline(always)]
    fn max_output_len(&self, input_len: usize) -> Option<usize> {
        self.engine.max_output_len::<char, C::Unit>(input_len)
    }

    /// Returns the maximum target units emitted by finishing internal state.
    #[inline(always)]
    fn max_finish_output_len(&self) -> Option<usize> {
        self.engine.max_finish_output_len::<char, C::Unit>()
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
        self.codec() == other.codec()
            && self.unmappable_action() == other.unmappable_action()
            && self.replacement() == other.replacement()
    }
}

impl<C> fmt::Debug for CharsetEncoder<C>
where
    C: CharsetEncodeProbe + fmt::Debug,
{
    /// Formats the encoder without exposing additional bounds for cached units.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharsetEncoder")
            .field("codec", self.codec())
            .field("unmappable_action", &self.unmappable_action())
            .field("replacement", &self.replacement())
            .field("replacement_units_len", &self.engine.hooks().replacement_units.len())
            .finish()
    }
}
