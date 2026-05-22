/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0.
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Generic, charset-independent coding traits and status types.

mod coder;
mod coder_progress;
mod coder_status;

pub use coder::Coder;
pub use coder_progress::CoderProgress;
pub use coder_status::CoderStatus;
