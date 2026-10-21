// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
/// Chooses how encoding handles a character unsupported by the target charset.
///
/// `Report` preserves the error for the caller, `Ignore` omits the character,
/// and `Replace` encodes the replacement configured by the encoder policy.
///
/// # Examples
///
/// ```
/// use qubit_codec_text::UnmappableAction;
///
/// let action = UnmappableAction::Replace;
/// assert_eq!(UnmappableAction::Replace, action);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum UnmappableAction {
    /// Return the unmappable-character error to the caller.
    Report,

    /// Skip the unmappable character and continue.
    Ignore,

    /// Encode the configured replacement character instead.
    #[default]
    Replace,
}
