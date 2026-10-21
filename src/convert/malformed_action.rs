// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
/// Policy used when input units do not form a valid character.
///
/// # Examples
///
/// ```
/// use qubit_codec_text::MalformedAction;
///
/// let action = MalformedAction::default();
/// assert_eq!(action, MalformedAction::Replace);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum MalformedAction {
    /// Return the decoding error to the caller.
    Report,

    /// Skip the malformed input units and continue.
    Ignore,

    /// Emit the configured replacement character and continue.
    #[default]
    Replace,
}
