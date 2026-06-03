/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use super::{
    charset_codec::CharsetCodec,
    charset_convert_error::CharsetConvertError,
    charset_convert_hooks::CharsetConvertHooks,
    charset_decode_policy::CharsetDecodePolicy,
    charset_encode_policy::CharsetEncodePolicy,
    charset_encode_probe::CharsetEncodeProbe,
    charset_encoder::CharsetEncoder,
};
use crate::CharsetEncodeError;
use qubit_codec::{
    BufferedConvertEngine,
    BufferedConverter,
    CapacityError,
    FinishError,
    TranscodeProgress,
    Transcoder,
};

/// Converts units encoded with one charset into units encoded with another charset.
///
/// The converter owns the source and target charset codecs plus the same
/// decode/encode policy hooks used by [`crate::CharsetDecoder`] and
/// [`crate::CharsetEncoder`].
/// A decoded character may be kept pending inside the common buffered convert
/// engine when the target output buffer is full. During [`Transcoder::finish`],
/// the converter drains internally retained output and finishes the composed
/// decode/encode policy hooks. Callers remain responsible for handling any
/// incomplete input tail before finishing the logical stream.
///
/// # Type Parameters
///
/// - `D`: Low-level charset codec used by the source decoder.
/// - `E`: Low-level charset codec used by the target encoder.
///
/// ```rust
/// use qubit_codec_text::{
///     CharsetConverter,
///     CharsetDecoder,
///     CharsetEncoder,
///     TranscodeStatus,
///     Transcoder,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
    E::Unit: Default,
{
    /// Common buffered converter engine.
    engine: BufferedConvertEngine<D, E, CharsetConvertHooks>,
}

impl<D, E> CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
    E::Unit: Default,
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
    /// Returns a converter with default decoder and encoder policies.
    #[must_use]
    pub fn from_codecs(source: D, target: E) -> Self {
        Self {
            engine: BufferedConvertEngine::new(source, target, CharsetConvertHooks::default()),
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
        let _ = CharsetEncoder::<E>::create_hooks(&target, encode_policy)?;
        Ok(Self {
            engine: BufferedConvertEngine::new(
                source,
                target,
                CharsetConvertHooks::with_policies(decode_policy, encode_policy),
            ),
        })
    }
}

impl<D, E> Transcoder<D::Unit, E::Unit> for CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
    E::Unit: Default,
{
    type Error = CharsetConvertError;

    /// Returns the target-side upper bound for converted output units.
    fn max_output_len(&self, input_len: usize) -> Result<usize, CapacityError> {
        self.engine.max_output_len(input_len)
    }

    /// Returns the maximum target units needed to finalize pending conversion state.
    fn max_finish_output_len(&self) -> Result<usize, CapacityError> {
        self.engine.max_finish_output_len()
    }

    /// Clears any pending decoded character.
    fn reset(&mut self) {
        self.engine.reset();
    }

    /// Converts source units to target units through the configured decoder and encoder.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetConvertError::Decode`] when `input_index` is outside
    /// the source input buffer or source decoding fails. Returns
    /// [`CharsetConvertError::Encode`] when target encoding fails.
    fn transcode(
        &mut self,
        input: &[D::Unit],
        input_index: usize,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, Self::Error> {
        self.engine.transcode(input, input_index, output, output_index)
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
    /// Returns [`FinishError`] when `output_index` is invalid, when output
    /// capacity is insufficient, or when encoding pending or final decoded
    /// characters violates target charset policy.
    fn finish(&mut self, output: &mut [E::Unit], output_index: usize) -> Result<usize, FinishError<Self::Error>> {
        self.engine.finish(output, output_index)
    }
}

impl<D, E> BufferedConverter<D::Unit, E::Unit> for CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
    E::Unit: Default,
{
}
