// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_codec_text::BomDetectStatus;
use qubit_codec_text::UnicodeBom;

#[test]
fn test_bom_detect_status_describes_incremental_detection_state() {
    assert_eq!(
        BomDetectStatus::Pending,
        UnicodeBom::detect_progress(&[0xef, 0xbb], false)
    );
    assert_eq!(
        BomDetectStatus::Match(UnicodeBom::Utf8),
        UnicodeBom::detect_progress(&[0xef, 0xbb, 0xbf], false),
    );
    assert_eq!(BomDetectStatus::None, UnicodeBom::detect_progress(&[0x12], false));
}
