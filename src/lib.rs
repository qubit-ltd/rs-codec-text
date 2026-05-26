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
//! primitives for ASCII, ISO-8859-1, UTF-8, UTF-16, and UTF-32-oriented code.
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
    AsciiCodec,
    CharsetCodec,
    CharsetConvertError,
    CharsetConverter,
    CharsetDecoder,
    CharsetEncoder,
    DecodeStatus,
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
pub use encoding::{
    Charset,
    UnicodeBom,
};
pub use error::{
    CharsetDecodeError,
    CharsetDecodeErrorKind,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
};
pub use qubit_io::{
    ByteOrder,
    Coder,
    CoderProgress,
    CoderStatus,
};
