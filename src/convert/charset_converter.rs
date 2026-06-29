// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use super::charset_convert_error::CharsetConvertError;
use crate::{
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeHooks,
    CharsetDecodePolicy,
    CharsetEncodeError,
    CharsetEncodeHooks,
    CharsetEncodePolicy,
    CharsetEncoder,
    MalformedAction,
    UnmappableAction,
};
use qubit_codec::{
    CapacityError,
    TranscodeConvertEngine,
    TranscodeConvertEngineError,
    TranscodeConverter,
    TranscodeDecodeEngineError,
    TranscodeEncodeEngineError,
    TranscodeError,
    TranscodeProgress,
    Transcoder,
};

type CharsetConvertEngineError = TranscodeConvertEngineError<
    TranscodeDecodeEngineError<CharsetDecodeError, CharsetDecodeError>,
    TranscodeEncodeEngineError<CharsetEncodeError, CharsetEncodeError>,
>;

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
/// responsible for handling any incomplete input tail before finishing the
/// logical stream.
///
/// # Type Parameters
///
/// - `D`: Low-level charset codec used by the source decoder.
/// - `E`: Low-level charset codec used by the target encoder.
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
#[derive(Debug)]
pub struct CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetCodec,
    E::Unit: Clone,
{
    /// Common buffered converter engine.
    engine: TranscodeConvertEngine<
        D,
        E,
        CharsetDecodeHooks,
        CharsetEncodeHooks<E::Unit>,
    >,
    /// Public malformed-input policy metadata.
    decode_policy: CharsetDecodePolicy,
    /// Public unmappable-input policy metadata.
    encode_policy: CharsetEncodePolicy,
}

impl<D, E> CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetCodec,
    E::Unit: Clone,
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
    /// implementations that cannot encode it must fail fast during converter
    /// construction rather than defer the invariant violation to user input.
    #[must_use]
    pub fn from_codecs(source: D, target: E) -> Self {
        let decode_policy = CharsetDecodePolicy::default();
        let (encode_policy, encode_hooks) =
            Self::default_encode_policy(&target);
        Self {
            engine: TranscodeConvertEngine::new(
                source,
                target,
                CharsetDecodeHooks::new(
                    decode_policy.malformed_action(),
                    decode_policy.replacement(),
                ),
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
        let (encode_hooks, _) =
            CharsetEncoder::<E>::create_hooks(&target, encode_policy)?;
        Ok(Self {
            engine: TranscodeConvertEngine::new(
                source,
                target,
                CharsetDecodeHooks::new(
                    decode_policy.malformed_action(),
                    decode_policy.replacement(),
                ),
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

    /// Returns the default encode policy that can be represented by `target`.
    ///
    /// # Panics
    ///
    /// Panics when neither the default replacement nor the fallback replacement
    /// can be encoded by `target`. This panic is intentional: reaching this
    /// branch means the target codec implementation violates the replacement
    /// fallback invariant and should fail fast.
    fn default_encode_policy(
        target: &E,
    ) -> (CharsetEncodePolicy, CharsetEncodeHooks<E::Unit>) {
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
        let (hooks, _) = CharsetEncoder::<E>::create_hooks(target, policy)
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

impl<D, E> Transcoder<D::Unit, E::Unit> for CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetCodec,
    E::Unit: Clone,
{
    type Error = CharsetConvertError;

    /// Returns the target-side upper bound for converted output units.
    #[inline]
    fn max_transcode_output_len(
        &self,
        input_len: usize,
    ) -> Result<usize, CapacityError> {
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
    fn reset(
        &mut self,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<usize, TranscodeError<Self::Error>> {
        self.engine
            .reset(output, output_index)
            .map_err(|error| error.map_domain(map_convert_engine_error))
    }

    /// Converts source units to target units through the configured decoder and
    /// encoder.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetConvertError::Decode`] when `input_index` is outside
    /// the source input buffer or source decoding fails. Returns
    /// [`CharsetConvertError::Encode`] when target encoding fails.
    #[inline]
    fn transcode(
        &mut self,
        input: &[D::Unit],
        input_index: usize,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, TranscodeError<Self::Error>> {
        self.engine
            .transcode(input, input_index, output, output_index)
            .map_err(|error| error.map_domain(map_convert_engine_error))
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
    /// Returns [`CharsetConvertError`] when `output_index` is invalid, when
    /// output capacity is insufficient, or when encoding pending or final
    /// decoded characters violates target charset policy.
    #[inline]
    fn finish(
        &mut self,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<usize, TranscodeError<Self::Error>> {
        self.engine
            .finish(output, output_index)
            .map_err(|error| error.map_domain(map_convert_engine_error))
    }
}

impl<D, E> TranscodeConverter<D::Unit, E::Unit> for CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetCodec,
    E::Unit: Clone,
{
    // empty
}

#[inline]
fn map_convert_engine_error(
    error: CharsetConvertEngineError,
) -> CharsetConvertError {
    match error {
        TranscodeConvertEngineError::Decode(error) => {
            CharsetConvertError::Decode(map_decode_engine_error(error))
        }
        TranscodeConvertEngineError::Encode(error) => {
            CharsetConvertError::Encode(map_encode_engine_error(error))
        }
    }
}

#[inline]
fn map_decode_engine_error(
    error: TranscodeDecodeEngineError<CharsetDecodeError, CharsetDecodeError>,
) -> CharsetDecodeError {
    match error {
        TranscodeDecodeEngineError::CodecDecode { source, .. }
        | TranscodeDecodeEngineError::CodecReset { source }
        | TranscodeDecodeEngineError::CodecFlush { source } => source,
        TranscodeDecodeEngineError::Hook(error) => error,
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
