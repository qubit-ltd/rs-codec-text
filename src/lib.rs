// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Qubit Text Codec
//!
//! Buffer-oriented Unicode and charset conversion for ASCII, ISO-8859-1,
//! UTF-8, UTF-16, and UTF-32.
//!
//! Use [`CharsetCodec`] implementations for strict single-value encoding and
//! decoding. Use [`CharsetDecoder`], [`CharsetEncoder`], or
//! [`CharsetConverter`] when a buffer-oriented stream needs malformed-input or
//! unmappable-character policies and progress reporting. These wrappers default
//! to `Replace`; configure a `Report` policy explicitly when replacement is
//! unacceptable. I/O adapters above this crate own stream buffering, EOF
//! handling, line endings, and `std::io::Error` mapping.
//!
//! The complete-conversion helpers use [`Transcoder`](qubit_codec::Transcoder)
//! to manage reset and finish. Streaming callers manage that lifecycle
//! themselves, retain unconsumed input, handle EOF, and finish only after EOF
//! input has been processed.
//!
//! ```
//! use qubit_codec::Transcoder;
//! use qubit_codec_text::{CharsetConverter, Utf16U16Codec, Utf8Codec};
//!
//! let input = b"A\xe4\xb8\xad";
//! let mut converter = CharsetConverter::from_codecs(Utf8Codec, Utf16U16Codec);
//! let capacity = converter.max_total_output_len(input.len()).expect("capacity bound");
//! let mut output = vec![0_u16; capacity];
//! let written = converter.transcode_complete_into(input, &mut output).expect("conversion");
//! assert_eq!([0x0041, 0x4e2d], output[..written]);
//! ```
//!
//! See the [English user guide](https://github.com/qubit-ltd/rs-codec-text/blob/main/doc/user_guide.md)
//! and [Chinese user guide](https://github.com/qubit-ltd/rs-codec-text/blob/main/doc/user_guide.zh_CN.md)
//! for complete workflows and streaming examples.

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

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
#[doc = include_str!("../README.zh_CN.md")]
mod readme_examples {}
