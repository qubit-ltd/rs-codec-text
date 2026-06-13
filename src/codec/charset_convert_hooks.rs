// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Policy hooks used by charset converters.

use qubit_codec::TranscodeConvertHooks;

use crate::{
    CharsetDecodeError, CharsetDecodeHooks, CharsetDecodePolicy, CharsetEncodeError,
    CharsetEncodeHooks, CharsetEncodePolicy, CharsetEncodeProbe,
};

use super::{charset_codec::CharsetCodec, charset_convert_error::CharsetConvertError};

/// Policy hooks for [`super::CharsetConverter`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CharsetConvertHooks<Unit> {
    /// Malformed source-input policy.
    decode_policy: CharsetDecodePolicy,
    /// Unmappable target-output policy.
    encode_policy: CharsetEncodePolicy,
    /// Prevalidated target-side encode hooks.
    encode_hooks: CharsetEncodeHooks<Unit>,
}

impl<Unit> CharsetConvertHooks<Unit> {
    /// Creates charset converter hooks with explicit policies.
    #[must_use]
    #[inline(always)]
    pub(super) const fn with_policies(
        decode_policy: CharsetDecodePolicy,
        encode_policy: CharsetEncodePolicy,
        encode_hooks: CharsetEncodeHooks<Unit>,
    ) -> Self {
        Self {
            decode_policy,
            encode_policy,
            encode_hooks,
        }
    }
}

impl<D, E> TranscodeConvertHooks<D, E> for CharsetConvertHooks<E::Unit>
where
    D: CharsetCodec,
    E: CharsetEncodeProbe,
{
    type DecodeError = CharsetDecodeError;
    type DecodeHooks = CharsetDecodeHooks;
    type EncodeError = CharsetEncodeError;
    type EncodeHooks = CharsetEncodeHooks<E::Unit>;
    type Error = CharsetConvertError;

    /// Creates default charset decode hooks.
    #[inline(always)]
    fn create_decode_hooks(&self, _decode_codec: &D, _encode_codec: &E) -> Self::DecodeHooks {
        CharsetDecodeHooks::from_policy(self.decode_policy)
    }

    /// Returns the prevalidated charset encode hooks.
    #[inline(always)]
    fn create_encode_hooks(&self, _decode_codec: &D, _encode_codec: &E) -> Self::EncodeHooks {
        self.encode_hooks.clone()
    }

    /// Maps decoder errors into converter decode errors.
    #[inline(always)]
    fn map_decode_error(&self, error: Self::DecodeError) -> Self::Error {
        CharsetConvertError::Decode(error)
    }

    /// Maps encoder errors into converter encode errors.
    #[inline(always)]
    fn map_encode_error(&self, error: Self::EncodeError) -> Self::Error {
        CharsetConvertError::Encode(error)
    }
}
