// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec::Codec;
use qubit_codec_text::{
    CharsetCodec,
    Utf8Codec,
};

fn assert_charset_codec<T>(_codec: &T)
where
    T: CharsetCodec<Unit = u8>,
{
}

fn assert_charset_codec_is_core_codec<T>(_codec: &T)
where
    T: CharsetCodec<Unit = u8>,
{
    fn assert_core_codec<C>()
    where
        C: Codec<Value = char, Unit = u8>,
    {
    }

    assert_core_codec::<T>();
}

#[test]
fn test_charset_codec_is_implemented_for_combined_codecs() {
    assert_charset_codec(&Utf8Codec);
    assert_charset_codec_is_core_codec(&Utf8Codec);
}
