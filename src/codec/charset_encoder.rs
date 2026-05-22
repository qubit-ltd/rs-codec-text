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
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
};

use super::{
    charset_codec::CharsetCodec,
    coder::Coder,
    coder_progress::CoderProgress,
    coder_status::CoderStatus,
    unmappable_action::UnmappableAction,
};

/// Converts Unicode scalar values into units of one charset.
///
/// `CharsetEncoder` wraps a low-level [`CharsetCodec`] and applies the
/// configured [`UnmappableAction`] whenever the codec reports that an input
/// character cannot be represented by the target charset.
///
/// # Type Parameters
///
/// - `C`: Low-level charset codec used to encode one character into target
///   storage units.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharsetEncoder<C>
where
    C: CharsetCodec,
{
    /// Low-level codec used for target encoding.
    codec: C,
    /// Action used for unmappable input characters.
    unmappable_action: UnmappableAction,
    /// Replacement character used by [`UnmappableAction::Replace`].
    replacement: char,
}

impl<C> CharsetEncoder<C>
where
    C: CharsetCodec,
{
    /// Default replacement character used when unmappable input is replaced.
    pub const DEFAULT_REPLACEMENT: char = '\u{fffd}';

    /// Fallback replacement used when the default replacement is unmappable.
    pub const DEFAULT_FALLBACK_REPLACEMENT: char = '?';

    /// Creates an encoder with default replacement policy.
    ///
    /// # Parameters
    ///
    /// - `codec`: Low-level charset codec used to encode output units.
    ///
    /// # Returns
    ///
    /// Returns an encoder whose unmappable action is
    /// [`UnmappableAction::Replace`] and whose replacement character is
    /// [`CharsetEncoder::DEFAULT_REPLACEMENT`]. If the default cannot be encoded
    /// by the codec, [`CharsetEncoder::DEFAULT_FALLBACK_REPLACEMENT`] is used.
    #[must_use]
    pub fn new(codec: C) -> Self {
        match Self::replacement_is_encodable(&codec, Self::DEFAULT_REPLACEMENT) {
            Ok(()) => Self {
                codec,
                unmappable_action: UnmappableAction::Replace,
                replacement: Self::DEFAULT_REPLACEMENT,
            },
            Err(default_error) => {
                match Self::replacement_is_encodable(&codec, Self::DEFAULT_FALLBACK_REPLACEMENT) {
                    Ok(()) => Self {
                        codec,
                        unmappable_action: UnmappableAction::Replace,
                        replacement: Self::DEFAULT_FALLBACK_REPLACEMENT,
                    },
                    Err(_) => panic!(
                        "cannot initialize CharsetEncoder for {:?}: neither {:?} nor {:?} is encodable ({default_error})",
                        codec.charset(),
                        Self::DEFAULT_REPLACEMENT,
                        Self::DEFAULT_FALLBACK_REPLACEMENT,
                    ),
                }
            }
        }
    }

    /// Creates an encoder with the provided replacement character.
    ///
    /// The replacement character is checked once on construction. If the codec
    /// cannot encode it, this returns an error immediately.
    ///
    /// # Parameters
    ///
    /// - `replacement`: Replacement character for unmappable input.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` when the character is encodable by the codec.
    /// - `Err(Self::Error)` when the replacement is unsupported.
    #[inline]
    pub fn with_replacement(mut self, replacement: char) -> Result<Self, CharsetEncodeError> {
        Self::replacement_is_encodable(&self.codec, replacement)?;
        self.replacement = replacement;
        Ok(self)
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

    /// Returns the configured unmappable-character action.
    ///
    /// # Returns
    ///
    /// Returns the action used when target encoding cannot represent a character.
    #[must_use]
    #[inline]
    pub const fn unmappable_action(&self) -> UnmappableAction {
        self.unmappable_action
    }

    /// Sets the unmappable-character action.
    ///
    /// # Parameters
    ///
    /// - `action`: New policy for unmappable input characters.
    #[inline]
    pub fn set_unmappable_action(&mut self, action: UnmappableAction) {
        self.unmappable_action = action;
    }

    /// Returns the configured replacement character.
    ///
    /// # Returns
    ///
    /// Returns the character encoded when [`UnmappableAction::Replace`] is used.
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
    ///
    /// # Errors
    ///
    /// Returns `Err` when the codec cannot encode the given replacement.
    #[inline]
    pub fn set_replacement(&mut self, replacement: char) -> Result<(), CharsetEncodeError> {
        Self::replacement_is_encodable(&self.codec, replacement)?;
        self.replacement = replacement;
        Ok(())
    }

    /// Returns whether the replacement character can be encoded by the codec.
    ///
    /// This helper is used during construction and configuration to fail fast on
    /// unsupported replacement characters.
    ///
    /// # Parameters
    ///
    /// - `codec`: Target codec to validate against.
    /// - `replacement`: Candidate replacement character.
    ///
    /// # Returns
    ///
    /// - `Ok(())` when the replacement character is encodable.
    /// - `Err(Self::Error)` when the replacement cannot be encoded.
    ///
    /// # Errors
    ///
    /// - `CharsetEncodeErrorKind::UnmappableCharacter` when `replacement` cannot
    ///   be represented by the codec.
    /// - `CharsetEncodeErrorKind::BufferTooSmall` if the temporary probe buffer is
    ///   unexpectedly too small.
    /// - `CharsetEncodeErrorKind::InvalidInputIndex` when the codec rejects a
    ///   zero index probe write.
    /// - `CharsetEncodeErrorKind::InvalidCodePoint` for invalid scalar values.
    fn replacement_is_encodable(codec: &C, replacement: char) -> Result<(), CharsetEncodeError> {
        let mut output = vec![C::Unit::default(); codec.max_units_per_char().max(1)];
        match codec.encode_one(replacement, output.as_mut_slice(), 0) {
            Ok(_) => Ok(()),
            Err(error) => match error.kind() {
                CharsetEncodeErrorKind::UnmappableCharacter { .. } => {
                    Err(CharsetEncodeError::unmappable_character(
                        codec.charset(),
                        error.index(),
                        replacement as u32,
                    ))
                }
                CharsetEncodeErrorKind::BufferTooSmall { .. }
                | CharsetEncodeErrorKind::InvalidInputIndex { .. }
                | CharsetEncodeErrorKind::InvalidCodePoint { .. } => Err(error),
            },
        }
    }
}

impl<C> Coder<char, C::Unit> for CharsetEncoder<C>
where
    C: CharsetCodec,
{
    type Error = CharsetEncodeError;

    /// Returns the maximum number of target units needed for `input_len` characters.
    #[inline]
    fn max_output_len(&self, input_len: usize) -> Option<usize> {
        input_len.checked_mul(self.codec.max_units_per_char())
    }

    /// Encodes characters into the target charset while applying unmappable policy.
    fn convert(
        &mut self,
        input: &[char],
        input_index: usize,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<CoderProgress, Self::Error> {
        if input_index > input.len() {
            return Err(CharsetEncodeError::invalid_input_index_with_len(
                self.codec.charset(),
                input_index,
                input.len(),
            ));
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
            let ch = input[input_cursor];
            match self.codec.encode_one(ch, output, output_cursor) {
                Ok(written) => {
                    input_cursor += 1;
                    output_cursor += written;
                }
                Err(error)
                    if matches!(error.kind(), CharsetEncodeErrorKind::BufferTooSmall { .. }) =>
                {
                    let required = error
                        .required()
                        .unwrap_or(output_cursor + 1)
                        .saturating_sub(output_cursor);
                    let available = error.available().unwrap_or(0);
                    let status = CoderStatus::NeedOutput {
                        output_index: output_cursor,
                        required,
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
                        CharsetEncodeErrorKind::UnmappableCharacter { value: _ }
                    ) =>
                {
                    match self.unmappable_action {
                        UnmappableAction::Report => {
                            return Err(CharsetEncodeError::unmappable_character(
                                self.codec.charset(),
                                input_cursor,
                                ch as u32,
                            ));
                        }
                        UnmappableAction::Ignore => {
                            input_cursor += 1;
                        }
                        UnmappableAction::Replace => {
                            let written = match self.encode_replacement(
                                output,
                                output_cursor,
                                input_cursor,
                            ) {
                                Ok(written) => written,
                                Err(error)
                                    if matches!(
                                        error.kind(),
                                        CharsetEncodeErrorKind::BufferTooSmall { .. }
                                    ) =>
                                {
                                    let required = error
                                        .required()
                                        .unwrap_or(output_cursor + 1)
                                        .saturating_sub(output_cursor);
                                    let available = error.available().unwrap_or(0);
                                    let status = CoderStatus::NeedOutput {
                                        output_index: output_cursor,
                                        required,
                                        available,
                                    };
                                    return Ok(CoderProgress::new(
                                        status,
                                        input_cursor - input_index,
                                        output_cursor - output_index,
                                    ));
                                }
                                Err(error) => return Err(error),
                            };
                            input_cursor += 1;
                            output_cursor += written;
                        }
                    }
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(CoderProgress::complete(
            input_cursor - input_index,
            output_cursor - output_index,
        ))
    }
}

impl<C> CharsetEncoder<C>
where
    C: CharsetCodec,
{
    /// Encodes the configured replacement character.
    ///
    /// # Parameters
    ///
    /// - `output`: Complete target output slice.
    /// - `output_index`: Absolute output index where replacement writing starts.
    /// - `input_index`: Absolute input character index associated with replacement.
    ///
    /// # Returns
    ///
    /// Returns the number of output units written for the replacement.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetEncodeError`] when the output buffer is too small or the
    /// configured replacement character is also unmappable.
    #[inline]
    fn encode_replacement(
        &self,
        output: &mut [C::Unit],
        output_index: usize,
        input_index: usize,
    ) -> CharsetEncodeResult<usize> {
        match self
            .codec
            .encode_one(self.replacement, output, output_index)
        {
            Ok(written) => Ok(written),
            Err(error) if matches!(error.kind(), CharsetEncodeErrorKind::BufferTooSmall { .. }) => {
                let required = error.required().unwrap_or(output_index + 1);
                let available = error.available().unwrap_or(0);
                Err(CharsetEncodeError::buffer_too_small(
                    self.codec.charset(),
                    output_index,
                    required,
                    available,
                ))
            }
            Err(error) => {
                if matches!(
                    error.kind(),
                    CharsetEncodeErrorKind::UnmappableCharacter { value: _ }
                ) {
                    return Err(CharsetEncodeError::unmappable_character(
                        self.codec.charset(),
                        input_index,
                        self.replacement as u32,
                    ));
                }
                Err(error)
            }
        }
    }
}
