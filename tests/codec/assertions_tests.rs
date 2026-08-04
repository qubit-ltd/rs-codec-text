// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
pub(crate) fn invalid_source(
    failure: qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
) -> qubit_codec_text::CharsetDecodeError {
    match failure {
        qubit_codec::DecodeFailure::Invalid { source, .. } => source,
        qubit_codec::DecodeFailure::Incomplete { .. } => {
            panic!("expected invalid charset decode failure")
        }
    }
}

pub(crate) fn incomplete_required(
    failure: qubit_codec::DecodeFailure<qubit_codec_text::CharsetDecodeError>,
) -> usize {
    match failure {
        qubit_codec::DecodeFailure::Incomplete { required_total, .. } => {
            required_total.get()
        }
        qubit_codec::DecodeFailure::Invalid { .. } => {
            panic!("expected incomplete charset decode failure")
        }
    }
}
