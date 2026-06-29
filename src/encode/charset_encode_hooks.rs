// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::{
    fmt,
    marker::PhantomData,
    num::NonZeroUsize,
};

use qubit_codec::{
    EncodeUnencodableAction,
    TranscodeEncodeHooks,
};

use crate::{
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
    UnmappableAction,
};

use crate::CharsetCodec;

/// Unmappable-input policy hooks used by [`super::CharsetEncoder`].
#[derive(Clone)]
pub(crate) struct CharsetEncodeHooks<Unit> {
    /// Action used for unmappable input characters.
    pub(super) unmappable_action: UnmappableAction,
    /// Replacement character used by [`UnmappableAction::Replace`].
    pub(super) replacement: char,
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
            .finish()
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
    #[inline]
    pub(crate) const fn new(
        unmappable_action: UnmappableAction,
        replacement: char,
    ) -> Self {
        Self {
            unmappable_action,
            replacement,
            unit: PhantomData,
        }
    }
}

impl<C> TranscodeEncodeHooks<C> for CharsetEncodeHooks<C::Unit>
where
    C: CharsetCodec,
{
    type Error = CharsetEncodeError;

    /// Handles one character rejected by the charset codec.
    #[inline]
    fn handle_unencodable_encode(
        &mut self,
        codec: &mut C,
        ch: &char,
        input_index: usize,
    ) -> Result<EncodeUnencodableAction<char>, Self::Error> {
        let ch = *ch;
        let error = unmappable_error(codec, ch, input_index);
        match self.unmappable_action {
            UnmappableAction::Report => Err(error),
            UnmappableAction::Ignore => Ok(EncodeUnencodableAction::Skip),
            UnmappableAction::Replace => {
                Ok(EncodeUnencodableAction::replace(self.replacement))
            }
        }
    }
}

/// Returns the encoded width of a replacement character.
pub(super) fn replacement_len<C>(
    codec: &C,
    ch: char,
) -> CharsetEncodeResult<NonZeroUsize>
where
    C: CharsetCodec,
{
    if !codec.can_encode_value(&ch) {
        return Err(unmappable_error(codec, ch, 0));
    }
    Ok(codec.encode_len(&ch))
}

/// Creates an unmappable-character error for `ch`.
fn unmappable_error<C>(codec: &C, ch: char, index: usize) -> CharsetEncodeError
where
    C: CharsetCodec,
{
    let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
    CharsetEncodeError::new(codec.charset(), kind, index)
}
