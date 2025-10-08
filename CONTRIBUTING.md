<!--
SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

<!-- omit in toc -->

# Contributing to gh-bofh

Thank you for considering a contribution to gh-bofh — contributions of all kinds are welcome. This file explains how to report bugs, suggest enhancements, contribute code (for example, adding new excuses), and follow our coding and commit conventions to make reviews faster.

If you don't have time to contribute code, you can still help by:

- starring the repository
- opening issues with clear reproduction steps
- spreading the word

## Table of Contents

- [Contributing to gh-bofh](#contributing-to-gh-bofh)
  - [Table of Contents](#table-of-contents)
  - [I have a question](#i-have-a-question)
  - [I want to contribute](#i-want-to-contribute)
    - [Legal](#legal)
    - [Reporting bugs](#reporting-bugs)
    - [Suggesting enhancements](#suggesting-enhancements)
    - [Adding or editing excuses](#adding-or-editing-excuses)
    - [Your first code contribution](#your-first-code-contribution)
  - [Improving the documentation](#improving-the-documentation)
  - [Style and tooling](#style-and-tooling)
    - [Commit messages](#commit-messages)
  - [Join the project team](#join-the-project-team)
  - [Attribution](#attribution)

## I have a question

First, check the published docs at [https://docs.rs/gh-bofh](https://docs.rs/gh-bofh) and search existing issues on GitHub: [https://github.com/AliSajid/gh-bofh/issues](https://github.com/AliSajid/gh-bofh/issues). If nothing answers your question, open a new issue describing what you tried, your environment, and any relevant output.

## I want to contribute

### Legal

By contributing you confirm that you have the right to submit the work and that you license it under the project's dual license (MIT OR Apache-2.0). See `LICENSE-MIT` and `LICENSE-APACHE`.

### Reporting bugs

Before filing a bug report, please:

1. Update to the latest release or the `main` branch.
2. Search existing issues for duplicates.
3. Gather diagnostic data:
   - OS and architecture (e.g., macos aarch64)
   - Rust and Cargo versions (`rustup show` / `rustc --version`)
   - Reproduction steps and sample input
   - Full output and backtrace (set `RUST_BACKTRACE=1`)

How to file:


- Open an issue at [https://github.com/AliSajid/gh-bofh/issues/new](https://github.com/AliSajid/gh-bofh/issues/new).
- Provide the diagnostics above and any minimal reproduction case.

Once it's filed:

### Suggesting enhancements

- Search and comment on existing issues first.
- For non-trivial changes (new flags, behavior changes), open an issue describing the motivation and proposed design before filing a PR.

### Adding or editing excuses

Excuses live in the library crate. To add or modify excuses:

1. Edit `src/gh_bofh_lib/excuses.rs` and add your entry following existing patterns.
2. Add or update unit tests in `tests/` or in the library's test modules to cover the new entries or selection logic.
3. Run the test suite and linters locally (see tooling below).

When adding new excuses prefer concise, non-offensive wording and follow the established tone used by the repository. The maintainers will review and may request edits.

### Your first code contribution

Getting started locally:

```bash
# install the Rust toolchain (recommended to use rustup)
rustup install 1.74.1
rustup default 1.74.1

# build and run tests
cargo build --release
cargo test

# format and lint
cargo +nightly fmt --all
cargo clippy -- -D warnings
```

Open a PR from a fork, include a clear description, link to any related issues, and add tests for your change.

## Improving the documentation

- Documentation lives in `README.md` and the docs comments in the crates. If you improve docs, please run `cargo +nightly fmt` and ensure existing examples continue to build.

## Style and tooling

- Code style: `rustfmt` via `cargo fmt`.
- Linting: `cargo clippy` (`-D warnings` in CI).
- Tests: `cargo test`.
- Dependency audits: run `cargo audit` and attach output to PRs that modify dependencies.

Suggested pre-PR checklist:

1. Run `cargo test` and ensure all tests pass.
2. Run `cargo +nightly fmt --all` and `cargo clippy -- -D warnings`.
3. If you changed dependencies run `cargo audit --file Cargo.lock` and include the report.

### Commit messages

We use Conventional Commits. Examples:

- feat: add a modern excuse set
- fix: correct CLI parsing for --type

Proper commit messages make review and changelog generation easier.

## Join the project team

If you want to join as a maintainer, open an issue explaining your interest and previous contributions. The team will follow up.

## Attribution

This guide was adapted for the gh-bofh project. Thank you for contributing!
