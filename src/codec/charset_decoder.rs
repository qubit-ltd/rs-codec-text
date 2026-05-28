/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use crate::{
    BufferedDecoder,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    TranscodeProgress,
    TranscodeStatus,
    Transcoder,
};

use super::{
    charset_codec::CharsetCodec,
    malformed_action::MalformedAction,
};

/// Converts units of one charset into Unicode scalar values.
///
/// `CharsetDecoder` wraps a low-level [`CharsetCodec`] and applies the
/// configured [`MalformedAction`] whenever the codec reports malformed input.
/// The decoder asks the wrapped codec whether one value can be decoded from the
/// currently available units. If the codec reports a valid incomplete prefix,
/// the tail is retained and [`TranscodeStatus::NeedInput`] is returned. When
/// [`Transcoder::finish`] marks EOF, the retained closed tail is handled by the
/// configured malformed-input policy if it is still incomplete or malformed.
///
/// # Type Parameters
///
/// - `C`: Low-level charset codec used to decode source storage units into one
///   Unicode scalar value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharsetDecoder<C>
where
    C: CharsetCodec,
{
    /// Low-level codec used for source decoding.
    codec: C,
    /// Action used for malformed input units.
    malformed_action: MalformedAction,
    /// Replacement character used by [`MalformedAction::Replace`].
    replacement: char,
    /// Buffered source units for the next character.
    pending_input: Vec<C::Unit>,
    /// Absolute source index of the first buffered source unit.
    pending_input_index: usize,
}

impl<C> CharsetDecoder<C>
where
    C: CharsetCodec,
{
    /// Default replacement character used when malformed input is replaced.
    pub const DEFAULT_REPLACEMENT: char = '\u{fffd}';

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
    #[must_use]
    #[inline]
    pub const fn new(codec: C) -> Self {
        Self {
            codec,
            malformed_action: MalformedAction::Replace,
            replacement: Self::DEFAULT_REPLACEMENT,
            pending_input: Vec::new(),
            pending_input_index: 0,
        }
    }

    /// Creates a decoder with a custom replacement character.
    ///
    /// This method performs no codec-level validation because malformed-input
    /// replacement for decoding writes directly to the output `char` buffer.
    ///
    /// # Parameters
    ///
    /// - `replacement`: Replacement character for malformed sequences.
    ///
    /// # Returns
    ///
    /// Returns a new decoder configured with the provided replacement.
    #[inline]
    pub fn with_replacement(mut self, replacement: char) -> Self {
        self.replacement = replacement;
        self
    }

    /// Returns the wrapped low-level codec.
    ///
    /// # Returns
    ///
    /// Returns a shared reference to the configured codec.
    #[must_use]
    #[inline]
    pub const fn codec(&self) -> &C {
        &self.codec
    }

    /// Returns a mutable reference to the wrapped codec.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the configured codec.
    #[must_use]
    #[inline]
    pub fn codec_mut(&mut self) -> &mut C {
        &mut self.codec
    }

    /// Returns the configured malformed-input action.
    ///
    /// # Returns
    ///
    /// Returns the action used when source input is malformed.
    #[must_use]
    #[inline]
    pub const fn malformed_action(&self) -> MalformedAction {
        self.malformed_action
    }

    /// Sets the malformed-input action.
    ///
    /// # Parameters
    ///
    /// - `action`: New policy for malformed input units.
    #[inline]
    pub fn set_malformed_action(&mut self, action: MalformedAction) {
        self.malformed_action = action;
    }

    /// Returns the configured replacement character.
    ///
    /// # Returns
    ///
    /// Returns the character emitted when [`MalformedAction::Replace`] is used.
    #[must_use]
    #[inline]
    pub const fn replacement(&self) -> char {
        self.replacement
    }

    /// Sets the replacement character.
    ///
    /// # Parameters
    ///
    /// - `replacement`: New replacement character used by replace policy.
    #[inline]
    pub fn set_replacement(&mut self, replacement: char) {
        self.replacement = replacement;
    }
}

