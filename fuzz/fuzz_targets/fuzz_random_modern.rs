// SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|_data: &[u8]| {
    // Fuzz the random_modern function by calling it repeatedly
    // This tests for panics, memory safety issues, and other unexpected behavior
    let _ = gh_bofh_lib::random_modern();

    // The function should always return a valid string reference
    let excuse = gh_bofh_lib::random_modern();
    assert!(!excuse.is_empty(), "Excuse should not be empty");

    // Verify the excuse is valid UTF-8 (should always be true for &str)
    assert!(
        excuse
            .chars()
            .all(|c| !c.is_control() || c == '\n' || c == '\t')
    );
});
