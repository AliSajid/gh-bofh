<!--
SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

<!-- .github/copilot-instructions.md - guidance for AI coding agents -->

# Quick guidance for AI agents working on gh-bofh

Be concise. This file highlights the concrete, discoverable patterns and commands
that make an AI agent immediately productive in this repository.

## Big picture

- Two main components:
  - `gh-bofh` (binary) at `src/gh_bofh/main.rs` — CLI glue using `clap`.
  - `gh_bofh_lib` (library) at `src/gh_bofh_lib/lib.rs` and `src/gh_bofh_lib/excuses.rs` — core logic and the static excuse lists.
- Data flow: CLI parses flags/env (`EXCUSE_TYPE` / `-t|--type` / `-c|--classic` / `-m|--modern`) and calls `random_classic()` or `random_modern()` from the library (see `main.rs::process_choice`).

## Key files to reference

- `Cargo.toml` — MSRV (1.85.1), dependencies, dev-deps and crate layout (`lib = gh_bofh_lib`, binary `gh-bofh`).
- `src/gh_bofh/cli.rs` — `Cli` struct and `ExcuseType` enum; tests show how CLI parsing is validated.
- `src/gh_bofh/main.rs` — entrypoint and `process_choice` logic.
- `src/gh_bofh_lib/excuses.rs` — the large static arrays `CLASSIC` and `MODERN` (where new excuses are added).
- `scripts/` — helper scripts; `generate_about_*.sh` requires `cargo-about` and `dos2unix`.
- `benches/`, `tests/` — benchmarks and integration/unit tests.
- `ARCHITECTURE.md`, `README.md`, `CONTRIBUTING.md`, `SECURITY*` — project rationale and policies (useful for PR/issue context).

## Build / test / lint workflow (explicit)

- Local quick build: `cargo build`
- Run tests: `cargo test` (unit + integration + property tests). Tests use `sealed_test`, `assert_cmd`, and `proptest` — preserve environment expectations and don't skip property tests.
- Run tests with assertions active: `cargo test` (debug_assert! macros are active in test builds; overflow-checks enabled)
- Benchmarks: `cargo bench --bench excuses_benchmark` (requires nightly or benchmark harness depending on toolchain).
- Format check (CI): `cargo +nightly fmt --all -- --check` (CI runs `+nightly fmt` — match when reproducing CI).
- Lint (CI): `cargo clippy -- -D warnings` — clippy is treated as errors in CI.
- Install locally (to test `gh` extension behavior): `cargo install --path .` then run `gh bofh-rs` or `./target/debug/gh-bofh`.

## CI specifics to mirror locally

- CI matrix tests `stable`, `beta`, `nightly` and an explicit MSRV `1.85.1` (see `.github/workflows/ci.yaml`).
- CI runs `cargo build`, `cargo test`, formatting check with nightly, and `cargo clippy` with `-D warnings`.

## Project-specific conventions and patterns

- No unsafe: `Cargo.toml` sets `unsafe_code = "forbid"` (do not introduce `unsafe`).
- Documentation & doc-tests are enforced (see `lints.rustdoc` entries) — keep examples in doc comments accurate.
- The library exposes `CLASSIC` and `MODERN` as `pub const` arrays; prefer editing `src/gh_bofh_lib/excuses.rs` to add/remove excuses rather than modifying runtime logic.
- Random selection uses `rand::rng()` and `IndexedRandom::choose` (see `random_classic` / `random_modern`) — preserve the RNG usage pattern when adding features.
- Tests sometimes set environment variables via `sealed_test` (see `cli.rs` tests). When writing tests, replicate that pattern rather than manipulating global state.
- **Assertions and testing patterns**:
  - Use `debug_assert!` for invariant checks in library code (active in tests, compiled out in release)
  - Add validation tests for new static data (follow patterns in `lib.rs::tests` module)
  - Consider property tests with `proptest` for functions that accept varied inputs
  - All new library functions should include assertions for preconditions/postconditions
  - Test and dev profiles have `overflow-checks = true` — arithmetic bugs will panic during tests

## Common edit tasks — examples

- Add a new modern excuse: edit `src/gh_bofh_lib/excuses.rs`, append to `MODERN`, run `cargo test` and `cargo +nightly fmt --all`.
- Change CLI option or help text: update `src/gh_bofh/cli.rs` (uses `clap::Parser/ValueEnum`) and add/adjust tests in the same file.
- Fix a failing test: run `cargo test -- --nocapture` locally to see output; tests may depend on deterministic non-empty arrays (don't change counts silently).

## External tools & integration points

- `gh` CLI: project is a `gh` extension — releases are packaged as binaries (check `scripts/build.sh` and release workflow in `.github/workflows/release.yaml`).
- `cargo-about`: used by `scripts/generate_about_*.sh` to create license reports.

## Safety rails for AI edits

- Respect `unsafe_code = "forbid"` and doc lint rules.
- Keep `MSRV = 1.85.1` compatibility in mind; avoid using newer stable-only APIs without mentioning alternative implementations.
- Formatting and clippy are enforced in CI; prefer `cargo +nightly fmt` and run `cargo clippy -- -D warnings` locally before pushing.

## Where to look for context in PRs or issues

- For design rationale, read `ARCHITECTURE.md` and `SECURITY_ASSURANCE.md`.
- For contribution norms and how to test locally, read `CONTRIBUTING.md` and `README.md`.

If anything here is unclear or you'd like more examples (test templates, a quick sample PR edit, or a checklist for adding excuses), tell me which section to expand and I'll iterate.
