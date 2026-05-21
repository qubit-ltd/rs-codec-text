/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0.
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
mod decode_status;
mod inner;
mod text_codec;
mod text_decoder;
mod text_encoder;
mod utf16_byte_codec;
mod utf16_byte_decoder;
mod utf16_byte_encoder;
mod utf16_u16_codec;
mod utf16_u16_decoder;
mod utf16_u16_encoder;
mod utf32_byte_codec;
mod utf32_byte_decoder;
mod utf32_byte_encoder;
mod utf32_u32_codec;
mod utf32_u32_decoder;
mod utf32_u32_encoder;
mod utf8_codec;
mod utf8_decoder;
mod utf8_encoder;

pub use decode_status::DecodeStatus;
pub use text_codec::TextCodec;
pub use text_decoder::TextDecoder;
pub use text_encoder::TextEncoder;
pub use utf8_codec::Utf8Codec;
pub use utf8_decoder::Utf8Decoder;
pub use utf8_encoder::Utf8Encoder;
pub use utf16_byte_codec::Utf16ByteCodec;
pub use utf16_byte_decoder::Utf16ByteDecoder;
pub use utf16_byte_encoder::Utf16ByteEncoder;
pub use utf16_u16_codec::Utf16U16Codec;
pub use utf16_u16_decoder::Utf16U16Decoder;
pub use utf16_u16_encoder::Utf16U16Encoder;
pub use utf32_byte_codec::Utf32ByteCodec;
pub use utf32_byte_decoder::Utf32ByteDecoder;
pub use utf32_byte_encoder::Utf32ByteEncoder;
pub use utf32_u32_codec::Utf32U32Codec;
pub use utf32_u32_decoder::Utf32U32Decoder;
pub use utf32_u32_encoder::Utf32U32Encoder;
