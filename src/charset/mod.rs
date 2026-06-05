/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0.
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
mod ascii;
mod ascii_folding;
#[allow(clippy::module_inception)]
mod charset;
mod unicode;
mod unicode_bom;
mod utf16;
mod utf32;
mod utf8;

pub use ascii::Ascii;
pub use charset::Charset;
pub use unicode::Unicode;
pub use unicode_bom::UnicodeBom;
pub use utf8::Utf8;
pub use utf16::Utf16;
pub use utf32::Utf32;