impl<C> Transcoder<C::Unit, char> for CharsetDecoder<C>
where
    C: CharsetCodec,
{
    type Error = CharsetDecodeError;

    /// Returns the maximum number of characters decoded from `input_len` units.
    #[inline]
    fn max_output_len(&self, input_len: usize) -> Option<usize> {
        Some(input_len)
    }

    /// Returns the maximum number of characters emitted by EOF finalization.
    #[inline]
    fn max_finish_output_len(&self) -> Option<usize> {
        match (self.pending_input.is_empty(), self.malformed_action) {
            (false, MalformedAction::Replace) => Some(self.pending_input.len()),
            (false, MalformedAction::Ignore | MalformedAction::Report) | (true, _) => Some(0),
        }
    }

    /// Clears the pending incomplete sequence while keeping decoder policy.
    #[inline]
    fn reset(&mut self) {
        self.pending_input.clear();
        self.pending_input_index = 0;
    }

    /// Decodes source units into Unicode scalar values while applying malformed policy.
    fn transcode(
        &mut self,
        input: &[C::Unit],
        input_index: usize,
        output: &mut [char],
        output_index: usize,
    ) -> Result<TranscodeProgress, Self::Error> {
        if input_index > input.len() {
            let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len: input.len() };
            return Err(CharsetDecodeError::new(self.codec.charset(), kind, input_index));
        }
        if output_index > output.len() {
            let status = TranscodeStatus::NeedOutput {
                output_index,
                required: 1,
                available: 0,
            };
            return Ok(TranscodeProgress::new(status, 0, 0));
        }

        let mut input_cursor = input_index;
        let mut output_cursor = output_index;
        let min_units = self.codec.min_units_per_value();
        loop {
            if self.pending_input.is_empty() && input_cursor >= input.len() {
                return Ok(TranscodeProgress::complete(
                    input_cursor - input_index,
                    output_cursor - output_index,
                ));
            }

            if output_cursor == output.len() {
                let status = TranscodeStatus::NeedOutput {
                    output_index: output_cursor,
                    required: 1,
                    available: 0,
                };
                return Ok(TranscodeProgress::new(
                    status,
                    input_cursor - input_index,
                    output_cursor - output_index,
                ));
            }

            if !self.pending_input.is_empty() {
                if self.pending_input.len() < min_units {
                    let additional = min_units - self.pending_input.len();
                    let copied = additional.min(input.len() - input_cursor);
                    if copied > 0 {
                        self.pending_input
                            .extend_from_slice(&input[input_cursor..input_cursor + copied]);
                        input_cursor += copied;
                        continue;
                    }
                    let status = TranscodeStatus::NeedInput {
                        input_index: input_cursor,
                        required: additional,
                        available: self.pending_input.len(),
                    };
                    return Ok(TranscodeProgress::new(
                        status,
                        input_cursor - input_index,
                        output_cursor - output_index,
                    ));
                }

                match unsafe { self.codec.decode_unchecked(&self.pending_input, 0) } {
                    Ok((value, consumed)) => {
                        assert!(consumed > 0, "Codec::decode_unchecked consumed zero input units");
                        assert!(
                            consumed <= self.pending_input.len(),
                            "Codec::decode_unchecked consumed beyond buffered input",
                        );
                        output[output_cursor] = value;
                        self.drain_pending(consumed);
                        output_cursor += 1;
                    }
                    Err(error) if is_incomplete_decode_error(error.kind()) => {
                        let (required, available) = incomplete_detail(error.kind());
                        assert!(required > available, "incomplete error did not require more input");
                        let additional = required - available;
                        let copied = additional.min(input.len() - input_cursor);
                        if copied > 0 {
                            self.pending_input
                                .extend_from_slice(&input[input_cursor..input_cursor + copied]);
                            input_cursor += copied;
                            continue;
                        }
                        let status = TranscodeStatus::NeedInput {
                            input_index: input_cursor,
                            required: additional,
                            available,
                        };
                        return Ok(TranscodeProgress::new(
                            status,
                            input_cursor - input_index,
                            output_cursor - output_index,
                        ));
                    }
                    Err(error) if is_policy_decode_error(error.kind()) => {
                        let absolute_error = error.offset_by(self.pending_input_index);
                        let consumed = pending_skip(error, self.pending_input.len());
                        match self.malformed_action {
                            MalformedAction::Report => return Err(absolute_error),
                            MalformedAction::Ignore => {
                                self.drain_pending(consumed);
                            }
                            MalformedAction::Replace => {
                                output[output_cursor] = self.replacement;
                                self.drain_pending(consumed);
                                output_cursor += 1;
                            }
                        }
                    }
                    Err(error) => {
                        return Err(error.offset_by(self.pending_input_index));
                    }
                }
                continue;
            }

            let available = input.len() - input_cursor;
            if available < min_units {
                self.pending_input_index = input_cursor;
                self.pending_input.extend_from_slice(&input[input_cursor..]);
                input_cursor = input.len();
                let status = TranscodeStatus::NeedInput {
                    input_index: input_cursor,
                    required: min_units - available,
                    available,
                };
                return Ok(TranscodeProgress::new(
                    status,
                    input_cursor - input_index,
                    output_cursor - output_index,
                ));
            }

            match unsafe { self.codec.decode_unchecked(input, input_cursor) } {
                Ok((value, consumed)) => {
                    assert!(consumed > 0, "Codec::decode_unchecked consumed zero input units");
                    assert!(
                        consumed <= input.len() - input_cursor,
                        "Codec::decode_unchecked consumed beyond available input",
                    );
                    output[output_cursor] = value;
                    input_cursor += consumed;
                    output_cursor += 1;
                }
                Err(error) if is_incomplete_decode_error(error.kind()) => {
                    let (required, available) = incomplete_detail(error.kind());
                    assert!(required > available, "incomplete error did not require more input");
                    self.pending_input_index = input_cursor;
                    self.pending_input.extend_from_slice(&input[input_cursor..]);
                    input_cursor = input.len();
                    let status = TranscodeStatus::NeedInput {
                        input_index: input_cursor,
                        required: required - available,
                        available,
                    };
                    return Ok(TranscodeProgress::new(
                        status,
                        input_cursor - input_index,
                        output_cursor - output_index,
                    ));
                }
                Err(error) if is_policy_decode_error(error.kind()) => {
                    let consumed = error
                        .consumed()
                        .unwrap_or_else(|| malformed_skip(input_cursor, input.len(), error.index()))
                        .max(1)
                        .min(input.len() - input_cursor);
                    match self.malformed_action {
                        MalformedAction::Report => return Err(error),
                        MalformedAction::Ignore => {
                            input_cursor += consumed;
                        }
                        MalformedAction::Replace => {
                            output[output_cursor] = self.replacement;
                            input_cursor += consumed;
                            output_cursor += 1;
                        }
                    }
                }
                Err(error) => return Err(error),
            }
        }
    }

    /// Finalizes incomplete source input after EOF.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetDecodeErrorKind::IncompleteSequence`] when the decoder
    /// has pending incomplete input and malformed input policy is
    /// [`MalformedAction::Report`].
    fn finish(&mut self, output: &mut [char], output_index: usize) -> Result<TranscodeProgress, Self::Error> {
        if self.pending_input.is_empty() {
            return Ok(TranscodeProgress::complete(0, 0));
        }

        let mut output_cursor = output_index;
        while !self.pending_input.is_empty() {
            if output_cursor >= output.len() {
                let status = TranscodeStatus::NeedOutput {
                    output_index: output_cursor,
                    required: 1,
                    available: output.len().saturating_sub(output_cursor),
                };
                return Ok(TranscodeProgress::new(status, 0, output_cursor - output_index));
            }

            if self.pending_input.len() < self.codec.min_units_per_value() {
                let kind = CharsetDecodeErrorKind::IncompleteSequence {
                    required: self.codec.min_units_per_value(),
                    available: self.pending_input.len(),
                };
                let error = CharsetDecodeError::new(self.codec.charset(), kind, 0);
                let absolute_error = error.offset_by(self.pending_input_index);
                match self.malformed_action {
                    MalformedAction::Report => return Err(absolute_error),
                    MalformedAction::Ignore => {
                        let pending_len = self.pending_input.len();
                        self.drain_pending(pending_len);
                    }
                    MalformedAction::Replace => {
                        output[output_cursor] = self.replacement;
                        let pending_len = self.pending_input.len();
                        self.drain_pending(pending_len);
                        output_cursor += 1;
                    }
                }
                continue;
            }

            match unsafe { self.codec.decode_unchecked(&self.pending_input, 0) } {
                Ok((value, consumed)) => {
                    assert!(consumed > 0, "Codec::decode_unchecked consumed zero input units");
                    assert!(
                        consumed <= self.pending_input.len(),
                        "Codec::decode_unchecked consumed beyond buffered input",
                    );
                    output[output_cursor] = value;
                    self.drain_pending(consumed);
                    output_cursor += 1;
                }
                Err(error) if is_policy_decode_error(error.kind()) => {
                    let skip = pending_skip(error, self.pending_input.len());
                    let absolute_error = error.offset_by(self.pending_input_index);
                    match self.malformed_action {
                        MalformedAction::Report => return Err(absolute_error),
                        MalformedAction::Ignore => {
                            self.drain_pending(skip);
                        }
                        MalformedAction::Replace => {
                            output[output_cursor] = self.replacement;
                            self.drain_pending(skip);
                            output_cursor += 1;
                        }
                    }
                }
                Err(error) => return Err(error.offset_by(self.pending_input_index)),
            }
        }

        Ok(TranscodeProgress::complete(0, output_cursor - output_index))
    }
}

