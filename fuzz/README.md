<!--
SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Fuzzing

This directory contains fuzz targets for `gh-bofh` using [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) and [libFuzzer](https://llvm.org/docs/LibFuzzer.html).

## What is Fuzzing?

Fuzzing is an automated software testing technique that involves providing invalid, unexpected, or random data as inputs to a program. The program is then monitored for exceptions such as crashes, failing built-in code assertions, or potential memory leaks.

## Prerequisites

Install cargo-fuzz:

```bash
cargo install cargo-fuzz
```

Note: Fuzzing requires a nightly Rust toolchain.

## Running Fuzz Tests

### Run a specific fuzz target

```bash
cd fuzz
cargo +nightly fuzz run fuzz_random_classic
```

```bash
cd fuzz
cargo +nightly fuzz run fuzz_random_modern
```

### Run with time limit

```bash
cd fuzz
cargo +nightly fuzz run fuzz_random_classic -- -max_total_time=60
```

### List all fuzz targets

```bash
cd fuzz
cargo +nightly fuzz list
```

## Fuzz Targets

### `fuzz_random_classic`

Fuzzes the `random_classic()` function to ensure it:

- Never panics
- Always returns a valid non-empty string
- Maintains memory safety
- Returns valid UTF-8

### `fuzz_random_modern`

Fuzzes the `random_modern()` function to ensure it:

- Never panics
- Always returns a valid non-empty string
- Maintains memory safety
- Returns valid UTF-8

## Continuous Integration

Fuzzing runs automatically in GitHub Actions:

- On every push to main/develop branches
- On every pull request to main
- Weekly on Sundays at 2:00 AM UTC
- Can be manually triggered via workflow dispatch

Each fuzz target runs for 60 seconds in CI to catch potential issues early.

## Analyzing Crashes

If a fuzz target discovers a crash:

1. The crashing input will be saved in `fuzz/artifacts/<target_name>/`
1. You can reproduce the crash with:

```bash
cd fuzz
cargo +nightly fuzz run <target_name> fuzz/artifacts/<target_name>/<crash_file>
```

1. Debug the crash:

```bash
cd fuzz
cargo +nightly fuzz run <target_name> --debug-assertions fuzz/artifacts/<target_name>/<crash_file>
```

## Best Practices

- Run fuzz tests locally before submitting PRs
- Let fuzz tests run for extended periods (hours/days) for thorough testing
- Report any crashes found as security issues
- Keep fuzz targets simple and focused on specific functionality

## Resources

- [cargo-fuzz documentation](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [libFuzzer documentation](https://llvm.org/docs/LibFuzzer.html)
- [Rust Fuzz Book](https://rust-fuzz.github.io/book/)
