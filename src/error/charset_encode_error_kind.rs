/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use thiserror::Error;

/// Classifies failures detected while encoding Unicode text into encoded units.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CharsetEncodeErrorKind {
    /// The supplied code point is not a valid Unicode scalar value.
    #[error("The code point is not a valid Unicode scalar value.")]
    InvalidCodePoint {
        /// Raw code point value reported by the codec.
        value: u32,
    },

    /// The requested input character index is outside the input buffer.
    #[error("The input character index is outside the input buffer.")]
    InvalidInputIndex {
        /// Length of the input provided to the codec call.
        input_len: usize,
    },

    /// The character cannot be represented by the target encoding.
    #[error("The character cannot be represented by the target encoding.")]
    UnmappableCharacter {
        /// Raw character value that cannot be represented.
        value: u32,
    },

    /// The supplied output buffer is too small for the encoded character.
    #[error("The output buffer is too small (required {required} units, available {available} units).")]
    BufferTooSmall {
        /// Total units required to encode the character.
        required: usize,

        /// Total units currently available for the requested output index.
        available: usize,
    },
}

impl CharsetEncodeErrorKind {
    /// Returns the raw value associated with the error kind, if available.
    ///
    /// # Returns
    ///
    /// - `Some(value)` for [`Self::InvalidCodePoint`] and [`Self::UnmappableCharacter`];
    /// - `None` for other kinds.
    #[must_use]
    pub const fn value(self) -> Option<u32> {
        match self {
            Self::InvalidCodePoint { value, .. } => Some(value),
            Self::UnmappableCharacter { value, .. } => Some(value),
            Self::BufferTooSmall { .. } | Self::InvalidInputIndex { .. } => None,
        }
    }

    /// Returns the number of required output units for this kind, if available.
    ///
    /// # Returns
    ///
    /// - `Some(required)` for [`Self::BufferTooSmall`];
    /// - `None` for all other variants.
    #[must_use]
    pub const fn required(self) -> Option<usize> {
        match self {
            Self::BufferTooSmall { required, .. } => Some(required),
            Self::InvalidInputIndex { .. } | Self::InvalidCodePoint { .. } | Self::UnmappableCharacter { .. } => None,
        }
    }

    /// Returns the number of currently available output units for this kind, if
    /// available.
    ///
    /// # Returns
    ///
    /// - `Some(available)` for [`Self::BufferTooSmall`];
    /// - `None` for all other variants.
    #[must_use]
    pub const fn available(self) -> Option<usize> {
        match self {
            Self::BufferTooSmall { available, .. } => Some(available),
            Self::InvalidInputIndex { .. } | Self::InvalidCodePoint { .. } | Self::UnmappableCharacter { .. } => None,
        }
    }

    /// Returns the input length when this error comes from an invalid input index.
    ///
    /// # Returns
    ///
    /// - `Some(input_len)` for [`Self::InvalidInputIndex`];
    /// - `None` for other variants.
    #[must_use]
    pub const fn input_len(self) -> Option<usize> {
        match self {
            Self::InvalidInputIndex { input_len } => Some(input_len),
            Self::InvalidCodePoint { .. } | Self::UnmappableCharacter { .. } | Self::BufferTooSmall { .. } => None,
        }
    }
}
