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
pub enum TextEncodeErrorKind {
    /// The supplied code point is not a valid Unicode scalar value.
    #[error("The code point is not a valid Unicode scalar value.")]
    InvalidCodePoint,

    /// The character cannot be represented by the target encoding.
    #[error("The character cannot be represented by the target encoding.")]
    UnmappableCharacter,

    /// The supplied output buffer is too small for the encoded character.
    #[error("The output buffer is too small.")]
    BufferTooSmall,
}
