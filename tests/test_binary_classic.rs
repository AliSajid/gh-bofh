// SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Integration tests for the `gh-bofh` binary.
//!
//! These tests ensure that the `gh-bofh` binary works as expected when executed
//! with different arguments and environment variables. The tests cover the
//! following scenarios:
//!
//! - Running the binary with no arguments.
//! - Running the binary with the `--type classic` argument.
//! - Running the binary with the `-c` short argument.
//! - Running the binary with the `EXCUSE_TYPE` environment variable set to
//!   `classic`.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use assert_cmd::Command;
    use gh_bofh_lib::CLASSIC;

    #[test]
    fn test_binary_plain_default() {
        let mut cmd = Command::cargo_bin("gh-bofh").unwrap();
        cmd.assert().success();
    }

    #[test]
    fn test_binary_output_classic() {
        let cmd = Command::cargo_bin("gh-bofh").unwrap().output().unwrap();
        assert!(cmd.status.success());
        assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
        assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
        assert!(CLASSIC.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
    }

    #[test]
    fn test_binary_output_flag_classic() {
        let cmd = Command::cargo_bin("gh-bofh")
            .unwrap()
            .args(["--type", "classic"])
            .output()
            .unwrap();
        assert!(cmd.status.success());
        assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
        assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
        assert!(CLASSIC.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
    }

    #[test]
    fn test_binary_output_flag_short_classic() {
        let cmd = Command::cargo_bin("gh-bofh")
            .unwrap()
            .arg("-c")
            .output()
            .unwrap();
        assert!(cmd.status.success());
        assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
        assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
        assert!(CLASSIC.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
    }

    #[test]
    fn test_binary_output_env_var_classic() {
        let cmd = Command::cargo_bin("gh-bofh")
            .unwrap()
            .env("EXCUSE_TYPE", "classic")
            .output()
            .unwrap();
        assert!(cmd.status.success());
        assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
        assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
        assert!(CLASSIC.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
    }
}
