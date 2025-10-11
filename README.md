<!--
SPDX-FileCopyrightText: 2024 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# A `gh` plugin to generate BOFH excuses

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/AliSajid/gh-bofh)](https://github.com/AliSajid/gh-bofh/releases/latest)
[![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/AliSajid/gh-bofh)](https://github.com/AliSajid/gh-bofh/releases/latest)
[![Crates.io Version](https://img.shields.io/crates/v/gh-bofh)](https://crates.io/crates/gh-bofh)
[![Crates.io Size](https://img.shields.io/crates/size/gh-bofh)](https://crates.io/crates/gh-bofh)
![Maintenance](https://img.shields.io/maintenance/yes/2025)
![OSS Lifecycle](https://img.shields.io/osslifecycle?file_url=https%3A%2F%2Fraw.githubusercontent.com%2FAliSajid%2Fgh-bofh%2Fmain%2FOSSMETADATA)

[![Continuous integration](https://github.com/AliSajid/gh-bofh/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/AliSajid/gh-bofh/actions/workflows/ci.yaml)
[![codecov](https://codecov.io/gh/AliSajid/gh-bofh/graph/badge.svg?token=rrsVYywjlx)](https://codecov.io/gh/AliSajid/gh-bofh)
[![Security Audit](https://github.com/AliSajid/gh-bofh/actions/workflows/audit.yaml/badge.svg?branch=main)](https://github.com/AliSajid/gh-bofh/actions/workflows/audit.yaml)
[![GitHub issues](https://img.shields.io/github/issues/AliSajid/gh-bofh)](https://github.com/AliSajid/gh-bofh/issues)

[![Crates.io License](https://img.shields.io/crates/l/gh-bofh)](https://crates.io/crates/gh-bofh)
[![REUSE Compliance](https://img.shields.io/reuse/compliance/github.com%2FAliSajid%2Fgh-bofh)](https://api.reuse.software/info/github.com/AliSajid/gh-bofh)
[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/9466/badge)](https://www.bestpractices.dev/projects/9466)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/AliSajid/gh-bofh/badge)](https://scorecard.dev/viewer/?uri=github.com/AliSajid/gh-bofh)

[![Libraries.io SourceRank](https://img.shields.io/librariesio/sourcerank/cargo/gh-bofh)](https://libraries.io/cargo/gh-bofh)

Generate a BOFH Excuse for github-cli from the command line as a `gh` extension.

## Builds

|         | Stable                                                                                                                                                             | Beta                                                                                                                                                           | Nightly                                                                                                                                                              | MSRV (1.85.1)                                                                                                                                                  |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Linux   | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-stable.json)   | ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-beta.json)   | ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-nightly.json)   | ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-msrv.json)   |
| Windows | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-stable.json) | ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-beta.json) | ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-nightly.json) | ![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-msrv.json) |
| macos   | ![macos x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-stable.json)     | ![macos x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-beta.json)     | ![macos x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-nightly.json)     | ![macos x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-msrv.json)     |

## Quick start

Install and run the `gh` extension in a few seconds.

From source (recommended for contributors):

```bash
cargo install gh-bofh
```

Install from a release (download the release archive and then):

```bash
gh extension install <path-to-release>
```

Install directly from GitHub via the GitHub CLI:

```bash
gh extension install AliSajid/gh-bofh
```

Run it:

```bash
gh bofh
```

If you aren't using the `gh` wrapper, the binary can also be invoked directly after building with `cargo run --bin gh-bofh`.

## What it does

This project provides a tiny CLI that prints a random "BOFH" (Bastard Operator From Hell) style excuse. It ships two flavors:

- classic — 90s era, old-school excuses
- modern  — up-to-date, contemporary-themed excuses

The main use-case is as a fun `gh` extension, but the binary is standalone and suitable for piping into other tools.

## Usage and options

Basic usage:

```bash
gh bofh
```

Flags and environment variables:

- -t, --type <TYPE>
  - Choose the excuse type. Allowed values: `classic`, `modern`. Default: `classic`.
  - Can be set via the `EXCUSE_TYPE` environment variable (e.g. `EXCUSE_TYPE=modern`).
- -c, --classic
  - Short flag to request a classic excuse.
- -m, --modern
  - Short flag to request a modern excuse.

Behavior notes:

- Short flags (`-c`/`-m`) take precedence over the `--type` value.
- If neither flag nor env var is provided, the default is `classic`.

Examples:

```bash
# print a classic excuse (default)
gh bofh

# print a modern excuse
gh bofh --type modern
gh bofh --modern

# set via environment
EXCUSE_TYPE=modern gh bofh
```

## Integration with the GitHub CLI

This repository is designed to work as a `gh` extension. After installation with `gh extension install`, the command is available as `gh bofh`.

If you plan to publish your own fork or distribute a release archive, include the compiled binary named `gh-bofh` in the top-level of the release archive so `gh extension install` can detect it.

## Development

The project is a Cargo workspace with the main binary in `src/gh_bofh` and a library crate `gh_bofh_lib`.

Build the project locally:

```bash
# build the workspace in release mode
cargo build --release

# run the binary directly
cargo run --bin gh-bofh --release -- --help
```

Run tests:

```bash
cargo test
```

Formatting and linting:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

Release checklist (maintainers):

1. Bump version in `Cargo.toml`.
2. Run tests and CI locally (where possible).
3. Build release artifacts for target platforms.
4. Create a GitHub release and attach archives containing the `gh-bofh` binary.

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for the contribution process, coding conventions, commit message guidelines, and how to add new excuses.

If you're adding excuses, prefer the pattern used in `src/gh_bofh_lib/excuses.rs` and add unit tests to `tests/` that exercise any parsing or selection logic.

## Troubleshooting

- If `gh` doesn't detect the extension after installing a release archive: ensure the archive has the binary `gh-bofh` at its root and that the binary is executable.
- If `cargo install gh-bofh` fails: ensure you have a compatible Rust toolchain installed (see project MSRV in the builds matrix). You can use `rustup` to install the required toolchain.

## Detailed documentation

High level architecture and design notes: [ARCHITECTURE.md](ARCHITECTURE.md)

Security requirements and expectations: [SECURITY_REQUIREMENTS.md](SECURITY_REQUIREMENTS.md)

Security assurance artifacts: [SECURITY_ASSURANCE.md](SECURITY_ASSURANCE.md)

For contributor-facing policies and code of conduct, see [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Reporting issues

Open issues at: [https://github.com/AliSajid/gh-bofh/issues](https://github.com/AliSajid/gh-bofh/issues)

When filing a bug report include:

- The command you ran and the output.
- Your OS and Rust version (if building from source).
- Any relevant backtrace or panic output.

## License

This project is dual-licensed under the [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE) licenses.

## Acknowledgements and links

- Project home: [https://github.com/AliSajid/gh-bofh](https://github.com/AliSajid/gh-bofh)
- Releases: [https://github.com/AliSajid/gh-bofh/releases](https://github.com/AliSajid/gh-bofh/releases)

Enjoy responsibly — it's intended for harmless fun.
