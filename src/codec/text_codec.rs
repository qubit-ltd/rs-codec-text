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
    TextDecoder,
    TextEncoder,
};

/// Combined text encoder and decoder for the same storage unit type.
///
/// # Examples
///
/// ```rust
/// use qubit_text_codec::{
///     TextCodec,
///     Utf8Codec,
/// };
///
/// fn accepts_utf8_codec<C: TextCodec<u8>>(_codec: C) {}
///
/// accepts_utf8_codec(Utf8Codec);
/// ```
pub trait TextCodec<T>: TextEncoder<T> + TextDecoder<T> {}

impl<T, C> TextCodec<T> for C where C: TextEncoder<T> + TextDecoder<T> {}
