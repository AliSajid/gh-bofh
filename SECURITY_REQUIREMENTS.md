<!--
SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Security requirements and expectations

This document describes the security expectations for the `gh-bofh` project and the mandatory security requirements the project aims to satisfy. It is written for users, contributors, and maintainers so they can understand the project's assumptions and responsibilities.

## Scope

This project is a small, local command-line tool that prints randomly selected BOFH-style excuses. The scope of the security requirements is limited to the behavior of the software itself (local execution, CLI input handling, and dependency management).

## What users can expect

- No sensitive data handling: `gh-bofh` does not collect, store, or transmit personal or sensitive data. All operations occur locally on the user's machine.
- No network communications: the tool does not perform network I/O as part of normal operation, so it does not introduce remote communication risks.
- Transparent source code: the project is open-source and the codebase can be inspected on GitHub for verification and review.
- Minimal dependencies: the implementation uses a small, well-known set of crates (for example, `clap` for CLI parsing and `rand` for randomness), limiting the third-party attack surface.

## What users cannot expect

- No formal security guarantees: the project is provided "as-is" and there are no explicit warranties that the software is free from vulnerabilities.
- No built-in protections against intentional misuse: beyond standard CLI argument parsing, the tool does not include advanced input sanitization or misuse prevention mechanisms.
- No guaranteed long-term security support: security fixes are made on a best-effort basis and there is no formal long-term support commitment.

## Security requirements (must-haves)

The project is intended to meet the following core security requirements:

1. No data collection: the software must not collect, persist, or transmit user data.
2. Local execution only: the software must operate entirely on the local machine with no network communication required for normal operation.
3. Minimal attack surface: the code should use only essential dependencies and avoid unnecessary features that increase risk.
4. Transparency: source code and documentation should be available so users and reviewers can inspect the implementation and verify behavior.

## Rationale and justification

- Simplicity reduces risk: a small, focused codebase is less likely to contain complex vulnerabilities.
- Limited scope reduces exposure: because the tool is local and does not handle sensitive data, common threat vectors (network, data leakage) are largely out of scope.
- Open-source review: public code enables community review and discovery of potential issues.
- Relying on well-maintained crates reduces dependency risk: using widely adopted libraries such as `clap` and `rand` lowers the likelihood of introducing obscure vulnerabilities via dependencies.

## Related documents

See also the repository's [`SECURITY.md`](SECURITY.md) (reporting and disclosure process) and [`SECURITY_ASSURANCE.md`](SECURITY_ASSURANCE.md) (assurance case and verification checklist) for additional context and operational guidance.
