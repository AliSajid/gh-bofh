// SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! # gh-bofh
//!
//! This is a binary crate that utilizes the `gh_bofh_lib` library to generate
//! random BOFH excuses. This crate is designed to be used as a plugin/extension for the [`gh`](https://cli.github.com/) CLI tool, but
//! can also be used as a standalone binary.
//!
//! The crate provides a CLI interface to generate random BOFH excuses. The
//! excuse type can be specified in multiple ways:
//!
//! * Using the `-t/--type` flag: this flag accepts two values: `classic` and
//!   `modern`.
//! * Using the `EXCUSE_TYPE` environment variable: this variable accepts the
//!   same values as the `-t/--type` flag.
//! * Using the `-c/--classic` flag: this flag generates a classic BOFH excuse.
//! * Using the `-m/--modern` flag: this flag generates a modern BOFH excuse.
//!
//! In case no flag or environment variable is specified, the default is to
//! generate a `classic` BOFH excuse.

mod cli;

use clap::Parser;
use cli::{
    Cli,
    ExcuseType,
};
use gh_bofh_lib::{
    random_classic,
    random_modern,
};

fn main() {
    let arguments = Cli::parse();
    let chosen_type = process_choice(&arguments);
    match chosen_type {
        ExcuseType::Classic => println!("{}", random_classic()),
        ExcuseType::Modern => println!("{}", random_modern()),
    }
}

/// Processes the choice of excuse type based on the CLI arguments.
///
/// If the `modern` flag is set (for example by using the `-m` or `--modern`
/// flag), the function returns `EXCUSE_TYPE::Modern`. Otherwise it defers to
/// the `excuse_type` field in the `Cli` struct as parsed by [`clap`].
const fn process_choice(arguments: &Cli) -> ExcuseType {
    if arguments.classic {
        ExcuseType::Classic
    } else if arguments.modern {
        ExcuseType::Modern
    } else {
        arguments.excuse_type
    }
}
