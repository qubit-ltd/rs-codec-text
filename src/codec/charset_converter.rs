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
    charset_decoder::CharsetDecoder,
    charset_encode_probe::CharsetEncodeProbe,
    charset_encoder::CharsetEncoder,
};
use crate::{
    BufferedConverter,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    TranscodeProgress,
    TranscodeStatus,
    Transcoder,
};

/// Converts units encoded with one charset into units encoded with another charset.
///
/// The converter owns a [`CharsetDecoder`] for the source charset and a
/// [`CharsetEncoder`] for the target charset. A decoded character may be kept
/// pending between calls when the target output buffer is full. During
/// [`Transcoder::finish`], the converter also finalizes the source decoder so
/// incomplete trailing input is handled by the decoder's malformed-input policy
/// before any resulting replacement character is encoded into the target buffer.
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
    /// Source charset decoder.
    decoder: CharsetDecoder<D>,
    /// Target charset encoder.
    encoder: CharsetEncoder<E>,
    /// Decoded character waiting for target output capacity.
    pending: Option<char>,
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
    #[inline]
    pub fn new(decoder: CharsetDecoder<D>, encoder: CharsetEncoder<E>) -> Self {
        Self {
            decoder,
            encoder,
            pending: None,
        }
    }

    /// Returns the source decoder.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the configured decoder.
    #[must_use]
    #[inline]
    pub const fn decoder(&self) -> &CharsetDecoder<D> {
        &self.decoder
    }

    /// Returns the target encoder.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the configured encoder.
    #[must_use]
    #[inline]
    pub const fn encoder(&self) -> &CharsetEncoder<E> {
        &self.encoder
    }

    /// Returns a mutable source decoder.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the configured decoder.
    #[must_use]
    #[inline]
    pub fn decoder_mut(&mut self) -> &mut CharsetDecoder<D> {
        &mut self.decoder
    }

    /// Returns a mutable target encoder.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the configured encoder.
    #[must_use]
    #[inline]
    pub fn encoder_mut(&mut self) -> &mut CharsetEncoder<E> {
        &mut self.encoder
    }

    /// Writes the pending character through the target encoder.
    ///
    /// # Parameters
    ///
    /// - `ch`: Pending character to encode.
    /// - `output`: Complete output slice visible to the converter.
    /// - `output_index`: Absolute output index where this conversion call started.
    /// - `written`: Number of output units already written by this conversion call.
    ///
    /// # Returns
    ///
    /// Returns [`TranscodeStatus::Complete`] when the pending character was written.
    /// Returns [`TranscodeStatus::NeedOutput`] when it must stay pending for a later call.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetConvertError::Encode`] when target encoding fails
    /// according to the configured encoder policy.
    #[inline]
    fn write_pending(
        &mut self,
        ch: char,
        output: &mut [E::Unit],
        output_index: usize,
        written: &mut usize,
    ) -> Result<TranscodeProgress, CharsetConvertError> {
        let single = [ch];
        let encode_progress = self.encoder.transcode(&single, 0, output, output_index + *written)?;
        if !matches!(encode_progress.status(), TranscodeStatus::NeedOutput { .. }) {
            self.pending = None;
        }
        *written += encode_progress.written();
        Ok(encode_progress)
    }
}

impl<D, E> Transcoder<D::Unit, E::Unit> for CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
{
    type Error = CharsetConvertError;

    /// Returns the target-side upper bound for converted output units.
    #[inline]
    fn max_output_len(&self, input_len: usize) -> Option<usize> {
        input_len.checked_mul(self.encoder.codec().max_units_per_value())
    }

    /// Returns the maximum target units needed to finalize pending conversion state.
    #[inline]
    fn max_finish_output_len(&self) -> Option<usize> {
        let units_per_char = self.encoder.codec().max_units_per_value();
        let pending_units = usize::from(self.pending.is_some()) * units_per_char;
        let decoder_units = self.decoder.max_finish_output_len()?.checked_mul(units_per_char)?;
        pending_units
            .checked_add(decoder_units)?
            .checked_add(self.encoder.max_finish_output_len()?)
    }

