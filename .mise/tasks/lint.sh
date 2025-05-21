#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

#MISE description="Lint the codebase"

cargo clippy -- -D warnings
