// SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// CLI Specification for clap
use clap::{
    Parser,
    ValueEnum,
};

/// An enum representing the type of BOFH excuse to generate.
///
/// This [`enum`](ExcuseType) is used to specify the type of BOFH excuse to
/// generate. The two variants of this enum are [`Classic`](ExcuseType::Classic)
/// and [`Modern`](ExcuseType::Modern).
///
/// The [`Classic`](ExcuseType::Classic) variant represents a 90s style BOFH
/// excuse, while the [`Modern`](ExcuseType::Modern) variant represents a more
/// modern BOFH excuse.
///
/// The ultimate purpose of this `enum` is to constrain the available valid
/// options for the [`Cli`] struct, which is used to parse the command line
/// arguments.
///
/// # Examples
///
/// ```
/// use gh_bofh::cli::ExcuseType;
///
/// let classic = ExcuseType::Classic;
/// let modern = ExcuseType::Modern;
/// ```
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum ExcuseType {
    // The classic 90s style BOFH excuse
    Classic,
    // A modern BOFH excuse for the 21st century
    Modern,
}

/// The CLI struct for the `gh-bofh` binary.
///
/// This struct is used to parse the command line arguments for the `gh-bofh`
/// binary. The struct is used by the [`clap`](https://docs.rs/clap) library to generate the command line interface
/// for the binary.
///
/// The struct defines the valid and available command line arguments for the
/// `gh-bofh` binary.
///
/// ## Command Line Arguments
///
/// The `Cli` struct defines the following command line arguments:
///
/// * `-t/--type`: The type of excuse to generate. The default is classic
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Generates a random BOFH excuse. The excuse type can be specified with the \
                  -t/--type flag. The default is classic, which generates a 90s style BOFH \
                  excuse. You can also specify modern, which generates a more modern BOFH excuse."
)]
pub struct Cli {
    /// The type of excuse to generate: classic or modern
    ///
    /// The default is classic, which generates a 90s style BOFH excuse. You can
    /// also specify modern, which generates a more modern BOFH excuse.
    #[clap(
        short = 't',
        long = "type",
        default_value = "classic",
        env = "EXCUSE_TYPE",
        value_name = "TYPE"
    )]
    #[arg(value_enum, group = "type")]
    pub excuse_type: ExcuseType,

    /// Generate a classic BOFH excuse
    ///
    /// Generates a 90s style BOFH excuse.
    #[arg(short = 'c', long = "classic", group = "type")]
    pub classic: bool,

    /// Generate a modern BOFH excuse
    ///
    /// Generates a more modern BOFH excuse.
    #[arg(short = 'm', long = "modern", group = "type")]
    pub modern: bool,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use sealed_test::prelude::*;

    use super::*;

    #[test]
    fn test_default_classic_excuse() {
        let args = ["test"];
        let parsed = Cli::try_parse_from(args);
        assert!(parsed.is_ok());
        let opts = parsed.unwrap();
        assert_eq!(opts.excuse_type, ExcuseType::Classic);
    }

    #[test]
    fn test_specify_classic_excuse() {
        let args = ["test", "--type", "classic"];
        let parsed = Cli::try_parse_from(args);
        assert!(parsed.is_ok());
        let opts = parsed.unwrap();
        assert_eq!(opts.excuse_type, ExcuseType::Classic);
    }

    #[test]
    fn test_specify_modern_excuse() {
        let args = ["test", "--type", "modern"];
        let parsed = Cli::try_parse_from(args);
        assert!(parsed.is_ok());
        let opts = parsed.unwrap();
        assert_eq!(opts.excuse_type, ExcuseType::Modern);
    }

    #[test]
    fn test_short_flag_classic() {
        let args = ["test", "-c"];
        let parsed = Cli::try_parse_from(args);
        assert!(parsed.is_ok());
        let opts = parsed.unwrap();
        assert!(opts.classic);
        assert!(!opts.modern);
    }

    #[test]
    fn test_short_flag_modern() {
        let args = ["test", "-m"];
        let parsed = Cli::try_parse_from(args);
        assert!(parsed.is_ok());
        let opts = parsed.unwrap();
        assert!(!opts.classic);
        assert!(opts.modern);
    }

    #[sealed_test(env = [ ("EXCUSE_TYPE", "classic") ])]
    fn test_env_var_classic() {
        let parsed = Cli::try_parse_from(["test"]);
        assert!(parsed.is_ok());
        let opts = parsed.unwrap();
        assert_eq!(opts.excuse_type, ExcuseType::Classic);
        assert!(!opts.classic);
        assert!(!opts.modern);
    }

    #[sealed_test(env = [ ("EXCUSE_TYPE", "modern") ])]
    fn test_env_var_modern() {
        let parsed = Cli::try_parse_from(["test"]);
        assert!(parsed.is_ok());
        let opts = parsed.unwrap();
        assert_eq!(opts.excuse_type, ExcuseType::Modern);
        assert!(!opts.classic);
        assert!(!opts.modern);
    }
}
