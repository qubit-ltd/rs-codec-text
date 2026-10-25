// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::CapacityError;
use qubit_codec::TranscodeConvertErrorOf;
use qubit_codec::TranscodeConverter;
use qubit_codec::TranscodeProgress;
use qubit_codec::Transcoder;
use qubit_codec::engine::TranscodeConvertEngine;

use crate::CharsetCodec;
use crate::CharsetConvertError;
use crate::CharsetDecodeError;
use crate::CharsetDecodeHooks;
use crate::CharsetDecodePolicy;
use crate::CharsetEncodeError;
use crate::CharsetEncodeHooks;
use crate::CharsetEncodePolicy;
use crate::CharsetEncoder;
use crate::MalformedAction;
use crate::UnmappableAction;

/// Converts units encoded with one charset into units encoded with another
/// charset.
///
/// The converter owns the source and target charset codecs plus the same
/// decode/encode policy hooks used by [`crate::CharsetDecoder`] and
/// [`crate::CharsetEncoder`].
/// A decoded character may be kept pending inside the common buffered convert
/// engine when the target output buffer is full. During
/// [`Transcoder::finish`], the converter drains internally retained
/// output and finishes the composed decode/encode policy hooks. Callers remain
/// responsible for passing any incomplete input tail to
/// [`Transcoder::transcode_eof`] before finishing the logical stream.
///
/// Construction does not initialize a stream. Call [`Self::reset`] and check
/// its result before streaming. During an open stream, `Complete` means only
/// that the current input was consumed. On `NeedInput`, retain the unread
/// suffix and append the next chunk; on `NeedOutput`, collect output and
/// advance by the reported `read()` and `written()` counts before resuming.
/// At confirmed EOF, pass the retained suffix to `transcode_eof` and continue
/// through output backpressure before calling `finish`. A successful finish
/// closes the stream; reset before starting another one. The complete helper
/// performs reset and finish itself. Lifecycle errors are not evidence of
/// malformed source text; see [`CharsetConvertError`] for the compatibility
/// mapping of those failures. Their core failure variants are
/// [`qubit_codec::TranscodeFailure::TranscodeBeforeReset`],
/// [`qubit_codec::TranscodeFailure::FinishBeforeReset`],
/// [`qubit_codec::TranscodeFailure::TranscodeAfterFinish`],
/// [`qubit_codec::TranscodeFailure::FinishAfterFinish`], and
/// [`qubit_codec::TranscodeFailure::LifecyclePoisoned`].
///
/// # Type Parameters
///
/// - `D`: Low-level charset codec used by the source decoder.
/// - `E`: Low-level charset codec used by the target encoder.
///
/// # Examples
///
/// ```rust
/// use qubit_codec::{
///     TranscodeStatus,
///     Transcoder,
/// };
/// use qubit_codec_text::{
///     CharsetConverter,
///     CharsetDecoder,
///     CharsetEncoder,
///     Utf16U16Codec,
///     Utf8Codec,
/// };
///
/// let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
/// let mut output = [0_u16; 2];
/// converter
///     .reset(&mut [], 0)
///     .expect("reset converter before streaming");
///
/// let progress = converter
///     .transcode("AB".as_bytes(), 0, &mut output, 0)
///     .expect("transcode bytes to utf-16");
///
/// assert_eq!(TranscodeStatus::Complete, progress.status());
/// assert_eq!(2, progress.read());
/// assert_eq!(2, progress.written());
/// assert_eq!([65, 66], output);
/// ```
///
/// The following complete example keeps an incomplete UTF-8 suffix between
/// chunks, drains output backpressure, and applies the default replacement
/// policy to an incomplete suffix at EOF. The two-unit output window makes the
/// supplementary character require another drain after the preceding BMP
/// character has used one slot.
///
/// ```rust
/// use qubit_codec::TranscodeStatus;
/// use qubit_codec_text::CharsetConverter;
/// use qubit_codec_text::Utf16U16Codec;
/// use qubit_codec_text::Utf8Codec;
///
/// fn drain(
///     converter: &mut CharsetConverter<Utf8Codec, Utf16U16Codec>,
///     pending: &mut Vec<u8>,
///     eof: bool,
///     result: &mut Vec<u16>,
///     observed: &mut (bool, bool),
/// ) {
///     loop {
///         let mut output = [0_u16; 2];
///         let progress = if eof {
///             converter.transcode_eof(pending, 0, &mut output, 0)
///         } else {
///             converter.transcode(pending, 0, &mut output, 0)
///         }.expect("default replacement policy");
///         result.extend_from_slice(&output[..progress.written()]);
///         pending.drain(..progress.read());
///         match progress.status() {
///             TranscodeStatus::Complete => break,
///             TranscodeStatus::NeedInput { .. } => {
///                 assert!(!eof);
///                 observed.0 = true;
///                 break;
///             }
///             TranscodeStatus::NeedOutput { .. } => {
///                 observed.1 = true;
///                 assert!(progress.read() > 0 || progress.written() > 0);
///             }
///         }
///     }
/// }
///
/// fn main() {
///     let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
///     // These built-in codecs have no reset or finish output.
///     assert_eq!(0, converter.reset(&mut [], 0).expect("reset"));
///     let chunks: [&[u8]; 3] = [
///         &[0x41, 0xe4],
///         &[0xb8, 0xad, 0xf0, 0x9f, 0x98, 0x80],
///         &[0xe2, 0x82],
///     ];
///     let mut pending = Vec::new();
///     let mut result = Vec::new();
///     let mut observed = (false, false);
///     for chunk in chunks {
///         pending.extend_from_slice(chunk);
///         drain(&mut converter, &mut pending, false, &mut result, &mut observed);
///     }
///     drain(&mut converter, &mut pending, true, &mut result, &mut observed);
///     assert!(pending.is_empty());
///     assert_eq!(0, converter.finish(&mut [], 0).expect("finish"));
///     assert!(observed.0, "the example must exercise NeedInput");
///     assert!(observed.1, "the example must exercise NeedOutput");
///     assert_eq!(vec![0x0041, 0x4e2d, 0xd83d, 0xde00, 0xfffd], result);
/// }
/// ```
#[derive(Debug)]
pub struct CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetCodec,
{
    /// Common buffered converter engine.
    engine: TranscodeConvertEngine<D, E, CharsetDecodeHooks, CharsetEncodeHooks>,
    /// Public malformed-input policy metadata.
    decode_policy: CharsetDecodePolicy,
    /// Public unmappable-input policy metadata.
    encode_policy: CharsetEncodePolicy,
}

