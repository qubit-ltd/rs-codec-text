/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Encoding plans used by charset encoders.

/// Write plan used by [`crate::CharsetEncoder`].
///
/// This plan is produced by the charset encoder's internal
/// [`qubit_codec::BufferedEncodeHooks`] implementation. Normal callers usually
/// interact with [`crate::CharsetEncoder`] through [`crate::Transcoder`] instead
/// of constructing plans directly.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CharsetEncodePlan {
    /// Encode the original input character.
    Original,

    /// Copy cached replacement units.
    Replacement,

    /// Consume the input character without writing output.
    Ignore,
}
