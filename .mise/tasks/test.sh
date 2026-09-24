#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

#MISE description="Run all the tests locally"
#MISE alias="ta"
#MISE sources=["Cargo.toml", "src/**/*.rs"]
#MISE outputs=["target/debug/gh-bofh*"]
#MISE depends=["clean", "build"]

cargo nextest run
