<!--
SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Security Testing

This document describes the security testing practices used in `gh-bofh`, including fuzzing and property-based testing.

## Overview

`gh-bofh` employs multiple layers of automated security testing:

1. **Fuzz Testing** with cargo-fuzz and `libFuzzer`
2. **Property-Based Testing** with `proptest`
3. **Static Analysis** with `CodeQL`
4. **Dependency Auditing** with cargo-audit

## Fuzzing

### What is Fuzzing?

Fuzzing is an automated testing technique that provides random or malformed inputs to discover bugs, crashes, memory leaks, and security vulnerabilities. It's particularly effective at finding edge cases that manual testing might miss.

### Fuzz Testing Infrastructure

Located in the `fuzz/` directory, we use [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) with `libFuzzer`:

**Fuzz Targets:**

- `fuzz_random_classic` - Tests the `random_classic()` function
- `fuzz_random_modern` - Tests the `random_modern()` function

**What We Test:**

- No panics or crashes
- Memory safety
- Valid UTF-8 output
- Non-empty results
- Absence of control characters (except newlines/tabs)

### Running Fuzz Tests Locally

```bash
# Install cargo-fuzz (requires nightly Rust)
cargo install cargo-fuzz

# Run a specific fuzz target
cd fuzz
cargo +nightly fuzz run fuzz_random_classic

# Run with time limit (60 seconds)
cargo +nightly fuzz run fuzz_random_classic -- -max_total_time=60

# List all fuzz targets
cargo +nightly fuzz list
```

See `fuzz/README.md` for detailed instructions.

### Continuous Fuzzing

Fuzzing runs automatically in CI:

- On every push to main/develop
- On every pull request
- Weekly on Sundays
- Each target runs for 60 seconds

## Property-Based Testing

### What is Property-Based Testing?

Property-based testing verifies that functions satisfy specified properties across a wide range of inputs. Instead of writing individual test cases, you define properties that should always hold true.

### `Proptest` Integration

We use [`proptest`](https://github.com/proptest-rs/proptest) for property-based testing. Tests are located in `src/gh_bofh_lib/lib.rs` in the `proptests` module.

**Properties We Test:**

1. **Non-empty output** - Functions never return empty strings
2. **Reasonable length** - Output is less than 1000 bytes
3. **Array membership** - Results are always from the corresponding arrays
4. **Uniqueness** - Both excuse arrays contain unique entries
5. **Stability** - Results are consistent across multiple calls

### Running Property Tests

```bash
# Run all tests including property tests
cargo test

# Run only property tests
cargo test proptests

# Run with more iterations for thorough testing
PROPTEST_CASES=10000 cargo test proptests
```

## Static Analysis

### `CodeQL`

We use GitHub's `CodeQL` for static analysis of the codebase:

- Detects potential security vulnerabilities
- Identifies code quality issues
- Runs on every commit and PR
- Results available in GitHub Security tab

Configuration: `.github/workflows/codeql.yaml`

### `Clippy`

Rust's official linter runs in CI with warnings-as-errors:

```bash
cargo clippy -- -D warnings
```

## Dependency Security

### Cargo Audit

We use `cargo-audit` to check for known security vulnerabilities in dependencies:

```bash
cargo audit --file Cargo.lock
```

This runs:

- On every push
- Twice monthly on scheduled basis
- When dependencies change

Configuration: `.github/workflows/audit.yaml`

## Security Best Practices

### Development Workflow

1. **Before submitting PRs:**
   - Run `cargo test` (includes property tests)
   - Run `cargo clippy -- -D warnings`
   - For library changes, run fuzz tests locally
   - Run `cargo audit` if dependencies changed

2. **During review:**
   - CI runs all security tests automatically
   - `CodeQL` analyzes code changes
   - Fuzzing validates robustness

3. **After merge:**
   - Weekly fuzzing continues
   - Scheduled dependency audits
   - `CodeQL` monitors for regressions

### Reporting Security Issues

If you discover a security vulnerability:

1. **DO NOT** open a public issue
2. Review `SECURITY.md` for reporting guidelines
3. Contact the maintainers privately
4. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Affected versions
   - Potential impact

## Testing Philosophy

### Defense in Depth

We employ multiple complementary testing strategies:

- **Unit tests** - Verify individual components
- **Integration tests** - Ensure components work together
- **Property tests** - Validate invariants across inputs
- **Fuzz tests** - Discover edge cases and crashes
- **Static analysis** - Catch potential issues before runtime

### Safety Guarantees

The project enforces:

```toml
[lints.rust]
unsafe_code = "forbid"
```

This means:

- No unsafe Rust code allowed
- All memory safety guaranteed by the compiler
- Reduced attack surface
- Easier security auditing

### Continuous Improvement

We regularly:

- Add new fuzz targets for new functionality
- Expand property test coverage
- Review and update security practices
- Monitor security advisories
- Update dependencies promptly

## Resources

- [Rust Fuzz Book](https://rust-fuzz.github.io/book/)
- [cargo-fuzz Documentation](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [`proptest` Documentation](https://proptest-rs.github.io/proptest/)
- [`CodeQL` for Rust](https://codeql.github.com/docs/codeql-language-guides/codeql-for-rust/)
- [Secure Rust Guidelines](https://anssi-fr.github.io/rust-guide/)

## Contributing

When adding new functionality:

1. Add appropriate unit tests
2. Consider property-based tests for complex logic
3. Add fuzz targets for user-facing APIs
4. Document security considerations
5. Run all tests locally before submitting PR

See `CONTRIBUTING.md` for detailed contribution guidelines.
