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
pub use charset::Ascii;
pub use charset::BomDetectStatus;
pub use charset::Charset;
pub use charset::CharsetRegistrationError;
pub use charset::CharsetRegistrationErrorKind;
pub use charset::Latin1;
pub use charset::Unicode;
pub use charset::UnicodeBom;
pub use charset::Utf8;
pub use charset::Utf16;
pub use charset::Utf32;
pub use codec::AsciiCodec;
pub use codec::CharsetCodec;
pub use codec::Latin1Codec;
pub use codec::Utf8Codec;
pub use codec::Utf16ByteCodec;
pub use codec::Utf16U16Codec;
pub use codec::Utf32ByteCodec;
pub use codec::Utf32U32Codec;
pub use convert::CharsetConvertError;
pub use convert::CharsetConverter;
pub use convert::MalformedAction;
pub use convert::UnmappableAction;
pub(crate) use decode::CharsetDecodeHooks;
pub use decode::CharsetDecodePolicy;
pub use decode::CharsetDecoder;
pub(crate) use encode::CharsetEncodeHooks;
pub use encode::CharsetEncodePolicy;
pub use encode::CharsetEncoder;
pub use error::CharsetDecodeError;
pub use error::CharsetDecodeErrorKind;
pub use error::CharsetDecodeResult;
pub use error::CharsetEncodeError;
pub use error::CharsetEncodeErrorKind;
pub use error::CharsetEncodeResult;
pub use util::normalize_label_loose;
pub use util::normalize_label_whatwg;
