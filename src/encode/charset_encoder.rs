// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::CapacityError;
use qubit_codec::TranscodeEncodeErrorOf;
use qubit_codec::TranscodeEncoder;
use qubit_codec::TranscodeProgress;
use qubit_codec::Transcoder;
use qubit_codec::engine::TranscodeEncodeEngine;

use super::charset_encode_hooks::CharsetEncodeHooks;
use super::charset_encode_hooks::replacement_len;
use super::charset_encode_policy::CharsetEncodePolicy;
use crate::CharsetCodec;
use crate::CharsetEncodeError;
use crate::UnmappableAction;

/// Converts Unicode scalar values into units of one charset.
///
/// `CharsetEncoder` wraps a low-level [`crate::CharsetCodec`] and applies the
/// configured [`UnmappableAction`] whenever the codec reports that an input
/// character cannot be represented by the target charset.
///
/// Construction does not initialize a stream. Call [`Self::reset`] and check
/// its result before using `transcode` or `finish`. During an open stream,
/// `Complete` means only that the current input was consumed; on `NeedOutput`,
/// drain output and resume from the reported progress. A successful `finish`
/// closes the stream; reset before starting another one. The complete helper
/// performs reset and finish itself.
/// An error does not roll back prior output or encoder state and does not
/// provide complete progress; reset starts a new logical stream but cannot
/// undo output already delivered to the application.
/// Lifecycle misuse is reported with lifecycle-specific
/// [`crate::CharsetEncodeErrorKind`] values; it is distinct from unmappable
/// input and policy-reported encode errors. The corresponding core failures are
/// [`qubit_codec::TranscodeFailure::TranscodeBeforeReset`],
/// [`qubit_codec::TranscodeFailure::FinishBeforeReset`],
/// [`qubit_codec::TranscodeFailure::TranscodeAfterFinish`],
/// [`qubit_codec::TranscodeFailure::FinishAfterFinish`], and
/// [`qubit_codec::TranscodeFailure::LifecyclePoisoned`].
///
/// # Type Parameters
///
/// - `C`: Low-level charset codec used to encode one character into target
///   storage units.
///
/// # Examples
///
/// ```
/// use qubit_codec_text::CharsetEncoder;
/// use qubit_codec_text::Utf8Codec;
///
/// let mut encoder = CharsetEncoder::new(Utf8Codec);
/// let mut output = [0; 20];
/// let written = encoder
///     .transcode_complete_into(&['h', 'e', 'l', 'l', 'o'], &mut output)
///     .expect("UTF-8 encoding succeeds");
/// assert_eq!(&output[..written], b"hello");
/// ```
///
/// A reported error can occur after a valid output prefix has been written.
/// This example stages one message locally, publishes it only after complete
/// success, and discards the staging buffer on failure. The error does not
/// provide complete progress and does not imply that every failure writes one
/// byte.
///
/// ```
/// use qubit_codec_text::AsciiCodec;
/// use qubit_codec_text::CharsetEncodePolicy;
/// use qubit_codec_text::CharsetEncoder;
///
/// let mut encoder = CharsetEncoder::with_policy(AsciiCodec, CharsetEncodePolicy::report())
///     .expect("Report needs no encodable replacement");
/// let mut output = [0_u8; 4];
/// encoder.reset(&mut output, 0).expect("reset");
/// let error = encoder.transcode(&['A', '中'], 0, &mut output, 0)
///     .expect_err("ASCII cannot encode the second character");
/// assert_eq!(b'A', output[0]);
/// let _diagnostic = encoder.map_transcode_error(error);
/// // This buffer is message-local staging. Publish it only after complete success.
/// // On failure discard this message's staging; do not replay it into a live sink.
/// ```
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
    /// `codec`. The constructor prefers U+FFFD and uses `?` when U+FFFD is
    /// unavailable.
    #[must_use]
    pub fn new(codec: C) -> Self {
        let policy = CharsetEncodePolicy::default_for(&codec).unwrap_or_else(|error| {
            // If default replacement selection gets here, the codec cannot
            // encode either supported default replacement.
            panic!(
                "cannot initialize CharsetEncoder for {:?}: neither {:?} nor {:?} is encodable ({error})",
                codec.charset(),
                CharsetEncodePolicy::DEFAULT_REPLACEMENT,
                CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT,
            )
        });
        let hooks = Self::create_hooks(&codec, policy)
            // A policy chosen by `default_for` must be encodable; failure here
            // means the selected replacement is not encodable.
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
    /// # Parameters
    ///
    /// - `codec`: Low-level codec used to encode output units.
    /// - `policy`: Action and replacement used for unmappable input.
    ///
    /// # Returns
    ///
    /// Returns an encoder configured with `policy`.
    ///
    /// # Errors
    ///
    /// Returns an error when `policy` uses replacement and the replacement
    /// character cannot be encoded by `codec`.
    pub fn with_policy(codec: C, policy: CharsetEncodePolicy) -> Result<Self, CharsetEncodeError> {
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
    #[inline]
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
    #[inline]
    #[must_use]
    pub const fn replacement(&self) -> char {
        self.policy.replacement()
    }

    /// Returns the charset encoded by the wrapped codec.
    ///
    /// # Returns
    ///
    /// Returns the charset reported by the low-level codec.
    #[inline]
    #[must_use]
    pub fn charset(&self) -> crate::Charset {
        self.codec().charset()
    }

    /// Returns the wrapped low-level codec.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the codec owned by this encoder.
    #[inline]
    #[must_use]
    pub fn codec(&self) -> &C {
        self.engine.codec()
    }

    /// Returns the wrapped low-level codec mutably.
    ///
    /// Mutating codec configuration while a stream is active can invalidate
    /// buffered encode state. Reset before continuing, but note that reset does
    /// not revalidate the configured replacement. Under a `Replace` policy,
    /// ensure the current replacement remains encodable after the change. The
    /// default constructor prefers U+FFFD and uses `?` only when U+FFFD cannot
    /// be encoded. If the current replacement's encodability cannot be
    /// guaranteed, save the policy, consume the encoder with
    /// [`Self::into_codec`], then reconstruct it with [`Self::with_policy`]
    /// and handle validation errors. Reconstruction discards wrapper state and
    /// does not preserve pending input or output.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the codec owned by this encoder.
    #[inline]
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
    #[inline]
    #[must_use]
    pub fn into_codec(self) -> C {
        let (codec, _) = self.engine.into_parts();
        codec
    }

    /// Runs encoder reset, initializing a new logical stream.
    ///
    /// # Errors
    ///
    /// Returns a transcode encode error when `output_index` is invalid, output
    /// capacity is insufficient, or encoder reset emits a charset-domain
    /// error. Index and capacity checks happen before reset begins, so correct
    /// the arguments and retry those failures. If reset has begun and then
    /// fails, state may have changed; successfully reset before reuse. Any
    /// business data that must be replayed or recovered remains the caller's
    /// responsibility.
    #[inline]
    pub fn reset(&mut self, output: &mut [C::Unit], output_index: usize) -> Result<usize, TranscodeEncodeErrorOf<C>> {
        self.engine.reset(output, output_index)
    }

    /// Encodes characters into target units.
    ///
    /// # Returns
    ///
    /// Returns progress relative to `input_index` and `output_index`.
    /// `Complete` means the available input was consumed; `NeedOutput` means
    /// more output capacity is needed to continue. The buffer may still have
    /// unused slots. `NeedOutput` is normal backpressure and is returned as
    /// `Ok`.
    /// On `NeedOutput`, consume the produced output and advance input by
    /// `read()` before resuming; do not submit already-read input again.
    ///
    /// # Errors
    ///
    /// Returns a transcode encode error for invalid indices, a character that
    /// is unmappable under the configured `Report` policy, or another
    /// encode-domain failure. Output backpressure is reported through
    /// `NeedOutput`, not as an error. An error does not roll back prior output
    /// or codec state and does not provide complete progress. Do not replay
    /// the original input range unless an explicit recovery protocol permits
    /// it. Reset starts a new logical stream; it cannot undo output already
    /// delivered to the application. See
    /// [`qubit_codec::Transcoder::transcode`] for the core contract.
    #[inline]
    pub fn transcode(
        &mut self,
        input: &[char],
        input_index: usize,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, TranscodeEncodeErrorOf<C>> {
        self.engine.transcode(input, input_index, output, output_index)
    }

    /// Maps an encoder transcode error into a charset encode error.
    ///
    /// Framework failures and unencodable values are attached to this
    /// encoder's charset. Codec-domain errors retain their source data, but
    /// mapping a domain error discards its outer phase, input index, and
    /// input-consumption metadata. Inspect the original transcode error before
    /// mapping it if that context is needed; source-owned index and consumption
    /// values may differ from the outer values.
    #[inline]
    pub fn map_transcode_error(&self, error: TranscodeEncodeErrorOf<C>) -> CharsetEncodeError {
        CharsetEncodeError::from_transcode_error(self.charset(), error)
    }

    /// Finishes encoder-owned final output after EOF.
    ///
    /// # Errors
    ///
    /// Returns a transcode encode error when finalization output cannot be
    /// written or when the codec reports a final encode-domain error. Call
    /// this after all input has been encoded. A preflight index or capacity
    /// failure can be corrected and retried; after finalization starts, a
    /// failure may change state and leave output written, so successfully
    /// reset before reuse. A successful finish closes the stream. Reset cannot
    /// undo output already delivered to the application.
    #[inline]
    pub fn finish(&mut self, output: &mut [C::Unit], output_index: usize) -> Result<usize, TranscodeEncodeErrorOf<C>> {
        self.engine.finish(output, output_index)
    }

    /// Runs a complete `reset -> transcode -> finish` encode stream. This
    /// helper initializes and closes the stream itself; do not call `reset`
    /// first. Use `transcode` and `finish` directly for incremental input.
    ///
    /// # Errors
    ///
    /// Returns a transcode encode error when the supplied output buffer is too
    /// small, a character cannot be represented under the configured policy,
    /// or the codec reports a charset-domain encode error. Failure may leave an
    /// output prefix and changed codec state; the error does not provide
    /// complete progress. Reset cannot undo output already delivered to the
    /// application, so stage output when the complete result must be published
    /// atomically.
    #[inline]
    pub fn transcode_complete_into(
        &mut self,
        input: &[char],
        output: &mut [C::Unit],
    ) -> Result<usize, TranscodeEncodeErrorOf<C>> {
        <Self as Transcoder>::transcode_complete_into(self, input, output)
    }

    /// Creates encode hooks for `policy`, validating the selected replacement
    /// only when the action is `Replace`.
    pub(crate) fn create_hooks(
        codec: &C,
        policy: CharsetEncodePolicy,
    ) -> Result<CharsetEncodeHooks, CharsetEncodeError> {
        let hooks = CharsetEncodeHooks::new(policy.unmappable_action(), policy.replacement());
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
    type Error = TranscodeEncodeErrorOf<C>;

    /// Returns the maximum number of target units needed for `input_len`
    /// characters.
    #[inline]
    fn max_transcode_output_len(&self, input_len: usize) -> Result<usize, CapacityError> {
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
    fn reset(&mut self, output: &mut [C::Unit], output_index: usize) -> Result<usize, Self::Error> {
        self.engine.reset(output, output_index)
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
    ) -> Result<TranscodeProgress, Self::Error> {
        self.engine.transcode(input, input_index, output, output_index)
    }

    /// Finishes encoder-owned final output after EOF.
    #[inline]
    fn finish(&mut self, output: &mut [C::Unit], output_index: usize) -> Result<usize, Self::Error> {
        self.engine.finish(output, output_index)
    }
}

impl<C> TranscodeEncoder for CharsetEncoder<C>
where
    C: CharsetCodec,
{
    type EncodeError = CharsetEncodeError;
}
