/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use core::marker::PhantomData;

use crate::{
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
    Coder,
    CoderProgress,
};

use super::{
    charset_codec::CharsetCodec,
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
/// - `T`: Target storage unit type produced by the encoder, such as `u8`,
///   `u16`, or `u32`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharsetEncoder<C, T> {
    /// Low-level codec used for target encoding.
    codec: C,
    /// Action used for unmappable input characters.
    unmappable_action: UnmappableAction,
    /// Replacement character used by [`UnmappableAction::Replace`].
    replacement: char,
    /// Storage unit marker for the target buffer.
    unit: PhantomData<fn() -> T>,
}

impl<C, T> CharsetEncoder<C, T> {
    /// Creates an encoder with default replacement policy.
    ///
    /// # Parameters
    ///
    /// - `codec`: Low-level charset codec used to encode output units.
    ///
    /// # Returns
    ///
    /// Returns an encoder whose unmappable action is
    /// [`UnmappableAction::Replace`] and whose replacement character is `'?'`.
    #[must_use]
    #[inline]
    pub const fn new(codec: C) -> Self {
        Self {
            codec,
            unmappable_action: UnmappableAction::Replace,
            replacement: '?',
            unit: PhantomData,
        }
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
    #[inline]
    pub fn set_replacement(&mut self, replacement: char) {
        self.replacement = replacement;
    }
}

impl<C, T> Coder<char, T> for CharsetEncoder<C, T>
where
    C: CharsetCodec<T>,
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
        output: &mut [T],
        output_index: usize,
    ) -> Result<CoderProgress, Self::Error> {
        if input_index > input.len() {
            return Err(CharsetEncodeError::invalid_input_index(
                self.codec.charset(),
                input_index,
            ));
        }
        if output_index > output.len() {
            return Ok(CoderProgress::need_output(0, 0));
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
                Err(error) if error.kind() == CharsetEncodeErrorKind::BufferTooSmall => {
                    return Ok(CoderProgress::need_output(
                        input_cursor - input_index,
                        output_cursor - output_index,
                    ));
                }
                Err(error) if error.kind() == CharsetEncodeErrorKind::UnmappableCharacter => {
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
                                    if error.kind() == CharsetEncodeErrorKind::BufferTooSmall =>
                                {
                                    return Ok(CoderProgress::need_output(
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

impl<C, T> CharsetEncoder<C, T> {
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
        output: &mut [T],
        output_index: usize,
        input_index: usize,
    ) -> CharsetEncodeResult<usize>
    where
        C: CharsetCodec<T>,
    {
        match self
            .codec
            .encode_one(self.replacement, output, output_index)
        {
            Ok(written) => Ok(written),
            Err(error) if error.kind() == CharsetEncodeErrorKind::BufferTooSmall => Err(
                CharsetEncodeError::buffer_too_small(self.codec.charset(), error.index()),
            ),
            Err(_) => Err(CharsetEncodeError::unmappable_character(
                self.codec.charset(),
                input_index,
                self.replacement as u32,
            )),
        }
    }
}
