// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::engine::{EncodeUnencodableAction, TranscodeEncodeHooks};
use qubit_codec::{CodecPhase, TranscodeError};

use crate::{CharsetEncodeError, CharsetEncodeErrorKind, CharsetEncodeResult, UnmappableAction};

use crate::CharsetCodec;

/// Unmappable-input policy hooks used by [`super::CharsetEncoder`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct CharsetEncodeHooks {
    /// Action used for unmappable input characters.
    pub(super) unmappable_action: UnmappableAction,
    /// Replacement character used by [`UnmappableAction::Replace`].
    pub(super) replacement: char,
}

impl CharsetEncodeHooks {
    /// Creates charset encode hooks.
    ///
    /// # Parameters
    ///
    /// - `unmappable_action`: Initial unmappable-character policy.
    /// - `replacement`: Initial replacement character.
    ///
    /// # Returns
    ///
    /// Returns hooks configured with the supplied policy.
    #[must_use]
    #[inline]
    pub(crate) const fn new(unmappable_action: UnmappableAction, replacement: char) -> Self {
        Self {
            unmappable_action,
            replacement,
        }
    }
}

impl<C> TranscodeEncodeHooks<C> for CharsetEncodeHooks
where
    C: CharsetCodec,
{
    /// Handles one character rejected by the charset codec.
    #[inline]
    fn handle_unencodable_encode(
        &mut self,
        codec: &mut C,
        ch: &char,
        input_index: usize,
    ) -> Result<EncodeUnencodableAction<char>, qubit_codec::TranscodeEncodeError<C>> {
        let ch = *ch;
        let error = unmappable_error(codec, ch, input_index);
        match self.unmappable_action {
            UnmappableAction::Report => Err(TranscodeError::domain(
                error,
                CodecPhase::Main,
                Some(input_index),
            )),
            UnmappableAction::Ignore => Ok(EncodeUnencodableAction::Skip),
            UnmappableAction::Replace => Ok(EncodeUnencodableAction::replace(self.replacement)),
        }
    }
}

/// Returns the encoded width of a replacement character.
pub(super) fn replacement_len<C>(codec: &C, ch: char) -> CharsetEncodeResult<usize>
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
