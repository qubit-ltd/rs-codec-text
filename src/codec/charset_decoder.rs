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
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    Coder,
    CoderProgress,
    CoderStatus,
};

use super::{
    charset_codec::CharsetCodec,
    decode_status::DecodeStatus,
    malformed_action::MalformedAction,
};

/// Converts units of one charset into Unicode scalar values.
///
/// `CharsetDecoder` wraps a low-level [`CharsetCodec`] and applies the
/// configured [`MalformedAction`] whenever the codec reports malformed input.
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

impl<C> Coder<C::Unit, char> for CharsetDecoder<C>
where
    C: CharsetCodec,
{
    type Error = CharsetDecodeError;

    /// Returns the maximum number of characters decoded from `input_len` units.
    #[inline]
    fn max_output_len(&self, input_len: usize) -> Option<usize> {
        Some(input_len)
    }

    /// Decodes source units into Unicode scalar values while applying malformed policy.
    fn convert(
        &mut self,
        input: &[C::Unit],
        input_index: usize,
        output: &mut [char],
        output_index: usize,
    ) -> Result<CoderProgress, Self::Error> {
        if input_index > input.len() {
            let kind = CharsetDecodeErrorKind::MalformedSequence { value: None };
            return Err(CharsetDecodeError::new(self.codec.charset(), kind, input_index));
        }
        if output_index > output.len() {
            let status = CoderStatus::NeedOutput {
                output_index,
                required: 1,
                available: 0,
            };
            return Ok(CoderProgress::new(status, 0, 0));
        }

        let mut input_cursor = input_index;
        let mut output_cursor = output_index;
        while input_cursor < input.len() {
            if output_cursor == output.len() {
                let status = CoderStatus::NeedOutput {
                    output_index: output_cursor,
                    required: 1,
                    available: 0,
                };
                return Ok(CoderProgress::new(
                    status,
                    input_cursor - input_index,
                    output_cursor - output_index,
                ));
            }
            match self.codec.decode_one(input, input_cursor) {
                Ok(DecodeStatus::Complete { value, consumed }) => {
                    output[output_cursor] = value;
                    input_cursor += consumed;
                    output_cursor += 1;
                }
                Ok(DecodeStatus::NeedMore { required, available }) => {
                    let needed = required.saturating_sub(input_cursor);
                    let status = CoderStatus::NeedInput {
                        input_index: input_cursor,
                        required: needed,
                        available,
                    };
                    return Ok(CoderProgress::new(
                        status,
                        input_cursor - input_index,
                        output_cursor - output_index,
                    ));
                }
                Err(error)
                    if matches!(
                        error.kind(),
                        CharsetDecodeErrorKind::MalformedSequence { .. }
                            | CharsetDecodeErrorKind::InvalidCodePoint { .. }
                    ) =>
                {
                    let skip = malformed_skip(input_cursor, input.len(), error.index());
                    match self.malformed_action {
                        MalformedAction::Report => return Err(error),
                        MalformedAction::Ignore => {
                            input_cursor += skip;
                        }
                        MalformedAction::Replace => {
                            output[output_cursor] = self.replacement;
                            input_cursor += skip;
                            output_cursor += 1;
                        }
                    }
                }
                Err(error) => return Err(error),
            }
        }
        Ok(CoderProgress::complete(
            input_cursor - input_index,
            output_cursor - output_index,
        ))
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
