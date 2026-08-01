// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::{TranscodeConvertError, TranscodeConvertErrorOf, TranscodeFailure};

use crate::{Charset, CharsetCodec, CharsetDecodeError, CharsetEncodeError};

/// Error reported while converting between two charsets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum CharsetConvertError {
    /// Source decoding failed.
    #[error("Failed to decode source charset: {0}")]
    Decode(#[from] CharsetDecodeError),

    /// Target encoding failed.
    #[error("Failed to encode target charset: {0}")]
    Encode(#[from] CharsetEncodeError),
}

impl CharsetConvertError {
    /// Maps a low-level converter error into a charset-specific error.
    ///
    /// # Parameters
    ///
    /// - `source_charset`: Charset attached to source-side framework failures.
    /// - `target_charset`: Charset attached to target-side framework failures.
    /// - `error`: Low-level converter error to map.
    ///
    /// # Returns
    ///
    /// Returns a source [`Self::Decode`] error for source failures and a target
    /// [`Self::Encode`] error for target failures. Domain errors retain their
    /// original charset context.
    #[must_use]
    pub fn from_transcode_error<D, E>(
        source_charset: Charset,
        target_charset: Charset,
        error: TranscodeConvertErrorOf<D, E>,
    ) -> Self
    where
        D: CharsetCodec,
        E: CharsetCodec,
    {
        match error {
            TranscodeConvertError::Failure(failure) => {
                Self::from_transcode_failure(source_charset, target_charset, failure)
            }
            TranscodeConvertError::DecodeDomain(error) => Self::Decode(error.into_source()),
            TranscodeConvertError::EncodeDomain(error) => Self::Encode(error.into_source()),
            TranscodeConvertError::Unencodable { input_index, value } => Self::Encode(
                CharsetEncodeError::map_unencodable(target_charset, input_index, value),
            ),
        }
    }

    /// Maps a framework-level converter failure into a charset-specific error.
    ///
    /// # Parameters
    ///
    /// - `source_charset`: Charset attached to source-side failures.
    /// - `target_charset`: Charset attached to target-side failures.
    /// - `failure`: Framework-level converter failure to map.
    ///
    /// # Returns
    ///
    /// Returns [`Self::Decode`] for source-input failures and [`Self::Encode`]
    /// for target-output failures. Other framework failures remain decode-side
    /// unexpected failures to preserve the prior error contract.
    #[must_use]
    fn from_transcode_failure(
        source_charset: Charset,
        target_charset: Charset,
        failure: TranscodeFailure,
    ) -> Self {
        match failure {
            failure @ (TranscodeFailure::InvalidInputIndex { .. }
            | TranscodeFailure::IncompleteInput { .. }
            | TranscodeFailure::TrailingInput { .. }) => Self::Decode(
                CharsetDecodeError::map_transcode_failure(source_charset, failure),
            ),
            failure @ (TranscodeFailure::InvalidOutputIndex { .. }
            | TranscodeFailure::InvalidOutputRange { .. }
            | TranscodeFailure::InsufficientOutput { .. }
            | TranscodeFailure::OutputLengthOverflow) => Self::Encode(
                CharsetEncodeError::map_transcode_failure(target_charset, failure),
            ),
            failure => Self::Decode(CharsetDecodeError::map_transcode_failure(
                source_charset,
                failure,
            )),
        }
    }
}
