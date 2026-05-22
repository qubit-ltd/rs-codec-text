/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0.
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Generic coding traits and helpers shared beyond text codecs.
//!
//! These abstractions are intentionally minimal and can be moved to a dedicated I/O
//! utility crate later.

mod byte_order;
pub mod coding;

pub use byte_order::ByteOrder;
pub use coding::{
    Coder,
    CoderProgress,
    CoderStatus,
};
