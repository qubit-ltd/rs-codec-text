// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::MalformedAction;

/// Malformed-input and incomplete-EOF policy used by charset decoders and
/// converters.
///
/// # Examples
///
/// ```
/// use qubit_codec_text::CharsetDecodePolicy;
/// use qubit_codec_text::MalformedAction;
///
/// let policy = CharsetDecodePolicy::replace('?');
/// assert_eq!(policy.malformed_action(), MalformedAction::Replace);
/// assert_eq!(policy.replacement(), '?');
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CharsetDecodePolicy {
    /// Action used for malformed input units and incomplete tails at EOF.
    malformed_action: MalformedAction,
    /// Replacement character used by [`MalformedAction::Replace`].
    replacement: char,
}

impl CharsetDecodePolicy {
    /// Default replacement character used when malformed input is replaced.
    pub const DEFAULT_REPLACEMENT: char = '\u{fffd}';

    /// Creates a policy for malformed input units and incomplete tails at EOF.
    ///
    /// # Parameters
    ///
    /// * `malformed_action` selects whether decoding reports, ignores, or
    ///   replaces invalid input.
    /// * `replacement` is emitted when `malformed_action` is
    ///   [`MalformedAction::Replace`].
    ///
    /// # Returns
    ///
    /// A policy containing the supplied action and replacement character.
    #[must_use]
    #[inline]
    pub const fn new(malformed_action: MalformedAction, replacement: char) -> Self {
        Self {
            malformed_action,
            replacement,
        }
    }

    /// Creates a replacement policy.
    ///
    /// # Parameters
    ///
    /// * `replacement` is emitted for malformed input and incomplete tails.
    ///
    /// # Returns
    ///
    /// A policy that replaces malformed input with `replacement`.
    #[must_use]
    #[inline]
    pub const fn replace(replacement: char) -> Self {
        Self::new(MalformedAction::Replace, replacement)
    }

    /// Creates an ignore policy with the default replacement retained for
    /// metadata.
    ///
    /// # Returns
    ///
    /// A policy that skips malformed input and retains
    /// [`Self::DEFAULT_REPLACEMENT`] as metadata.
    #[must_use]
    #[inline]
    pub const fn ignore() -> Self {
        Self::ignore_with_replacement(Self::DEFAULT_REPLACEMENT)
    }

    /// Creates an ignore policy with explicit replacement metadata.
    ///
    /// # Parameters
    ///
    /// * `replacement` is retained as policy metadata; it is not emitted by the
    ///   ignore action.
    ///
    /// # Returns
    ///
    /// A policy that skips malformed input and retains `replacement`.
    #[must_use]
    #[inline]
    pub const fn ignore_with_replacement(replacement: char) -> Self {
        Self::new(MalformedAction::Ignore, replacement)
    }

    /// Creates a report policy with the default replacement retained for
    /// metadata.
    ///
    /// # Returns
    ///
    /// A policy that reports malformed input and retains
    /// [`Self::DEFAULT_REPLACEMENT`] as metadata.
    #[must_use]
    #[inline]
    pub const fn report() -> Self {
        Self::new(MalformedAction::Report, Self::DEFAULT_REPLACEMENT)
    }

    /// Returns the malformed-input action.
    ///
    /// # Returns
    ///
    /// The action applied to malformed input and incomplete tails at EOF.
    #[must_use]
    #[inline]
    pub const fn malformed_action(self) -> MalformedAction {
        self.malformed_action
    }

    /// Returns the replacement character.
    ///
    /// # Returns
    ///
    /// The replacement character retained by this policy. It is emitted only
    /// when [`Self::malformed_action`] is [`MalformedAction::Replace`].
    #[must_use]
    #[inline]
    pub const fn replacement(self) -> char {
        self.replacement
    }
}

impl Default for CharsetDecodePolicy {
    #[inline]
    fn default() -> Self {
        Self::replace(Self::DEFAULT_REPLACEMENT)
    }
}
