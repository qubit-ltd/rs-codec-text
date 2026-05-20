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

/// Classifies errors reported by low-level Unicode operations.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UnicodeErrorKind {
    /// The output buffer does not have enough room for the encoded code point.
    #[error("The buffer overflows.")]
    BufferOverflow,

    /// The input code-unit sequence is malformed.
    #[error("The Unicode code unit sequence is malformed.")]
    Malformed,

    /// The input code-unit sequence ends before a full code point is available.
    #[error("The Unicode code unit sequence is incomplete.")]
    Incomplete,
}
