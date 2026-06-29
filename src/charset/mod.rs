// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0.
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
mod ascii;
mod ascii_folding;
mod bom_detect_status;
#[allow(clippy::module_inception)]
mod charset;
mod charset_registration_error;
mod charset_registration_error_kind;
mod latin1;
mod unicode;
mod unicode_bom;
mod utf16;
mod utf32;
mod utf8;

pub use ascii::Ascii;
pub use bom_detect_status::BomDetectStatus;
pub use charset::Charset;
pub use charset_registration_error::CharsetRegistrationError;
pub use charset_registration_error_kind::CharsetRegistrationErrorKind;
pub use latin1::Latin1;
pub use unicode::Unicode;
pub use unicode_bom::UnicodeBom;
pub use utf8::Utf8;
pub use utf16::Utf16;
pub use utf32::Utf32;
