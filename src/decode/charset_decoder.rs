// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::CapacityError;
use qubit_codec::Codec;
use qubit_codec::TranscodeDecodeErrorOf;
use qubit_codec::TranscodeDecoder;
use qubit_codec::TranscodeProgress;
use qubit_codec::Transcoder;
use qubit_codec::engine::TranscodeDecodeEngine;

use super::charset_decode_hooks::CharsetDecodeHooks;
use super::charset_decode_policy::CharsetDecodePolicy;
use crate::BomDetectStatus;
use crate::CharsetCodec;
use crate::CharsetDecodeError;
use crate::MalformedAction;
use crate::UnicodeBom;

/// Converts units of one charset into Unicode scalar values.
///
/// `CharsetDecoder` wraps a low-level [`CharsetCodec`] and applies the
/// configured [`MalformedAction`] whenever the codec reports malformed input
/// or an incomplete tail is encountered at EOF.
/// The decoder asks the wrapped codec whether one value can be decoded from the
/// currently available units. If the codec reports a valid incomplete prefix,
/// the tail is left in the caller-provided input slice and
/// [`qubit_codec::TranscodeStatus::NeedInput`] is returned. Callers must handle
/// incomplete EOF tails through [`Transcoder::transcode_eof`] before calling
/// [`Transcoder::finish`].
///
/// Construction does not initialize a stream. Call [`Self::reset`] and check
/// its result before using `transcode`, `transcode_eof`, or `finish`. During an
/// open stream, `Complete` means only that the current input was consumed;
/// `NeedInput` leaves an incomplete suffix unconsumed, and `NeedOutput` asks
/// the caller to drain output and resume from the reported progress. A
/// successful `finish` closes the stream; reset before starting another one.
/// The complete helper performs reset and finish itself.
/// An error does not roll back prior output or decoder state and does not
/// provide complete progress; reset starts a new logical stream but cannot
/// undo output already delivered to the application.
/// Lifecycle misuse is reported with lifecycle-specific
/// [`crate::CharsetDecodeErrorKind`] values; it is distinct from malformed
/// input and policy-reported decode errors. The corresponding core failures are
/// [`qubit_codec::TranscodeFailure::TranscodeBeforeReset`],
/// [`qubit_codec::TranscodeFailure::FinishBeforeReset`],
/// [`qubit_codec::TranscodeFailure::TranscodeAfterFinish`],
/// [`qubit_codec::TranscodeFailure::FinishAfterFinish`], and
/// [`qubit_codec::TranscodeFailure::LifecyclePoisoned`].
///
/// # Type Parameters
///
/// - `C`: Low-level charset codec used to decode source storage units into one
///   Unicode scalar value.
///
/// # Examples
///
/// ```
/// use qubit_codec_text::CharsetDecoder;
/// use qubit_codec_text::Utf8Codec;
///
/// let mut decoder = CharsetDecoder::new(Utf8Codec);
/// let mut output = ['\0'; 5];
/// let count = decoder
///     .transcode_complete_into(b"hello", &mut output)
///     .expect("valid UTF-8 input should decode");
/// assert_eq!(&output[..count], &['h', 'e', 'l', 'l', 'o']);
/// ```
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
    #[inline]
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
    #[inline]
    #[must_use]
    pub fn charset(&self) -> crate::Charset {
        self.codec().charset()
    }

    /// Returns the wrapped low-level codec.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the codec owned by this decoder.
    #[inline]
    #[must_use]
    pub fn codec(&self) -> &C {
        self.engine.codec()
    }

    /// Returns the wrapped low-level codec mutably.
    ///
    /// Mutating codec configuration while a stream is active can invalidate
    /// buffered decode state. Reset the decoder before continuing with the
    /// modified codec.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the codec owned by this decoder.
    #[inline]
    #[must_use]
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
    #[inline]
    #[must_use]
    pub const fn malformed_action(&self) -> MalformedAction {
        self.policy.malformed_action()
    }

    /// Returns the configured replacement character.
    ///
    /// # Returns
    ///
    /// Returns the character emitted when [`MalformedAction::Replace`] is used.
    #[inline]
    #[must_use]
    pub const fn replacement(&self) -> char {
        self.policy.replacement()
    }

    /// Runs decoder reset.
    ///
    /// # Parameters
    ///
    /// - `output`: Buffer receiving any characters emitted during reset.
    /// - `output_index`: Starting index at which reset output is written.
    ///
    /// # Errors
    ///
    /// Returns a transcode decode error when `output_index` is invalid, output
    /// capacity is insufficient, or decoder reset emits a charset-domain
    /// error. Index and capacity checks happen before reset begins, so correct
    /// the arguments and retry those failures. If reset has begun and then
    /// fails, state may have changed; successfully reset before reuse. Any
    /// business data that must be replayed or recovered remains the caller's
    /// responsibility.
    #[inline]
    pub fn reset(&mut self, output: &mut [char], output_index: usize) -> Result<usize, TranscodeDecodeErrorOf<C>> {
        self.engine.reset(output, output_index)
    }

    /// Decodes source units into Unicode scalar values.
    ///
    /// # Parameters
    ///
    /// - `input`: Source units available for decoding.
    /// - `input_index`: Starting index of the source units to decode.
    /// - `output`: Buffer receiving decoded Unicode scalar values.
    /// - `output_index`: Starting index at which decoded values are written.
    ///
    /// # Returns
    ///
    /// Returns progress relative to `input_index` and `output_index`.
    /// `Complete` means the available input was consumed, `NeedInput` means a
    /// valid incomplete suffix remains, and `NeedOutput` means more output
    /// capacity is needed to continue. The buffer may still have unused slots.
    /// `NeedOutput` is normal backpressure and is returned as `Ok`.
    /// Preserve an unconsumed suffix on `NeedInput`. On `NeedOutput`, consume
    /// the produced output and advance input by `read()` before resuming; do
    /// not submit already-read input again.
    ///
    /// # Errors
    ///
    /// Returns a transcode decode error for invalid indices, malformed input
    /// under the configured `Report` policy, or another decode-domain failure.
    /// Output backpressure is reported through `NeedOutput`, not as an error.
    /// An error does not roll back prior output or codec state and does not
    /// provide complete progress. Do not replay the original input range
    /// unless an explicit recovery protocol permits it. Reset starts a new
    /// logical stream; it cannot undo output already delivered to the
    /// application. See [`qubit_codec::Transcoder::transcode`] for the core
    /// contract.
    #[inline]
    pub fn transcode(
        &mut self,
        input: &[C::Unit],
        input_index: usize,
        output: &mut [char],
        output_index: usize,
    ) -> Result<TranscodeProgress, TranscodeDecodeErrorOf<C>> {
        self.engine.transcode(input, input_index, output, output_index)
    }

    /// Decodes source units after the caller has established end of input.
    ///
    /// Any valid but incomplete input tail is handled according to the
    /// configured malformed-input policy.
    /// Keep calling after `NeedOutput` until EOF input and pending output have
    /// been handled. Do not use `finish` to process an input tail.
    /// An error may leave decoded output already written and codec state
    /// changed. It does not provide complete progress; do not replay the
    /// original input range without an explicit recovery protocol. Reset
    /// starts a new logical stream but cannot undo output already delivered
    /// to the application. See [`qubit_codec::Transcoder::transcode`].
    ///
    /// # Parameters
    ///
    /// - `input`: Source units available at end of input.
    /// - `input_index`: Starting index of the source units to decode.
    /// - `output`: Buffer receiving decoded values or replacement characters.
    /// - `output_index`: Starting index at which decoded values are written.
    #[inline]
    pub fn transcode_eof(
        &mut self,
        input: &[C::Unit],
        input_index: usize,
        output: &mut [char],
        output_index: usize,
    ) -> Result<TranscodeProgress, TranscodeDecodeErrorOf<C>> {
        self.engine.transcode_eof(input, input_index, output, output_index)
    }

    /// Maps a decoder transcode error into a charset decode error.
    ///
    /// Framework failures are attached to this decoder's charset. Codec-domain
    /// errors retain their source data, but mapping a domain error discards
    /// its outer phase, input index, and input-consumption metadata. Inspect
    /// the original transcode error before mapping it if that context is
    /// needed; source-owned index and consumption values may differ from the
    /// outer values.
    #[inline]
    pub fn map_transcode_error(&self, error: TranscodeDecodeErrorOf<C>) -> CharsetDecodeError {
        CharsetDecodeError::from_transcode_error(self.charset(), error)
    }

    /// Finishes decoder-owned final output after EOF.
    ///
    /// # Parameters
    ///
    /// - `output`: Buffer receiving any characters emitted during finalization.
    /// - `output_index`: Starting index at which final output is written.
    ///
    /// # Errors
    ///
    /// Returns a transcode decode error when finalization output cannot be
    /// written or when the codec reports a final decode-domain error. Call
    /// this after EOF input has been handled. A preflight index or capacity
    /// failure can be corrected and retried; after finalization starts, a
    /// failure may change state and leave output written, so successfully
    /// reset before reuse. A successful finish closes the stream. Reset cannot
    /// undo output already delivered to the application.
    #[inline]
    pub fn finish(&mut self, output: &mut [char], output_index: usize) -> Result<usize, TranscodeDecodeErrorOf<C>> {
        self.engine.finish(output, output_index)
    }

    /// Runs a complete `reset -> transcode -> finish` decode stream. This
    /// helper initializes and closes the stream itself; do not call `reset`
    /// first. Use `transcode`, `transcode_eof`, and `finish` directly for
    /// chunked input.
    ///
    /// Before resetting, this helper conservatively checks the sum of the
    /// maximum reset, input-length transcode, and finish output capacities.
    /// The input length is measured in source `C::Unit` values. This bound is
    /// based on codec contracts rather than the actual text, so an output
    /// buffer that would fit the result may still fail the preflight. Use
    /// [`Transcoder::max_total_output_len`] with the input length to size the
    /// output, or use the streaming methods with a fixed small output window.
    ///
    /// # Parameters
    ///
    /// - `input`: Complete source stream to decode.
    /// - `output`: Buffer receiving decoded Unicode scalar values.
    ///
    /// # Errors
    ///
    /// Returns a transcode decode error when the conservative capacity bound
    /// overflows, the supplied output buffer is too small, the configured
    /// policy reports an incomplete sequence, or the codec reports another
    /// charset-domain decode error. Capacity preflight failures occur before
    /// reset and do not modify the output or lifecycle state. Once processing
    /// starts, failure may leave an output prefix and changed codec state;
    /// the error does not provide complete progress. Reset cannot undo output
    /// already delivered to the application, so stage output when the complete
    /// result must be published atomically.
    #[inline]
    pub fn transcode_complete_into(
        &mut self,
        input: &[C::Unit],
        output: &mut [char],
    ) -> Result<usize, TranscodeDecodeErrorOf<C>> {
        <Self as Transcoder>::transcode_complete_into(self, input, output)
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
    pub fn detect_and_strip_bom_progress(input: &[u8], eof: bool) -> (BomDetectStatus, &[u8]) {
        match UnicodeBom::detect_progress(input, eof) {
            BomDetectStatus::Match(bom) => (BomDetectStatus::Match(bom), &input[bom.bytes().len()..]),
            status @ (BomDetectStatus::Pending | BomDetectStatus::None) => (status, input),
        }
    }
}