impl<D, E> CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetCodec,
{
    /// Creates a charset converter from raw source and target codecs.
    ///
    /// # Parameters
    ///
    /// - `source`: Source charset codec.
    /// - `target`: Target charset codec.
    ///
    /// # Returns
    ///
    /// Returns a converter with the default decoder policy and the target
    /// encoder policy that can be represented by `target`. The encoder policy
    /// first tries [`CharsetEncodePolicy::DEFAULT_REPLACEMENT`] and falls back
    /// to [`CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT`] when needed.
    ///
    /// # Panics
    ///
    /// Panics when neither default replacement can be encoded by `target`.
    /// This panic is intentional by design: reaching this branch means the
    /// target codec implementation is wrong, because the API requires a
    /// fallback replacement that the codec can encode. Built-in codecs can
    /// always encode the fallback `?`; custom [`crate::CharsetCodec`]
    /// implementations that cannot encode it violate the replacement
    /// invariant and must fail fast during converter construction rather than
    /// defer the bug to user input.
    #[must_use]
    pub fn from_codecs(source: D, target: E) -> Self {
        let decode_policy = CharsetDecodePolicy::default();
        let (encode_policy, encode_hooks) = Self::default_encode_policy(&target);
        Self {
            engine: TranscodeConvertEngine::new(
                source,
                target,
                CharsetDecodeHooks::new(decode_policy.malformed_action(), decode_policy.replacement()),
                encode_hooks,
            ),
            decode_policy,
            encode_policy,
        }
    }

