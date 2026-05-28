/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use core::fmt;

use crate::{
    BufferedEncoder,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
    TranscodeProgress,
    TranscodeStatus,
    Transcoder,
};

use super::{
    charset_encode_probe::CharsetEncodeProbe,
    unmappable_action::UnmappableAction,
};

/// Converts Unicode scalar values into units of one charset.
///
/// `CharsetEncoder` wraps a low-level [`crate::CharsetCodec`] and applies the
/// configured [`UnmappableAction`] whenever the codec reports that an input
/// character cannot be represented by the target charset.
///
/// # Type Parameters
///
/// - `C`: Low-level charset codec used to encode one character into target
///   storage units.
#[derive(Clone)]
pub struct CharsetEncoder<C>
where
    C: CharsetEncodeProbe,
{
    /// Low-level codec used for target encoding.
    codec: C,
    /// Action used for unmappable input characters.
    unmappable_action: UnmappableAction,
    /// Replacement character used by [`UnmappableAction::Replace`].
    replacement: char,
    /// Pre-encoded units for the configured replacement character.
    replacement_units: Vec<C::Unit>,
}

impl<C> CharsetEncoder<C>
where
    C: CharsetEncodeProbe,
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
    ///
    /// # Panics
    ///
    /// Panics when neither [`Self::DEFAULT_REPLACEMENT`] nor
    /// [`Self::DEFAULT_FALLBACK_REPLACEMENT`] can be encoded by `codec`.
    /// Built-in codecs can always encode the fallback `?`; failure here means
    /// the supplied codec cannot encode a minimal ASCII replacement. For custom
    /// [`crate::CharsetCodec`] implementations, this indicates a broken codec
    /// invariant rather than recoverable input data.
    #[must_use]
    pub fn new(codec: C) -> Self {
        let mut encoder = Self {
            codec,
            unmappable_action: UnmappableAction::Replace,
            replacement: Self::DEFAULT_REPLACEMENT,
            replacement_units: Vec::new(),
        };
        match encoder.encode_replacement(Self::DEFAULT_REPLACEMENT) {
            Ok(replacement_units) => {
                encoder.replacement = Self::DEFAULT_REPLACEMENT;
                encoder.replacement_units = replacement_units;
                encoder
            }
            Err(default_error) => match encoder.encode_replacement(Self::DEFAULT_FALLBACK_REPLACEMENT) {
                Ok(replacement_units) => {
                    encoder.replacement = Self::DEFAULT_FALLBACK_REPLACEMENT;
                    encoder.replacement_units = replacement_units;
                    encoder
                }
                Err(_) => panic!(
                    "cannot initialize CharsetEncoder for {:?}: neither {:?} nor {:?} is encodable ({default_error})",
                    encoder.codec.charset(),
                    Self::DEFAULT_REPLACEMENT,
                    Self::DEFAULT_FALLBACK_REPLACEMENT,
                ),
            },
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
        let replacement_units = self.encode_replacement(replacement)?;
        self.replacement = replacement;
        self.replacement_units = replacement_units;
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
        let replacement_units = self.encode_replacement(replacement)?;
        self.replacement = replacement;
        self.replacement_units = replacement_units;
        Ok(())
    }

    /// Encodes a replacement character into a temporary buffer and returns the
    /// encoded unit sequence.
    ///
    /// # Parameters
    ///
    /// - `ch`: Replacement character to validate and encode.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<C::Unit>)` when the character is encodable.
    /// - `Err(CharsetEncodeError)` with codec-specific context when encoding fails.
    ///
    /// # Errors
    ///
    /// Returns an error when the target charset cannot encode the character.
    #[inline]
    fn encode_replacement(&self, ch: char) -> CharsetEncodeResult<Vec<C::Unit>> {
        let mut output = vec![C::Unit::default(); self.codec.max_units_per_value().max(1)];
        let written = self.encode_char_to_units(ch, 0, output.as_mut_slice(), 0, |_, _, error| Err(error))?;
        output.truncate(written);
        Ok(output)
    }

    /// Encodes a single Unicode scalar value into the caller-provided unit buffer.
    ///
    /// This is the common template around [`CharsetEncodeProbe::encode_len`] and
    /// [`Codec::encode_unchecked`]. It
    /// keeps all direct codec error inspection in one place while allowing the
    /// caller to decide how an unmappable character should be handled.
    ///
    /// # Parameters
    ///
    /// - `ch`: Character to encode.
    /// - `input_index`: Input character index used for unmappable error context.
    /// - `output`: Target unit buffer to write into.
    /// - `output_index`: Start position in `output` to write the encoded units.
    /// - `on_unmappable`: Handler called when the codec reports
    ///   [`CharsetEncodeErrorKind::UnmappableCharacter`].
    ///
    /// # Returns
    ///
    /// - `Ok(usize)` of how many units were written.
    /// - `Err(CharsetEncodeError)` when encoding fails.
    ///
    /// # Errors
    ///
    /// - `CharsetEncodeError` if the codec cannot encode the character.
    fn encode_char_to_units(
        &self,
        ch: char,
        input_index: usize,
        output: &mut [C::Unit],
        output_index: usize,
        on_unmappable: impl FnOnce(&mut [C::Unit], usize, CharsetEncodeError) -> CharsetEncodeResult<usize>,
    ) -> CharsetEncodeResult<usize> {
        let required = match self.codec.encode_len(ch, input_index) {
            Ok(required) => required,
            Err(error) if matches!(error.kind(), CharsetEncodeErrorKind::UnmappableCharacter { .. }) => {
                return on_unmappable(output, output_index, error);
            }
            Err(error) => return Err(error),
        };
        let available = output.len().saturating_sub(output_index);
        if available < required {
            let kind = CharsetEncodeErrorKind::BufferTooSmall {
                required: output_index.saturating_add(required),
                available,
            };
            return Err(CharsetEncodeError::new(self.codec.charset(), kind, output_index));
        }
        // SAFETY: The probe and capacity check above guarantee that `output`
        // has enough writable units for this character.
        unsafe { self.codec.encode_unchecked(&ch, output, output_index) }
    }

    /// Writes the cached replacement units into the target output slice.
    ///
    /// # Parameters
    ///
    /// - `output`: Complete target output slice.
    /// - `output_index`: Absolute output index where replacement writing starts.
    ///
    /// # Returns
    ///
    /// Returns the number of output units written for the replacement.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetEncodeError`] when the output buffer is too small.
    #[inline]
    fn write_replacement(&self, output: &mut [C::Unit], output_index: usize) -> CharsetEncodeResult<usize> {
        let available = output.len().saturating_sub(output_index);
        if available < self.replacement_units.len() {
            let kind = CharsetEncodeErrorKind::BufferTooSmall {
                required: output_index + self.replacement_units.len(),
                available,
            };
            return Err(CharsetEncodeError::new(self.codec.charset(), kind, output_index));
        }
        if self.replacement_units.is_empty() {
            return Ok(0);
        }
        let end = output_index + self.replacement_units.len();
        output[output_index..end].copy_from_slice(&self.replacement_units[..]);
        Ok(self.replacement_units.len())
    }
}

