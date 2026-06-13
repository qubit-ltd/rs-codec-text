// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::{fmt, marker::PhantomData};

use qubit_codec::{EncodeContext, EncodePlan, TranscodeEncodeHooks};

use crate::{CharsetEncodeError, CharsetEncodeErrorKind, CharsetEncodeResult, UnmappableAction};

use super::{charset_encode_action::CharsetEncodeAction, charset_encode_probe::CharsetEncodeProbe};

/// Unmappable-input policy hooks used by [`super::CharsetEncoder`].
#[derive(Clone)]
pub(crate) struct CharsetEncodeHooks<Unit> {
    /// Action used for unmappable input characters.
    pub(super) unmappable_action: UnmappableAction,
    /// Replacement character used by [`UnmappableAction::Replace`].
    pub(super) replacement: char,
    /// Number of units needed for the configured replacement character.
    pub(super) replacement_units_len: usize,
    /// Unit marker keeping hook identity tied to the concrete output unit
    /// type.
    unit: PhantomData<fn() -> Unit>,
}

impl<Unit> fmt::Debug for CharsetEncodeHooks<Unit> {
    /// Formats hooks without requiring unit values to implement [`fmt::Debug`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CharsetEncodeHooks")
            .field("unmappable_action", &self.unmappable_action)
            .field("replacement", &self.replacement)
            .field("replacement_units_len", &self.replacement_units_len)
            .finish()
    }
}

impl<Unit> Eq for CharsetEncodeHooks<Unit> {}

impl<Unit> PartialEq for CharsetEncodeHooks<Unit> {
    /// Compares policy-visible hook state without requiring unit equality.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.unmappable_action == other.unmappable_action
            && self.replacement == other.replacement
            && self.replacement_units_len == other.replacement_units_len
    }
}

impl<Unit> CharsetEncodeHooks<Unit> {
    /// Creates charset encode hooks without replacement output units.
    ///
    /// # Parameters
    ///
    /// - `unmappable_action`: Initial unmappable-character policy.
    /// - `replacement`: Initial replacement character.
    ///
    /// # Returns
    ///
    /// Returns hooks configured with no replacement output units.
    #[must_use]
    #[inline(always)]
    pub(crate) const fn new(unmappable_action: UnmappableAction, replacement: char) -> Self {
        Self {
            unmappable_action,
            replacement,
            replacement_units_len: 0,
            unit: PhantomData,
        }
    }

    /// Records the replacement output width.
    ///
    /// # Parameters
    ///
    /// - `replacement_units_len`: Number of target units used by the
    ///   replacement.
    #[inline(always)]
    pub(crate) const fn set_replacement_units_len(&mut self, replacement_units_len: usize) {
        self.replacement_units_len = replacement_units_len;
    }
}

impl<C> TranscodeEncodeHooks<C> for CharsetEncodeHooks<C::Unit>
where
    C: CharsetEncodeProbe,
{
    type Error = CharsetEncodeError;
    type PlanAction = CharsetEncodeAction;

    /// Prepares a charset-specific encoding plan.
    #[inline]
    fn prepare_encode(
        &mut self,
        codec: &mut C,
        ch: &char,
        input_index: usize,
    ) -> Result<EncodePlan<Self::PlanAction>, Self::Error> {
        match CharsetEncodeProbe::encode_len(codec, *ch, input_index) {
            Ok(max_output_units) => Ok(EncodePlan::new(
                max_output_units,
                CharsetEncodeAction::WriteOriginal,
            )),
            Err(error)
                if matches!(
                    error.kind(),
                    CharsetEncodeErrorKind::UnmappableCharacter { .. }
                ) =>
            {
                match self.unmappable_action {
                    UnmappableAction::Report => Err(error),
                    UnmappableAction::Ignore => Ok(EncodePlan::new(0, CharsetEncodeAction::Skip)),
                    UnmappableAction::Replace if self.replacement_units_len == 0 => {
                        Ok(EncodePlan::new(0, CharsetEncodeAction::Skip))
                    }
                    UnmappableAction::Replace => Ok(EncodePlan::new(
                        self.replacement_units_len,
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
        codec: &mut C,
        context: EncodeContext<'_, char, C::Unit>,
        plan: EncodePlan<Self::PlanAction>,
    ) -> Result<usize, Self::Error> {
        match plan.action {
            // SAFETY: The engine checked the exact capacity requested by
            // `prepare_encode`.
            CharsetEncodeAction::WriteOriginal => unsafe {
                codec
                    .encode(context.input_value, context.output, context.output_index)
                    .map(core::num::NonZeroUsize::get)
            },
            // SAFETY: The engine checked the replacement capacity reported by
            // `prepare_encode`.
            CharsetEncodeAction::WriteReplacement => unsafe {
                codec
                    .encode(&self.replacement, context.output, context.output_index)
                    .map(core::num::NonZeroUsize::get)
            },
            CharsetEncodeAction::Skip => Ok(0),
        }
    }

    /// Maps charset encode reset errors unchanged.
    #[inline(always)]
    fn map_encode_reset_error(&mut self, _codec: &mut C, error: CharsetEncodeError) -> Self::Error {
        error
    }
}

/// Returns the encoded width of a replacement character.
#[inline(always)]
pub(super) fn replacement_len<C>(codec: &C, ch: char) -> CharsetEncodeResult<usize>
where
    C: CharsetEncodeProbe,
{
    CharsetEncodeProbe::encode_len(codec, ch, 0)
}
