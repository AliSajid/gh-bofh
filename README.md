
# Gh Bofh Rs

![GitHub release (latest by date)](https://img.shields.io/github/v/release/AliSajid/gh-bofh-rs)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/AliSajid/gh-bofh-rs)
[![Continuous integration](https://github.com/AliSajid/gh-bofh-rs/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/AliSajid/gh-bofh-rs/actions/workflows/ci.yaml)
[![Contribute with Gitpod](https://img.shields.io/badge/Contribute%20with-Gitpod-908a85?logo=gitpod)](https://gitpod.io/#AliSajid/gh-bofh-rs)


Generate a BOFH Excuse for github from the commandline as a `gh` extension.

## Builds

| Platform | Rust Version |Status |
| -------- | ------ | ------ |
| Linux    | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-stable.json) <br/> ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-beta.json) <br/> ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-nightly.json) <br/> ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/ubuntu-msrv.json) |
| Windows  | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![macos x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-stable.json) <br/> ![macos x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-beta.json) <br/> ![macos x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-nightly.json) <br/> ![macos x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/windows-msrv.json) |
| macOS    | stable <br/> beta <br/> nightly <br/> MSRV (1.64.0) | ![Windows x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-stable.json) <br/> ![Windows x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-beta.json) <br/> ![Windows x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-nightly.json) <br/> ![Windows x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/68f75dde24f65f2a9333a4ca3d38c82b/raw/macos-msrv.json) |

## Installation

### From Source

```bash

cargo install gh-bofh-rs

```

### From Release

Download the latest release from [here](https://github.com/AliSajid/gh-bofh-rs/releases/latest) and install it using `gh extension install <path-to-release>`

### From The GitHub CLI

```bash
gh extension install AliSajid/gh-bofh-rs
```

## Usage

```bash
gh bofh
```

You can switch between the different types of excuses using the `--type` flag.

```bash

gh bofh --type [modern|classic]

```

You can also use them as standalone flags.

```bash

gh bofh --modern
gh bofh --classic

```


## Current Status

This project is currently being maintained. We are happy to include more modern excuses. Please see [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## Contributing

If you want to contribute to this project, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file.

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#AliSajid/gh-bofh-rs)

## License

This project is dual-licensed under the [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE) licenses.
