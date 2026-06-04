/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use crate::UnmappableAction;

/// Unmappable-input policy used by charset encoders and converters.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CharsetEncodePolicy {
    /// Action used for unmappable input characters.
    unmappable_action: UnmappableAction,
    /// Replacement character used by [`UnmappableAction::Replace`].
    replacement: char,
}

impl CharsetEncodePolicy {
    /// Default replacement character used when unmappable input is replaced.
    pub const DEFAULT_REPLACEMENT: char = '\u{fffd}';
    /// Fallback replacement used when the default replacement is unmappable.
    pub const DEFAULT_FALLBACK_REPLACEMENT: char = '?';

    /// Creates an unmappable-input policy.
    #[must_use]
    pub const fn new(unmappable_action: UnmappableAction, replacement: char) -> Self {
        Self {
            unmappable_action,
            replacement,
        }
    }

    /// Creates a replacement policy.
    #[must_use]
    pub const fn replace(replacement: char) -> Self {
        Self::new(UnmappableAction::Replace, replacement)
    }

    /// Creates an ignore policy with the default replacement retained for metadata.
    #[must_use]
    pub const fn ignore() -> Self {
        Self::ignore_with_replacement(Self::DEFAULT_REPLACEMENT)
    }

    /// Creates an ignore policy with explicit replacement metadata.
    #[must_use]
    pub const fn ignore_with_replacement(replacement: char) -> Self {
        Self::new(UnmappableAction::Ignore, replacement)
    }

    /// Creates a report policy with the default replacement retained for metadata.
    #[must_use]
    pub const fn report() -> Self {
        Self::new(UnmappableAction::Report, Self::DEFAULT_REPLACEMENT)
    }

    /// Returns the unmappable-input action.
    #[must_use]
    pub const fn unmappable_action(self) -> UnmappableAction {
        self.unmappable_action
    }

    /// Returns the replacement character.
    #[must_use]
    pub const fn replacement(self) -> char {
        self.replacement
    }
}

impl Default for CharsetEncodePolicy {
    fn default() -> Self {
        Self::replace(Self::DEFAULT_REPLACEMENT)
    }
}
