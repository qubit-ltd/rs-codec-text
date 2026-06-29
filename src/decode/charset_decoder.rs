// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::{
    CapacityError, Codec, TranscodeDecodeEngine, TranscodeDecoder, TranscodeError,
    TranscodeProgress, Transcoder,
};

use crate::{
    BomDetectStatus, CharsetCodec, CharsetDecodeError, MalformedAction, UnicodeBom,
    map_charset_decode_error,
};

use super::{charset_decode_hooks::CharsetDecodeHooks, charset_decode_policy::CharsetDecodePolicy};

/// Converts units of one charset into Unicode scalar values.
///
/// `CharsetDecoder` wraps a low-level [`CharsetCodec`] and applies the
/// configured [`MalformedAction`] whenever the codec reports malformed input.
/// The decoder asks the wrapped codec whether one value can be decoded from the
/// currently available units. If the codec reports a valid incomplete prefix,
/// the tail is left in the caller-provided input slice and
/// [`qubit_codec::TranscodeStatus::NeedInput`] is returned. Callers must handle
/// incomplete EOF tails before calling [`Transcoder::finish`].
///
/// # Type Parameters
///
/// - `C`: Low-level charset codec used to decode source storage units into one
///   Unicode scalar value.
#[derive(Debug)]
pub struct CharsetDecoder<C>
where
    C: CharsetCodec,
{
    /// Common buffered decode engine.
    engine: TranscodeDecodeEngine<C, CharsetDecodeHooks>,
    /// Public malformed-input policy metadata.
    policy: CharsetDecodePolicy,
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
    pub fn with_policy(codec: C, policy: CharsetDecodePolicy) -> Self {
        let hooks = CharsetDecodeHooks::new(policy.malformed_action(), policy.replacement());
        Self {
            engine: TranscodeDecodeEngine::new(codec, hooks),
            policy,
        }
    }

    /// Returns the charset decoded by the wrapped codec.
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
    /// Returns a shared reference to the codec owned by this decoder.
    #[must_use]
    #[inline]
    pub fn codec(&self) -> &C {
        self.engine.codec()
    }

    /// Returns the wrapped low-level codec mutably.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the codec owned by this decoder.
    #[must_use]
    #[inline]
    pub fn codec_mut(&mut self) -> &mut C {
        self.engine.codec_mut()
    }

    /// Consumes the decoder and returns its codec.
    ///
    /// Decoder policy and hook state are discarded.
    ///
    /// # Returns
    ///
    /// Returns the low-level codec owned by this decoder.
    #[must_use]
    #[inline]
    pub fn into_codec(self) -> C {
        let (codec, _) = self.engine.into_parts();
        codec
    }

    /// Returns the configured malformed-input action.
    ///
    /// # Returns
    ///
    /// Returns the action used when source input is malformed.
    #[must_use]
    #[inline]
    pub const fn malformed_action(&self) -> MalformedAction {
        self.policy.malformed_action()
    }

    /// Returns the configured replacement character.
    ///
    /// # Returns
    ///
    /// Returns the character emitted when [`MalformedAction::Replace`] is used.
    #[must_use]
    #[inline]
    pub const fn replacement(&self) -> char {
        self.policy.replacement()
    }
}

impl<C> CharsetDecoder<C>
where
    C: CharsetCodec + Codec<Unit = u8>,
{
    /// Detects and strips a Unicode byte order mark from a closed byte input.
    ///
    /// This helper treats `input` as EOF-reached input. Streaming callers that
    /// still may receive more prefix bytes should use
    /// [`Self::detect_and_strip_bom_progress`] to avoid stripping ambiguous BOM
    /// prefixes too early.
    ///
    /// # Returns
    ///
    /// Returns the detected BOM, if any, plus the input slice after the BOM
    /// prefix. If no BOM is detected, returns `None` and the original input
    /// slice.
    #[must_use]
    pub fn detect_and_strip_bom(input: &[u8]) -> (Option<UnicodeBom>, &[u8]) {
        match Self::detect_and_strip_bom_progress(input, true) {
            (BomDetectStatus::Match(bom), stripped) => (Some(bom), stripped),
            (BomDetectStatus::Pending | BomDetectStatus::None, stripped) => (None, stripped),
        }
    }

    /// Detects and strips a Unicode byte order mark with an explicit EOF
    /// signal.
    ///
    /// # Parameters
    ///
    /// - `input`: Bytes currently available from the beginning of the stream.
    /// - `eof`: Whether no more bytes can arrive.
    ///
    /// # Returns
    ///
    /// Returns the incremental BOM detection status plus the original input
    /// slice for [`BomDetectStatus::Pending`] and [`BomDetectStatus::None`],
    /// or the input slice after the BOM prefix for [`BomDetectStatus::Match`].
    #[must_use]
    pub fn detect_and_strip_bom_progress(input: &[u8], eof: bool) -> (BomDetectStatus, &[u8]) {
        match UnicodeBom::detect_progress(input, eof) {
            BomDetectStatus::Match(bom) => {
                (BomDetectStatus::Match(bom), &input[bom.bytes().len()..])
            }
            status @ (BomDetectStatus::Pending | BomDetectStatus::None) => (status, input),
        }
    }
}

impl<C> TranscodeDecoder<C::Unit, char> for CharsetDecoder<C> where C: CharsetCodec {}

impl<C> Transcoder<C::Unit, char> for CharsetDecoder<C>
where
    C: CharsetCodec,
{
    type Error = CharsetDecodeError;
    type DomainError = CharsetDecodeError;

    /// Maps transcode-layer failures into charset decode errors.
    #[inline]
    fn map_error(&self, error: TranscodeError<Self::DomainError>) -> Self::Error {
        map_charset_decode_error(self.charset(), error)
    }

    /// Returns the maximum number of characters decoded from `input_len` units.
    #[inline]
    fn max_transcode_output_len(&self, input_len: usize) -> Result<usize, CapacityError> {
        self.engine
            .max_transcode_output_len(input_len)
            .map_err(|_| CapacityError::OutputLengthOverflow)
    }

    /// Returns the maximum number of characters emitted by finishing internal
    /// state.
    #[inline]
    fn max_finish_output_len(&self) -> Result<usize, CapacityError> {
        self.engine
            .max_finish_output_len()
            .map_err(|_| CapacityError::OutputLengthOverflow)
    }

    /// Returns the maximum characters emitted when resetting stream state.
    #[inline]
    fn max_reset_output_len(&self) -> Result<usize, CapacityError> {
        self.engine.max_reset_output_len()
    }

    /// Runs decoder reset while keeping decoder policy.
    #[inline]
    fn reset(&mut self, output: &mut [char], output_index: usize) -> Result<usize, Self::Error> {
        let charset = self.charset();
        self.engine
            .reset(output, output_index)
            .map_err(|error| map_charset_decode_error(charset, error))
    }

    /// Decodes source units into Unicode scalar values while applying malformed
    /// policy.
    #[inline]
    fn transcode(
        &mut self,
        input: &[C::Unit],
        input_index: usize,
        output: &mut [char],
        output_index: usize,
    ) -> Result<TranscodeProgress, Self::Error> {
        let charset = self.charset();
        self.engine
            .transcode(input, input_index, output, output_index)
            .map_err(|error| map_charset_decode_error(charset, error))
    }

    /// Finishes decoder-owned final output after EOF.
    #[inline]
    fn finish(&mut self, output: &mut [char], output_index: usize) -> Result<usize, Self::Error> {
        let charset = self.charset();
        self.engine
            .finish(output, output_index)
            .map_err(|error| map_charset_decode_error(charset, error))
    }
}
