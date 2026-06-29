// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use crate::UnicodeBom;

/// Result of incremental Unicode BOM detection.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BomDetectStatus {
    /// The supplied bytes are still a possible BOM prefix.
    Pending,

    /// A supported Unicode BOM was detected.
    Match(UnicodeBom),

    /// No supported Unicode BOM can match the supplied bytes.
    None,
}
