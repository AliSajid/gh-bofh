// SPDX-FileCopyrightText: 2023 - 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Integration tests for the `gh-bofh_lib` crate focusing on classic excuses.
//!
//! These tests ensure that the `random_classic` function from the `gh_bofh_lib`
//! crate works as expected. The tests cover the following scenarios:
//!
//! - Generating multiple classic excuses and ensuring they are not empty.
//! - Ensuring that the generated excuses do not contain a specific error
//!   message.
//! - Verifying that the generated excuses are unique up to a certain limit.
//! - Checking that all generated excuses are part of the predefined `CLASSIC`
//!   set.
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use gh_bofh_lib::{
        random_classic,
        CLASSIC,
    };

    #[test]
    fn test_common_excuses() {
        let mut excuses = HashSet::new();
        for _ in 0..500 {
            let excuse = random_classic();
            assert!(!excuse.is_empty());
            assert_ne!(excuse, "No excuse found, try again later");
            excuses.insert(excuse);
        }

        assert!(excuses.len() > 1);
        assert!(excuses.len() <= 467);

        assert!(excuses.iter().all(|excuse| CLASSIC.contains(excuse)));
    }
}
