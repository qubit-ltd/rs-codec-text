// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::DecodeFailure;
use qubit_codec_text::CharsetDecodeError;

pub(crate) fn invalid_source(
    failure: DecodeFailure<CharsetDecodeError>,
) -> CharsetDecodeError {
    match failure {
        DecodeFailure::Invalid { source, .. } => source,
        DecodeFailure::Incomplete { .. } => {
            panic!("expected invalid charset decode failure")
        }
    }
}

pub(crate) fn incomplete_required(
    failure: DecodeFailure<CharsetDecodeError>,
) -> usize {
    match failure {
        DecodeFailure::Incomplete { required_total, .. } => {
            required_total.get()
        }
        DecodeFailure::Invalid { .. } => {
            panic!("expected incomplete charset decode failure")
        }
    }
}
