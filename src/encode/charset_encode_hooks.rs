// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::TranscodeEncodeError;
use qubit_codec::TranscodeEncodeErrorOf;
use qubit_codec::engine::EncodeContext;
use qubit_codec::engine::EncodeUnencodableAction;
use qubit_codec::engine::TranscodeEncodeHooks;

use crate::CharsetCodec;
use crate::CharsetEncodeError;
use crate::CharsetEncodeErrorKind;
use crate::CharsetEncodeResult;
use crate::UnmappableAction;

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
    /// Applies the configured policy to one character rejected by the codec.
    ///
    /// `Report` returns a charset error tied to the input index, `Ignore`
    /// skips the character, and `Replace` emits the configured replacement.
    #[inline]
    fn handle_unencodable_encode(
        &mut self,
        codec: &mut C,
        context: &EncodeContext<'_, char>,
    ) -> Result<EncodeUnencodableAction<char>, TranscodeEncodeErrorOf<C>> {
        let ch = *context.input_value();
        let input_index = context.input_index();
        let error = unmappable_error(codec, ch, input_index);
        match self.unmappable_action {
            UnmappableAction::Report => Err(TranscodeEncodeError::domain_main(error, input_index)),
            UnmappableAction::Ignore => Ok(EncodeUnencodableAction::Skip),
            UnmappableAction::Replace => Ok(EncodeUnencodableAction::replace(self.replacement)),
        }
    }
}

/// Returns the encoded width of a replacement character.
///
/// # Type Parameters
///
/// - `C`: Charset codec used to determine encodability and encoded width.
///
/// # Parameters
///
/// - `codec`: Codec whose target encoding determines the character width.
/// - `ch`: Character whose encoded width is requested.
///
/// # Returns
///
/// Returns the number of encoded units, or an error carrying the target
/// charset and the unmappable character when `codec` cannot encode `ch`.
pub(super) fn replacement_len<C>(codec: &C, ch: char) -> CharsetEncodeResult<usize>
where
    C: CharsetCodec,
{
    if !codec.can_encode_value(&ch) {
        return Err(unmappable_error(codec, ch, 0));
    }
    Ok(codec.encode_len(&ch))
}

/// Creates an unmappable-character error with the codec charset and input
/// index.
///
/// The error kind stores `ch` as its raw Unicode scalar value. The caller
/// supplies `index` so the same helper can report either a source position or
/// a validation error with a sentinel position.
///
/// # Type Parameters
///
/// - `C`: Charset codec that supplies the target charset.
fn unmappable_error<C>(codec: &C, ch: char, index: usize) -> CharsetEncodeError
where
    C: CharsetCodec,
{
    let kind = CharsetEncodeErrorKind::UnmappableCharacter { value: ch as u32 };
    CharsetEncodeError::new(codec.charset(), kind, index)
}
