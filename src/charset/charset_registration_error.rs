// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use core::{
    error::Error,
    fmt,
};

use super::{
    charset::Charset,
    charset_registration_error_kind::CharsetRegistrationErrorKind,
};

/// Error returned when a charset cannot be registered.
///
/// Registration rejects labels that normalize to an empty lookup key or would
/// make global lookup ambiguous. Label comparison uses the same loose
/// normalization as [`Charset::from_label`], so conflicts include case
/// differences and `-` / `_` separator differences.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharsetRegistrationError {
    /// Candidate label that failed validation or conflicts.
    label: &'static str,
    /// Registration failure category.
    kind: CharsetRegistrationErrorKind,
    /// Charset that failed to register.
    candidate: Charset,
}

impl CharsetRegistrationError {
    /// Creates a conflicting-label registration error.
    ///
    /// # Parameters
    ///
    /// - `label`: Candidate label that conflicts.
    /// - `existing`: Charset already owning the label.
    /// - `candidate`: Charset being registered.
    ///
    /// # Returns
    ///
    /// Returns an error carrying the conflicting registration context.
    #[inline]
    pub(crate) const fn conflicting_label(
        label: &'static str,
        existing: Charset,
        candidate: Charset,
    ) -> Self {
        Self {
            label,
            kind: CharsetRegistrationErrorKind::ConflictingLabel { existing },
            candidate,
        }
    }

    /// Creates an invalid-label registration error.
    ///
    /// # Parameters
    ///
    /// - `label`: Candidate label that is invalid.
    /// - `candidate`: Charset being registered.
    ///
    /// # Returns
    ///
    /// Returns an error carrying the invalid registration context.
    #[inline]
    pub(crate) const fn invalid_label(
        label: &'static str,
        candidate: Charset,
    ) -> Self {
        Self {
            label,
            kind: CharsetRegistrationErrorKind::InvalidLabel,
            candidate,
        }
    }

    /// Returns the candidate label that caused the error.
    ///
    /// # Returns
    ///
    /// Returns the static label from the candidate charset descriptor.
    #[inline]
    pub const fn label(self) -> &'static str {
        self.label
    }

    /// Returns the registration failure category.
    ///
    /// # Returns
    ///
    /// Returns the stored [`CharsetRegistrationErrorKind`].
    #[inline]
    pub const fn kind(self) -> CharsetRegistrationErrorKind {
        self.kind
    }

    /// Returns the charset that already owns the conflicting label.
    ///
    /// # Returns
    ///
    /// Returns `Some(charset)` for conflicting labels, or `None` when the
    /// candidate label itself is invalid.
    #[inline]
    pub const fn existing(self) -> Option<Charset> {
        match self.kind {
            CharsetRegistrationErrorKind::ConflictingLabel { existing } => {
                Some(existing)
            }
            CharsetRegistrationErrorKind::InvalidLabel => None,
        }
    }

    /// Returns the charset that failed to register.
    ///
    /// # Returns
    ///
    /// Returns the candidate charset descriptor.
    #[inline]
    pub const fn candidate(self) -> Charset {
        self.candidate
    }
}

impl fmt::Display for CharsetRegistrationError {
    /// Formats this registration error.
    ///
    /// # Parameters
    ///
    /// - `formatter`: The formatter receiving the diagnostic message.
    ///
    /// # Errors
    ///
    /// Returns any formatting error reported by `formatter`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            CharsetRegistrationErrorKind::InvalidLabel => {
                write!(
                    formatter,
                    "charset label {:?} for {} is invalid",
                    self.label, self.candidate,
                )
            }
            CharsetRegistrationErrorKind::ConflictingLabel { existing } => {
                write!(
                    formatter,
                    "charset label {:?} for {} conflicts with {}",
                    self.label, self.candidate, existing,
                )
            }
        }
    }
}

impl Error for CharsetRegistrationError {}
