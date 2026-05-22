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
/// - `InputUnit`: Source storage unit type consumed by the decoder.
/// - `OutputUnit`: Target storage unit type produced by the encoder.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharsetConverter<D, E, InputUnit, OutputUnit> {
    /// Source charset decoder.
    decoder: CharsetDecoder<D, InputUnit>,
    /// Target charset encoder.
    encoder: CharsetEncoder<E, OutputUnit>,
    /// Decoded character waiting for target output capacity.
    pending: Option<char>,
}

impl<D, E, InputUnit, OutputUnit> CharsetConverter<D, E, InputUnit, OutputUnit> {
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
    pub const fn new(
        decoder: CharsetDecoder<D, InputUnit>,
        encoder: CharsetEncoder<E, OutputUnit>,
    ) -> Self {
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
    pub const fn decoder(&self) -> &CharsetDecoder<D, InputUnit> {
        &self.decoder
    }

    /// Returns the target encoder.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the configured encoder.
    #[must_use]
    #[inline]
    pub const fn encoder(&self) -> &CharsetEncoder<E, OutputUnit> {
        &self.encoder
    }

    /// Returns a mutable source decoder.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the configured decoder.
    #[must_use]
    #[inline]
    pub fn decoder_mut(&mut self) -> &mut CharsetDecoder<D, InputUnit> {
        &mut self.decoder
    }

    /// Returns a mutable target encoder.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the configured encoder.
    #[must_use]
    #[inline]
    pub fn encoder_mut(&mut self) -> &mut CharsetEncoder<E, OutputUnit> {
        &mut self.encoder
    }
}

impl<D, E, InputUnit, OutputUnit> Coder<InputUnit, OutputUnit>
    for CharsetConverter<D, E, InputUnit, OutputUnit>
where
    D: CharsetCodec<InputUnit>,
    E: CharsetCodec<OutputUnit>,
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
    fn convert(
        &mut self,
        input: &[InputUnit],
        input_index: usize,
        output: &mut [OutputUnit],
        output_index: usize,
    ) -> Result<CoderProgress, Self::Error> {
        let mut read = 0;
        let mut written = 0;

        if let Some(ch) = self.pending {
            let status = self.write_pending(ch, output, output_index, &mut written)?;
            if status == CoderStatus::NeedOutput {
                return Ok(CoderProgress::need_output(read, written));
            }
        }

        loop {
            let mut decoded = ['\0'; 1];
            let decode_progress =
                self.decoder
                    .convert(input, input_index + read, &mut decoded, 0)?;
            read += decode_progress.read();

            if decode_progress.written() == 1 {
                let ch = decoded[0];
                self.pending = Some(ch);
                let status = self.write_pending(ch, output, output_index, &mut written)?;
                if status == CoderStatus::NeedOutput {
                    return Ok(CoderProgress::need_output(read, written));
                }
            }

            match decode_progress.status() {
                CoderStatus::Complete => {
                    return Ok(CoderProgress::complete(read, written));
                }
                CoderStatus::NeedInput => {
                    return Ok(CoderProgress::need_input(read, written));
                }
                CoderStatus::NeedOutput => {
                    continue;
                }
            }
        }
    }
}

impl<D, E, InputUnit, OutputUnit> CharsetConverter<D, E, InputUnit, OutputUnit> {
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
    /// Returns [`CharsetConvertError::Encode`] when target encoding fails according
    /// to the configured encoder policy.
    #[inline]
    fn write_pending(
        &mut self,
        ch: char,
        output: &mut [OutputUnit],
        output_index: usize,
        written: &mut usize,
    ) -> Result<CoderStatus, CharsetConvertError>
    where
        E: CharsetCodec<OutputUnit>,
    {
        let single = [ch];
        let encode_progress = self
            .encoder
            .convert(&single, 0, output, output_index + *written)?;
        *written += encode_progress.written();
        if encode_progress.read() == 1 {
            self.pending = None;
            Ok(CoderStatus::Complete)
        } else {
            Ok(CoderStatus::NeedOutput)
        }
    }
}