impl<C> Transcoder for CharsetDecoder<C>
where
    C: CharsetCodec,
{
    type Input = C::Unit;
    type Output = char;
    type Error = TranscodeDecodeErrorOf<C>;

    /// Returns the maximum number of characters decoded from `input_len` units.
    #[inline]
    fn max_transcode_output_len(&self, input_len: usize) -> Result<usize, CapacityError> {
        self.engine.max_transcode_output_len(input_len)
    }

    /// Returns the maximum number of characters emitted by finishing internal
    /// state.
    #[inline]
    fn max_finish_output_len(&self) -> Result<usize, CapacityError> {
        self.engine.max_finish_output_len()
    }

    /// Returns the maximum characters emitted when resetting stream state.
    #[inline]
    fn max_reset_output_len(&self) -> Result<usize, CapacityError> {
        self.engine.max_reset_output_len()
    }

    /// Runs decoder reset while keeping decoder policy.
    #[inline]
    fn reset(&mut self, output: &mut [char], output_index: usize) -> Result<usize, Self::Error> {
        self.engine.reset(output, output_index)
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
        self.engine.transcode(input, input_index, output, output_index)
    }

    #[inline]
    fn transcode_eof(
        &mut self,
        input: &[C::Unit],
        input_index: usize,
        output: &mut [char],
        output_index: usize,
    ) -> Result<TranscodeProgress, Self::Error> {
        self.engine.transcode_eof(input, input_index, output, output_index)
    }

    /// Finishes decoder-owned final output after EOF.
    #[inline]
    fn finish(&mut self, output: &mut [char], output_index: usize) -> Result<usize, Self::Error> {
        self.engine.finish(output, output_index)
    }
}

impl<C> TranscodeDecoder for CharsetDecoder<C>
where
    C: CharsetCodec,
{
    type DecodeError = CharsetDecodeError;
}
