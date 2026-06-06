// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::{
    CharsetCodec,
    CharsetEncodeResult,
};

/// Encoding-size probe used by [`crate::CharsetEncoder`].
///
/// Encoders use this trait to validate mappability and compute the exact output
/// unit count before calling unsafe [`qubit_codec::Codec::encode_unchecked`].
///
/// # Implementor Contract
///
/// For the same codec state, input character, and output index, a successful
/// [`Self::encode_len`] call must return exactly the number of units that
/// [`qubit_codec::Codec::encode_unchecked`] will write when the caller supplies
/// sufficient output capacity. Both methods must also agree on charset
/// mappability: a character accepted by `encode_len` must not later be reported
/// as unmappable by `encode_unchecked`, and a character rejected as unmappable
/// by `encode_len` must not be encoded by `encode_unchecked`.
pub trait CharsetEncodeProbe: CharsetCodec {
    /// Computes the number of units needed to encode one character.
    ///
    /// # Parameters
    ///
    /// - `ch`: Unicode scalar value to encode.
    /// - `index`: Input character index used for error context.
    ///
    /// # Returns
    ///
    /// Returns the exact number of output units required for `ch`.
    ///
    /// # Errors
    ///
    /// Returns [`crate::CharsetEncodeError`] when `ch` cannot be represented by
    /// this charset.
    fn encode_len(&self, ch: char, index: usize) -> CharsetEncodeResult<usize>;
}
