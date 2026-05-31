/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use crate::{
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetEncodeError,
};
use qubit_codec::ConvertErrorFactory;

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

impl<D> ConvertErrorFactory<D> for CharsetConvertError
where
    D: CharsetCodec,
{
    /// Creates an input-index error for a charset converter.
    fn invalid_input_index(decoder: &D, index: usize, input_len: usize) -> Self {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len };
        Self::Decode(CharsetDecodeError::new(decoder.charset(), kind, index))
    }
}
