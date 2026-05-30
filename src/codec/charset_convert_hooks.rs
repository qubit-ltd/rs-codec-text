/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Policy hooks used by charset converters.

use core::num::NonZeroUsize;

use qubit_codec::{
    BufferedConvertHooks,
    ConvertDecodeResult,
    ConvertState,
    ConvertWriteResult,
};

use crate::{
    TranscodeProgress,
    TranscodeStatus,
    Transcoder,
};

use super::{
    charset_codec::CharsetCodec,
    charset_convert_error::CharsetConvertError,
    charset_decoder::CharsetDecoder,
    charset_encode_probe::CharsetEncodeProbe,
    charset_encoder::CharsetEncoder,
};

/// Policy hooks for [`super::CharsetConverter`].
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub(super) struct CharsetConvertHooks {
    /// Decoded character waiting for target output capacity.
    pending: Option<char>,
}

impl CharsetConvertHooks {
    /// Creates charset converter hooks.
    ///
    /// # Returns
    ///
    /// Returns hooks with no pending decoded character.
    #[must_use]
    #[inline(always)]
    pub(super) const fn new() -> Self {
        Self { pending: None }
    }

    /// Writes one pending character through the target encoder.
    ///
    /// # Parameters
    ///
    /// - `encoder`: Target charset encoder.
    /// - `ch`: Pending character to encode.
    /// - `output`: Complete output slice visible to the converter.
    /// - `output_index`: Absolute output index where writing starts.
    ///
    /// # Returns
    ///
    /// Returns encoder progress for the single character.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetConvertError::Encode`] when target encoding fails.
    #[inline]
    fn write_char<E>(
        &mut self,
        encoder: &mut CharsetEncoder<E>,
        ch: char,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, CharsetConvertError>
    where
        E: CharsetEncodeProbe,
    {
        let single = [ch];
        let progress = encoder.transcode(&single, 0, output, output_index)?;
        if !matches!(progress.status(), TranscodeStatus::NeedOutput { .. }) {
            self.pending = None;
        }
        Ok(progress)
    }

    /// Writes a retained pending character, if any.
    ///
    /// # Parameters
    ///
    /// - `encoder`: Target charset encoder.
    /// - `state`: Current conversion state.
    ///
    /// # Returns
    ///
    /// Returns `Some(progress)` when output capacity is missing, or `None` when
    /// conversion may continue.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetConvertError::Encode`] when target encoding fails.
    #[inline]
    fn drain_pending_into_state<D, E>(
        &mut self,
        encoder: &mut CharsetEncoder<E>,
        state: &mut ConvertState<'_, D::Unit, E::Unit>,
    ) -> Result<Option<TranscodeProgress>, CharsetConvertError>
    where
        D: CharsetCodec,
        E: CharsetEncodeProbe,
    {
        let Some(ch) = self.pending else {
            return Ok(None);
        };
        let output_cursor = state.output_cursor();
        let progress = self.write_char(encoder, ch, state.output_mut(), output_cursor)?;
        state.advance_output(progress.written());
        if matches!(progress.status(), TranscodeStatus::NeedOutput { .. }) {
            return Ok(Some(state.need_output_progress(
                NonZeroUsize::new(progress.additional()).unwrap_or(NonZeroUsize::MIN),
                progress.available(),
            )));
        }
        Ok(None)
    }
}

impl<D, E> BufferedConvertHooks<CharsetDecoder<D>, CharsetEncoder<E>, D::Unit, char, E::Unit> for CharsetConvertHooks
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
{
    type Error = CharsetConvertError;

    /// Returns one target character width for invalid output indices.
    #[inline(always)]
    fn invalid_output_additional(&self, _decoder: &CharsetDecoder<D>, encoder: &CharsetEncoder<E>) -> NonZeroUsize {
        encoder.codec().max_units_per_value()
    }

    /// Returns the target-side upper bound for converted output units.
    #[inline(always)]
    fn max_output_len(
        &self,
        decoder: &CharsetDecoder<D>,
        encoder: &CharsetEncoder<E>,
        input_len: usize,
    ) -> Option<usize> {
        let units_per_char = encoder.codec().max_units_per_value().get();
        let pending_units = usize::from(self.pending.is_some()).checked_mul(units_per_char)?;
        pending_units.checked_add(decoder.max_output_len(input_len)?.checked_mul(units_per_char)?)
    }

    /// Returns the maximum target units needed to finalize pending conversion state.
    #[inline(always)]
    fn max_finish_output_len(&self, decoder: &CharsetDecoder<D>, encoder: &CharsetEncoder<E>) -> Option<usize> {
        let units_per_char = encoder.codec().max_units_per_value().get();
        let pending_units = usize::from(self.pending.is_some()) * units_per_char;
        let decoder_units = decoder.max_finish_output_len()?.checked_mul(units_per_char)?;
        pending_units
            .checked_add(decoder_units)?
            .checked_add(encoder.max_finish_output_len()?)
    }

    /// Clears pending state and resets source/target components.
    #[inline(always)]
    fn reset(&mut self, decoder: &mut CharsetDecoder<D>, encoder: &mut CharsetEncoder<E>) {
        self.pending = None;
        decoder.reset();
        encoder.reset();
    }

    /// Writes a retained decoded character before consuming new source input.
    #[inline]
    fn drain_pending(
        &mut self,
        _decoder: &mut CharsetDecoder<D>,
        encoder: &mut CharsetEncoder<E>,
        state: &mut ConvertState<'_, D::Unit, E::Unit>,
    ) -> Result<Option<TranscodeProgress>, Self::Error> {
        self.drain_pending_into_state::<D, E>(encoder, state)
    }

    /// Decodes one source character.
    #[inline]
    fn decode_next(
        &mut self,
        decoder: &mut CharsetDecoder<D>,
        state: &mut ConvertState<'_, D::Unit, E::Unit>,
    ) -> Result<ConvertDecodeResult<char>, Self::Error> {
        let mut decoded = ['\0'; 1];
        let decode_progress = decoder.transcode(state.input(), state.input_cursor(), &mut decoded, 0)?;
        let decode_status = decode_progress.status();
        if decode_progress.written() > 0 {
            return Ok(ConvertDecodeResult::Decoded {
                value: decoded[0],
                consumed: NonZeroUsize::new(decode_progress.read()).unwrap_or(NonZeroUsize::MIN),
            });
        }
        if decode_progress.read() > 0 {
            return Ok(ConvertDecodeResult::Skipped {
                consumed: NonZeroUsize::new(decode_progress.read()).expect("non-zero read"),
            });
        }

        match decode_status {
            TranscodeStatus::Complete => Ok(ConvertDecodeResult::NeedInput {
                additional: NonZeroUsize::MIN,
                available: 0,
            }),
            TranscodeStatus::NeedInput { .. } => Ok(ConvertDecodeResult::NeedInput {
                additional: NonZeroUsize::new(decode_progress.additional()).unwrap_or(NonZeroUsize::MIN),
                available: decode_progress.available(),
            }),
            TranscodeStatus::NeedOutput { .. } => {
                unreachable!("one-character decode buffer cannot be full without a decoded character")
            }
        }
    }

    /// Writes one decoded character.
    #[inline]
    fn write_value(
        &mut self,
        encoder: &mut CharsetEncoder<E>,
        value: char,
        state: &mut ConvertState<'_, D::Unit, E::Unit>,
    ) -> Result<ConvertWriteResult, Self::Error> {
        self.pending = Some(value);
        let output_cursor = state.output_cursor();
        let progress = self.write_char(encoder, value, state.output_mut(), output_cursor)?;
        if matches!(progress.status(), TranscodeStatus::NeedOutput { .. }) {
            return Ok(ConvertWriteResult::NeedOutput {
                additional: NonZeroUsize::new(progress.additional()).unwrap_or(NonZeroUsize::MIN),
                available: progress.available(),
                written: progress.written(),
            });
        }
        Ok(ConvertWriteResult::Written {
            written: progress.written(),
        })
    }

    /// Finalizes pending decoded characters and source decoder state.
    #[inline]
    fn finish(
        &mut self,
        decoder: &mut CharsetDecoder<D>,
        encoder: &mut CharsetEncoder<E>,
        output: &mut [E::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, Self::Error> {
        let mut written = 0;

        if let Some(ch) = self.pending {
            let progress = self.write_char(encoder, ch, output, output_index + written)?;
            written += progress.written();
            if matches!(progress.status(), TranscodeStatus::NeedOutput { .. }) {
                return Ok(TranscodeProgress::need_output(
                    output_index + written,
                    progress.additional(),
                    progress.available(),
                    0,
                    written,
                ));
            }
        }

        loop {
            let mut decoded = ['\0'; 1];
            let decode_finish = decoder.finish(&mut decoded, 0)?;
            for &ch in decoded.iter().take(decode_finish.written()) {
                self.pending = Some(ch);
                let progress = self.write_char(encoder, ch, output, output_index + written)?;
                written += progress.written();
                if matches!(progress.status(), TranscodeStatus::NeedOutput { .. }) {
                    return Ok(TranscodeProgress::need_output(
                        output_index + written,
                        progress.additional(),
                        progress.available(),
                        0,
                        written,
                    ));
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

        let encoder_finish = encoder.finish(output, output_index + written)?;
        written += encoder_finish.written();
        if matches!(encoder_finish.status(), TranscodeStatus::NeedOutput { .. }) {
            return Ok(TranscodeProgress::need_output(
                output_index + written,
                encoder_finish.additional(),
                encoder_finish.available(),
                0,
                written,
            ));
        }
        debug_assert_eq!(
            TranscodeStatus::Complete,
            encoder_finish.status(),
            "encoder finish should not request input",
        );

        Ok(TranscodeProgress::complete(0, written))
    }
}