    /// Creates a charset converter from raw codecs and explicit policies.
    ///
    /// # Parameters
    ///
    /// - `source`: Source charset codec.
    /// - `target`: Target charset codec.
    /// - `decode_policy`: Malformed source-input policy.
    /// - `encode_policy`: Unmappable target-output policy.
    ///
    /// # Returns
    ///
    /// Returns a converter configured with the supplied policies.
    ///
    /// # Errors
    ///
    /// Returns an error when `encode_policy` uses replacement and the target
    /// codec cannot encode the replacement character.
    pub fn from_codecs_with_policies(
        source: D,
        target: E,
        decode_policy: CharsetDecodePolicy,
        encode_policy: CharsetEncodePolicy,
    ) -> Result<Self, CharsetEncodeError> {
        let encode_hooks = CharsetEncoder::<E>::create_hooks(&target, encode_policy)?;
        Ok(Self {
            engine: TranscodeConvertEngine::new(
                source,
                target,
                CharsetDecodeHooks::new(decode_policy.malformed_action(), decode_policy.replacement()),
                encode_hooks,
            ),
            decode_policy,
            encode_policy,
        })
    }

    /// Returns the charset decoded from source units.
    ///
    /// # Returns
    ///
    /// Returns the charset reported by the source codec.
    #[inline]
    #[must_use]
    pub fn source_charset(&self) -> crate::Charset {
        self.source_codec().charset()
    }

    /// Returns the charset encoded to target units.
    ///
    /// # Returns
    ///
    /// Returns the charset reported by the target codec.
    #[inline]
    #[must_use]
    pub fn target_charset(&self) -> crate::Charset {
        self.target_codec().charset()
    }

    /// Returns the source codec.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the source codec owned by this converter.
    #[inline]
    #[must_use]
    pub fn source_codec(&self) -> &D {
        self.engine.source_codec()
    }

    /// Returns the source codec mutably.
    ///
    /// Mutating source codec configuration while a conversion is active can
    /// invalidate buffered decode state. Reset the converter before continuing
    /// with the modified codec.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the source codec owned by this converter.
    #[inline]
    #[must_use]
    pub fn source_codec_mut(&mut self) -> &mut D {
        self.engine.source_codec_mut()
    }

    /// Returns the target codec.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the target codec owned by this converter.
    #[inline]
    #[must_use]
    pub fn target_codec(&self) -> &E {
        self.engine.target_codec()
    }

    /// Returns the target codec mutably.
    ///
    /// Mutating target codec configuration while a conversion is active can
    /// invalidate buffered encode state. Reset before continuing, but note
    /// that reset does not revalidate the configured replacement. Under a
    /// `Replace` policy, ensure the replacement remains encodable after the
    /// change; the default policy also relies on the codec's existing `?`
    /// fallback contract. If that cannot be guaranteed, save the intended
    /// policies, consume the converter with [`Self::into_codecs`], then
    /// reconstruct it with [`Self::from_codecs_with_policies`] and handle
    /// validation errors. Reconstruction discards wrapper state and does not
    /// preserve pending input or output.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the target codec owned by this converter.
    #[inline]
    #[must_use]
    pub fn target_codec_mut(&mut self) -> &mut E {
        self.engine.target_codec_mut()
    }

    /// Consumes the converter and returns its codecs.
    ///
    /// Converter policies, hook state, and pending conversion state are
    /// discarded.
    ///
    /// # Returns
    ///
    /// Returns the source codec followed by the target codec.
    #[inline]
    #[must_use]
    pub fn into_codecs(self) -> (D, E) {
        let (source, target, _, _) = self.engine.into_parts();
        (source, target)
    }

    /// Returns the configured malformed source-input policy.
    ///
    /// # Returns
    ///
    /// Returns the decoder policy used by this converter.
    #[inline]
    #[must_use]
    pub const fn decode_policy(&self) -> CharsetDecodePolicy {
        self.decode_policy
    }

    /// Returns the configured unmappable target-output policy.
    ///
    /// # Returns
    ///
    /// Returns the encoder policy used by this converter.
    #[inline]
    #[must_use]
    pub const fn encode_policy(&self) -> CharsetEncodePolicy {
        self.encode_policy
    }

    /// Returns the configured malformed-input action.
    ///
    /// # Returns
    ///
    /// Returns the action used when source input is malformed.
    #[inline]
    #[must_use]
    pub const fn malformed_action(&self) -> MalformedAction {
        self.decode_policy.malformed_action()
    }

    /// Returns the configured source replacement character.
    ///
    /// # Returns
    ///
    /// Returns the character emitted when malformed source input is replaced.
    #[inline]
    #[must_use]
    pub const fn decode_replacement(&self) -> char {
        self.decode_policy.replacement()
    }

