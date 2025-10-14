// SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! This crate provides functionality to generate random BOFH (Bastard Operator
//! From Hell) excuses.
//!
//! The purpose of this module is to allow an interface to generate random BOFH
//! excuses. There are two _flavors_ of excuses: [classic](excuses::CLASSIC) and
//! [modern](excuses::MODERN). Both flavors are available as static arrays of
//! string slices.
//!
//! ## Classic excuses
//! The classic excuses are inspired by the original BOFH excuse list from the
//! 90s. They revolve around the problems around physcial hardware, network
//! infrastructure and in-person enterprise support. There are a total of 467
//! classic excuses in the list.
//!
//! You can see the list of classic excuses by importing the `CLASSIC` constant
//! from [`gh_bofh_lib`](crate).
//!
//! You can also generate a random classic excuse by calling the
//! [`random_classic`] function.
//!
//! ### Examples
//! ```
//! use gh_bofh_lib::random_classic;
//! let excuse = random_classic();
//! println!("{}", excuse);
//! ```
//!
//! ## Modern excuses
//!
//! The modern excuses are inspired by the modern problems faced by IT
//! professionals. They revolve around cloud infrastructure, software
//! development, and remote support. There are a total of 105 modern excuses in
//! the list.
//!
//! You can see the list of modern excuses by importing the `MODERN` constant
//! from [`gh_bofh_lib`](crate).
//!
//! You can also generate a random modern excuse by calling the
//! [`random_modern`] function.
//!
//! ### Examples
//!
//! ```
//! use gh_bofh_lib::random_modern;
//! let excuse = random_modern();
//! println!("{}", excuse);
//! ```
//!
//! ## Other Examples
//!
//! ```
//! use gh_bofh_lib::{
//!     random_classic,
//!     random_modern,
//! };
//!
//! let classic_excuse = random_classic();
//! println!("Classic excuse: {}", classic_excuse);
//!
//! let modern_excuse = random_modern();
//! println!("Modern excuse: {}", modern_excuse);
//! ```

pub mod excuses;

pub use excuses::{
    CLASSIC,
    MODERN,
};
use rand::prelude::IndexedRandom;

type ClassicExcuse = &'static str;
type ModernExcuse = &'static str;

/// Returns a random classic excuse
///
/// This function returns a random BOFH excuse from the classic list.
///
/// # Examples
///
/// ```
/// use gh_bofh_lib::random_classic;
///
/// let excuse = random_classic();
/// println!("{}", excuse);
/// ```
#[must_use]
pub fn random_classic() -> ClassicExcuse {
    // Invariant: CLASSIC array is non-empty (validated by test suite)
    // Invariant: All excuses must be valid UTF-8 (compile-time guarantee, but
    // verify in debug)
    #[cfg(debug_assertions)]
    {
        debug_assert!(
            CLASSIC
                .iter()
                .all(|s| std::str::from_utf8(s.as_bytes()).is_ok()),
            "CLASSIC array contains invalid UTF-8"
        );
    }

    CLASSIC
        .choose(&mut rand::rng())
        .unwrap_or(&"No excuse found, try again later")
}

