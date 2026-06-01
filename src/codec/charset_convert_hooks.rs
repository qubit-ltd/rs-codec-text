/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Policy hooks used by charset converters.

use qubit_codec::BufferedConvertHooks;

use crate::{
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetEncodeError,
};

use super::{
    charset_codec::CharsetCodec,
    charset_convert_error::CharsetConvertError,
    charset_decode_hooks::CharsetDecodeHooks,
    charset_decode_policy::CharsetDecodePolicy,
    charset_encode_hooks::CharsetEncodeHooks,
    charset_encode_policy::CharsetEncodePolicy,
    charset_encode_probe::CharsetEncodeProbe,
    charset_encoder::CharsetEncoder,
};

/// Policy hooks for [`super::CharsetConverter`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) struct CharsetConvertHooks {
    /// Malformed source-input policy.
    decode_policy: CharsetDecodePolicy,
    /// Unmappable target-output policy.
    encode_policy: CharsetEncodePolicy,
    /// Whether default replacement initialization may fall back to `?`.
    allow_default_fallback: bool,
}

impl CharsetConvertHooks {
    /// Creates charset converter hooks with explicit policies.
    #[must_use]
    pub(super) const fn with_policies(decode_policy: CharsetDecodePolicy, encode_policy: CharsetEncodePolicy) -> Self {
        Self {
            decode_policy,
            encode_policy,
            allow_default_fallback: false,
        }
    }
}

impl Default for CharsetConvertHooks {
    /// Creates default charset converter hooks.
    fn default() -> Self {
        Self {
            decode_policy: CharsetDecodePolicy::replace(CharsetDecodePolicy::DEFAULT_REPLACEMENT),
            encode_policy: CharsetEncodePolicy::replace(CharsetEncodePolicy::DEFAULT_REPLACEMENT),
            allow_default_fallback: true,
        }
    }
}

impl<D, E> BufferedConvertHooks<D, E, D::Unit, char, E::Unit> for CharsetConvertHooks
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
    fn create_decode_hooks(&self, _decode_codec: &D, _encode_codec: &E) -> Self::DecodeHooks {
        CharsetDecodeHooks::from_policy(self.decode_policy)
    }

    /// Creates default charset encode hooks.
    fn create_encode_hooks(&self, _decode_codec: &D, encode_codec: &E) -> Self::EncodeHooks {
        match CharsetEncoder::<E>::create_hooks(encode_codec, self.encode_policy) {
            Ok((hooks, _)) => hooks,
            Err(_default_error) => {
                assert!(
                    self.allow_default_fallback,
                    "explicit charset converter policies must be prevalidated before building hooks"
                );
                let fallback_policy = CharsetEncodePolicy::replace(CharsetEncodePolicy::DEFAULT_FALLBACK_REPLACEMENT);
                CharsetEncoder::<E>::create_hooks(encode_codec, fallback_policy)
                    .map(|(hooks, _)| hooks)
                    .expect("default converter fallback replacement should be encodable")
            }
        }
    }

    /// Maps decoder errors into converter decode errors.
    fn map_decode_error(&self, error: Self::DecodeError) -> Self::Error {
        CharsetConvertError::Decode(error)
    }

    /// Maps encoder errors into converter encode errors.
    fn map_encode_error(&self, error: Self::EncodeError) -> Self::Error {
        CharsetConvertError::Encode(error)
    }

    /// Creates an input-index error using the source charset.
    fn invalid_input_index(&self, decode_codec: &D, index: usize, input_len: usize) -> Self::Error {
        let kind = CharsetDecodeErrorKind::InvalidInputIndex { input_len };
        CharsetConvertError::Decode(CharsetDecodeError::new(decode_codec.charset(), kind, index))
    }
}
