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

use qubit_codec::{
    BufferedEncodeHooks,
    EncodeContext,
    EncodePlan,
};

use crate::{
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
};

use super::{
    charset_encode_action::CharsetEncodeAction,
    charset_encode_probe::CharsetEncodeProbe,
    unmappable_action::UnmappableAction,
};

/// Unmappable-input policy hooks used by [`super::CharsetEncoder`].
#[derive(Clone)]
pub(super) struct CharsetEncodeHooks<Unit> {
    /// Action used for unmappable input characters.
    pub(super) unmappable_action: UnmappableAction,
    /// Replacement character used by [`UnmappableAction::Replace`].
    pub(super) replacement: char,
    /// Pre-encoded units for the configured replacement character.
    pub(super) replacement_units: Vec<Unit>,
}

impl<Unit> fmt::Debug for CharsetEncodeHooks<Unit> {
    /// Formats hooks without requiring cached units to implement [`fmt::Debug`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharsetEncodeHooks")
            .field("unmappable_action", &self.unmappable_action)
            .field("replacement", &self.replacement)
            .field("replacement_units_len", &self.replacement_units.len())
            .finish()
    }
}

impl<Unit> Eq for CharsetEncodeHooks<Unit> {}

impl<Unit> PartialEq for CharsetEncodeHooks<Unit> {
    /// Compares policy-visible hook state without requiring unit equality.
    fn eq(&self, other: &Self) -> bool {
        self.unmappable_action == other.unmappable_action
            && self.replacement == other.replacement
            && self.replacement_units.len() == other.replacement_units.len()
    }
}

impl<Unit> CharsetEncodeHooks<Unit> {
    /// Creates charset encode hooks without cached replacement units.
    ///
    /// # Parameters
    ///
    /// - `unmappable_action`: Initial unmappable-character policy.
    /// - `replacement`: Initial replacement character.
    ///
    /// # Returns
    ///
    /// Returns hooks configured with an empty replacement-unit cache.
    #[must_use]
    pub(super) const fn new(unmappable_action: UnmappableAction, replacement: char) -> Self {
        Self {
            unmappable_action,
            replacement,
            replacement_units: Vec::new(),
        }
    }
}

impl<Unit> CharsetEncodeHooks<Unit>
where
    Unit: Copy,
{
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
    fn write_replacement(&self, output: &mut [Unit], output_index: usize) -> CharsetEncodeResult<usize> {
        if self.replacement_units.is_empty() {
            return Ok(0);
        }
        let end = output_index + self.replacement_units.len();
        output[output_index..end].copy_from_slice(&self.replacement_units[..]);
        Ok(self.replacement_units.len())
    }
}

impl<C> BufferedEncodeHooks<C> for CharsetEncodeHooks<C::Unit>
where
    C: CharsetEncodeProbe,
{
    type Error = CharsetEncodeError;
    type PlanAction = CharsetEncodeAction;

    /// Prepares a charset-specific encoding plan.
    #[inline]
    fn prepare_encode(
        &mut self,
        codec: &C,
        ch: &char,
        input_index: usize,
    ) -> Result<EncodePlan<Self::PlanAction>, Self::Error> {
        match codec.encode_len(*ch, input_index) {
            Ok(max_output_units) => Ok(EncodePlan::new(max_output_units, CharsetEncodeAction::WriteOriginal)),
            Err(error) if matches!(error.kind(), CharsetEncodeErrorKind::UnmappableCharacter { .. }) => {
                match self.unmappable_action {
                    UnmappableAction::Report => Err(error),
                    UnmappableAction::Ignore => Ok(EncodePlan::new(0, CharsetEncodeAction::Skip)),
                    UnmappableAction::Replace => Ok(EncodePlan::new(
                        self.replacement_units.len(),
                        CharsetEncodeAction::WriteReplacement,
                    )),
                }
            }
            Err(error) => Err(error),
        }
    }

    /// Writes one character according to a charset-specific plan.
    #[inline]
    unsafe fn write_encode(
        &mut self,
        codec: &C,
        context: EncodeContext<'_, char, C::Unit, Self::PlanAction>,
    ) -> Result<usize, Self::Error> {
        match context.plan_action {
            // SAFETY: The engine checked the exact capacity requested by
            // `prepare_encode`.
            CharsetEncodeAction::WriteOriginal => unsafe {
                codec.encode_unchecked(context.input_value, context.output, context.output_index)
            },
            CharsetEncodeAction::WriteReplacement => self.write_replacement(context.output, context.output_index),
            CharsetEncodeAction::Skip => Ok(0),
        }
    }

    /// Creates an input-index error using the charset from `codec`.
    fn invalid_input_index(&mut self, codec: &C, index: usize, input_len: usize) -> Self::Error {
        let kind = CharsetEncodeErrorKind::InvalidInputIndex { input_len };
        CharsetEncodeError::new(codec.charset(), kind, index)
    }
}

/// Encodes a replacement character for charset encode hooks.
pub(super) fn encode_replacement<C>(codec: &C, ch: char) -> CharsetEncodeResult<Vec<C::Unit>>
where
    C: CharsetEncodeProbe,
    C::Unit: Default,
{
    let required = codec.encode_len(ch, 0)?;
    let mut output = vec![C::Unit::default(); required];
    // SAFETY: CharsetEncodeProbe reports the exact output width accepted by
    // charset codec implementations.
    let written = unsafe { codec.encode_unchecked(&ch, output.as_mut_slice(), 0) }?;
    debug_assert!(written <= required);
    output.truncate(written);
    Ok(output)
}
