// SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Integration tests for the `gh-bofh` library focusing on modern excuses.
//!
//! These tests ensure that the `random_modern` function from the `gh_bofh_lib`
//! crate works as expected. The tests cover the following scenarios:
//!
//! - Generating multiple modern excuses and ensuring they are not empty.
//! - Ensuring that the generated excuses do not contain a specific error
//!   message.
//! - Verifying that the generated excuses are unique up to a certain limit.
//! - Checking that all generated excuses are part of the predefined `MODERN`
//!   set.

use std::collections::HashSet;

use gh_bofh_lib::{
    MODERN,
    random_modern,
};

#[test]
fn test_modern_excuses() {
    let mut excuses = HashSet::new();
    for _ in 0..200 {
        let excuse = random_modern();
        assert!(!excuse.is_empty());
        assert_ne!(
            excuse,
            "Excuse engine not initialized. Please try again later."
        );
        excuses.insert(excuse);
    }

    assert!(excuses.len() > 1);
    assert!(excuses.len() <= 105);

    assert!(excuses.iter().all(|excuse| MODERN.contains(excuse)));
}
