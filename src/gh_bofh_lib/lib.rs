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
}
