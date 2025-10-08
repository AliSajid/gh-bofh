<!--
SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Security Policy

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

## Related documents

For additional context and operational guidance, see:

- `SECURITY_REQUIREMENTS.md` — security requirements and expectations for the project.
- `SECURITY_ASSURANCE.md` — assurance case and verification checklist describing how security requirements are met and verified.

Maintainers regularly run `cargo audit` against `Cargo.lock` to detect known vulnerable dependency versions. If your report relates to a dependency advisory, please include any relevant `cargo-audit` output (text or JSON) to help triage the issue more quickly.
