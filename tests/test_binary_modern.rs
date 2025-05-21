// SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Integration tests for the `gh-bofh` binary with the `modern` type.
//!
//! These tests ensure that the `gh-bofh` binary works as expected when executed
//! with different arguments and environment variables related to the `modern`
//! type. The tests cover the following scenarios:
//!
//! - Running the binary with no arguments and verifying it does not produce
//!   `modern` output.
//! - Running the binary with the `--type modern` argument.
//! - Running the binary with the `-m` short argument.
//! - Running the binary with the `EXCUSE_TYPE` environment variable set to
//!   `modern`.

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use assert_cmd::Command;
    use gh_bofh_lib::MODERN;

    #[test]
    fn test_binary_output_modern() {
        let cmd = Command::cargo_bin("gh-bofh").unwrap().output().unwrap();
        assert!(cmd.status.success());
        assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
        assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
        assert!(!MODERN.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
    }

    #[test]
    fn test_binary_output_flag_modern() {
        let cmd = Command::cargo_bin("gh-bofh")
            .unwrap()
            .args(["--type", "modern"])
            .output()
            .unwrap();
        assert!(cmd.status.success());
        assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
        assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
        assert!(MODERN.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
    }

    #[test]
    fn test_binary_output_flag_short_modern() {
        let cmd = Command::cargo_bin("gh-bofh")
            .unwrap()
            .arg("-m")
            .output()
            .unwrap();
        assert!(cmd.status.success());
        assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
        assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
        assert!(MODERN.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
    }

    #[test]
    fn test_binary_output_env_var_modern() {
        let cmd = Command::cargo_bin("gh-bofh")
            .unwrap()
            .env("EXCUSE_TYPE", "modern")
            .output()
            .unwrap();
        assert!(cmd.status.success());
        assert!(!String::from_utf8_lossy(&cmd.stdout).is_empty());
        assert!(String::from_utf8_lossy(&cmd.stderr).is_empty());
        assert!(MODERN.contains(&String::from_utf8_lossy(&cmd.stdout).trim()));
    }
}
