// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::TranscodeError;

use crate::{
    Charset,
    CharsetDecodeError,
    CharsetEncodeError,
};

/// Error reported while converting between two charsets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum CharsetConvertError {
    /// Source decoding failed.
    #[error("Failed to decode source charset: {0}")]
    Decode(#[from] CharsetDecodeError),

    /// Target encoding failed.
    #[error("Failed to encode target charset: {0}")]
    Encode(#[from] CharsetEncodeError),
}

impl TranscodeError<(Charset, Charset)> for CharsetConvertError {
    #[inline(always)]
    fn invalid_input_index(
        context: (Charset, Charset),
        index: usize,
        len: usize,
    ) -> Self {
        Self::Decode(CharsetDecodeError::invalid_input_index(
            context.0, index, len,
        ))
    }

    #[inline(always)]
    fn invalid_output_index(
        context: (Charset, Charset),
        index: usize,
        len: usize,
    ) -> Self {
        Self::Encode(CharsetEncodeError::invalid_output_index(
            context.1, index, len,
        ))
    }

    #[inline(always)]
    fn insufficient_output(
        context: (Charset, Charset),
        output_index: usize,
        required: usize,
        available: usize,
    ) -> Self {
        Self::Encode(CharsetEncodeError::insufficient_output(
            context.1,
            output_index,
            required,
            available,
        ))
    }
}