impl<C> BufferedDecoder<C::Unit, char> for CharsetDecoder<C> where C: CharsetCodec {}

impl<C> CharsetDecoder<C>
where
    C: CharsetCodec,
{
    /// Drains consumed units from the pending input buffer.
    ///
    /// # Parameters
    ///
    /// - `count`: Number of pending units to remove from the front.
    #[inline]
    fn drain_pending(&mut self, count: usize) {
        let drained = count.min(self.pending_input.len());
        self.pending_input.drain(..drained);
        self.pending_input_index += drained;
        if self.pending_input.is_empty() {
            self.pending_input_index = 0;
        }
    }
}

/// Calculates how many pending units should be skipped for a decode error.
///
/// # Parameters
///
/// - `error`: Decode error reported against the pending buffer.
/// - `pending_len`: Number of units currently stored in the pending buffer.
///
/// # Returns
///
/// Returns all pending units for an incomplete closed tail, otherwise returns
/// the malformed range containing the reported failing unit.
#[inline]
fn pending_skip(error: CharsetDecodeError, pending_len: usize) -> usize {
    match error.kind() {
        CharsetDecodeErrorKind::IncompleteSequence { .. } => pending_len,
        _ => error
            .consumed()
            .unwrap_or_else(|| malformed_skip(0, pending_len, error.index()))
            .max(1)
            .min(pending_len),
    }
}

