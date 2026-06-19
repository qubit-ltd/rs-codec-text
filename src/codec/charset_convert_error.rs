// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::{
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
