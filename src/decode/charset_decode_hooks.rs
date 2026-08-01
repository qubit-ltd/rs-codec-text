// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow inline-tests
use qubit_codec::engine::{DecodeContext, DecodeInvalidAction, TranscodeDecodeHooks};
use qubit_codec::{CapacityError, TranscodeDecodeError, TranscodeDecodeErrorOf};

use crate::{CharsetCodec, CharsetDecodeError, MalformedAction};

/// Malformed-input policy hooks used by [`super::CharsetDecoder`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct CharsetDecodeHooks {
    /// Action used for malformed input units.
    pub(super) malformed_action: MalformedAction,
    /// Replacement character used by [`MalformedAction::Replace`].
    pub(super) replacement: char,
}

impl CharsetDecodeHooks {
    /// Creates charset decode hooks.
    ///
    /// # Parameters
    ///
    /// - `malformed_action`: Initial malformed-input policy.
    /// - `replacement`: Replacement character used by replace policy.
    ///
    /// # Returns
    ///
    /// Returns hooks carrying the supplied policy.
    #[must_use]
    #[inline]
    pub(crate) const fn new(malformed_action: MalformedAction, replacement: char) -> Self {
        Self {
            malformed_action,
            replacement,
        }
    }
}

impl<C> TranscodeDecodeHooks<C> for CharsetDecodeHooks
where
    C: CharsetCodec,
{
    /// Returns the maximum number of characters decoded from `input_len` units.
    #[inline]
    fn max_transcode_output_len(
        &self,
        _codec: &C,
        input_len: usize,
    ) -> Result<usize, CapacityError> {
        Ok(input_len)
    }

    /// Handles a charset decode failure during `transcode`.
    fn handle_invalid_decode(
        &mut self,
        _codec: &mut C,
        error: &CharsetDecodeError,
        _consumed: Option<core::num::NonZeroUsize>,
        context: DecodeContext,
    ) -> Result<DecodeInvalidAction<char>, TranscodeDecodeErrorOf<C>> {
        if error.kind().is_malformed_input() {
            let consumed = error
                .consumed()
                .expect("malformed decode errors carry consumed width");
            return match self.malformed_action {
                MalformedAction::Report => Err(TranscodeDecodeError::domain_main(
                    *error,
                    context.input_index(),
                )),
                MalformedAction::Ignore => Ok(DecodeInvalidAction::Skip { consumed }),
                MalformedAction::Replace => Ok(DecodeInvalidAction::Emit {
                    value: self.replacement,
                    consumed,
                }),
            };
        }
        Err(TranscodeDecodeError::domain_main(
            *error,
            context.input_index(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroUsize;

    use qubit_codec::engine::{DecodeContext, DecodeInvalidAction, TranscodeDecodeHooks};

    use super::CharsetDecodeHooks;
    use crate::{Charset, CharsetDecodeError, CharsetDecodeErrorKind, MalformedAction, Utf8Codec};

    fn malformed_error() -> CharsetDecodeError {
        CharsetDecodeError::new(Charset::UTF_8, CharsetDecodeErrorKind::malformed(0x80), 3)
            .with_consumed(NonZeroUsize::MIN)
    }

    #[test]
    fn test_charset_decode_hooks_handle_each_policy_action() {
        let mut codec = Utf8Codec;
        let context = DecodeContext::new(0, 3, 0, 2, 1);
        let error = malformed_error();

        let hooks = CharsetDecodeHooks::new(MalformedAction::Replace, '!');
        assert_eq!(
            Ok(4),
            <CharsetDecodeHooks as TranscodeDecodeHooks<Utf8Codec>>::max_transcode_output_len(
                &hooks, &codec, 4,
            )
        );

        let mut hooks = CharsetDecodeHooks::new(MalformedAction::Replace, '!');
        assert_eq!(
            Ok(DecodeInvalidAction::Emit {
                value: '!',
                consumed: NonZeroUsize::MIN,
            }),
            hooks.handle_invalid_decode(&mut codec, &error, None, context)
        );

        let mut hooks = CharsetDecodeHooks::new(MalformedAction::Ignore, '!');
        assert_eq!(
            Ok(DecodeInvalidAction::Skip {
                consumed: NonZeroUsize::MIN,
            }),
            hooks.handle_invalid_decode(&mut codec, &error, None, context)
        );

        let mut hooks = CharsetDecodeHooks::new(MalformedAction::Report, '!');
        assert!(
            hooks
                .handle_invalid_decode(&mut codec, &error, None, context)
                .is_err()
        );

        let non_malformed = CharsetDecodeError::new(
            Charset::UTF_8,
            CharsetDecodeErrorKind::InvalidInputIndex { input_len: 3 },
            4,
        );
        assert!(
            hooks
                .handle_invalid_decode(&mut codec, &non_malformed, None, context)
                .is_err()
        );
    }
}
