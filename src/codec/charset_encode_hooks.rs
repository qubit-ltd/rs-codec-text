/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use qubit_codec::{
    BufferedEncodeHooks,
    EncodePlan,
};

use crate::{
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
};

use super::{
    charset_encode_plan::CharsetEncodePlan,
    charset_encode_probe::CharsetEncodeProbe,
    unmappable_action::UnmappableAction,
};

/// Unmappable-input policy hooks used by [`super::CharsetEncoder`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CharsetEncodeHooks<Unit> {
    /// Action used for unmappable input characters.
    pub(super) unmappable_action: UnmappableAction,
    /// Replacement character used by [`UnmappableAction::Replace`].
    pub(super) replacement: char,
    /// Pre-encoded units for the configured replacement character.
    pub(super) replacement_units: Vec<Unit>,
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
    #[inline(always)]
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
    #[inline]
    fn write_replacement(&self, output: &mut [Unit], output_index: usize) -> CharsetEncodeResult<usize> {
        if self.replacement_units.is_empty() {
            return Ok(0);
        }
        let end = output_index + self.replacement_units.len();
        output[output_index..end].copy_from_slice(&self.replacement_units[..]);
        Ok(self.replacement_units.len())
    }
}

impl<C> BufferedEncodeHooks<C, char, C::Unit> for CharsetEncodeHooks<C::Unit>
where
    C: CharsetEncodeProbe,
{
    type Error = CharsetEncodeError;
    type PlanPayload = CharsetEncodePlan;

    /// Prepares a charset-specific encoding plan.
    #[inline]
    fn prepare_encode(
        &mut self,
        codec: &C,
        ch: &char,
        input_index: usize,
    ) -> Result<EncodePlan<Self::PlanPayload>, Self::Error> {
        match codec.encode_len(*ch, input_index) {
            Ok(max_output_units) => Ok(EncodePlan::new(max_output_units, CharsetEncodePlan::Original)),
            Err(error) if matches!(error.kind(), CharsetEncodeErrorKind::UnmappableCharacter { .. }) => {
                match self.unmappable_action {
                    UnmappableAction::Report => Err(error),
                    UnmappableAction::Ignore => Ok(EncodePlan::new(0, CharsetEncodePlan::Ignore)),
                    UnmappableAction::Replace => Ok(EncodePlan::new(
                        self.replacement_units.len(),
                        CharsetEncodePlan::Replacement,
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
        ch: &char,
        _input_index: usize,
        plan_payload: Self::PlanPayload,
        output: &mut [C::Unit],
        output_index: usize,
    ) -> Result<usize, Self::Error> {
        match plan_payload {
            // SAFETY: The engine checked the exact capacity requested by
            // `prepare_encode`.
            CharsetEncodePlan::Original => unsafe { codec.encode_unchecked(ch, output, output_index) },
            CharsetEncodePlan::Replacement => self.write_replacement(output, output_index),
            CharsetEncodePlan::Ignore => Ok(0),
        }
    }
}
