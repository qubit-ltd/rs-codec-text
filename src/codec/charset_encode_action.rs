/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Encoding actions used by charset encoders.

/// Write action used by [`crate::CharsetEncoder`].
///
/// This action is produced by the charset encoder's internal
/// [`qubit_codec::BufferedEncodeHooks`] implementation. Normal callers usually
/// interact with [`crate::CharsetEncoder`] through [`crate::BufferedTranscoder`] instead
/// of constructing actions directly.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CharsetEncodeAction {
    /// Encode the original input character.
    WriteOriginal,

    /// Encode the configured replacement character.
    WriteReplacement,

    /// Consume the input character without writing output.
    Skip,
}