    /// Returns the configured unmappable-character action.
    ///
    /// # Returns
    ///
    /// Returns the action used when the target charset cannot represent a
    /// character.
    #[inline]
    #[must_use]
    pub const fn unmappable_action(&self) -> UnmappableAction {
        self.encode_policy.unmappable_action()
    }

    /// Returns the configured target replacement character.
    ///
    /// # Returns
    ///
    /// Returns the character encoded when unmappable target input is replaced.
    #[inline]
    #[must_use]
    pub const fn replacement(&self) -> char {
        self.encode_policy.replacement()
    }

    /// Maps a low-level converter error into the charset-specific error model.
    ///
    /// # Parameters
    ///
    /// - `error`: Error returned by this converter's transcode operations.
    ///
    /// # Returns
    ///
    /// Returns a [`CharsetConvertError`] carrying this converter's source or
    /// target charset as appropriate.
    #[inline]
    pub fn map_transcode_error(&self, error: TranscodeConvertErrorOf<D, E>) -> CharsetConvertError {
        CharsetConvertError::from_transcode_error::<D, E>(self.source_charset(), self.target_charset(), error)
    }

    /// Resets converter state.
    ///
    /// # Errors
    ///
    /// Returns a transcode convert error when reset output cannot be written or
    /// when source decode reset or target encode reset reports a
    /// charset-domain error. Index and capacity checks happen before reset
    /// begins, so correct the arguments and retry those failures. If reset has
    /// begun and then fails, state may have changed; successfully reset before
    /// reuse. Any business data that must be replayed or recovered remains the
    /// caller's responsibility.
    #[inline]
    pub fn reset(
        &mut self,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<usize, TranscodeConvertErrorOf<D, E>> {
        self.engine.reset(output, output_index)
    }

    /// Converts source units into target units.
    ///
    /// `Complete` means only that the available input was consumed. On
    /// `NeedInput`, preserve the unread suffix and append later input. On
    /// `NeedOutput`, collect output and advance input by `read()` before
    /// resuming; the converter may retain a decoded character internally.
    ///
    /// # Errors
    ///
    /// Returns a transcode convert error when source decoding or target
    /// encoding fails, or when framework-level index and capacity checks
    /// fail.
    #[inline]
    pub fn transcode(
        &mut self,
        input: &[D::Unit],
        input_index: usize,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, TranscodeConvertErrorOf<D, E>> {
        self.engine.transcode(input, input_index, output, output_index)
    }

    /// Converts source units after the caller has established end of input.
    /// Continue after `NeedOutput` until the EOF input and pending output have
    /// been handled. `finish` does not accept or process an input tail.
    ///
    /// # Errors
    ///
    /// Returns a transcode convert error when EOF source input violates the
    /// configured decode policy, target encoding fails, or framework-level
    /// index and capacity checks fail.
    #[inline]
    pub fn transcode_eof(
        &mut self,
        input: &[D::Unit],
        input_index: usize,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, TranscodeConvertErrorOf<D, E>> {
        self.engine.transcode_eof(input, input_index, output, output_index)
    }

    /// Finishes retained converter output.
    ///
    /// # Errors
    ///
    /// Returns a transcode convert error when pending or final decoded values
    /// cannot be encoded or written to the target output buffer. Call this
    /// after EOF input is handled. A preflight index or capacity failure can
    /// be corrected and retried; after finalization starts, a failure may
    /// change state, so successfully reset before reuse. A successful finish
    /// closes the stream.
    #[inline]
    pub fn finish(
        &mut self,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<usize, TranscodeConvertErrorOf<D, E>> {
        self.engine.finish(output, output_index)
    }

    /// Runs a complete `reset -> transcode -> finish` conversion stream. This
    /// helper initializes and closes the stream itself; do not call `reset`
    /// first. Use the streaming methods for chunked input.
    ///
    /// # Errors
    ///
    /// Returns a transcode convert error when source decoding or target
    /// encoding fails under the configured policies, or when the supplied
    /// output buffer is too small.
    #[inline]
    pub fn transcode_complete_into(
        &mut self,
        input: &[D::Unit],
        output: &mut [E::Unit],
    ) -> Result<usize, TranscodeConvertErrorOf<D, E>> {
        <Self as Transcoder>::transcode_complete_into(self, input, output)
    }

    /// Returns the default encode policy that can be represented by `target`.
    ///
    /// # Panics
    ///
    /// Panics when neither the default replacement nor the fallback replacement
    /// can be encoded by `target`. This panic is intentional: reaching this
    /// branch means the target codec implementation violates the replacement
    /// fallback invariant and should fail fast.
    fn default_encode_policy(target: &E) -> (CharsetEncodePolicy, CharsetEncodeHooks) {
        let policy = CharsetEncodePolicy::default_for(target).unwrap_or_else(|error| {
            // This panic is intentional. If default replacement selection gets
            // here, the target codec cannot encode even the required fallback
            // `?`. That violates the codec invariant expected by this API, so
            // construction fails fast to expose the broken codec implementation.
            panic!(
                "cannot initialize CharsetConverter target for {:?}: neither {:?} nor {:?} is encodable ({error})",
                target.charset(),
                CharsetEncodePolicy::DEFAULT_REPLACEMENT,
                CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT,
            )
        });
        let hooks = CharsetEncoder::<E>::create_hooks(target, policy)
            // A policy chosen by `default_encode_policy` must be encodable;
            // failing here can only mean the target codec violates the
            // replacement fallback invariant. This panic is deliberate for the
            // same fail-fast reason as the default-policy panic above.
            .expect(
                "validated default encode policy should create hooks; \
                 failure means the codec violated its replacement invariant",
            );
        (policy, hooks)
    }
}

impl<D, E> Transcoder for CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetCodec,
{
    type Input = D::Unit;
    type Output = E::Unit;
    type Error = TranscodeConvertErrorOf<D, E>;

    /// Returns the target-side upper bound for converted output units.
    #[inline]
    fn max_transcode_output_len(&self, input_len: usize) -> Result<usize, CapacityError> {
        self.engine.max_transcode_output_len(input_len)
    }

    /// Returns the maximum target units needed to finalize pending conversion
    /// state.
    #[inline]
    fn max_finish_output_len(&self) -> Result<usize, CapacityError> {
        self.engine.max_finish_output_len()
    }

    /// Returns the maximum target units emitted when resetting stream state.
    #[inline]
    fn max_reset_output_len(&self) -> Result<usize, CapacityError> {
        self.engine.max_reset_output_len()
    }

    /// Clears any pending decoded character and emits stream-start encode
    /// output.
    #[inline]
    fn reset(&mut self, output: &mut [E::Unit], output_index: usize) -> Result<usize, Self::Error> {
        self.engine.reset(output, output_index)
    }

    /// Converts source units to target units through the configured decoder and
    /// encoder.
    ///
    /// # Errors
    ///
    /// Returns framework failures when indices or output capacity are invalid,
    /// and domain errors when source decoding or target encoding fails.
    #[inline]
    fn transcode(
        &mut self,
        input: &[D::Unit],
        input_index: usize,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, Self::Error> {
        self.engine.transcode(input, input_index, output, output_index)
    }

    /// Converts source units after the caller has established end of input.
    ///
    /// # Errors
    ///
    /// Returns framework failures when indices or output capacity are invalid,
    /// and domain errors when EOF source input violates the configured decode
    /// policy or target encoding policy.
    #[inline]
    fn transcode_eof(
        &mut self,
        input: &[D::Unit],
        input_index: usize,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, Self::Error> {
        CharsetConverter::transcode_eof(self, input, input_index, output, output_index)
    }

    /// Finalizes internally retained decoded characters and policy hook state.
    ///
    /// # Parameters
    ///
    /// - `output`: Complete output slice visible to the converter.
    /// - `output_index`: Absolute output index where writing starts.
    ///
    /// # Returns
    ///
    /// Returns the number of target units written during finalization.
    ///
    /// # Errors
    ///
    /// Returns framework failures when `output_index` is invalid or output
    /// capacity is insufficient, and domain errors when encoding pending or
    /// final decoded characters violates target charset policy.
    #[inline]
    fn finish(&mut self, output: &mut [E::Unit], output_index: usize) -> Result<usize, Self::Error> {
        self.engine.finish(output, output_index)
    }
}

impl<D, E> TranscodeConverter for CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetCodec,
{
    type DecodeError = CharsetDecodeError;
    type EncodeError = CharsetEncodeError;
    type Value = char;
}
