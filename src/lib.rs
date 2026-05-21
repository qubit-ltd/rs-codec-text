/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Qubit Text Codec
//!
//! Low-level Unicode constants, character classification helpers, and text codec
//! primitives for UTF-8, UTF-16, UTF-32, and ASCII-oriented code.
//!
//! This crate deliberately stays below `std::io::Read` and `std::io::Write`.
//! Concrete text I/O adapters are expected to own buffering, EOF handling, line
//! endings, and `std::io::Error` mapping while using the codecs from this crate
//! for strict buffer-level encoding and decoding.

mod charset;
mod codec;
mod encoding;
mod error;

pub mod prelude;
pub use charset::{
    Ascii,
    Unicode,
    Utf8,
    Utf16,
    Utf32,
};
pub use codec::{
    DecodeStatus,
    TextCodec,
    TextDecoder,
    TextEncoder,
};
pub use codec::{
    Utf8Codec,
    Utf8Decoder,
    Utf8Encoder,
    Utf16ByteCodec,
    Utf16ByteDecoder,
    Utf16ByteEncoder,
    Utf16U16Codec,
    Utf16U16Decoder,
    Utf16U16Encoder,
    Utf32ByteCodec,
    Utf32ByteDecoder,
    Utf32ByteEncoder,
    Utf32U32Codec,
    Utf32U32Decoder,
    Utf32U32Encoder,
};
pub use encoding::{
    ByteOrder,
    Charset,
    UnicodeBom,
};
pub use error::{
    TextDecodeError,
    TextDecodeErrorKind,
    TextDecodeResult,
    TextEncodeError,
    TextEncodeErrorKind,
    TextEncodeResult,
};
