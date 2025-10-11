<!--
SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Security requirements and expectations

This document describes the security expectations for the `gh-bofh` project and the mandatory security requirements the project aims to satisfy. It is written for users, contributors, and maintainers so they can understand the project's assumptions and responsibilities.

## Scope

This project is a small, local command-line tool that prints randomly selected BOFH-style excuses. The scope of the security requirements is limited to the behavior of the software itself:

- Local execution with no network or filesystem I/O during normal operation.
- Input surface limited to command-line arguments and an optional environment variable `EXCUSE_TYPE` that mirrors the `--type` flag.
- Dependency management and build policies (MSRV and linting) that constrain implementation choices.

## What users can expect

- No sensitive data handling: `gh-bofh` does not collect, store, or transmit personal or sensitive data. All operations occur locally on the user's machine.
- No network or filesystem I/O: the tool does not perform network requests or read/write files as part of normal operation; output is printed to stdout.
- Transparent source code: the project is open-source and the codebase can be inspected on GitHub for verification and review.
- Minimal dependencies: the implementation uses a small, well-known set of crates (for example, `clap` for CLI parsing and `rand` for randomness), limiting the third-party attack surface.

## What users cannot expect

- No formal security guarantees: the project is provided "as-is" and there are no explicit warranties that the software is free from vulnerabilities.
- No built-in protections against intentional misuse: beyond standard CLI argument parsing, the tool does not include advanced input sanitization or misuse prevention mechanisms.
- No guaranteed long-term security support: security fixes are made on a best-effort basis and there is no formal long-term support commitment.

## Security requirements (must-haves)

The project is intended to meet the following core security requirements:

1. No data collection: the software must not collect, persist, or transmit user data.
2. Local execution only: the software must operate entirely on the local machine with no network or filesystem I/O required for normal operation.
3. Minimal attack surface: the code should use only essential dependencies and avoid unnecessary features that increase risk.
4. Transparency: source code and documentation should be available so users and reviewers can inspect the implementation and verify behavior.
5. Implementation constraints: maintain MSRV at 1.85.1 and forbid `unsafe_code`; keep examples valid via rustdoc lints.

## Rationale and justification

- Simplicity reduces risk: a small, focused codebase with no network or filesystem I/O is less likely to contain complex vulnerabilities.
- Limited scope reduces exposure: because the tool is local and does not handle sensitive data, common threat vectors (network, data leakage) are largely out of scope.
- Open-source review: public code enables community review and discovery of potential issues.
- Relying on well-maintained crates reduces dependency risk: using widely adopted libraries such as `clap` and `rand` lowers the likelihood of introducing obscure vulnerabilities via dependencies.
- Explicit implementation constraints (MSRV, `unsafe_code = forbid`, rustdoc lints) keep the codebase within a safer subset of Rust and catch issues early in CI.

## Threats out of scope

Given the project's design and limited scope, the following threats are considered out of scope:

- Network-based threats (no network I/O occurs during normal operation).
- Data breaches or leakage through the app (no data is collected or stored).
- Filesystem compromise via application behavior (no files are read/written in normal use).
- Physical access or OS-level compromise of the host running the tool.
- Broader software supply-chain attacks beyond standard dependency hygiene and audits.

## Related documents

See also the repository's [`SECURITY.md`](SECURITY.md) (reporting and disclosure process) and [`SECURITY_ASSURANCE.md`](SECURITY_ASSURANCE.md) (assurance case and verification checklist) for additional context and operational guidance.

Related design reference:

- [`ARCHITECTURE.md`](ARCHITECTURE.md) — components, data/control flows, and trust boundaries including the CLI/environment inputs and stdout-only output.

Related documents:

- [`SECURITY.md`](SECURITY.md) — reporting and disclosure process.
- [`SECURITY_ASSURANCE.md`](SECURITY_ASSURANCE.md) — assurance case and verification checklist.
