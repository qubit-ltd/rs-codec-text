/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use crate::CharsetEncodeResult;

use super::charset_codec::CharsetCodec;

/// Encoding-size probe used by [`crate::CharsetEncoder`].
///
/// Encoders use this trait to validate mappability and compute the exact output
/// unit count before calling unsafe [`qubit_codec::Codec::encode_unchecked`].
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
