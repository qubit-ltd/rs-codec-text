/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0.
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
mod text_decode_error;
mod text_decode_error_kind;
mod text_encode_error;
mod text_encode_error_kind;

pub use text_decode_error::{
    TextDecodeError,
    TextDecodeResult,
};
pub use text_decode_error_kind::TextDecodeErrorKind;
pub use text_encode_error::{
    TextEncodeError,
    TextEncodeResult,
};
pub use text_encode_error_kind::TextEncodeErrorKind;
