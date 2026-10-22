// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::TranscodeConvertError;
use qubit_codec::TranscodeConvertErrorOf;
use qubit_codec::TranscodeFailure;

use crate::Charset;
use crate::CharsetCodec;
use crate::CharsetDecodeError;
use crate::CharsetEncodeError;

/// Error reported while converting between two charsets.
///
/// # Examples
///
/// ```
/// use qubit_codec_text::Charset;
/// use qubit_codec_text::CharsetConvertError;
/// use qubit_codec_text::CharsetDecodeError;
/// use qubit_codec_text::CharsetDecodeErrorKind;
///
/// let decode_error = CharsetDecodeError::new(
///     Charset::UTF_8,
///     CharsetDecodeErrorKind::UnexpectedTranscodeFailure,
///     0,
/// );
/// let error = CharsetConvertError::from(decode_error);
///
/// assert!(matches!(error, CharsetConvertError::Decode(_)));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[must_use]
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
    /// # Type Parameters
    ///
    /// - `D`: Source codec used to identify the low-level decoder.
    /// - `E`: Target codec used to identify the low-level encoder.
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
            TranscodeConvertError::Unencodable { input_index, value } => {
                Self::Encode(CharsetEncodeError::map_unencodable(target_charset, input_index, value))
            }
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
    fn from_transcode_failure(source_charset: Charset, target_charset: Charset, failure: TranscodeFailure) -> Self {
        match failure {
            failure @ (TranscodeFailure::InvalidInputIndex { .. }
            | TranscodeFailure::IncompleteInput { .. }
            | TranscodeFailure::TrailingInput { .. }) => {
                Self::Decode(CharsetDecodeError::map_transcode_failure(source_charset, failure))
            }
            failure @ (TranscodeFailure::InvalidOutputIndex { .. }
            | TranscodeFailure::InvalidOutputRange { .. }
            | TranscodeFailure::InsufficientOutput { .. }
            | TranscodeFailure::OutputLengthOverflow) => {
                Self::Encode(CharsetEncodeError::map_transcode_failure(target_charset, failure))
            }
            failure => Self::Decode(CharsetDecodeError::map_transcode_failure(source_charset, failure)),
        }
    }
}
