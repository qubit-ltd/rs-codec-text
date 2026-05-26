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
    charset_encoder::CharsetEncoder,
};
use crate::{
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    Coder,
    CoderProgress,
    CoderStatus,
};

/// Converts units encoded with one charset into units encoded with another charset.
///
/// The converter owns a [`CharsetDecoder`] for the source charset and a
/// [`CharsetEncoder`] for the target charset. A decoded character may be kept
/// pending between calls when the target output buffer is full.
///
/// # Type Parameters
///
/// - `D`: Low-level charset codec used by the source decoder.
/// - `E`: Low-level charset codec used by the target encoder.
///
/// ```rust
/// use qubit_text_codec::{Coder, CharsetDecoder, CharsetEncoder, CharsetConverter, Utf8Codec, Utf16U16Codec};
///
/// let mut converter = CharsetConverter::new(
///     CharsetDecoder::new(Utf8Codec),
///     CharsetEncoder::new(Utf16U16Codec),
/// );
/// let mut output = [0_u16; 2];
///
/// let progress = converter
///     .convert("AB".as_bytes(), 0, &mut output, 0)
///     .expect("convert bytes to utf-16");
///
/// assert_eq!(2, progress.read());
/// assert_eq!(2, progress.written());
/// assert_eq!([65, 66], output);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetCodec,
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
    /// - `decoder`: Decoder configured for the source charset.
    /// - `encoder`: Encoder configured for the target charset.
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
    /// Returns [`CoderStatus::Complete`] when the pending character was written.
    /// Returns [`CoderStatus::NeedOutput`] when it must stay pending for a later call.
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
    ) -> Result<CoderProgress, CharsetConvertError> {
        let single = [ch];
        let encode_progress = self.encoder.convert(&single, 0, output, output_index + *written)?;
        if encode_progress.status() == CoderStatus::Complete && encode_progress.read() == 1 {
            self.pending = None;
        }
        *written += encode_progress.written();
        Ok(encode_progress)
    }
}

impl<D, E> Coder<D::Unit, E::Unit> for CharsetConverter<D, E>
where
    D: CharsetCodec,
    E: CharsetCodec,
{
    type Error = CharsetConvertError;

    /// Returns the target-side upper bound for converted output units.
    #[inline]
    fn max_output_len(&self, input_len: usize) -> Option<usize> {
        input_len.checked_mul(self.encoder.codec().max_units_per_char())
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
    fn convert(
        &mut self,
        input: &[D::Unit],
        input_index: usize,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<CoderProgress, Self::Error> {
        if input_index > input.len() {
            let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
            return Err(CharsetDecodeError::new(self.decoder.codec().charset(), kind, input_index).into());
        }

        let mut read = 0;
        let mut written = 0;

        if let Some(ch) = self.pending {
            let status = self.write_pending(ch, output, output_index, &mut written)?;
            if matches!(status.status(), CoderStatus::NeedOutput { .. }) {
                let status = CoderStatus::NeedOutput {
                    output_index: output_index + written,
                    required: status.required(),
                    available: status.available(),
                };
                return Ok(CoderProgress::new(status, read, written));
            }
        }

        while input_index + read < input.len() {
            let mut decoded = ['\0'; 4];
            let decode_progress = self.decoder.convert(input, input_index + read, &mut decoded, 0)?;
            let decode_status = decode_progress.status();
            let decode_read = decode_progress.read();
            read += decode_read;

            if decode_progress.written() > 0 {
                for &ch in decoded.iter().take(decode_progress.written()) {
                    self.pending = Some(ch);
                    let status = self.write_pending(ch, output, output_index, &mut written)?;
                    if matches!(status.status(), CoderStatus::NeedOutput { .. }) {
                        let status = CoderStatus::NeedOutput {
                            output_index: output_index + written,
                            required: status.required(),
                            available: status.available(),
                        };
                        return Ok(CoderProgress::new(status, read, written));
                    }
                }
            }

            match decode_status {
                CoderStatus::Complete if input_index + read >= input.len() || decode_read == 0 => {
                    return Ok(CoderProgress::complete(read, written));
                }
                CoderStatus::Complete => {}
                CoderStatus::NeedInput { .. } => {
                    let status = CoderStatus::NeedInput {
                        input_index: input_index + read,
                        required: decode_progress.required(),
                        available: decode_progress.available(),
                    };
                    return Ok(CoderProgress::new(status, read, written));
                }
                CoderStatus::NeedOutput { .. } => {
                    debug_assert!(
                        decode_read > 0,
                        "Decoder must consume at least one input unit when reporting NeedOutput"
                    );
                }
            }
        }

        Ok(CoderProgress::complete(read, written))
    }

    /// Flushes one pending decoded character, if any.
    ///
    /// # Parameters
    ///
    /// - `output`: Complete output slice visible to the converter.
    /// - `output_index`: Absolute output index where writing starts.
    ///
    /// # Returns
    ///
    /// Returns completed progress when no pending character exists.
    /// Returns `NeedOutput` when pending output cannot be flushed due to
    /// missing output capacity.
    ///
    /// # Errors
    ///
    /// Returns `CharsetConvertError::Encode` when encoding the pending
    /// character violates target charset policy.
    #[inline]
    fn finish(&mut self, output: &mut [E::Unit], output_index: usize) -> Result<CoderProgress, Self::Error> {
        if let Some(ch) = self.pending {
            let mut written = 0;
            let status = self.write_pending(ch, output, output_index, &mut written)?;
            if matches!(status.status(), CoderStatus::NeedOutput { .. }) {
                let status = CoderStatus::NeedOutput {
                    output_index: output_index + written,
                    required: status.required(),
                    available: status.available(),
                };
                return Ok(CoderProgress::new(status, 0, written));
            }
            return Ok(CoderProgress::complete(0, written));
        }
        Ok(CoderProgress::complete(0, 0))
    }
}
