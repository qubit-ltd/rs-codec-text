// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::CharsetCodec;
use crate::CharsetEncodeError;
use crate::CharsetEncodeErrorKind;
use crate::UnmappableAction;

/// Unmappable-input policy used by charset encoders and converters.
///
/// Constructing a policy does not validate it against a target codec. The
/// default policy always replaces with U+FFFD; use [`Self::default_for`] to
/// select U+FFFD or the fallback `?` based on a codec. For `Ignore` and
/// `Report`, the replacement character is retained as metadata and is not
/// encoded.
///
/// # Examples
///
/// ```
/// use qubit_codec_text::CharsetEncodePolicy;
///
/// let policy = CharsetEncodePolicy::default();
/// assert_eq!(
///     policy.replacement(),
///     CharsetEncodePolicy::DEFAULT_REPLACEMENT,
/// );
/// assert_eq!(
///     policy.unmappable_action(),
///     qubit_codec_text::UnmappableAction::Replace,
/// );
/// ```
///
/// The automatic encoder constructor selects a replacement supported by the
/// target, while an explicit default policy keeps U+FFFD and is rejected by
/// ASCII. Resolve the choice for the target before passing a policy explicitly.
///
/// ```
/// use qubit_codec_text::AsciiCodec;
/// use qubit_codec_text::CharsetEncodeErrorKind;
/// use qubit_codec_text::CharsetEncodePolicy;
/// use qubit_codec_text::CharsetEncoder;
///
/// let automatic = CharsetEncoder::new(AsciiCodec);
/// assert_eq!('?', automatic.replacement());
///
/// let error = CharsetEncoder::with_policy(
///     AsciiCodec,
///     CharsetEncodePolicy::default(),
/// ).expect_err("ASCII cannot encode U+FFFD");
/// assert_eq!(
///     CharsetEncodeErrorKind::UnmappableCharacter { value: 0xfffd },
///     error.kind(),
/// );
///
/// let policy = CharsetEncodePolicy::default_for(&AsciiCodec).unwrap();
/// let explicit = CharsetEncoder::with_policy(AsciiCodec, policy).unwrap();
/// assert_eq!('?', explicit.replacement());
/// ```
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

    /// Creates an unmappable-input policy without validating its replacement
    /// against a target codec. A replacement is checked when configured on an
    /// encoder or converter with the `Replace` action.
    #[must_use]
    #[inline]
    pub const fn new(unmappable_action: UnmappableAction, replacement: char) -> Self {
        Self {
            unmappable_action,
            replacement,
        }
    }

    /// Creates a replacement policy without checking whether a target codec
    /// can encode `replacement`.
    #[must_use]
    #[inline]
    pub const fn replace(replacement: char) -> Self {
        Self::new(UnmappableAction::Replace, replacement)
    }

    /// Creates an ignore policy with the default replacement retained for
    /// metadata.
    #[must_use]
    #[inline]
    pub const fn ignore() -> Self {
        Self::ignore_with_replacement(Self::DEFAULT_REPLACEMENT)
    }

    /// Creates an ignore policy with explicit replacement metadata. The
    /// replacement is not encoded while the action is `Ignore`.
    #[must_use]
    #[inline]
    pub const fn ignore_with_replacement(replacement: char) -> Self {
        Self::new(UnmappableAction::Ignore, replacement)
    }

    /// Creates a report policy with the default replacement retained for
    /// metadata. The replacement is not encoded while the action is `Report`.
    #[must_use]
    #[inline]
    pub const fn report() -> Self {
        Self::new(UnmappableAction::Report, Self::DEFAULT_REPLACEMENT)
    }

    /// Selects the default replacement policy supported by `codec`.
    ///
    /// # Parameters
    ///
    /// - `codec`: Codec used to check replacement encodability.
    ///
    /// # Returns
    ///
    /// Returns a replacement policy using U+FFFD when it is encodable, or the
    /// fallback `?` when only that candidate is encodable.
    ///
    /// # Errors
    ///
    /// Returns [`CharsetEncodeError`] when neither replacement can be encoded
    /// by `codec`.
    pub fn default_for<C>(codec: &C) -> Result<Self, CharsetEncodeError>
    where
        C: CharsetCodec,
    {
        if codec.can_encode_value(&Self::DEFAULT_REPLACEMENT) {
            return Ok(Self::default());
        }
        if codec.can_encode_value(&Self::DEFAULT_FALLBACK_REPLACEMENT) {
            return Ok(Self::replace(Self::DEFAULT_FALLBACK_REPLACEMENT));
        }
        let kind = CharsetEncodeErrorKind::UnmappableCharacter {
            value: Self::DEFAULT_FALLBACK_REPLACEMENT as u32,
        };
        Err(CharsetEncodeError::new(codec.charset(), kind, 0))
    }

    /// Returns the unmappable-input action.
    #[must_use]
    #[inline]
    pub const fn unmappable_action(self) -> UnmappableAction {
        self.unmappable_action
    }

    /// Returns the replacement character.
    #[must_use]
    #[inline]
    pub const fn replacement(self) -> char {
        self.replacement
    }
}

impl Default for CharsetEncodePolicy {
    #[inline]
    fn default() -> Self {
        Self::replace(Self::DEFAULT_REPLACEMENT)
    }
}
