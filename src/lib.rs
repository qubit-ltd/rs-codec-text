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
mod convert;
mod decode;
mod encode;
mod error;
mod util;
pub use charset::{
    Ascii,
    BomDetectStatus,
    Charset,
    CharsetRegistrationError,
    CharsetRegistrationErrorKind,
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
    Latin1Codec,
};
pub use codec::{
    Utf8Codec,
    Utf16ByteCodec,
    Utf16U16Codec,
    Utf32ByteCodec,
    Utf32U32Codec,
};
pub use convert::{
    CharsetConvertError,
    CharsetConverter,
    MalformedAction,
    UnmappableAction,
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
pub use util::{
    normalize_label_loose,
    normalize_label_whatwg,
};