/// Returns a random modern excuse
///
/// This function returns a random BOFH excuse from the modern list.
///
/// # Examples
///
/// ```
/// use gh_bofh_lib::random_modern;
///
/// let excuse = random_modern();
///
/// println!("{}", excuse);
/// ```
#[must_use]
pub fn random_modern() -> ModernExcuse {
    // Invariant: MODERN array is non-empty (validated by test suite)
    // Invariant: All excuses must be valid UTF-8 (compile-time guarantee, but
    // verify in debug)
    #[cfg(debug_assertions)]
    {
        debug_assert!(
            MODERN
                .iter()
                .all(|s| std::str::from_utf8(s.as_bytes()).is_ok()),
            "MODERN array contains invalid UTF-8"
        );
    }

    MODERN
        .choose(&mut rand::rng())
        .unwrap_or(&"Excuse engine not initialized. Please try again later.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_classic() {
        let excuse: ClassicExcuse = random_classic();
        assert_ne!(excuse, "No excuse found, try again later");
    }

    #[test]
    fn test_random_modern() {
        let excuse: ModernExcuse = random_modern();
        assert_ne!(
            excuse,
            "Excuse engine not initialized. Please try again later."
        );
    }

    // Strategy 2: Array validation tests
    #[test]
    #[allow(clippy::const_is_empty)]
    fn classic_excuses_are_nonempty() {
        assert!(
            !CLASSIC.is_empty(),
            "CLASSIC array must contain at least one excuse"
        );
    }

    #[test]
    #[allow(clippy::const_is_empty)]
    fn modern_excuses_are_nonempty() {
        assert!(
            !MODERN.is_empty(),
            "MODERN array must contain at least one excuse"
        );
    }

    #[test]
    fn all_classic_excuses_are_nonempty_strings() {
        for (idx, excuse) in CLASSIC.iter().enumerate() {
            assert!(!excuse.is_empty(), "CLASSIC[{idx}] is an empty string");
            assert!(
                excuse.len() < 1000,
                "CLASSIC[{}] is suspiciously long ({} bytes)",
                idx,
                excuse.len()
            );
        }
    }

    #[test]
    fn all_modern_excuses_are_nonempty_strings() {
        for (idx, excuse) in MODERN.iter().enumerate() {
            assert!(!excuse.is_empty(), "MODERN[{idx}] is an empty string");
            assert!(
                excuse.len() < 1000,
                "MODERN[{}] is suspiciously long ({} bytes)",
                idx,
                excuse.len()
            );
        }
    }

    #[test]
    fn random_classic_never_returns_fallback() {
        // Run multiple times to ensure no RNG edge case triggers fallback
        for _ in 0..100 {
            let excuse = random_classic();
            assert_ne!(
                excuse, "No excuse found, try again later",
                "random_classic() returned fallback string (indicates empty array)"
            );
        }
    }

    #[test]
    fn random_modern_never_returns_fallback() {
        // Run multiple times to ensure no RNG edge case triggers fallback
        for _ in 0..100 {
            let excuse = random_modern();
            assert_ne!(
                excuse, "Excuse engine not initialized. Please try again later.",
                "random_modern() returned fallback string (indicates empty array)"
            );
        }
    }

    #[test]
    fn array_indexing_does_not_overflow() {
        // This would panic if choose() implementation had off-by-one bug
        for _ in 0..1000 {
            let _ = random_classic();
            let _ = random_modern();
        }
    }
}

#[cfg(test)]
mod proptests {
    // Strategy 5: Property-based testing with proptest
    use proptest::prelude::*;

    use super::*;

    proptest! {
        /// Property: random_classic always returns a non-empty string
        #[test]
        fn random_classic_always_nonempty(_seed in any::<u64>()) {
            let excuse = random_classic();
            prop_assert!(!excuse.is_empty(), "random_classic() returned empty string");
            prop_assert!(excuse.len() < 1000, "Excuse too long: {} bytes", excuse.len());
        }

        /// Property: random_modern always returns a non-empty string
        #[test]
        fn random_modern_always_nonempty(_seed in any::<u64>()) {
            let excuse = random_modern();
            prop_assert!(!excuse.is_empty(), "random_modern() returned empty string");
            prop_assert!(excuse.len() < 1000, "Excuse too long: {} bytes", excuse.len());
        }

        /// Property: returned excuses must exist in source arrays
        #[test]
        fn random_classic_returns_known_excuse(iterations in 0..100usize) {
            for _ in 0..iterations {
                let excuse = random_classic();
                prop_assert!(
                    CLASSIC.contains(&excuse),
                    "random_classic() returned unknown excuse: {}",
                    excuse
                );
            }
        }

        /// Property: returned excuses must exist in source arrays
        #[test]
        fn random_modern_returns_known_excuse(iterations in 0..100usize) {
            for _ in 0..iterations {
                let excuse = random_modern();
                prop_assert!(
                    MODERN.contains(&excuse),
                    "random_modern() returned unknown excuse: {}",
                    excuse
                );
            }
        }

        /// Property: excuses are valid UTF-8 strings
        #[test]
        fn random_classic_is_valid_utf8(_seed in any::<u64>()) {
            let excuse = random_classic();
            prop_assert!(
                std::str::from_utf8(excuse.as_bytes()).is_ok(),
                "random_classic() returned invalid UTF-8"
            );
        }

        /// Property: excuses are valid UTF-8 strings
        #[test]
        fn random_modern_is_valid_utf8(_seed in any::<u64>()) {
            let excuse = random_modern();
            prop_assert!(
                std::str::from_utf8(excuse.as_bytes()).is_ok(),
                "random_modern() returned invalid UTF-8"
            );
        }
    }
}