/// Calculates how many malformed input units should be skipped.
///
/// # Parameters
///
/// - `input_index`: Absolute index where decoding of the current character started.
/// - `input_len`: Length of the complete input slice.
/// - `error_index`: Absolute index reported by the low-level codec.
///
/// # Returns
///
/// Returns at least one unit when input remains. When the codec reports an error
/// after the start index, the skipped range includes the reported failing unit.
#[inline]
fn malformed_skip(input_index: usize, input_len: usize, error_index: usize) -> usize {
    let available = input_len.saturating_sub(input_index);
    let end = error_index.saturating_add(1).min(input_len);
    end.saturating_sub(input_index).max(1).min(available)
}

/// Returns whether a decode error represents incomplete input.
///
/// # Parameters
///
/// - `kind`: Decode error kind reported by the low-level codec.
///
/// # Returns
///
/// Returns `true` for incomplete input.
#[inline]
const fn is_incomplete_decode_error(kind: CharsetDecodeErrorKind) -> bool {
    matches!(kind, CharsetDecodeErrorKind::IncompleteSequence { .. })
}

/// Extracts incomplete-input details from a decode error kind.
///
/// # Parameters
///
/// - `kind`: Decode error kind known to be incomplete.
///
/// # Returns
///
/// Returns `(required, available)` unit counts.
#[inline]
const fn incomplete_detail(kind: CharsetDecodeErrorKind) -> (usize, usize) {
    match kind {
        CharsetDecodeErrorKind::IncompleteSequence { required, available } => (required, available),
        _ => (0, 0),
    }
}

/// Returns whether a decode error is governed by malformed-input policy.
///
/// # Parameters
///
/// - `kind`: Decode error kind reported by the low-level codec.
///
/// # Returns
///
/// Returns `true` for malformed, incomplete, and invalid-scalar input.
#[inline]
const fn is_policy_decode_error(kind: CharsetDecodeErrorKind) -> bool {
    matches!(
        kind,
        CharsetDecodeErrorKind::MalformedSequence { .. }
            | CharsetDecodeErrorKind::IncompleteSequence { .. }
            | CharsetDecodeErrorKind::InvalidCodePoint { .. }
    )
}
