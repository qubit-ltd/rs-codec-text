// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Qubit Text Codec
//!
//! Low-level Unicode constants, character classification helpers, and text
//! codec primitives for ASCII, ISO-8859-1, UTF-8, UTF-16, and UTF-32-oriented
//! code.
//!
//! This crate deliberately stays below `std::io::Read` and `std::io::Write`.
//! Concrete text I/O adapters are expected to own buffering, EOF handling, line
//! endings, and `std::io::Error` mapping while using the codecs from this crate
//! for strict buffer-level encoding and decoding.

mod charset;
mod codec;
mod decode;
mod encode;
mod error;

pub mod prelude;
pub use charset::{
    Ascii,
    Charset,
    Latin1,
    Unicode,
    UnicodeBom,
    Utf8,
    Utf16,
    Utf32,
};
pub use codec::{
    AsciiCodec,
    CharsetCodec,
    CharsetConvertError,
    CharsetConverter,
    Latin1Codec,
    MalformedAction,
    UnmappableAction,
};
pub use codec::{
    Utf8Codec,
    Utf16ByteCodec,
    Utf16U16Codec,
    Utf32ByteCodec,
    Utf32U32Codec,
};
pub(crate) use decode::CharsetDecodeHooks;
pub use decode::{
    CharsetDecodePolicy,
    CharsetDecoder,
};
pub(crate) use encode::CharsetEncodeHooks;
pub use encode::{
    CharsetEncodePolicy,
    CharsetEncoder,
};
pub use error::{
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
};
pub use qubit_codec::{
    ByteOrder,
    CapacityError,
    Codec,
    TranscodeConvertEngine,
    TranscodeConvertHooks,
    TranscodeConverter,
    TranscodeDecodeEngine,
    TranscodeDecodeHooks,
    TranscodeDecoder,
    TranscodeEncodeEngine,
    TranscodeEncodeHooks,
    TranscodeEncoder,
    TranscodeProgress,
    TranscodeStatus,
    Transcoder,
};
