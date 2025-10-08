#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

#MISE description="Generate the cargo-about files"
#MISE dir="{{cwd}}"

"./scripts/generate_about_json.sh"
"./scripts/generate_about_md.sh" "./meta/licenses.hbs"
