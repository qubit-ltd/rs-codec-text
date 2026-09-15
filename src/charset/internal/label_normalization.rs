// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
/// Label normalization flavor used by charset lookup.
#[derive(Clone, Copy)]
pub(crate) enum LabelNormalization {
    /// Loose matching trims ASCII whitespace, folds ASCII case, and ignores
    /// `-` / `_` separators.
    Loose,
    /// WHATWG-style preprocessing trims ASCII whitespace and folds ASCII case.
    Whatwg,
}