impl<C> Transcoder<char, C::Unit> for CharsetEncoder<C>
where
    C: CharsetEncodeProbe,
{
    type Error = CharsetEncodeError;

    /// Returns the maximum number of target units needed for `input_len` characters.
    #[inline]
    fn max_output_len(&self, input_len: usize) -> Option<usize> {
        input_len.checked_mul(self.codec.max_units_per_value())
    }

    /// Encodes characters into the target charset while applying unmappable policy.
    fn transcode(
        &mut self,
        input: &[char],
        input_index: usize,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<TranscodeProgress, Self::Error> {
        if input_index > input.len() {
            let kind = CharsetEncodeErrorKind::InvalidInputIndex { input_len: input.len() };
            return Err(CharsetEncodeError::new(self.codec.charset(), kind, input_index));
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
        while input_cursor < input.len() {
            let ch = input[input_cursor];
            match self.encode_char_to_units(ch, input_cursor, output, output_cursor, |output, output_index, _| {
                match self.unmappable_action {
                    UnmappableAction::Report => {
                        let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
                        Err(CharsetEncodeError::new(self.codec.charset(), kind, input_cursor))
                    }
                    UnmappableAction::Ignore => Ok(0),
                    UnmappableAction::Replace => self.write_replacement(output, output_index),
                }
            }) {
                Ok(written) => {
                    input_cursor += 1;
                    output_cursor += written;
                }
                Err(error) if matches!(error.kind(), CharsetEncodeErrorKind::BufferTooSmall { .. }) => {
                    let required = error
                        .required()
                        .unwrap_or(output_cursor + 1)
                        .saturating_sub(output_cursor);
                    let available = error.available().unwrap_or(0);
                    let status = TranscodeStatus::NeedOutput {
                        output_index: output_cursor,
                        required,
                        available,
                    };
                    return Ok(TranscodeProgress::new(
                        status,
                        input_cursor - input_index,
                        output_cursor - output_index,
                    ));
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }
        Ok(TranscodeProgress::complete(
            input_cursor - input_index,
            output_cursor - output_index,
        ))
    }
}

impl<C> BufferedEncoder<char, C::Unit> for CharsetEncoder<C> where C: CharsetEncodeProbe {}

impl<C> Eq for CharsetEncoder<C> where C: CharsetEncodeProbe + Eq {}

impl<C> PartialEq for CharsetEncoder<C>
where
    C: CharsetEncodeProbe + PartialEq,
{
    /// Compares encoder configuration without leaking cached-unit trait bounds.
    fn eq(&self, other: &Self) -> bool {
        self.codec == other.codec
            && self.unmappable_action == other.unmappable_action
            && self.replacement == other.replacement
    }
}

impl<C> fmt::Debug for CharsetEncoder<C>
where
    C: CharsetEncodeProbe + fmt::Debug,
{
    /// Formats the encoder without exposing additional bounds for cached units.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharsetEncoder")
            .field("codec", &self.codec)
            .field("unmappable_action", &self.unmappable_action)
            .field("replacement", &self.replacement)
            .field("replacement_units_len", &self.replacement_units.len())
            .finish()
    }
}
