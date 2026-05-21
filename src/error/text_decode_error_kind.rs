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

/// Classifies failures detected while decoding encoded units into Unicode text.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TextDecodeErrorKind {
    /// The input units do not form a well-formed encoded sequence.
    #[error("The encoded text sequence is malformed.")]
    MalformedSequence,

    /// The closed input ended before a complete character was available.
    #[error("The encoded text sequence is incomplete.")]
    IncompleteSequence,

    /// The decoded numeric value is not a valid Unicode scalar value.
    #[error("The decoded code point is not a valid Unicode scalar value.")]
    InvalidCodePoint,
}
