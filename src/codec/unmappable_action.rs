// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
/// Policy used when a character cannot be represented by the target charset.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum UnmappableAction {
    /// Return the unmappable-character error to the caller.
    Report,

    /// Skip the unmappable character and continue.
    Ignore,

    /// Encode the configured replacement character instead.
    #[default]
    Replace,
}
