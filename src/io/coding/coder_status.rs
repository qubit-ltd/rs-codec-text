/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
/// Reports why a [`crate::Coder`] stopped converting input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoderStatus {
    /// All currently supplied input was consumed.
    Complete,

    /// More input is needed to complete the next output value.
    NeedInput,

    /// More output capacity is needed before conversion can continue.
    NeedOutput,
}
