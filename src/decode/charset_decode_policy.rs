// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::MalformedAction;

/// Malformed-input policy used by charset decoders and converters.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CharsetDecodePolicy {
    /// Action used for malformed input units.
    malformed_action: MalformedAction,
    /// Replacement character used by [`MalformedAction::Replace`].
    replacement: char,
}

impl CharsetDecodePolicy {
    /// Default replacement character used when malformed input is replaced.
    pub const DEFAULT_REPLACEMENT: char = '\u{fffd}';

    /// Creates a malformed-input policy.
    #[must_use]
    #[inline(always)]
    pub const fn new(
        malformed_action: MalformedAction,
        replacement: char,
    ) -> Self {
        Self {
            malformed_action,
            replacement,
        }
    }

    /// Creates a replacement policy.
    #[must_use]
    #[inline(always)]
    pub const fn replace(replacement: char) -> Self {
        Self::new(MalformedAction::Replace, replacement)
    }

    /// Creates an ignore policy with the default replacement retained for
    /// metadata.
    #[must_use]
    #[inline(always)]
    pub const fn ignore() -> Self {
        Self::ignore_with_replacement(Self::DEFAULT_REPLACEMENT)
    }

    /// Creates an ignore policy with explicit replacement metadata.
    #[must_use]
    #[inline(always)]
    pub const fn ignore_with_replacement(replacement: char) -> Self {
        Self::new(MalformedAction::Ignore, replacement)
    }

    /// Creates a report policy with the default replacement retained for
    /// metadata.
    #[must_use]
    #[inline(always)]
    pub const fn report() -> Self {
        Self::new(MalformedAction::Report, Self::DEFAULT_REPLACEMENT)
    }

    /// Returns the malformed-input action.
    #[must_use]
    #[inline(always)]
    pub const fn malformed_action(self) -> MalformedAction {
        self.malformed_action
    }

    /// Returns the replacement character.
    #[must_use]
    #[inline(always)]
    pub const fn replacement(self) -> char {
        self.replacement
    }
}

impl Default for CharsetDecodePolicy {
    #[inline(always)]
    fn default() -> Self {
        Self::replace(Self::DEFAULT_REPLACEMENT)
    }
}
