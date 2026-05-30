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
    charset_decoder::CharsetDecoder,
    charset_encode_probe::CharsetEncodeProbe,
    charset_encoder::CharsetEncoder,
};
use crate::{
    BufferedConvertEngine,
    BufferedConverter,
    TranscodeProgress,
    Transcoder,
};

/// Converts units encoded with one charset into units encoded with another charset.
///
/// The converter owns a [`CharsetDecoder`] for the source charset and a
/// [`CharsetEncoder`] for the target charset. A decoded character may be kept
/// pending between calls when the target output buffer is full. During
/// [`Transcoder::finish`], the converter only drains internally retained output.
/// Callers remain responsible for handling any incomplete input tail before
/// finishing the logical stream.
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
/// let mut converter = CharsetConverter::new(
///     CharsetDecoder::new(Utf8Codec),
///     CharsetEncoder::new(Utf16U16Codec),
/// );
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
{
    /// Common buffered converter engine.
    engine: BufferedConvertEngine<CharsetDecoder<D>, CharsetEncoder<E>, CharsetConvertHooks, D::Unit>,
}

impl<D, E> CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
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
    #[inline]
    pub fn from_codecs(source: D, target: E) -> Self {
        Self::new(CharsetDecoder::new(source), CharsetEncoder::new(target))
    }

    /// Creates a charset converter from a decoder and an encoder.
    ///
    /// # Parameters
    ///
    /// - `decoder`: Charset decoder configured for the source charset.
    /// - `encoder`: Charset encoder configured for the target charset.
    ///
    /// # Returns
    ///
    /// Returns a converter that composes the supplied decoder and encoder.
    #[must_use]
    #[inline(always)]
    pub fn new(decoder: CharsetDecoder<D>, encoder: CharsetEncoder<E>) -> Self {
        Self {
            engine: BufferedConvertEngine::new(decoder, encoder, CharsetConvertHooks::new()),
        }
    }

    /// Returns the source decoder.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the configured decoder.
    #[must_use]
    #[inline(always)]
    pub const fn decoder(&self) -> &CharsetDecoder<D> {
        self.engine.decoder()
    }

    /// Returns the target encoder.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the configured encoder.
    #[must_use]
    #[inline(always)]
    pub const fn encoder(&self) -> &CharsetEncoder<E> {
        self.engine.encoder()
    }

    /// Returns a mutable source decoder.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the configured decoder.
    #[must_use]
    #[inline(always)]
    pub fn decoder_mut(&mut self) -> &mut CharsetDecoder<D> {
        self.engine.decoder_mut()
    }

    /// Returns a mutable target encoder.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the configured encoder.
    #[must_use]
    #[inline(always)]
    pub fn encoder_mut(&mut self) -> &mut CharsetEncoder<E> {
        self.engine.encoder_mut()
    }
}

impl<D, E> Transcoder<D::Unit, E::Unit> for CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
{
    type Error = CharsetConvertError;

    /// Returns the target-side upper bound for converted output units.
    #[inline(always)]
    fn max_output_len(&self, input_len: usize) -> Option<usize> {
        self.engine.max_output_len::<char, E::Unit>(input_len)
    }

    /// Returns the maximum target units needed to finalize pending conversion state.
    #[inline(always)]
    fn max_finish_output_len(&self) -> Option<usize> {
        self.engine.max_finish_output_len::<char, E::Unit>()
    }

    /// Clears any pending decoded character.
    #[inline(always)]
    fn reset(&mut self) {
        self.engine.reset::<char, E::Unit>();
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
        self.engine
            .transcode::<char, E::Unit>(input, input_index, output, output_index)
    }

    /// Finalizes pending decoded characters and source decoder state.
    ///
    /// # Parameters
    ///
    /// - `output`: Complete output slice visible to the converter.
    /// - `output_index`: Absolute output index where writing starts.
    ///
    /// # Returns
    ///
    /// Returns completed progress when no pending state exists. Returns
    /// `NeedOutput` when pending output cannot be flushed due to missing target
    /// capacity.
    ///
    /// # Errors
    ///
    /// Returns `CharsetConvertError::Decode` when the source decoder rejects
    /// incomplete EOF input. Returns `CharsetConvertError::Encode` when encoding
    /// pending decoded characters violates target charset policy.
    fn finish(&mut self, output: &mut [E::Unit], output_index: usize) -> Result<TranscodeProgress, Self::Error> {
        self.engine.finish::<char, E::Unit>(output, output_index)
    }
}

impl<D, E> BufferedConverter<D::Unit, E::Unit> for CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
{
}
