// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Utility helpers shared by text codec metadata.

mod label_normalize;

pub use label_normalize::{normalize_label_loose, normalize_label_whatwg};
