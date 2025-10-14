<!--
SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Security Policy

## Scope and assumptions

`gh-bofh` is a small local CLI that prints a randomly selected excuse. Its runtime behavior is intentionally minimal:

- Input surface: command-line arguments and a single environment variable `EXCUSE_TYPE` (mirrors `--type`).
- Output only to stdout; no network I/O and no filesystem writes as part of normal operation.
- Implementation constraints: MSRV is 1.85.1 and `unsafe_code` is forbidden; strict rustdoc lints keep examples accurate.

For a high-level view of components and trust boundaries, see `ARCHITECTURE.md`.

## Supported versions

The canonical release for `gh-bofh` is the most recent version published from the `main` branch. For security support we will also maintain the current major release with patches and fixes as required.

| Version | Supported          |
| ------- | ------------------ |
| v1.x    | :white_check_mark: |
| v0.x    | :x:                |

## Reporting a vulnerability

If you discover a security vulnerability or other issue that could affect users, please report it privately and include as much relevant detail as possible. Do not publicly disclose the issue until a fix has been made available.

To report a vulnerability, email [security@imamiland.com](mailto:security@imamiland.com) and provide the following information when available:

- Steps to reproduce the issue.
- A clear description of the security impact.
- Any suggested fixes or mitigations, if you have them.

You should receive an acknowledgement of your report within 48 hours.

After assessment, we will provide a timeline for remediation and release. Security patches will be backported to the supported major release(s) when appropriate.

## Vulnerability disclosure timeline

- Critical vulnerabilities: our goal is to resolve these within **7 days** of reporting.
- Non-critical vulnerabilities: these will be addressed during the normal development cycle and included in future updates.

## Threats out of scope

The following threats are out of scope for `gh-bofh` based on its architecture and intended use:

- Network-based attacks (no network I/O is performed during normal operation).
- Data exfiltration or leakage (the tool does not collect or store user data).
- Filesystem tampering via the application (no files are read/written as part of normal use).
- Host/OS compromise, physical access, or malicious local actors.
- Supply-chain attacks beyond our dependency management and auditing practices.

## Related documents

For additional context and operational guidance, see:

- `SECURITY_REQUIREMENTS.md` — security requirements and expectations for the project.
- `SECURITY_ASSURANCE.md` — assurance case and verification checklist describing how security requirements are met and verified.
- `ARCHITECTURE.md` — high-level design, component boundaries, and data/control flows.

Maintainers regularly run `cargo audit` against `Cargo.lock` to detect known vulnerable dependency versions. If your report relates to a dependency advisory, please include any relevant `cargo-audit` output (text or JSON) to help triage the issue more quickly.
