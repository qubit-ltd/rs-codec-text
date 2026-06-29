// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0.
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
mod charset_decode_error;
mod charset_decode_error_kind;
mod charset_encode_error;
mod charset_encode_error_kind;

pub(crate) use charset_decode_error::{
    CharsetCodecDecodeResult, map_charset_decode_error, map_charset_decode_failure,
};
pub use charset_decode_error::{CharsetDecodeError, CharsetDecodeResult};
pub use charset_decode_error_kind::CharsetDecodeErrorKind;
pub(crate) use charset_encode_error::map_charset_encode_error;
pub use charset_encode_error::{CharsetEncodeError, CharsetEncodeResult};
pub use charset_encode_error_kind::CharsetEncodeErrorKind;