    /// Clears any pending decoded character.
    #[inline]
    fn reset(&mut self) {
        self.pending = None;
        self.decoder.reset();
        self.encoder.reset();
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
        if input_index > input.len() {
            let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
            return Err(CharsetConvertError::Decode(CharsetDecodeError::new(
                self.decoder.codec().charset(),
                kind,
                input_index,
            )));
        }

        let mut read = 0;
        let mut written = 0;

        if let Some(ch) = self.pending {
            let status = self.write_pending(ch, output, output_index, &mut written)?;
            if matches!(status.status(), TranscodeStatus::NeedOutput { .. }) {
                let status = TranscodeStatus::NeedOutput {
                    output_index: output_index + written,
                    required: status.required(),
                    available: status.available(),
                };
                return Ok(TranscodeProgress::new(status, read, written));
            }
        }

        while input_index + read < input.len() {
            let mut decoded = ['\0'; 1];
            let decode_progress = self.decoder.transcode(input, input_index + read, &mut decoded, 0)?;
            let decode_status = decode_progress.status();
            let decode_read = decode_progress.read();
            read += decode_read;

            if decode_progress.written() > 0 {
                for &ch in decoded.iter().take(decode_progress.written()) {
                    self.pending = Some(ch);
                    let status = self.write_pending(ch, output, output_index, &mut written)?;
                    if matches!(status.status(), TranscodeStatus::NeedOutput { .. }) {
                        let status = TranscodeStatus::NeedOutput {
                            output_index: output_index + written,
                            required: status.required(),
                            available: status.available(),
                        };
                        return Ok(TranscodeProgress::new(status, read, written));
                    }
                }
            }

            match decode_status {
                TranscodeStatus::Complete => return Ok(TranscodeProgress::complete(read, written)),
                TranscodeStatus::NeedInput { .. } => {
                    let status = TranscodeStatus::NeedInput {
                        input_index: input_index + read,
                        required: decode_progress.required(),
                        available: decode_progress.available(),
                    };
                    return Ok(TranscodeProgress::new(status, read, written));
                }
                TranscodeStatus::NeedOutput { .. } => {
                    debug_assert!(
                        decode_read > 0,
                        "Charset decoder must consume at least one input unit when reporting NeedOutput"
                    );
                }
            }
        }

        Ok(TranscodeProgress::complete(read, written))
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
        let mut written = 0;

        if let Some(ch) = self.pending {
            let status = self.write_pending(ch, output, output_index, &mut written)?;
            if matches!(status.status(), TranscodeStatus::NeedOutput { .. }) {
                let status = TranscodeStatus::NeedOutput {
                    output_index: output_index + written,
                    required: status.required(),
                    available: status.available(),
                };
                return Ok(TranscodeProgress::new(status, 0, written));
            }
        }

        loop {
            let mut decoded = ['\0'; 1];
            let decode_finish = self.decoder.finish(&mut decoded, 0)?;
            for &ch in decoded.iter().take(decode_finish.written()) {
                self.pending = Some(ch);
                let status = self.write_pending(ch, output, output_index, &mut written)?;
                if matches!(status.status(), TranscodeStatus::NeedOutput { .. }) {
                    let status = TranscodeStatus::NeedOutput {
                        output_index: output_index + written,
                        required: status.required(),
                        available: status.available(),
                    };
                    return Ok(TranscodeProgress::new(status, 0, written));
                }
            }
            if decode_finish.status() == TranscodeStatus::Complete {
                break;
            }
            debug_assert!(
                decode_finish.written() > 0,
                "decoder finish has one output slot and should only request more output after writing",
            );
        }

        Ok(TranscodeProgress::complete(0, written))
    }
}

impl<D, E> BufferedConverter<D::Unit, E::Unit> for CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
{
}
